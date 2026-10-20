use opentelemetry::trace::TraceContextExt;
use serde_json::{Map, Value};
use std::fmt;
use tracing::{
    Event, Subscriber,
    field::{Field, Visit},
};
use tracing_opentelemetry::OpenTelemetrySpanExt;
use tracing_subscriber::{
    fmt::{
        FmtContext, FormatEvent, FormatFields,
        format::Writer,
        time::{FormatTime, SystemTime},
    },
    registry::LookupSpan,
};

pub(super) struct JsonWithTraceContext;

impl<S, N> FormatEvent<S, N> for JsonWithTraceContext
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        _ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        let metadata = event.metadata();
        let mut line = Map::new();

        let mut timestamp = String::new();
        SystemTime.format_time(&mut Writer::new(&mut timestamp))?;
        line.insert("timestamp".into(), timestamp.into());
        line.insert("level".into(), metadata.level().as_str().into());
        line.insert("target".into(), metadata.target().into());

        event.record(&mut FieldVisitor(&mut line));

        let span_context = tracing::Span::current()
            .context()
            .span()
            .span_context()
            .clone();
        if span_context.is_valid() {
            line.insert(
                "trace_id".into(),
                span_context.trace_id().to_string().into(),
            );
            line.insert("span_id".into(), span_context.span_id().to_string().into());
        }

        writeln!(writer, "{}", Value::Object(line))
    }
}

struct FieldVisitor<'a>(&'a mut Map<String, Value>);

impl Visit for FieldVisitor<'_> {
    fn record_str(&mut self, field: &Field, value: &str) {
        self.0.insert(field.name().into(), value.into());
    }

    fn record_i64(&mut self, field: &Field, value: i64) {
        self.0.insert(field.name().into(), value.into());
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        self.0.insert(field.name().into(), value.into());
    }

    fn record_f64(&mut self, field: &Field, value: f64) {
        self.0.insert(field.name().into(), value.into());
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        self.0.insert(field.name().into(), value.into());
    }

    fn record_error(&mut self, field: &Field, value: &(dyn std::error::Error + 'static)) {
        self.0.insert(field.name().into(), value.to_string().into());
    }

    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        self.0
            .insert(field.name().into(), format!("{value:?}").into());
    }
}
