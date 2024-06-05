use actix_web::web;
use crate::routes::infos;
use crate::routes::users;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(users::config)
            .configure(infos::config)
    );
}
