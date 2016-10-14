extern crate protobuf;
extern crate byteorder;

mod message;

use std::io::prelude::*;
use std::net::{TcpListener};

use message::Node;

use byteorder::{LittleEndian, ByteOrder};

fn main() {

	let listener = TcpListener::bind("127.0.0.1:6565").unwrap();
    println!("Listening on 127.0.0.1:6565");

    let (mut stream, _) = listener.accept().unwrap();

    loop {
    	let mut buffer = [0; 4];

    	stream.read_exact(&mut buffer).unwrap();

    	// let buffer = Cursor::new(buffer);

    	let len = LittleEndian::read_u32(&buffer);

    	// stream

    	let mut buf: Vec<u8> = Vec::new();
    	buf.resize(len as usize, 0);

    	let mut buf = buf.into_boxed_slice();

    	stream.read_exact(&mut buf).unwrap();

    	// println!("{:?}", buf);

    	let node = protobuf::core::parse_from_bytes::<Node>(&buf).unwrap();

    	println!("{:?}", node);

    	if node.get_field_type() == message::Node_MsgType::DONE {
    		break;
    	}
    }

    drop(listener);
}
