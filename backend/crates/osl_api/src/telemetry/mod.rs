mod log_format;
mod metrics;

pub use metrics::{init_recorder, track_http};

use opentelemetry::{global, trace::TracerProvider as _};
use opentelemetry_sdk::{Resource, propagation::TraceContextPropagator, trace::SdkTracerProvider};
use tracing_subscriber::{
    EnvFilter, fmt::format::JsonFields, layer::SubscriberExt, util::SubscriberInitExt,
};

const DEFAULT_LOG_FILTER: &str = "info,sqlx=warn";
const TRACER_NAME: &str = env!("CARGO_PKG_NAME");

#[must_use = "dropping the guard immediately stops trace export"]
pub struct TelemetryGuard {
    tracer_provider: SdkTracerProvider,
}

impl Drop for TelemetryGuard {
    fn drop(&mut self) {
        if let Err(err) = self.tracer_provider.shutdown() {
            eprintln!("failed to flush traces: {err}");
        }
    }
}

pub fn init() -> TelemetryGuard {
    let tracer_provider = build_tracer_provider();

    global::set_text_map_propagator(TraceContextPropagator::new());
    global::set_tracer_provider(tracer_provider.clone());

    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(DEFAULT_LOG_FILTER));

    let log_layer = tracing_subscriber::fmt::layer()
        .fmt_fields(JsonFields::new())
        .event_format(log_format::JsonWithTraceContext);

    let trace_layer =
        tracing_opentelemetry::layer().with_tracer(tracer_provider.tracer(TRACER_NAME));

    tracing_subscriber::registry()
        .with(filter)
        .with(log_layer)
        .with(trace_layer)
        .init();

    TelemetryGuard { tracer_provider }
}

fn build_tracer_provider() -> SdkTracerProvider {
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_http()
        .build()
        .expect("OTLP span exporter must build");

    SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(Resource::builder().build())
        .build()
}
