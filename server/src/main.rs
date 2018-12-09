extern crate clap;
extern crate commons;
extern crate protobuf;
extern crate serde;

use clap::{App, Arg};
use commons::models::Person;
use commons::networking::{send, shutdown, serialize};
use protobuf::Message;
use std::os::unix::net::UnixListener;

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
                let data = commons::networking::read(stream);
                let msg = protobuf::parse_from_bytes::<Person>(&data).unwrap();
                println!("Client sent: {:?}", msg);
                let bytes = serialize(&msg);
                send(&bytes, stream);
                shutdown(stream, std::net::Shutdown::Both);
            }
            Err(err) => panic!("Error occured when listening from the stream. {}", err),
        }
    }

    std::fs::remove_file(socket_path).unwrap();
}
