// In order to use the Serialize and Deserialize macros in the model,
// we need to declare in the main module, that we are using them.
#[macro_use]
extern crate serde_derive;
extern crate protobuf;

pub mod models;
pub mod networking {
    use std::io::Read;
    use std::os::unix::net::UnixStream;

    pub fn read(stream: &mut UnixStream) -> String {
        let mut s = Vec::new();
        stream.read_to_end(&mut s).unwrap();
        String::from_utf8(s).unwrap()
    }

    extern crate libc;
    use std::io::Error;
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
    pub fn connect_or_panic(socket: &str) -> UnixStream {
        match UnixStream::connect(socket) {
            Err(err) => panic!("Failed to connect to socket {}.", err),
            Ok(stream) => stream,
        }
    }
}
