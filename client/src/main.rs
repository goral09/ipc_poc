extern crate clap;

use clap::{App, Arg};
use std::io::prelude::*;
use std::os::unix::net::UnixStream;

fn main() {
    let matches = App::new("rust_client")
        .arg(Arg::with_name("socket").required(true))
        .arg(Arg::with_name("message").required(true).takes_value(true))
        .get_matches();

    let socket = matches.value_of("socket").unwrap();
    let message = matches.value_of("message").expect("Message to send.");

    let mut stream = match UnixStream::connect(socket) {
        Err(err) => panic!("Failed to connect to socket {}.", err),
        Ok(stream) => stream,
    };

    write(&mut stream, message);
    let msg_received = read(&mut stream);
    println!("Received {} from server.", msg_received);

    use std::net::Shutdown;

    stream
        .shutdown(Shutdown::Write)
        .expect("Shutdown function.");
}

fn write(stream: &mut UnixStream, message: &str) {
    match stream.write_all(message.as_bytes()) {
        Err(err) => println!("Error when sending message {}", err),
        Ok(_) => {}
    };
}

fn read(stream: &mut UnixStream) -> String {
    let mut _vec = [0u8; 4];
    stream.read_exact(&mut _vec).unwrap();
    std::str::from_utf8(&_vec).unwrap().to_string()
}
