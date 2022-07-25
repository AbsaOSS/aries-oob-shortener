use tracing::subscriber::set_global_default;
use tracing::Subscriber;
use tracing_log::LogTracer;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use crate::logging::layer_json::LayerJson;
use crate::logging::layer_pretty::LayerPretty;

// TODO: Deduplicate
pub fn get_subscriber_json<Sink>(
    name: String,
    default_env_filter: String,
    sink: Sink,
) -> impl Subscriber + Sync + Send
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_env_filter));
    Registry::default()
        .with(env_filter)
        .with(LayerJson)
}

pub fn get_subscriber_pretty<Sink>(
    name: String,
    default_env_filter: String,
    sink: Sink
) -> impl Subscriber + Sync + Send
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_env_filter));
    Registry::default()
        .with(env_filter)
        .with(LayerPretty)
}

pub fn init_subscriber(subscriber: impl Subscriber + Sync + Send) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}
