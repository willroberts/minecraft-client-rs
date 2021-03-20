use minecraft_client_rs::Client;

#[cfg(test)]
mod tests {
	use super::*;

	const DEFAULT_HOSTPORT: &str = "127.0.0.1:25575";
	const DEFAULT_PASSWORD: &str = "minecraft";
	
	#[test]
	fn test_authenticate() {
		let mut client = Client::new(DEFAULT_HOSTPORT.to_string()).unwrap();
		match client.authenticate(DEFAULT_PASSWORD.to_string()) {
			Ok(_) => {
				client.close().unwrap();
			},
			Err(_) => {
				client.close().unwrap();
				panic!("failed to authenticate");
			},
		}
	}
}