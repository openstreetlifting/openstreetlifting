mod log_format;

pub use log_format::JsonWithTraceContext;

use anyhow::Context;
use opentelemetry::{KeyValue, global, trace::TracerProvider as _};
use opentelemetry_sdk::{Resource, propagation::TraceContextPropagator, trace::SdkTracerProvider};
use tracing_subscriber::{
    EnvFilter, Layer,
    filter::{FilterExt, LevelFilter, filter_fn},
    fmt::format::JsonFields,
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

const TRACER_NAME: &str = env!("CARGO_PKG_NAME");

fn is_span(metadata: &tracing::Metadata<'_>) -> bool {
    metadata.is_span()
}

#[must_use = "dropping the guard immediately stops trace export"]
pub struct TelemetryGuard {
    tracer_provider: SdkTracerProvider,
}

impl Drop for TelemetryGuard {
    fn drop(&mut self) {
        if self.tracer_provider.shutdown().is_err() {
            tracing::error!(error.type = "telemetry_shutdown", "Rust trace exporter shutdown failed");
        }
    }
}

pub fn init(service_name: &str, default_filter: &str) -> anyhow::Result<TelemetryGuard> {
    let resource = Resource::builder()
        .with_service_name(
            std::env::var("OTEL_SERVICE_NAME").unwrap_or_else(|_| service_name.into()),
        )
        .with_attribute(KeyValue::new("service.version", env!("CARGO_PKG_VERSION")))
        .build();
    let tracer_provider = build_tracer_provider(resource.clone())?;

    global::set_text_map_propagator(TraceContextPropagator::new());
    global::set_tracer_provider(tracer_provider.clone());

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(default_filter))
        .or(filter_fn(is_span));

    let trace_layer = tracing_opentelemetry::layer()
        .with_tracer(tracer_provider.tracer(TRACER_NAME))
        .with_filter(LevelFilter::INFO);
    let formatter = log_format::JsonWithTraceContext::new(&resource);
    let log_layer = if std::env::var("LOG_FORMAT").as_deref() == Ok("json") {
        tracing_subscriber::fmt::layer()
            .with_ansi(false)
            .fmt_fields(JsonFields::new())
            .event_format(formatter.clone())
            .with_filter(filter)
            .boxed()
    } else {
        tracing_subscriber::fmt::layer()
            .with_writer(std::io::stderr)
            .with_filter(filter)
            .boxed()
    };

    tracing_subscriber::registry()
        .with(trace_layer)
        .with(formatter)
        .with(log_layer)
        .try_init()
        .context("Failed to install telemetry subscriber")?;

    Ok(TelemetryGuard { tracer_provider })
}

fn build_tracer_provider(resource: Resource) -> anyhow::Result<SdkTracerProvider> {
    let mut builder = SdkTracerProvider::builder().with_resource(resource);

    let export_enabled = std::env::var("OTEL_SDK_DISABLED").as_deref() != Ok("true")
        && [
            "OTEL_EXPORTER_OTLP_ENDPOINT",
            "OTEL_EXPORTER_OTLP_TRACES_ENDPOINT",
        ]
        .iter()
        .any(|key| std::env::var(key).is_ok_and(|value| !value.is_empty()));
    if export_enabled {
        let exporter = opentelemetry_otlp::SpanExporter::builder()
            .with_http()
            .build()
            .context("Failed to build OTLP/HTTP trace exporter")?;
        builder = builder.with_batch_exporter(exporter);
    }

    Ok(builder.build())
}

pub fn report_startup_failure(service_name: &str) {
    eprintln!(
        "{}",
        serde_json::json!({
            "timestamp": chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Nanos, true),
            "severity_text": "FATAL",
            "severity_number": 21,
            "message": "Rust telemetry initialization failed",
            "logger": "osl_telemetry",
            "resource": { "service.name": service_name },
            "attributes": { "error.type": "telemetry_initialization" },
            "spans": [],
        })
    );
}
