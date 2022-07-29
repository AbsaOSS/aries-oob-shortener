pub mod redirect;
pub mod internal;
pub mod health;

use actix_web::web;

pub(crate) fn configure_scopes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::scope("/api")
                .service(
                    web::scope("/health")
                        .service(health::get_health)
                )
                .service(
                    web::scope("/internal")
                        .service(internal::shorten)
                )
        )
        .service(
            web::scope("")
                .service(redirect::redirect)
        );
}
