use std::time::Duration;

use axum::{body::Body, Router};

use tower_http::trace::{DefaultOnRequest, TraceLayer};
use tower_request_id::RequestId;

use http::Response;
use hyper::Request;
use tracing::{Level, Span};

use crate::{error::Result, routes};

pub fn create_app() -> Result<Router> {
    let router = routes::routes();
    let app = router
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<Body>| {
                    let request_id = request
                        .extensions()
                        .get::<RequestId>()
                        .map(ToString::to_string)
                        .unwrap_or_else(|| "unknown".into());

                    tracing::info_span!(
                        "request",
                        id = %request_id,
                        method = %request.method(),
                        uri = %request.uri()
                    )
                })
                .on_request(DefaultOnRequest::new().level(Level::INFO)),
        )
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|_request: &Request<Body>| tracing::info_span!("response"))
                .on_response(
                    |_response: &Response<Body>, _latency: Duration, _span: &Span| {
                        tracing::info!("response generated");
                    },
                ),
        );

    Ok(app)
}
