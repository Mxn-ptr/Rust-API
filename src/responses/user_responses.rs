use serde::Serialize;

use crate::models::user::User;

#[derive(Serialize)]
pub struct GenericResponse {
	pub status: String,
	pub message: String,
}

#[derive(Serialize)]
pub struct UserData {
	pub user: User
}

#[derive(Serialize)]
pub struct SingleUserReponse {
	pub status: String,
	pub data: UserData,
}

pub struct UserListResponse {
	pub status: String,
	pub count: usize,
	pub users: Vec<User>
}
