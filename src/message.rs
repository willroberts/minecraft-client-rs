use std::convert::TryInto;
use std::error::Error;
use std::str::from_utf8;

pub const HEADER_SIZE: i32 = 10;

#[repr(i32)]
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

pub fn decode_message(bytes: Vec<u8>) -> Result<Message, Box<dyn Error>> {
	let size = i32::from_le_bytes(bytes[0..4].try_into()?);
	let id = i32::from_le_bytes(bytes[4..8].try_into()?);
	let msg_type = i32::from_le_bytes(bytes[8..12].try_into()?);

	let mut body = "".to_string();
	let body_len: usize = (size - HEADER_SIZE).try_into()?;
	if body_len > 0 {
		let body_bytes = from_utf8(&bytes[12..12+body_len])?;
		body = body_bytes.to_string();
	}

	Ok(Message{
		size: size,
		id: id,
		msg_type: msg_type,
		body: body,
	})
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_encode_message() {
		let bytes = encode_message(Message{
			size: 5 + HEADER_SIZE,
			id: 1,
			msg_type: MessageType::Command as i32,
			body: "hello".to_string(),
		});

		let expected: Vec<u8> = vec!(15, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 104, 101, 108, 108, 111, 0, 0);

		assert_eq!(bytes, expected);
	}

	#[test]
	fn test_decode_message() {
		let bytes: Vec<u8> = vec!(12, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 104, 105);
		let msg = decode_message(bytes).unwrap();

		let expected = Message{
			size: 2 + HEADER_SIZE,
			id: 2,
			msg_type: MessageType::Response as i32,
			body: "hi".to_string(),
		};

		assert_eq!(msg.size, expected.size);
		assert_eq!(msg.id, expected.id);
		assert_eq!(msg.msg_type, expected.msg_type);
		assert_eq!(msg.body, expected.body);
	}
}