# minecraft-client-rs

[![Crates.io Badge]][Crates.io]
[![docs.rs Badge]][docs.rs]
[![Travis Badge]][Travis]
[![License Badge]][License]

A client for the Minecraft RCON protocol.

## Library Usage

```rust
// Create a new client and connect to the server.
let mut client = Client::new("127.0.0.1:25575".to_string()).unwrap();

// Send some commands.
match client.authenticate("password".to_string()) {
	Ok(_) => { },
	Err(e) => { /* handle authentication error */ },
}
match client.send_command("seed".to_string()) {
	Ok(resp) => { println!("{}", resp.body); }, // "Seed: [1871644822592853811]"
	Err(e) => { /* handle error */ },
}

// Disconnect cleanly when finished.
client.close().unwrap();
```

## Shell Utility

If you are looking for a tool rather than a library, try the shell command:

```bash
$ cargo run -- --hostport 127.0.0.1:25575 --password minecraft
Starting RCON shell. Use 'exit', 'quit', or Ctrl-C to exit.
> list
There are 0 of a max of 20 players online:
> seed
Seed: [1871644822592853811]
```

## Limitations

Response bodies over 4KB will be truncated.

## Starting a server for testing

```
$ docker pull itzg/minecraft-server
$ docker run --name=minecraft-server -p 25575:25575 -d -e EULA=TRUE itzg/minecraft-server
```

## Running Tests

To run unit tests:

```
$ cargo test --lib
```

To run integration tests after starting the test server in Docker:

```
$ cargo test
```

## Reference

- https://wiki.vg/Rcon

[Crates.io]: https://crates.io/crates/minecraft-client-rs
[Crates.io Badge]: https://img.shields.io/badge/crates.io-v0.1.0-orange
[docs.rs]: https://docs.rs/minecraft-client-rs/0.1.0/minecraft_client_rs/
[docs.rs Badge]: https://docs.rs/minecraft-client-rs/badge.svg?version=0.1.0
[Travis]: https://travis-ci.org/willroberts/minecraft-client-rs
[Travis Badge]: https://api.travis-ci.org/willroberts/minecraft-client-rs.svg?branch=main
[License]: https://www.gnu.org/licenses/gpl-3.0
[License Badge]: https://img.shields.io/badge/License-GPLv3-blue.svg