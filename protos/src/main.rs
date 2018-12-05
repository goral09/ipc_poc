extern crate protobuf_codegen;
extern crate protoc_rust;

pub use protobuf_codegen::Customize;

fn main() {
    protoc_rust::run(protoc_rust::Args {
        out_dir: "../server/src/",
        input: &["src/protos/models.proto"],
        includes: &["src/protos"],
        customize: Customize {
            serde_derive: Some(true),
            ..Default::default()
        },
    }).expect("Problem generating protobuf classes");
}
