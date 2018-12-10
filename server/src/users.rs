use commons::models::{User, UserProfile};
use std::collections::HashMap;

pub fn create_user(id: u64, name: &str, surname: &str) -> (User, UserProfile) {
    let mut user = User::new();
    user.set_name(name.to_owned());
    let mut user_profile = UserProfile::new();
    user_profile.set_name(name.to_owned());
    user_profile.set_surname(surname.to_owned());
    user_profile.set_id(id + 1);
    (user, user_profile)
}

pub fn build_users_map(users: &Vec<(&str, &str)>) -> HashMap<User, UserProfile> {
    users.iter().fold(HashMap::new(), |mut m, tuple| {
        let (user, profile) = create_user(m.len() as u64, tuple.0, tuple.1);
        m.insert(user, profile);
        m
    })
}
