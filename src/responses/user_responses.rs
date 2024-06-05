use serde::Serialize;

use crate::{models::user::UserResponse, utils::enums::Status};


#[derive(Serialize)]
pub struct UserData {
	pub user: UserResponse
}

#[derive(Serialize)]
pub struct SingleUserReponse {
	pub status: String,
	pub data: UserData,
}

#[derive(Serialize)]

pub struct UserListResponse {
	pub status: String,
	pub count: usize,
	pub users: Vec<UserResponse>
}

impl SingleUserReponse {
	pub fn new(status: Status, data: UserData) -> SingleUserReponse {
		SingleUserReponse {
			status: status.to_string(),
			data
		}
	}
}

impl UserListResponse {
	pub fn new(status: Status,count: usize, users: Vec<UserResponse>) -> UserListResponse {
		UserListResponse {
			status: status.to_string(),
			count,
			users
		}
	}
}
