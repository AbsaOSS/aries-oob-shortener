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
