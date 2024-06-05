use actix_web::{HttpResponse, Responder};
use crate::responses::user_responses::GenericResponse;

pub async fn check_api() -> impl Responder {
	let response = GenericResponse {
		status: "success".to_string(),
		message: "Api is running".to_string()
	};
    HttpResponse::Ok().json(response)
}
