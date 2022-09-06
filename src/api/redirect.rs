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

use actix_web::{
    get,
    http::{header, StatusCode},
    web, HttpResponse,
};
use serde_json::Value;

fn send_redirect(long_url: &str) -> SResult<HttpResponse> {
    Ok(HttpResponse::Ok()
        .status(StatusCode::PERMANENT_REDIRECT)
        .append_header((
            header::LOCATION,
            header::HeaderValue::from_str(long_url).map_err(|err| {
                SError::from_msg(
                    SErrorType::ParsingError,
                    &format!(
                        "Failed to convert long url {} to header value, error: {}",
                        long_url, err
                    ),
                )
            })?,
        ))
        .finish())
}

fn send_json(msg: &Value) -> SResult<HttpResponse> {
    Ok(HttpResponse::Ok().json(msg))
}

#[get("/{msg_hash}")]
async fn redirect(
    services: web::Data<Services>,
    path: web::Path<String>,
    ctype: web::Header<header::Accept>,
) -> SResult<HttpResponse> {
    if ctype.0 == header::Accept::json() {
        let msg = services
            .service_shorten
            .get_message(&path.into_inner())
            .await?;
        send_json(&msg)
    } else {
        let long_url = services
            .service_shorten
            .get_long_url(&path.into_inner())
            .await?;
        send_redirect(&long_url)
    }
}
