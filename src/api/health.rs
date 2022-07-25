use crate::error::prelude::*;

use actix_web::get;

#[get("")]
async fn get_health() -> SResult<String> {
    Ok(json!({ "status": "success" }).to_string())
}
