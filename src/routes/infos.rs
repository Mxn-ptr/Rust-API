use actix_web::web;

use crate::handlers::general_handler::check_api;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/infos")
            .route(web::get().to(check_api))
    );
}

