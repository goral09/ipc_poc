extern crate clap;
extern crate commons;
extern crate protobuf;

use clap::{App, Arg};
use commons::models::Person;
use commons::networking::{connect_or_panic, shutdown};
use protobuf::Message;
use std::net::Shutdown;
use std::os::unix::net::UnixStream;

fn to_person(name: &str, age: u32) -> Person {
    let mut p = Person::new();
    p.age = age;
    p.name = name.to_string();
    p
}

fn send_and_receive<'a, M: Message + 'a>(msg: &'a M, stream: &mut UnixStream) {
    let mut socket_clone = stream.try_clone().expect("Couldn't clone socket.");
    msg.write_to_writer(stream).unwrap();
    println!("Finished sending...");
    shutdown(stream, Shutdown::Write);
    let msg = protobuf::parse_from_reader::<Person>(&mut socket_clone).unwrap();
    println!("Received {:?}", msg);
    shutdown(stream, Shutdown::Read);
}

fn main() {
    let matches = App::new("rust_client")
        .arg(Arg::with_name("socket").required(true))
        .arg(Arg::with_name("name").required(true).takes_value(true))
        .arg(Arg::with_name("age").required(true).takes_value(true))
        .get_matches();

    let socket = matches.value_of("socket").unwrap();
    let name = matches.value_of("name").unwrap();
    let age: u32 = matches.value_of("age").unwrap().parse().unwrap();
    let person = to_person(name, age);

    let mut stream = connect_or_panic(socket);
    send_and_receive(&person, &mut stream);
}
