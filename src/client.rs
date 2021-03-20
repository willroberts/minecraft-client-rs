use std::error::Error;
use std::fmt;
use std::io::prelude::*;
use std::net::{Shutdown, TcpStream};
use std::sync::atomic::{AtomicI32, Ordering};

use crate::message;

const MAX_MESSAGE_SIZE: usize = 4110; // https://wiki.vg/Rcon#Fragmentation

#[derive(Debug)]
struct RequestIDMismatchError;

impl Error for RequestIDMismatchError {}

impl fmt::Display for RequestIDMismatchError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "received error response from server")
	}
}

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

	pub fn authenticate(&mut self, password: String) -> Result<message::Message, Box<dyn Error>> {
		self.send_message(message::MessageType::Authenticate as i32, password)
	}

	pub fn send_command(&mut self, command: String) -> Result<message::Message, Box<dyn Error>> {
		self.send_message(message::MessageType::Command as i32, command)
	}

	fn next_id(&self) -> i32 {
		let prev = self.last_id.load(Ordering::Relaxed);
		let next = prev + 1;
		self.last_id.store(next, Ordering::Relaxed);
		next
	}

	fn send_message(&mut self, msg_type: i32, msg_body: String) -> Result<message::Message, Box<dyn Error>> {
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
		let resp = message::decode_message(resp_bytes.to_vec())?;

		if req_id == resp.id {
			Ok(resp)
		} else {
			Err(Box::new(RequestIDMismatchError))
		}
	}
}