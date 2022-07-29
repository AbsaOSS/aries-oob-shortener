mod subscriber;
mod layers;

use std::collections::HashMap;

use crate::error::prelude::*;

use serde_json::Value;

pub fn init_logger(log_json: bool, name: Option<&str>, default_log_level: Option<&str>, default_fields: Option<HashMap<String, Value>>) -> SResult<()> {
    let subscriber_name = name.unwrap_or("dlt-shortener").to_string();
    let default_log_level = default_log_level.unwrap_or("info").to_string();
    let sink = std::io::stdout;
    if log_json {
        let subscriber = subscriber::get_subscriber_json(subscriber_name, default_log_level, sink, default_fields.unwrap_or_default());
        subscriber::init_subscriber(subscriber);
    } else {
        let subscriber = subscriber::get_subscriber_pretty(subscriber_name, default_log_level, sink);
        subscriber::init_subscriber(subscriber);
    };
    Ok(())
}
