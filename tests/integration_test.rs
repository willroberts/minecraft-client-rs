use minecraft_client_rs::Client;
use minecraft_client_rs::message;

#[cfg(test)]
mod tests {
	use super::*;

	const DEFAULT_HOSTPORT: &str = "127.0.0.1:25575";
	const DEFAULT_PASSWORD: &str = "minecraft";
	
	#[test]
	fn test_authenticate() {
		let mut client = Client::new(DEFAULT_HOSTPORT.to_string());
		match client.authenticate(DEFAULT_PASSWORD.to_string()) {
			Ok(_) => {
				client.close();
			},
			Err(e) => {
				client.close();
				panic!("failed to authenticate");
			},
		}
	}
}