extern crate clap;
extern crate commons;
extern crate protobuf;
extern crate byteorder;

use clap::{App, Arg};
use commons::models::Person;
use commons::networking::{connect_or_panic, send, serialize};
use protobuf::Message;
use std::os::unix::net::UnixStream;

fn to_person(name: &str, age: u32) -> Person {
    let mut p = Person::new();
    p.age = age;
    p.name = name.to_string();
    p
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
    let bytes = serialize(&person);

    let mut stream = connect_or_panic(socket);
    send(&bytes, &mut stream);
}
