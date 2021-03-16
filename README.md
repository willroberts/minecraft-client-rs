# minecraft-client-rs

A client for the Minecraft RCON protocol.

## Library Usage

```rust
// Create a new client and connect to the server
let mut client = Client::new("127.0.0.1:25575".to_string());

// Send some commands.
client.authenticate("minecraft".to_string()).unwrap();
let resp = client.send_command("seed".to_string()).unwrap();
println!("{}", resp.body); // "Seed: [1871644822592853811]"

// Disconnect cleanly when finished
client.close();
```

## Shell Utility

If you are looking for a tool rather than a library, try the shell command in
`bin/shell.rs`:

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

After starting the test server in Docker:

```
$ cargo test
```

## Reference

- https://wiki.vg/Rcon