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

use std::collections::HashMap;
use std::ffi::OsStr;
use std::io::Write;
use std::path::Path;

use chrono::Local;
use serde::ser::{SerializeMap, Serializer};
use serde_json::Value;
use tracing::{Event, Id, Subscriber};
use tracing_core::metadata::Metadata;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::layer::Context;
use tracing_subscriber::registry::{LookupSpan, SpanRef};
use tracing_subscriber::Layer;

use crate::logging::layers::common::{
    Type, FILENAME, LEVEL, MESSAGE, MODULE, NAME, RESERVED_FIELDS, TARGET, TIMESTAMP,
};
use crate::logging::layers::layer_storage::Storage;

// TODO: Do we want to format the context like this
fn format_span_context<S: Subscriber + for<'a> LookupSpan<'a>>(
    span: &SpanRef<S>,
    ty: Type,
) -> String {
    format!("[{} - {}]", span.metadata().name().to_uppercase(), ty)
}

fn format_event_message<S: Subscriber + for<'a> LookupSpan<'a>>(
    span: &Option<SpanRef<S>>,
    event: &Event,
    visitor: &Storage,
) -> String {
    let mut message = visitor
        .values()
        .get("message")
        .and_then(|v| match v {
            Value::String(s) => Some(s.as_str()),
            _ => None,
        })
        .unwrap_or_else(|| event.metadata().target())
        .to_owned();

    if let Some(span) = &span {
        message = format!("{} {}", format_span_context(span, Type::Event), message);
    }

    message
}

pub struct LayerJson<W: for<'a> MakeWriter<'a> + 'static> {
    make_writer: W,
    name: String,
    default_fields: HashMap<String, Value>,
}

impl<W: for<'a> MakeWriter<'a> + 'static> LayerJson<W> {
    pub fn new(name: String, make_writer: W, default_fields: HashMap<String, Value>) -> Self {
        Self {
            name,
            make_writer,
            default_fields,
        }
    }

    fn emit(&self, mut buffer: Vec<u8>) -> Result<(), std::io::Error> {
        buffer.write_all(b"\n")?;
        self.make_writer.make_writer().write_all(&buffer)
    }

    fn serialize_core_fields(
        &self,
        map_serializer: &mut impl SerializeMap<Error = serde_json::Error>,
        message: &str,
        metadata: &Metadata,
    ) -> Result<(), std::io::Error> {
        let file_path = metadata.file().unwrap_or("");
        let filename = Path::new(&file_path)
            .file_name()
            .unwrap_or_else(|| OsStr::new(""))
            .to_str();
        map_serializer.serialize_entry(NAME, &self.name)?;
        map_serializer.serialize_entry(LEVEL, &metadata.level().to_string().to_lowercase())?;
        map_serializer.serialize_entry(MESSAGE, &message)?;
        map_serializer.serialize_entry(MODULE, &metadata.module_path())?;
        map_serializer.serialize_entry(TARGET, &metadata.target())?;
        map_serializer.serialize_entry(FILENAME, &filename)?;
        map_serializer.serialize_entry(
            TIMESTAMP,
            &Local::now().format("%Y-%m-%d %H:%M.%S").to_string(),
        )?;
        Ok(())
    }

    fn serialize_span<S: Subscriber + for<'a> LookupSpan<'a>>(
        &self,
        span: &SpanRef<S>,
        ty: Type,
    ) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer = Vec::new();
        let mut serializer = serde_json::Serializer::new(&mut buffer);
        let mut map_serializer = serializer.serialize_map(None)?;
        let message = format_span_context(span, ty);
        self.serialize_core_fields(&mut map_serializer, &message, span.metadata())?;

        for (key, value) in self.default_fields.iter() {
            if !RESERVED_FIELDS.contains(&key.as_str()) {
                map_serializer.serialize_entry(key, value)?;
            } else {
                tracing::debug!("{} is a reserved field. Skipping it.", key);
            }
        }

        let extensions = span.extensions();
        if let Some(visitor) = extensions.get::<Storage>() {
            for (key, value) in visitor.values() {
                if !RESERVED_FIELDS.contains(key) {
                    map_serializer.serialize_entry(key, value)?;
                } else {
                    tracing::debug!("{} is a reserved field. Skipping it.", key);
                }
            }
        }
        map_serializer.end()?;
        Ok(buffer)
    }
}

impl<S, W> Layer<S> for LayerJson<W>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    W: for<'a> MakeWriter<'a> + 'static,
{
    fn on_new_span(
        &self,
        _attrs: &tracing::span::Attributes<'_>,
        id: &tracing::span::Id,
        ctx: Context<'_, S>,
    ) {
        let span = ctx.span(id).expect("Span not found, this is a bug.");
        if let Ok(serialized) = self.serialize_span(&span, Type::EnterSpan) {
            let _ = self.emit(serialized);
        }
    }

    fn on_record(
        &self,
        _id: &tracing::span::Id,
        values: &tracing::span::Record<'_>,
        ctx: Context<'_, S>,
    ) {
        if let Some(span) = ctx.lookup_current() {
            let mut extensions = span.extensions_mut();
            if let Some(visitor) = extensions.get_mut::<Storage>() {
                values.record(visitor)
            }
        }
    }

    fn on_event(&self, event: &tracing::Event<'_>, ctx: Context<'_, S>) {
        let mut visitor = Storage::default();
        event.record(&mut visitor);

        let span = ctx.lookup_current();

        let format = || {
            let mut buffer = Vec::new();

            let mut serializer = serde_json::Serializer::new(&mut buffer);
            let mut map_serializer = serializer.serialize_map(None)?;

            let message = format_event_message(&span, event, &visitor);
            self.serialize_core_fields(&mut map_serializer, &message, event.metadata())?;

            for (key, value) in self.default_fields.iter().filter(|(key, _)| {
                key.as_str() != "message" && !RESERVED_FIELDS.contains(&key.as_str())
            }) {
                map_serializer.serialize_entry(key, value)?;
            }

            for (key, value) in visitor
                .values()
                .iter()
                .filter(|(&key, _)| key != "message" && !RESERVED_FIELDS.contains(&key))
            {
                map_serializer.serialize_entry(key, value)?;
            }

            if let Some(span) = &span {
                let extensions = span.extensions();
                if let Some(visitor) = extensions.get::<Storage>() {
                    for (key, value) in visitor.values() {
                        if !RESERVED_FIELDS.contains(key) {
                            map_serializer.serialize_entry(key, value)?;
                        } else {
                            tracing::debug!("{} is a reserved field. Skipping it.", key);
                        }
                    }
                }
            }
            map_serializer.end()?;
            Ok(buffer)
        };

        let result: std::io::Result<Vec<u8>> = format();
        if let Ok(formatted) = result {
            let _ = self.emit(formatted);
        }
    }

    fn on_close(&self, id: Id, ctx: Context<'_, S>) {
        let span = ctx.span(&id).expect("Span not found, this is a bug");
        if let Ok(serialized) = self.serialize_span(&span, Type::ExitSpan) {
            let _ = self.emit(serialized);
        }
    }
}
