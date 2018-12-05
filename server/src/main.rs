// In order to use the Serialize and Deserialize macros in the model,
// we need to declare in the main module, that we are using them.
#[macro_use]
extern crate serde_derive;
extern crate clap;
extern crate ipc_channel;
extern crate libc;
extern crate protobuf;
extern crate serde;

use clap::{App, Arg};
use ipc::IpcOneShotServer;
use ipc_channel::ipc;
use std::io::prelude::*;
use std::io::Error;
use std::os::unix::net::{UnixListener, UnixStream};

pub mod models;

use models::Person;

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

    let (server, server_name) = IpcOneShotServer::<Person>::new(Some(socket)).unwrap();

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

fn read(stream: &mut UnixStream) -> String {
    let mut _vec = [0u8; 4];
    stream.read_exact(&mut _vec).unwrap();
    std::str::from_utf8(&_vec).unwrap().to_string()
}
