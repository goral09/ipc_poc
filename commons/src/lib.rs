// In order to use the Serialize and Deserialize macros in the model,
// we need to declare in the main module, that we are using them.
#[macro_use]
extern crate serde_derive;
extern crate protobuf;

pub mod models;
pub mod networking {
    use std::io::prelude::*;
    use std::os::unix::net::UnixStream;

    pub fn read(stream: &mut UnixStream) -> String {
        let mut s = Vec::new();
        stream.read_to_end(&mut s).unwrap();
        String::from_utf8(s).unwrap()
    }
}
