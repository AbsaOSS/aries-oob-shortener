use std::collections::BTreeMap;

use chrono::Local;
use tracing_subscriber::Layer;

use crate::logging::common::{CustomFieldStorage, JsonVisitor};

pub struct LayerPretty;

impl<S> Layer<S> for LayerPretty
where
    S: tracing::Subscriber,
    S: for<'lookup> tracing_subscriber::registry::LookupSpan<'lookup>,
{
    fn on_new_span(
        &self,
        attrs: &tracing::span::Attributes<'_>,
        id: &tracing::span::Id,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        // Store span attributes in tracing's extensions
        let span = ctx.span(id).unwrap();
        let mut fields = BTreeMap::new();
        let mut visitor = JsonVisitor(&mut fields);
        attrs.record(&mut visitor);
        let storage = CustomFieldStorage(fields);
        let mut extensions = span.extensions_mut();
        extensions.insert(storage);
    }

    fn on_record(
        &self,
        id: &tracing::span::Id,
        values: &tracing::span::Record<'_>,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        // Append new values to extensions
        let span = ctx.span(id).unwrap();
        let mut extensions_mut = span.extensions_mut();
        let custom_field_storage: &mut CustomFieldStorage =
            extensions_mut.get_mut::<CustomFieldStorage>().unwrap();
        let json_data: &mut BTreeMap<String, serde_json::Value> = &mut custom_field_storage.0;
        let mut visitor = JsonVisitor(json_data);
        values.record(&mut visitor);
    }

    fn on_event(&self, event: &tracing::Event<'_>, _ctx: tracing_subscriber::layer::Context<'_, S>) {
        // Store custom fields of the event
        let mut fields = BTreeMap::new();
        let mut visitor = JsonVisitor(&mut fields);
        event.record(&mut visitor);

        // Print output
        println!("{:>5}|{:}|{:>25}:{:<4}| {}",
                     event.metadata().level().to_string().to_lowercase(),
                     Local::now().format("%Y-%m-%d %H:%M.%S"),
                     std::path::Path::new(&event.metadata().file().unwrap_or("")).file_name().unwrap_or(std::ffi::OsStr::new("")).to_str().unwrap_or(""),
                     event.metadata().line().unwrap_or(0),
                     fields["message"].as_str().unwrap_or("")
                 );
    }
}
