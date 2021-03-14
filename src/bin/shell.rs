use std::io;
use std::io::prelude::*;

use minecraft_client_rs::client::Client;

const DEFAULT_HOSTPORT: &str = "127.0.0.1:25575";
const DEFAULT_PASSWORD: &str = "minecraft";

fn main() {
	let mut client = Client::new(DEFAULT_HOSTPORT.to_string());

	client.authenticate(DEFAULT_PASSWORD.to_string());

	println!("Starting RCON shell. Use 'exit', 'quit', or Ctrl-C to exit.");
	let quit_commands = vec!("exit", "quit");
	loop {
		print!("> ");
		io::stdout().flush().unwrap();

		let mut command = String::new();
		io::stdin().read_line(&mut command).unwrap();
		command.pop(); command.pop(); // Remove trailing newline (\r\n).

		if quit_commands.contains(&&command[..]) {
			break;
		}

		let resp = client.send_command(command);
		println!("{}", resp.body);
	}

	client.close();
}