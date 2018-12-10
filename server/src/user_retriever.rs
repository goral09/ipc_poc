extern crate futures;
extern crate grpc;
extern crate protobuf;

use commons::models::{User, UserProfile};
use commons::models_grpc::RetrieveService;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct UserRetriever {
    users: HashMap<User, UserProfile>,
    last_user: Arc<Mutex<Option<UserProfile>>>,
}

impl UserRetriever {
    pub fn new(users_map: HashMap<User, UserProfile>) -> UserRetriever {
        UserRetriever {
            users: users_map,
            last_user: Arc::new(Mutex::new(None)),
        }
    }
}

impl RetrieveService for UserRetriever {
    fn get_user_info(
        &self,
        _m: ::grpc::RequestOptions,
        u: commons::models::User,
    ) -> grpc::SingleResponse<UserProfile> {
        println!("/get_user_info {}", u.name);
        let resp = self
            .users
            .get(&u)
            .map(|profile| {
                self.last_user
                    .lock()
                    .map(|mut cell| cell.replace(profile.clone()))
                    .expect("Error when updating last_user stats");
                grpc::SingleResponse::completed(profile.clone())
            })
            .unwrap_or(grpc::SingleResponse::err(grpc::Error::Other(
                "User not found.",
            )));
        println!("Sending response back...");
        resp
    }

    fn last_user_info(
        &self,
        _: ::grpc::RequestOptions,
        _: commons::models::Unit,
    ) -> grpc::SingleResponse<UserProfile> {
        println!("/last_user_info");
        self.last_user
            .lock()
            .map(|cell| match cell.clone() {
                Some(profile) => grpc::SingleResponse::completed(profile),
                None => grpc::SingleResponse::err(grpc::Error::Other("Last user not available.")),
            })
            .unwrap_or(grpc::SingleResponse::err(grpc::Error::Other(
                "Error during access to resource.",
            )))
    }
}
