use actix_web::web;

use crate::handlers::user_handler::{check_api, create_user};

pub fn init(cfg: &mut web::ServiceConfig) {
	cfg.service(
		web::scope("/users")
			.route("", web::get().to(check_api))
			.route("", web::post().to(create_user)),
	);
}
