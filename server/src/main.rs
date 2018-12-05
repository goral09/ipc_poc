extern crate clap;
extern crate commons;
extern crate libc;
extern crate protobuf;
extern crate serde;

use commons::models::Person;
use commons::networking::*;

use clap::{App, Arg};
use std::io::prelude::*;
use std::io::Error;
use std::os::unix::net::{UnixListener, UnixStream};

pub struct Args<'a> {
    pub socket: &'a str,
    pub echo: bool,
}

pub unsafe fn fork<F: FnOnce()>(child_func: F) -> libc::pid_t {
    match libc::fork() {
        -1 => panic!("Fork failed: {}", Error::last_os_error()),
        0 => {
            child_func();
            libc::exit(0);
        }
        pid => pid,
    }
}

fn main() {
    let matches = App::new("rust_server")
        .arg(
            Arg::with_name("socket")
                .required(true)
                .help("Socket address"),
        ).arg(Arg::with_name("e").short("e").help("Echo server"))
        .get_matches();

    let socket = matches.value_of("socket").unwrap();
    let echo = match matches.occurrences_of("e") {
        0 => false,
        _ => true,
    };
    let args = Args { socket, echo };

    let listener = match UnixListener::bind(args.socket) {
        Err(err) => panic!("Failed to bind to socket: {}.", err),
        Ok(stream) => stream,
    };

    println!("Listening on `{}`. Is echo? {}.", args.socket, args.echo);

    for mut stream in listener.incoming() {
        match stream {
            Ok(ref mut stream) => {
                println!("New connection.");
                let msg = read(stream);
                println!("Client said: {}", msg);
                if args.echo {
                    stream.write_all(msg.as_bytes()).expect("Echo");
                    println!("Sending echo back");
                }
            }
            Err(err) => panic!("Error occured when listening from the stream. {}", err),
        }
    }
}
