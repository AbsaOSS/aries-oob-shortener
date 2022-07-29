use std::collections::HashMap;

use tracing::subscriber::set_global_default;
use tracing::Subscriber;
use tracing_log::LogTracer;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::fmt;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use serde_json::Value;

use crate::logging::layers::layer_json::LayerJson;
use crate::logging::layers::layer_storage::LayerStorage;

// TODO: Deduplicate
pub fn get_subscriber_json<Sink>(
    name: String,
    default_env_filter: String,
    sink: Sink,
    default_fields: HashMap<String, Value>
) -> impl Subscriber + Sync + Send
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_env_filter));
    let layer_json = LayerJson::new(name, sink, default_fields);
    Registry::default()
        .with(env_filter)
        .with(LayerStorage)
        .with(layer_json)
}

pub fn get_subscriber_pretty<Sink>(
    _name: String,
    default_env_filter: String,
    _sink: Sink
) -> impl Subscriber + Sync + Send
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_env_filter));
    let fmt_layer = fmt::layer()
        .with_target(true)
        .with_level(true)
        .compact();
    Registry::default()
        .with(env_filter)
        .with(fmt_layer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Sync + Send) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
    tracing::info!("Subscriber initialized!");
}
