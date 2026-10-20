use axum::{
    extract::{MatchedPath, Request},
    middleware::Next,
    response::Response,
};
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};
use std::time::Instant;

const REQUESTS_TOTAL: &str = "http_requests_total";
const REQUEST_DURATION: &str = "http_request_duration_seconds";
const LATENCY_BUCKETS: &[f64] = &[
    0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
];

pub(super) fn method_label(method: &axum::http::Method) -> &'static str {
    match method.as_str() {
        "GET" => "GET",
        "HEAD" => "HEAD",
        "POST" => "POST",
        "PUT" => "PUT",
        "DELETE" => "DELETE",
        "CONNECT" => "CONNECT",
        "OPTIONS" => "OPTIONS",
        "TRACE" => "TRACE",
        "PATCH" => "PATCH",
        _ => "_OTHER",
    }
}

pub fn init_recorder() -> PrometheusHandle {
    PrometheusBuilder::new()
        .set_buckets_for_metric(Matcher::Full(REQUEST_DURATION.into()), LATENCY_BUCKETS)
        .expect("latency buckets must be non-empty")
        .install_recorder()
        .expect("metrics recorder must be installed once")
}

pub async fn track_http(req: Request, next: Next) -> Response {
    let start = Instant::now();

    // Route template ("/orders/{id}"), never the raw path: bounded cardinality.
    let route = req
        .extensions()
        .get::<MatchedPath>()
        .map(|path| path.as_str().to_owned())
        .unwrap_or_else(|| "unmatched".to_owned());
    let method = method_label(req.method()).to_owned();

    let response = next.run(req).await;

    let status = format!("{}xx", response.status().as_u16() / 100);
    let labels = [("method", method), ("route", route), ("status", status)];

    metrics::counter!(REQUESTS_TOTAL, &labels).increment(1);
    metrics::histogram!(REQUEST_DURATION, &labels).record(start.elapsed().as_secs_f64());

    response
}
