use crate::error::prelude::*;
use crate::service::Services;

use actix_web::{post, web};

#[derive(serde::Deserialize)]
struct LinkData {
    msg: String,
    base_url: Option<url::Url>,
    expire_in_secs: Option<u32>,
}

#[post("/shorten-link")]
async fn shorten(services: web::Data<Services>, req: web::Json<LinkData>) -> SResult<String> {
    let LinkData {
        msg,
        base_url,
        expire_in_secs,
    } = req.0;
    let shortened = services
        .service_shorten
        .shorten(&msg, base_url, expire_in_secs)
        .await?;
    Ok(json!({ "shortened": shortened }).to_string())
}
