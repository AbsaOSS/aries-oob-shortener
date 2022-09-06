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
use std::fmt;
use std::time::Instant;
use tracing::field::{Field, Visit};
use tracing::span::{Attributes, Record};
use tracing::{Id, Subscriber};
use tracing_subscriber::layer::Context;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::Layer;

#[derive(Clone, Debug)]
pub struct LayerStorage;

#[derive(Clone, Debug, Default)]
pub struct Storage<'a> {
    values: HashMap<&'a str, serde_json::Value>,
}

impl<'a> Storage<'a> {
    pub fn values(&self) -> &HashMap<&'a str, serde_json::Value> {
        &self.values
    }
}

impl Visit for Storage<'_> {
    fn record_i64(&mut self, field: &Field, value: i64) {
        self.values
            .insert(field.name(), serde_json::Value::from(value));
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        self.values
            .insert(field.name(), serde_json::Value::from(value));
    }

    fn record_f64(&mut self, field: &Field, value: f64) {
        self.values
            .insert(field.name(), serde_json::Value::from(value));
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        self.values
            .insert(field.name(), serde_json::Value::from(value));
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        self.values
            .insert(field.name(), serde_json::Value::from(value));
    }

    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        self.values.insert(
            field.name(),
            serde_json::Value::from(format!("{:?}", value)),
        );
    }
}

impl<S: Subscriber + for<'a> LookupSpan<'a>> Layer<S> for LayerStorage {
    fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        let span = ctx.span(id).expect("Span not found, this is a bug");

        let mut visitor = if let Some(parent_span) = span.parent() {
            let mut extensions = parent_span.extensions_mut();
            extensions
                .get_mut::<Storage>()
                .map(|v| v.to_owned())
                .unwrap_or_default()
        } else {
            Storage::default()
        };

        let mut extensions = span.extensions_mut();

        attrs.record(&mut visitor);
        extensions.insert(visitor);
    }

    fn on_record(&self, span: &Id, values: &Record<'_>, ctx: Context<'_, S>) {
        let span = ctx.span(span).expect("Span not found, this is a bug");

        let mut extensions = span.extensions_mut();
        let visitor = extensions
            .get_mut::<Storage>()
            .expect("Visitor not found on 'record', this is a bug");
        values.record(visitor);
    }

    fn on_enter(&self, span: &Id, ctx: Context<'_, S>) {
        let span = ctx.span(span).expect("Span not found, this is a bug");

        let mut extensions = span.extensions_mut();
        if extensions.get_mut::<Instant>().is_none() {
            extensions.insert(Instant::now());
        }
    }

    fn on_close(&self, span: Id, ctx: Context<'_, S>) {
        let span = ctx.span(&span).expect("Span not found, this is a bug");

        let elapsed_milliseconds = {
            let extensions = span.extensions();
            extensions
                .get::<Instant>()
                .map(|i| i.elapsed().as_millis())
                .unwrap_or(0)
        };

        #[cfg(not(feature = "arbitrary-precision"))]
        let elapsed_milliseconds: u64 = { elapsed_milliseconds.try_into().unwrap_or_default() };

        let mut extensions_mut = span.extensions_mut();
        let visitor = extensions_mut
            .get_mut::<Storage>()
            .expect("Visitor not found on 'record', this is a bug");

        if let Ok(elapsed) = serde_json::to_value(elapsed_milliseconds) {
            visitor.values.insert("elapsed_milliseconds", elapsed);
        }
    }
}
