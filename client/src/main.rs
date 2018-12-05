extern crate clap;
extern crate commons;

use clap::{App, Arg};
use commons::models::Person;
use commons::networking::*;
use std::io::prelude::*;
use std::os::unix::net::UnixStream;

fn main() {
    let matches = App::new("rust_client")
        .arg(Arg::with_name("socket").required(true))
        .arg(Arg::with_name("name").required(true).takes_value(true))
        .arg(Arg::with_name("age").required(true).takes_value(true))
        .get_matches();

    let socket = matches.value_of("socket").unwrap();
    let name = matches.value_of("name").unwrap();
    let age: u32 = matches.value_of("age").unwrap().parse().unwrap();
    let mut person = Person::new();
    person.age = age;
    person.name = name.to_string();

    let mut stream = match UnixStream::connect(socket) {
        Err(err) => panic!("Failed to connect to socket {}.", err),
        Ok(stream) => stream,
    };

    stream
        .write_all(format!("{:?}", person).as_bytes())
        .unwrap();
    let msg_received = read(&mut stream);
    println!("Received {} from server.", msg_received);

    use std::net::Shutdown;

    stream
        .shutdown(Shutdown::Write)
        .expect("Shutdown function.");
}
