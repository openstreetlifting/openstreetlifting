use axum::{
    Router,
    extract::{MatchedPath, Request},
    middleware::Next,
    response::Response,
    routing::get,
};
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};
use std::time::Instant;

const REQUESTS_TOTAL: &str = "http_server_requests_total";
const REQUEST_DURATION: &str = "http_server_request_duration_seconds";
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

pub fn init_recorder() -> anyhow::Result<PrometheusHandle> {
    let handle = PrometheusBuilder::new()
        .set_buckets_for_metric(Matcher::Full(REQUEST_DURATION.into()), LATENCY_BUCKETS)?
        .install_recorder()?;
    metrics::describe_counter!(REQUESTS_TOTAL, "Completed HTTP requests");
    metrics::describe_histogram!(
        REQUEST_DURATION,
        metrics::Unit::Seconds,
        "Time until response headers, including middleware and database work"
    );
    metrics::describe_gauge!(
        "http_server_active_requests",
        "Requests currently waiting for response headers"
    );
    Ok(handle)
}

pub fn metrics_router(handle: PrometheusHandle, db: std::sync::Arc<osl_db::Database>) -> Router {
    Router::new().route(
        "/metrics",
        get(move || {
            let handle = handle.clone();
            let db = db.clone();
            async move {
                db.record_pool_metrics();
                (
                    [(
                        axum::http::header::CONTENT_TYPE,
                        "text/plain; version=0.0.4; charset=utf-8",
                    )],
                    handle.render(),
                )
            }
        }),
    )
}

struct ActiveRequest(metrics::Gauge);

impl Drop for ActiveRequest {
    fn drop(&mut self) {
        self.0.decrement(1.0);
    }
}

pub async fn track_http(req: Request, next: Next) -> Response {
    let start = Instant::now();

    let route = req
        .extensions()
        .get::<MatchedPath>()
        .map_or_else(|| "unmatched".to_owned(), |path| path.as_str().to_owned());
    let method = method_label(req.method()).to_owned();
    let active = metrics::gauge!("http_server_active_requests", "http_request_method" => method.clone(), "http_route" => route.clone());
    active.increment(1.0);

    let _active = ActiveRequest(active);

    let response = next.run(req).await;

    let status = response.status().as_u16().to_string();
    let labels = [
        ("http_request_method", method),
        ("http_route", route),
        ("http_response_status_code", status),
    ];

    metrics::counter!(REQUESTS_TOTAL, &labels).increment(1);
    metrics::histogram!(REQUEST_DURATION, &labels).record(start.elapsed().as_secs_f64());

    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Method, Request as HttpRequest},
        middleware::from_fn,
        routing::get,
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn records_bounded_request_metrics() {
        let recorder = PrometheusBuilder::new().build_recorder();
        let handle = recorder.handle();
        let _recorder = metrics::set_default_local_recorder(&recorder);
        let router = Router::new()
            .route("/athletes/{slug}", get(|| async { "ok" }))
            .layer(from_fn(track_http));
        for uri in ["/athletes/one", "/athletes/two", "/not-a-route"] {
            router
                .clone()
                .oneshot(HttpRequest::builder().uri(uri).body(Body::empty()).unwrap())
                .await
                .unwrap();
        }
        let output = handle.render();
        assert!(output.contains("http_route=\"/athletes/{slug}\""));
        assert!(output.contains("http_route=\"unmatched\""));
        assert!(output.contains("http_response_status_code=\"404\""));
        assert!(output.contains("http_server_request_duration_seconds"));
        assert!(!output.contains("/athletes/one"));
        assert!(!output.contains("/athletes/two"));
        for line in output
            .lines()
            .filter(|line| line.starts_with("http_server_active_requests{"))
        {
            assert!(line.ends_with(" 0"), "{line}");
        }
        assert_eq!(
            method_label(&Method::from_bytes(b"CUSTOM").unwrap()),
            "_OTHER"
        );
    }

    #[tokio::test]
    async fn records_metrics_through_application_router() {
        let recorder = PrometheusBuilder::new().build_recorder();
        let handle = recorder.handle();
        let _recorder = metrics::set_default_local_recorder(&recorder);
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy("postgres://localhost/unused")
            .unwrap();
        let state = crate::AppState::new(osl_db::Database::from_pool(pool), false);
        let response = crate::app(state)
            .oneshot(
                HttpRequest::builder()
                    .uri("/health/live")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), axum::http::StatusCode::OK);
        let output = handle.render();
        assert!(output.contains("http_server_requests"), "{output}");
        assert!(output.contains("http_route=\"/health/live\""), "{output}");
    }
}
