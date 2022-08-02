#[cfg(feature = "cli")]
mod cli {
    use clap::{ArgGroup, Parser};
    use minecraft_client_rs::Client;
    use regex::Regex;
    use std::io;
    use std::io::prelude::*;
    use std::process::exit;

    const DEFAULT_HOSTPORT: &str = "127.0.0.1:25575";
    const DEFAULT_PASSWORD: &str = "minecraft";

    #[derive(Parser)]
    #[clap(author, version, about, long_about = None)]
    #[clap(propagate_version = true)]
    #[clap(group(
	ArgGroup::new("mode")
		.required(true)
		.args(&["single-command", "interactive"]),
    ))]
    struct Args {
        /// Starts an interactive shell.
        #[clap(short, long, default_value_t = false, action)]
        interactive: bool,

        /// Only send an individual command.
        #[clap(value_parser)]
        single_command: Option<String>,

        /// The host and port of the RCON server.
        #[clap(short, long, default_value_t = String::from(DEFAULT_HOSTPORT), value_parser)]
        hostport: String,

        /// The password for RCON.
        #[clap(short, long, default_value_t = String::from(DEFAULT_PASSWORD), value_parser)]
        password: String,

        /// Removes Minecraft color and formatting codes from the output.
        #[clap(short, long, default_value_t = false, action)]
        keep_formatting: bool,
    }

    pub fn start() {
        let args = Args::parse();

        let hostport = &args.hostport;
        let password = &args.password;

        // Connect and authenticate.
        let mut client: Client;

        match Client::new(hostport.to_string()) {
            Ok(c) => client = c,
            Err(e) => {
                println!("failed to connect to server: {}", e.to_string());
                exit(1);
            }
        }

        match client.authenticate(password.to_string()) {
            Ok(_) => {}
            Err(_) => {
                println!("failed to authenticate");
                disconnect_and_exit(&mut client, 1);
            }
        }

        if args.interactive {
            start_interactive_shell(&mut client, args.keep_formatting);
        } else {
            let single_command = args.single_command.unwrap_or_else(|| {
                println!("no command to send!");
                disconnect_and_exit(&mut client, 1);
            });

            send_single_command(&mut client, single_command, args.keep_formatting);
        }
    }

    fn start_interactive_shell(client: &mut Client, keep_formatting: bool) {
        // Start RCON shell.
        println!("Starting RCON shell. Use 'exit', 'quit', or Ctrl-C to exit.");
        let quit_commands = vec!["exit", "quit"];
        loop {
            print!("> ");
            match io::stdout().flush() {
                Ok(_) => {}
                Err(e) => {
                    println!("failed to write to stdout: {}", e);
                    disconnect_and_exit(client, 1);
                }
            }

            let mut command = String::new();

            match io::stdin().read_line(&mut command) {
                Ok(_) => {}
                Err(e) => {
                    println!("failed to read from stdin: {}", e);
                    disconnect_and_exit(client, 1);
                }
            }

            // Remove trailing whitespace
            let command = command.trim().to_string();

            if quit_commands.contains(&&command[..]) {
                disconnect_and_exit(client, 0);
            }

            match client.send_command(command) {
                Ok(msg) => println!(
                    "{}",
                    if keep_formatting {
                        msg.body
                    } else {
                        remove_minecraft_format_codes(msg.body)
                    }
                ),

                Err(e) => {
                    println!("failed to send command: {}", e);
                    disconnect_and_exit(client, 1);
                }
            }
        }
    }

    fn send_single_command(client: &mut Client, command: String, keep_formatting: bool) {
        match client.send_command(command) {
            Ok(msg) => {
                println!(
                    "{}",
                    if keep_formatting {
                        msg.body
                    } else {
                        remove_minecraft_format_codes(msg.body)
                    }
                );

                disconnect_and_exit(client, 0);
            }
            Err(e) => {
                println!("failed to send command: {}", e);
                disconnect_and_exit(client, 1);
            }
        }
    }

    fn disconnect_and_exit(client: &mut Client, exit_code: i32) -> ! {
        match client.close() {
            Ok(_) => {
                exit(exit_code);
            }
            Err(e) => {
                println!("failed to disconnect: {}", e.to_string());
                exit(1);
            }
        }
    }

    fn remove_minecraft_format_codes(output: String) -> String {
        let re = Regex::new(r"(§[a-z0-9])").unwrap();
        re.replace_all(output.as_str(), "").into_owned()
    }
}

fn main() {
    #[cfg(feature = "cli")]
    cli::start();
}
