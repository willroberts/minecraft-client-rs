use std::io::prelude::*;
use std::net::{Shutdown, TcpStream};
use std::sync::atomic::{AtomicI32, Ordering};

use crate::message;

const MAX_MESSAGE_SIZE: usize = 4110; // https://wiki.vg/Rcon#Fragmentation

pub struct Client {
	conn: TcpStream,
	last_id: AtomicI32,
}

impl Client {
	pub fn new(hostport: String) -> Client {
		Client{
			conn: TcpStream::connect(hostport).unwrap(),
			last_id: AtomicI32::new(0),
		}
	}

	pub fn close(&mut self) {
		self.conn.shutdown(Shutdown::Both).unwrap();
	}

	pub fn authenticate(&mut self, password: String) -> message::Message {
		self.send_message(message::MessageType::Authenticate as i32, password)
	}

	pub fn send_command(&mut self, command: String) -> message::Message {
		self.send_message(message::MessageType::Command as i32, command)
	}

	fn next_id(&self) -> i32 {
		let prev = self.last_id.load(Ordering::Relaxed);
		let next = prev + 1;
		self.last_id.store(next, Ordering::Relaxed);
		next
	}

	fn send_message(&mut self, msg_type: i32, msg_body: String) -> message::Message {
		let req_id = self.next_id();
		let req = message::Message{
			size: msg_body.len() as i32 + message::HEADER_SIZE,
			id: req_id.clone(),
			msg_type: msg_type,
			body: msg_body,
		};

		self.conn.write_all(&message::encode_message(req)[..]).unwrap();
		let mut resp_bytes = [0u8; MAX_MESSAGE_SIZE];
		self.conn.read(&mut resp_bytes).unwrap();
		let resp = message::decode_message(resp_bytes.to_vec());

		assert_eq!(req_id, resp.id);

		resp
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const DEFAULT_HOSTPORT: &str = "127.0.0.1:25575";
	const DEFAULT_PASSWORD: &str = "minecraft";

	#[test]
	fn test_id_generation() {
		let mut client = Client::new(DEFAULT_HOSTPORT.to_string());
		assert_eq!(client.next_id(), 1);
		assert_eq!(client.next_id(), 2);
		assert_eq!(client.next_id(), 3);
		client.close();
	}
	
	#[test]
	fn test_send_message() {
		let mut client = Client::new(DEFAULT_HOSTPORT.to_string());
		client.send_message(message::MessageType::Authenticate as i32, DEFAULT_PASSWORD.to_string());
		client.close();
	}
}