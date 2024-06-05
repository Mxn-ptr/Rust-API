use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponse {
    pub id: String,
    pub email: String
}

impl User {
    pub fn to_user_reponse(self: User) -> UserResponse {
        UserResponse {
            id: self.id,
            email: self.email
        }
    }
}
