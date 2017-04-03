extern crate protobuf;
extern crate byteorder;

mod message;

use std::io::prelude::*;
use std::net::{TcpListener};
use std::fs::File;

use message::Node;

use byteorder::{LittleEndian, ByteOrder};

fn read_stream(mut stream: std::net::TcpStream, path: String) {

	let mut file = File::create(path).unwrap();
	let mut nodes_str = Vec::<String>::new();

	loop {
		let mut buffer = [0; 4];

		stream.read_exact(&mut buffer).unwrap();

		file.write_all(&buffer).unwrap();

		println!("{:?}", buffer);

		let len = LittleEndian::read_u32(&buffer);

		let mut buf: Vec<u8> = Vec::new();
		buf.resize(len as usize, 0);
		let mut buf = buf.into_boxed_slice();

		stream.read_exact(&mut buf).unwrap();

		file.write_all(&buf).unwrap();

		let node = protobuf::core::parse_from_bytes::<Node>(&buf).unwrap();

		println!("{:?}", node);

		let node_str:String = protobuf::text_format::print_to_string(&node);;

		nodes_str.push(node_str);

		if node.get_field_type() == message::Node_MsgType::DONE {
			break;
		}
	}

}

fn main() {

	let mut path = String::from("data.log");
	let args : Vec<String> = std::env::args().collect();

	if args.len() == 1 {
		println!("Using default path: {}", path);
	} else if args.len() != 2 {
		println!("Usage: .\\rust-reader <path>");
		std::process::exit(0);
	} else {
		path = args[1].clone();
		println!("Using path: {}", path);
	}

	let listener = TcpListener::bind("127.0.0.1:6565").unwrap();
	println!("Listening on 127.0.0.1:6565");

	let (stream, _) = listener.accept().unwrap();

	read_stream(stream, path);

	drop(listener);
}
