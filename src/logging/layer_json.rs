use std::collections::BTreeMap;

use tracing_subscriber::Layer;

use crate::logging::common::{CustomFieldStorage, JsonVisitor};

pub struct LayerJson;

impl<S> Layer<S> for LayerJson
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

    fn on_event(&self, event: &tracing::Event<'_>, ctx: tracing_subscriber::layer::Context<'_, S>) {
        // Collect field data from all wrapping spans
        let mut spans = vec![];
        if let Some(scope) = ctx.event_scope(event) {
            for span in scope.from_root() {
                let extensions = span.extensions();
                let storage = extensions.get::<CustomFieldStorage>().unwrap();
                let field_data: &BTreeMap<String, serde_json::Value> = &storage.0;
                spans.push(serde_json::json!({
                    "target": span.metadata().target(),
                    "name": span.name(),
                    "level": format!("{:?}", span.metadata().level()),
                    "fields": field_data,
                }));
            }
        };

        // Store custom fields of the event
        let mut fields = BTreeMap::new();
        let mut visitor = JsonVisitor(&mut fields);
        event.record(&mut visitor);

        // Print output
        let output = serde_json::json!({
            "target": event.metadata().target(),
            "name": event.metadata().name(),
            "message": fields["message"],
            "module": event.metadata().module_path(),
            "level": event.metadata().level().to_string().to_lowercase(),
            "filename": std::path::Path::new(&event.metadata().file().unwrap_or("")).file_name().unwrap_or(std::ffi::OsStr::new("")).to_str(),
            "target": event.metadata().target(),
            "spans": spans,
        });
        println!("{}", serde_json::to_string_pretty(&output).unwrap());
    }
}
