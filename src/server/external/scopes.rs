use actix_web::web;

use crate::api::{health, redirect};

pub(crate) fn configure_scopes_external(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").service(web::scope("/health").service(health::get_health)))
        .service(web::scope("").service(redirect::redirect));
}
