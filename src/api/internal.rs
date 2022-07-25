use crate::error::prelude::*;
use crate::server::services::Services;

use actix_web::{post, delete, web};

#[derive(serde::Deserialize)]
struct LinkData {
    base_url: String,
    msg: String
}

#[post("/shorten-link")]
async fn shorten(services: web::Data<Services>, req: web::Json<LinkData>) -> SResult<String> {
    let LinkData { base_url, msg } = req.0;
    let shortened = services.service_shorten.shorten(&base_url, &msg).await?;
    Ok(json!({ "shortened": shortened }).to_string())
}

#[delete("/")]
async fn delete() -> SResult<String> {
    todo!()
}
