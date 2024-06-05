use actix_web::{HttpResponse, Responder};
use crate::responses::generic_responses::GenericResponse;
use crate::utils::enums::Status;

pub async fn check_api() -> impl Responder {
	let response = GenericResponse::new(
		Status::Success,
		"API is running".to_string()
	);
    HttpResponse::Ok().json(response)
}
