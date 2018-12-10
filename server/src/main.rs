extern crate clap;
extern crate commons;
extern crate futures;
extern crate grpc;
extern crate protobuf;

use clap::{App, Arg};
use commons::models_grpc::RetrieveServiceServer;

pub mod user_retriever;
pub mod users;

fn main() {
    let matches = App::new("rust_server")
        .arg(
            Arg::with_name("socket")
                .required(true)
                .help("Socket address"),
        )
        .arg(Arg::with_name("e").short("e").help("Echo server"))
        .get_matches();

    let socket = matches.value_of("socket").unwrap();

    let socket_path = std::path::Path::new(socket);
    if socket_path.exists() {
        std::fs::remove_file(socket_path).unwrap();
    }

    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_unix_addr(socket.to_owned()).unwrap();

    let users: Vec<_> = vec![
        ("Donald", "Trump"),
        ("Hilary", "Clinton"),
        ("George", "W. Bush"),
    ];
    let user_retriever = user_retriever::UserRetriever::new(users::build_users_map(&users));
    server.add_service(RetrieveServiceServer::new_service_def(user_retriever));
    server.http.set_cpu_pool_threads(1);

    let _server = server.build().expect("Start server");

    loop {
        std::thread::park();
    }
}
