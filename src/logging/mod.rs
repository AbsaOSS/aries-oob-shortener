mod subscriber;
mod common;
mod layer_json;
mod layer_pretty;

use crate::error::prelude::*;

pub fn init_logger(log_json: bool, name: Option<&str>, default_log_level: Option<&str>) -> SResult<()> {
    let subscriber_name = name.unwrap_or("dlt-shortener").to_string();
    let default_log_level = default_log_level.unwrap_or("info").to_string();
    let sink = std::io::stdout;
    if log_json {
        let subscriber = subscriber::get_subscriber_json(subscriber_name, default_log_level, sink);
        subscriber::init_subscriber(subscriber);
    } else {
        let subscriber = subscriber::get_subscriber_pretty(subscriber_name, default_log_level, sink);
        subscriber::init_subscriber(subscriber);
    };
    Ok(())
}
