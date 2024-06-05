use serde::Serialize;

use crate::models::user::UserResponse;

#[derive(Serialize)]
pub struct GenericResponse {
	pub status: String,
	pub message: String,
}

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
