extern crate protoc_rust_grpc;

fn main() {
    protoc_rust_grpc::run(protoc_rust_grpc::Args {
        out_dir: "../commons/src/",
        input: &["src/protos/models.proto"],
        includes: &["src/protos"],
        rust_protobuf: true, // also generate protobuf messages, not just services
        ..Default::default()
    }).expect("Problem generating protobuf classes");
}
