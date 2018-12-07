extern crate clap;
extern crate commons;
extern crate protobuf;
extern crate serde;

use commons::models::Person;
use clap::{App, Arg};
use std::os::unix::net::{UnixListener};
use protobuf::Message;

fn main() {
    let matches = App::new("rust_server")
        .arg(
            Arg::with_name("socket")
                .required(true)
                .help("Socket address"),
        )
        .arg(Arg::with_name("e").short("e").help("Echo server"))
        .get_matches();

    let socket = matches.value_of("socket").unwrap();
    let echo = match matches.occurrences_of("e") {
        0 => false,
        _ => true,
    };

    let socket_path = std::path::Path::new(socket);
    if socket_path.exists() {
        std::fs::remove_file(socket_path).unwrap();
    }

    let unix_listener = UnixListener::bind(&socket_path).unwrap();

    println!("Listening on `{}`. Is echo? {}.", socket, echo);

    for mut stream in unix_listener.incoming() {
        match stream {
            Ok(ref mut stream) => {
                println!("New connection.");
                let msg = protobuf::parse_from_reader::<Person>(stream).unwrap();
                println!("Client said: {:?}", msg);
                msg.write_to_writer(stream).expect("Couldn't write back to client.");
            }
            Err(err) => panic!("Error occured when listening from the stream. {}", err),
        }
    }
}
