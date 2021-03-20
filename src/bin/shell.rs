use std::env;
use std::io;
use std::io::prelude::*;
use std::process::exit;

use minecraft_client_rs::Client;
use minecraft_client_rs::Message;

const DEFAULT_HOSTPORT: &str = "127.0.0.1:25575";
const DEFAULT_PASSWORD: &str = "minecraft";

fn main() {
	let mut hostport: &String = &DEFAULT_HOSTPORT.to_string();
	let mut password: &String = &DEFAULT_PASSWORD.to_string();

	// Parse arguments.
	let args: Vec<String> = env::args().collect();
	for i in 1..args.len() {
		match args[i].as_str() {
			"--hostport" => { hostport = &args[i+1]; },
			"--password" => { password = &args[i+1]; },
			_ => {},
		}
	}

	// Connect and authenticate.
	let mut client: Client;
	match Client::new(hostport.to_string()) {
		Ok(c) => { client = c },
		Err(e) => {
			println!("failed to connect to server: {}", e.to_string());
			exit(1);
		}
	}

	match client.authenticate(password.to_string()) {
		Ok(_) => { },
		Err(_) => {
			println!("failed to authenticate");
			disconnect(&mut client);
			exit(1);
		}
	}

	// Start RCON shell.
	println!("Starting RCON shell. Use 'exit', 'quit', or Ctrl-C to exit.");
	let quit_commands = vec!("exit", "quit");
	loop {
		print!("> ");
		match io::stdout().flush() {
			Ok(_) => {},
			Err(e) => {
				println!("failed to write to stdout: {}", e);
				disconnect(&mut client);
				exit(1);
			}
		}

		let mut command = String::new();
		match io::stdin().read_line(&mut command) {
			Ok(_) => {},
			Err(e) => {
				println!("failed to read from stdin: {}", e);
				disconnect(&mut client);
				exit(1);
			}
		}
		command.pop(); command.pop(); // Remove trailing newline (\r\n).

		if quit_commands.contains(&&command[..]) {
			break;
		}

		let resp: Message;
		match client.send_command(command) {
			Ok(msg) => { resp = msg },
			Err(e) => {
				println!("failed to send command: {}", e);
				disconnect(&mut client);
				exit(1);
			}
		}
		println!("{}", resp.body);
	}

	match client.close() {
		Ok(_) => { },
		Err(e) => { println!("failed to disconnect: {}", e.to_string()) }
	}
}

fn disconnect(c: &mut Client) {
	match c.close() {
		Ok(_) => { },
		Err(e) => { println!("failed to disconnect: {}", e.to_string()) }
	}
}