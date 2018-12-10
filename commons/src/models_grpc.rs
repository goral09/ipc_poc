// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait PingService {
    fn send_ping(&self, o: ::grpc::RequestOptions, p: super::models::Ping) -> ::grpc::SingleResponse<super::models::Pong>;
}

// client

pub struct PingServiceClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_SendPing: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::models::Ping, super::models::Pong>>,
}

impl ::grpc::ClientStub for PingServiceClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        PingServiceClient {
            grpc_client: grpc_client,
            method_SendPing: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/PingService/SendPing".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl PingService for PingServiceClient {
    fn send_ping(&self, o: ::grpc::RequestOptions, p: super::models::Ping) -> ::grpc::SingleResponse<super::models::Pong> {
        self.grpc_client.call_unary(o, p, self.method_SendPing.clone())
    }
}

// server

pub struct PingServiceServer;


impl PingServiceServer {
    pub fn new_service_def<H : PingService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/PingService",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/PingService/SendPing".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.send_ping(o, p))
                    },
                ),
            ],
        )
    }
}

// interface

pub trait RetrieveService {
    fn get_user_info(&self, o: ::grpc::RequestOptions, p: super::models::User) -> ::grpc::SingleResponse<super::models::UserProfile>;

    fn last_user_info(&self, o: ::grpc::RequestOptions, p: super::models::Unit) -> ::grpc::SingleResponse<super::models::UserProfile>;
}

// client

pub struct RetrieveServiceClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_GetUserInfo: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::models::User, super::models::UserProfile>>,
    method_LastUserInfo: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::models::Unit, super::models::UserProfile>>,
}

impl ::grpc::ClientStub for RetrieveServiceClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        RetrieveServiceClient {
            grpc_client: grpc_client,
            method_GetUserInfo: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/RetrieveService/GetUserInfo".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_LastUserInfo: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/RetrieveService/LastUserInfo".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl RetrieveService for RetrieveServiceClient {
    fn get_user_info(&self, o: ::grpc::RequestOptions, p: super::models::User) -> ::grpc::SingleResponse<super::models::UserProfile> {
        self.grpc_client.call_unary(o, p, self.method_GetUserInfo.clone())
    }

    fn last_user_info(&self, o: ::grpc::RequestOptions, p: super::models::Unit) -> ::grpc::SingleResponse<super::models::UserProfile> {
        self.grpc_client.call_unary(o, p, self.method_LastUserInfo.clone())
    }
}

// server

pub struct RetrieveServiceServer;


impl RetrieveServiceServer {
    pub fn new_service_def<H : RetrieveService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/RetrieveService",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/RetrieveService/GetUserInfo".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_user_info(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/RetrieveService/LastUserInfo".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.last_user_info(o, p))
                    },
                ),
            ],
        )
    }
}
