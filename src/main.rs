extern crate byteorder;
extern crate clap;

mod message;


use clap::Arg;

use std::io::prelude::*;
use std::net::TcpListener;
use std::fs::File;
use std::error::Error;

use message::Node;

use byteorder::{LittleEndian, ByteOrder};

fn read_stream(mut stream: std::net::TcpStream, path: &str, debug: bool) {

    let mut file = File::create(path).unwrap();


    loop {

        if debug {
            println!("Press <ENTER> to send next node\n");
            let mut line = String::new();
            std::io::stdin().read_line(&mut line);
        }

        let mut buffer = [0; 4];

        stream.read_exact(&mut buffer).unwrap();

        file.write_all(&buffer).unwrap();

        let len = LittleEndian::read_u32(&buffer);

        let mut buf: Vec<u8> = Vec::new();
        buf.resize(len as usize, 0);
        let mut buf = buf.into_boxed_slice();

        stream.read_exact(&mut buf).unwrap();

        file.write_all(&buf).unwrap();

        let msg = match message::parse_from_bytes(&buf) {
            Ok(v) => v,
            Err(e) => {
                println!("{:?}", e);
                std::process::exit(0);
            }
        };

        if debug {
            println!("{:?}", msg);
        }

        match msg {
            message::Message::NODE {
                ref n_uid,
                ref p_uid,
                ref alt,
                ref kids,
                ref status,
                ref label,
                ref nogood,
                ref info,
            } => {},
            message::Message::START{ref version, ref info} => {
                println!("START");
            },
            _ => {}
        };

        if msg == message::Message::DONE {
            println!("done, break");
            break;
        }

    }

}

fn main() {

    let args = clap::App::new("Search Logger")
        .version("1.0")
        .about("Prints nodes and/or wirtes them to a file.")
        .author("Maxim Shishmarev")
        .arg(Arg::with_name("file")
                 .help("Sets the input file to use")
                 .required(false)
                 .index(1))
        .arg(Arg::with_name("print only")
                 .short("p")
                 .long("print-only")
                 .help("Whether to print only or write to a file as well.")
                 .takes_value(false))
        .arg(Arg::with_name("debug")
                 .short("d")
                 .long("debug")
                 .help("Allows to send nodes one-by-one.")
                 .takes_value(false))
        .get_matches();

    let path = args.value_of("file").unwrap_or("data.log");
    let print_only = args.is_present("print only");
    let debug = args.is_present("debug");

    println!("path: {:?}", path);
    println!("print only: {:?}", debug);

    let listener = match TcpListener::bind("127.0.0.1:6565") {
        Ok(v) => v,
        Err(e) => {
            if e.kind() == std::io::ErrorKind::AddrInUse {
                println!("Port 6565 in use");
            } else {
                println!("{:?}", e.kind());
            }
            std::process::exit(0);
        }
    };
    println!("Listening on 127.0.0.1:6565");

    let (stream, _) = listener.accept().unwrap();

    read_stream(stream, path, debug);

    drop(listener);
}
