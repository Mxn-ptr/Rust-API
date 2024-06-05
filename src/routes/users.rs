use actix_web::web;

use crate::handlers::user_handler::{create_user, get_users};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users")
            .route(web::post().to(create_user))
			.route(web::get().to(get_users))
    );
}
