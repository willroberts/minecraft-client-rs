use std::convert::TryInto;
use std::str::from_utf8;

const HEADER_SIZE: i32 = 10;

pub enum MessageType {
	Response,
	_Unused,
	Command,
	Authenticate,
}

#[derive(Debug)]
pub struct Message {
	pub size: i32,
	pub id: i32,
	pub msg_type: i32,
	pub body: String,
}

pub fn encode_message(msg: Message) -> Vec<u8> {
	let mut bytes: Vec<u8> = vec!();

	bytes.extend_from_slice(&msg.size.to_le_bytes());
	bytes.extend_from_slice(&msg.id.to_le_bytes());
	bytes.extend_from_slice(&msg.msg_type.to_le_bytes());
	bytes.extend_from_slice(msg.body.as_bytes());
	bytes.extend_from_slice(&[0, 0]);

	bytes
}

pub fn decode_message(bytes: Vec<u8>) -> Message {
	let size = i32::from_le_bytes(bytes[0..4].try_into().expect("invalid message size"));
	let id = i32::from_le_bytes(bytes[4..8].try_into().expect("invalid message id"));
	let msg_type = i32::from_le_bytes(bytes[8..12].try_into().expect("invalid message type"));

	let mut body = "".to_string();
	let body_len: usize = (size - HEADER_SIZE).try_into().expect("invalid message body length");
	if body_len > 0 {
		body = from_utf8(&bytes[12..12+body_len]).unwrap().to_string();
	}

	Message{
		size: size,
		id: id,
		msg_type: msg_type,
		body: body,
	}
}