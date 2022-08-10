use actix_web::web;

use crate::api::{health, internal};

pub(crate) fn configure_scopes_internal(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::scope("/health").service(health::get_health))
            .service(web::scope("/internal").service(internal::shorten)),
    );
}
