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
use std::io::Error;
use std::ptr;

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

#[cfg(not(any(
    feature = "force-inprocess",
    target_os = "windows",
    target_os = "android",
    target_os = "ios"
)))]
pub trait Wait {
    fn wait(self);
}

#[cfg(not(any(
    feature = "force-inprocess",
    target_os = "windows",
    target_os = "android",
    target_os = "ios"
)))]
impl Wait for libc::pid_t {
    fn wait(self) {
        unsafe {
            libc::waitpid(self, ptr::null_mut(), 0);
        }
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

    println!("Listening on `{}`. Is echo? {}.", args.socket, args.echo);

    let (server, server_name) = IpcOneShotServer::<Person>::new(Some(socket)).unwrap();
    println!("Server name: {}", server_name);
    let (rec, send) = server.accept().unwrap();

    let pid = unsafe {
        fork(|| {
            for event in rec.recv() {
                println!("{:?}", event);
            }
        })
    };

    pid.wait();
}
