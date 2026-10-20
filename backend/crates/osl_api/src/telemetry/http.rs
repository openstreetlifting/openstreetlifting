use axum::{
    extract::{MatchedPath, Request},
    http::{HeaderMap, HeaderValue},
    middleware::Next,
    response::Response,
};
use opentelemetry::{global, propagation::Extractor, trace::TraceContextExt};
use tracing::{Instrument, field::Empty};
use tracing_opentelemetry::OpenTelemetrySpanExt;

struct RequestHeaders<'a>(&'a HeaderMap);

impl Extractor for RequestHeaders<'_> {
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key)?.to_str().ok()
    }

    fn keys(&self) -> Vec<&str> {
        self.0.keys().map(axum::http::HeaderName::as_str).collect()
    }
}

pub async fn instrument_http(request: Request, next: Next) -> Response {
    let route = request
        .extensions()
        .get::<MatchedPath>()
        .map_or("unmatched", MatchedPath::as_str);
    let method = super::metrics::method_label(request.method());
    let request_id = request
        .headers()
        .get("x-request-id")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("");
    let span = tracing::info_span!(
        "http.server.request",
        otel.name = %format!("{method} {route}"),
        otel.kind = "server",
        http.request.method = method,
        http.route = route,
        http.response.status_code = Empty,
        error.type = Empty,
        otel.status_code = Empty,
        request_id,
    );
    let parent = global::get_text_map_propagator(|propagator| {
        propagator.extract(&RequestHeaders(request.headers()))
    });
    let _ = span.set_parent(parent);

    async {
        let mut response = next.run(request).await;
        let span = tracing::Span::current();
        let status = response.status();
        span.record("http.response.status_code", i64::from(status.as_u16()));
        if status.is_server_error() {
            span.record("otel.status_code", "ERROR");
            span.record("error.type", status.as_str());
        }
        let context = span.context();
        let active = context.span();
        let span_context = active.span_context();
        if span_context.is_valid() {
            let traceparent = format!(
                "00-{}-{}-{:02x}",
                span_context.trace_id(),
                span_context.span_id(),
                span_context.trace_flags().to_u8()
            );
            if let Ok(value) = HeaderValue::from_str(&traceparent) {
                response.headers_mut().insert("traceparent", value);
            }
        }
        tracing::info!("HTTP request completed");
        response
    }
    .instrument(span)
    .await
}
