use super::http::instrument_http;
use axum::{Router, body::Body, http::Request, middleware::from_fn, routing::get};
use opentelemetry::{
    global,
    trace::{SpanKind, Status, TracerProvider as _},
};
use opentelemetry_sdk::{
    Resource,
    propagation::TraceContextPropagator,
    trace::{InMemorySpanExporter, SdkTracerProvider},
};
use osl_telemetry::JsonWithTraceContext;
use serde_json::Value;
use std::{
    io::{self, Write},
    sync::{Arc, Mutex},
};
use tower::ServiceExt;
use tracing::instrument::WithSubscriber;
use tracing_subscriber::{
    Layer,
    filter::{FilterExt, LevelFilter, filter_fn},
    fmt::{MakeWriter, format::JsonFields},
    layer::SubscriberExt,
};

fn include_span_metadata(metadata: &tracing::Metadata<'_>) -> bool {
    metadata.is_span()
}

#[derive(Clone, Default)]
struct Buffer(Arc<Mutex<Vec<u8>>>);

impl Write for Buffer {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<'a> MakeWriter<'a> for Buffer {
    type Writer = Self;
    fn make_writer(&'a self) -> Self::Writer {
        self.clone()
    }
}

#[tokio::test]
async fn propagates_remote_trace_context() {
    global::set_text_map_propagator(TraceContextPropagator::new());
    let exporter = InMemorySpanExporter::default();
    let provider = SdkTracerProvider::builder()
        .with_simple_exporter(exporter.clone())
        .build();
    let output = Buffer::default();
    let formatter =
        JsonWithTraceContext::new(&Resource::builder().with_service_name("test-api").build());
    let subscriber = tracing_subscriber::registry()
        .with(tracing_opentelemetry::layer().with_tracer(provider.tracer("test")))
        .with(formatter.clone())
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(false)
                .with_writer(output.clone())
                .fmt_fields(JsonFields::new())
                .event_format(formatter)
                .with_filter(LevelFilter::WARN.or(filter_fn(include_span_metadata))),
        );
    let service = Router::new()
        .route(
            "/athletes/{slug}",
            get(|| async {
                tracing::warn!(attempt = 2_u64, "Database unavailable");
                axum::http::StatusCode::SERVICE_UNAVAILABLE
            }),
        )
        .layer(from_fn(instrument_http));
    let response = service
        .oneshot(
            Request::builder()
                .uri("/athletes/example?secret=do-not-log")
                .header(
                    "traceparent",
                    "00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01",
                )
                .header("x-request-id", "eb9957b3-ffb8-43b7-bc48-4a2a8414c2ef")
                .body(Body::empty())
                .unwrap(),
        )
        .with_subscriber(subscriber)
        .await
        .unwrap();
    let lines: Vec<Value> = String::from_utf8(output.0.lock().unwrap().clone())
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    assert_eq!(lines.len(), 2);
    let log = lines
        .iter()
        .find(|log| log["severity_text"] == "WARN")
        .unwrap();
    assert_eq!(log["trace_id"], "4bf92f3577b34da6a3ce929d0e0e4736");
    assert_eq!(log["severity_number"], 13);
    assert_eq!(log["severity_text"], "WARN");
    assert_eq!(log["message"], "Database unavailable");
    assert_eq!(log["attributes"]["attempt"], 2);
    assert_eq!(log["resource"]["service.name"], "test-api");
    assert_eq!(log["spans"][0]["http.route"], "/athletes/{slug}");
    assert_eq!(
        log["spans"][0]["request_id"],
        "eb9957b3-ffb8-43b7-bc48-4a2a8414c2ef"
    );
    assert!(log["timestamp"].as_str().unwrap().ends_with('Z'));
    assert!(log["logger"].as_str().unwrap().contains("telemetry::tests"));
    let spans = exporter.get_finished_spans().unwrap();
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].name, "GET /athletes/{slug}");
    assert_eq!(spans[0].span_kind, SpanKind::Server);
    assert!(matches!(spans[0].status, Status::Error { .. }));
    assert_eq!(spans[0].parent_span_id.to_string(), "00f067aa0ba902b7");
    assert_eq!(
        spans[0].span_context.span_id().to_string(),
        log["span_id"].as_str().unwrap()
    );
    assert!(
        response.headers()["traceparent"]
            .to_str()
            .unwrap()
            .contains(log["span_id"].as_str().unwrap())
    );
    assert!(!log.to_string().contains("do-not-log"));
    assert!(!format!("{spans:?}").contains("do-not-log"));
}

#[test]
fn preserves_event_parents_and_late_span_fields() {
    let provider = SdkTracerProvider::builder().build();
    let output = Buffer::default();
    let formatter = JsonWithTraceContext::new(&Resource::builder_empty().build());
    let subscriber = tracing_subscriber::registry()
        .with(tracing_opentelemetry::layer().with_tracer(provider.tracer("test")))
        .with(formatter.clone())
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(false)
                .with_writer(output.clone())
                .fmt_fields(JsonFields::new())
                .event_format(formatter),
        );
    tracing::subscriber::with_default(subscriber, || {
        let parent = tracing::info_span!("operation", result = tracing::field::Empty);
        parent.record("result", "done");
        let other = tracing::info_span!("unrelated");
        let _entered = other.enter();
        tracing::info!(parent: &parent, timestamp = "cannot override timestamp", "Operation completed");
        tracing::error!(parent: None, fatal = true, error.type = "fixture", "Test process terminated");
    });
    let records: Vec<Value> = String::from_utf8(output.0.lock().unwrap().clone())
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    let log = &records[0];
    assert_eq!(log["spans"][0]["name"], "operation");
    assert_eq!(log["spans"][0]["result"], "done");
    assert!(log["trace_id"].as_str().is_some_and(|id| id.len() == 32));
    assert_ne!(log["timestamp"], log["attributes"]["timestamp"]);
    assert_eq!(records[1]["severity_text"], "FATAL");
    assert_eq!(records[1]["severity_number"], 21);
    assert!(records[1]["attributes"].get("fatal").is_none());
    assert!(records[1].get("trace_id").is_none());
}
