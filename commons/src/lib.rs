extern crate byteorder;
extern crate grpc;
extern crate protobuf;

pub mod models;
pub mod models_grpc;
pub mod networking {
    use byteorder::{ByteOrder, LittleEndian};
    use protobuf::Message;
    use std::io::Read;
    use std::io::Write;
    use std::net::Shutdown;
    use std::os::unix::net::UnixStream;

    pub fn shutdown(stream: &mut UnixStream, mode: Shutdown) {
        stream
            .shutdown(mode)
            .expect("Error when trying to sthudown stream.")
    }

    pub fn serialize<M: Message>(msg: &M) -> Vec<u8> {
        let msg_size = msg.compute_size();
        let mut buf = [0u8; 4];
        LittleEndian::write_u32(&mut buf, msg_size);
        let mut result = Vec::with_capacity((msg_size + 4) as usize);
        result.extend_from_slice(&buf);
        let msg_bytes = msg.write_to_bytes().expect("Serialize message");
        result.extend_from_slice(&msg_bytes);
        result
    }

    pub fn send(bytes: &[u8], stream: &mut UnixStream) {
        let bytes = stream.write(bytes).unwrap();
        println!("Sent {} bytes.", bytes)
    }

    fn msg_size<I: Iterator<Item = Result<u8, Error>>>(iter: &mut I) -> u32 {
        let mut bs = [0u8; 4];
        for i in 0..4 {
            //TODO:: handle errors properly
            bs[i] = iter.next().unwrap().unwrap();
        }
        LittleEndian::read_u32(&bs)
    }

    fn consume_msg<I: Iterator<Item = Result<u8, Error>>>(iter: I, size: usize) -> Vec<u8> {
        iter.take(size).map(|r| r.unwrap()).collect()
    }

    pub fn read(stream: &mut UnixStream) -> Vec<u8> {
        let mut bytes = stream.bytes();
        let size = msg_size(&mut bytes);
        println!("Msg size: {}", size);
        consume_msg(bytes, size as usize)
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
