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

mod layers;
mod subscriber;

use std::collections::HashMap;

use crate::error::prelude::*;

use serde_json::Value;

fn log_json() -> bool {
    match std::env::var("LOG_JSON") {
        Ok(val) if val.to_lowercase() == *"false" => false,
        Err(_) => false,
        _ => true,
    }
}

pub fn init_logger(
    default_fields: Option<HashMap<String, Value>>,
    name: Option<&str>,
) -> SResult<()> {
    if log_json() {
        let subscriber_name = name.unwrap_or("dlt-shortener").to_string();
        let subscriber =
            subscriber::get_subscriber_json(subscriber_name, default_fields.unwrap_or_default());
        subscriber::init_subscriber(subscriber);
    } else {
        let subscriber = subscriber::get_subscriber_pretty();
        subscriber::init_subscriber(subscriber);
    };
    Ok(())
}
