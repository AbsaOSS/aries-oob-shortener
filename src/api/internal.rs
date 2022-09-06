/*
 * Copyright 2022 ABSA Group Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

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
