use std::time::Duration;

use axum::{
    Router,
    extract::MatchedPath,
    http::{Request, Response},
};
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};
use tracing::{Span, info, info_span};

use crate::state::AppState;

pub fn apply_common_layers(router: Router<AppState>) -> Router<AppState> {
    router.layer(
        ServiceBuilder::new()
            .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
            .layer(PropagateRequestIdLayer::x_request_id())
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(|request: &Request<_>| {
                        let matched_path = request
                            .extensions()
                            .get::<MatchedPath>()
                            .map(MatchedPath::as_str)
                            .unwrap_or("-");

                        info_span!(
                            "http_request",
                            method = %request.method(),
                            path = %request.uri().path(),
                            matched_path = matched_path,
                        )
                    })
                    .on_request(())
                    .on_response(
                        |response: &Response<_>, latency: Duration, span: &Span| {
                            info!(
                                parent: span,
                                status = response.status().as_u16(),
                                latency_ms = latency.as_millis(),
                                "request completed"
                            );
                        },
                    ),
            )
            .layer(CompressionLayer::new())
            .layer(CorsLayer::permissive()),
    )
}
