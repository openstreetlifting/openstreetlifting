use opentelemetry::trace::TraceContextExt;
use serde_json::{Map, Value};
use std::{
    fmt,
    sync::{Arc, OnceLock},
};
use tracing::{
    Event, Subscriber,
    field::{Field, Visit},
};
use tracing_opentelemetry::get_otel_context;
use tracing_subscriber::{
    fmt::{
        FmtContext, FormatEvent, FormatFields, FormattedFields,
        format::Writer,
        time::{FormatTime, SystemTime},
    },
    registry::LookupSpan,
};

#[derive(Clone)]
pub(super) struct JsonWithTraceContext {
    resource: Value,
    dispatch: Arc<OnceLock<tracing::dispatcher::WeakDispatch>>,
}

impl JsonWithTraceContext {
    pub(super) fn new(resource: &opentelemetry_sdk::Resource) -> Self {
        Self {
            dispatch: Arc::default(),
            resource: resource
                .iter()
                .map(|(key, value)| (key.as_str().to_owned(), Value::String(value.to_string())))
                .collect::<Map<_, _>>()
                .into(),
        }
    }
}

// tracing suppresses the thread's default dispatch while formatting an event.
// Keep a weak reference to the owning subscriber to resolve explicit parents.
impl<S: Subscriber> tracing_subscriber::Layer<S> for JsonWithTraceContext {
    fn on_register_dispatch(&self, dispatch: &tracing::Dispatch) {
        let _ = self.dispatch.set(dispatch.downgrade());
    }
}

impl<S, N> FormatEvent<S, N> for JsonWithTraceContext
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        let metadata = event.metadata();
        let mut line = Map::new();

        let mut timestamp = String::new();
        SystemTime.format_time(&mut Writer::new(&mut timestamp))?;
        line.insert("timestamp".into(), timestamp.into());
        line.insert("severity_text".into(), metadata.level().as_str().into());
        let severity = match *metadata.level() {
            tracing::Level::TRACE => 1,
            tracing::Level::DEBUG => 5,
            tracing::Level::INFO => 9,
            tracing::Level::WARN => 13,
            tracing::Level::ERROR => 17,
        };
        line.insert("severity_number".into(), severity.into());
        line.insert("logger".into(), metadata.target().into());
        line.insert("resource".into(), self.resource.clone());

        let mut attributes = Map::new();
        event.record(&mut FieldVisitor(&mut attributes));
        line.insert(
            "message".into(),
            attributes.remove("message").unwrap_or_else(|| "".into()),
        );
        line.insert("attributes".into(), attributes.into());

        let spans = ctx
            .event_scope()
            .into_iter()
            .flat_map(tracing_subscriber::registry::Scope::from_root)
            .map(|span| {
                let mut fields = span
                    .extensions()
                    .get::<FormattedFields<N>>()
                    .and_then(|fields| {
                        serde_json::from_str::<Map<String, Value>>(fields.fields.as_str()).ok()
                    })
                    .unwrap_or_default();
                fields.insert("name".into(), span.name().into());
                Value::Object(fields)
            })
            .collect::<Vec<_>>();
        line.insert("spans".into(), spans.into());

        // Resolve the event's parent, which may differ from the current span.
        let context = ctx.parent_span().and_then(|span| {
            let dispatch = self.dispatch.get()?.upgrade()?;
            get_otel_context(&span.id(), &dispatch)
        });
        if let Some(context) = context {
            let span = context.span();
            let span_context = span.span_context();
            if span_context.is_valid() {
                line.insert(
                    "trace_id".into(),
                    span_context.trace_id().to_string().into(),
                );
                line.insert("span_id".into(), span_context.span_id().to_string().into());
                line.insert(
                    "trace_flags".into(),
                    span_context.trace_flags().to_u8().into(),
                );
            }
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
