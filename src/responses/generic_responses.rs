use serde::Serialize;
use crate::utils::enums::Status;

#[derive(Serialize)]
pub struct GenericResponse {
	pub status: String,
	pub message: String,
}

impl GenericResponse {
    pub fn new(status: Status, message: String) -> Self {
        GenericResponse {
            status: status.to_string(),
            message,
        }
    }
}
