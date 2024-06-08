use actix_web::web;

use crate::handlers::user_handler::{create_user, delete_user, get_user, get_users, login, reset_password, update_user};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::post().to(create_user))
            .route("/login", web::post().to(login))
            .route("/reset-password/{id}", web::post().to(reset_password))
			.route("", web::get().to(get_users))
            .route("/{id}", web::delete().to(delete_user))
            .route("/{id}", web::get().to(get_user))
            .route("/{id}", web::put().to(update_user))
    );
}
