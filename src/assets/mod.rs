use axum::Router;
use axum::response::IntoResponse;
use axum::routing::get;
use http::StatusCode;
use http::header::CACHE_CONTROL;
use http::header::CONTENT_TYPE;

use crate::server::CACHE_CONTROL_IMMUTABLE;
use crate::server::CACHE_CONTROL_NONE;
use crate::server::CONTENT_TYPE_CSS;
use crate::server::CONTENT_TYPE_PNG;
use crate::server::ServerState;

pub fn route_assets(router: Router<ServerState>) -> Router<ServerState> {
    router
        .route("/static/style.css", get(css_handler))
        .route("/static/reset.css", get(css_reset_handler))
        .route("/favicon.ico", get(favicon_handler))
}

async fn css_handler() -> impl IntoResponse {
    (
        StatusCode::OK,
        [
            (CONTENT_TYPE, CONTENT_TYPE_CSS),
            (CACHE_CONTROL, CACHE_CONTROL_NONE),
        ],
        include_str!("style.css"),
    )
}

async fn css_reset_handler() -> impl IntoResponse {
    (
        StatusCode::OK,
        [
            (CONTENT_TYPE, CONTENT_TYPE_CSS),
            (CACHE_CONTROL, CACHE_CONTROL_NONE),
        ],
        include_str!("reset.css"),
    )
}

async fn favicon_handler() -> impl IntoResponse {
    let bytes = include_bytes!("favicon.png");
    (
        StatusCode::OK,
        [
            (CONTENT_TYPE, CONTENT_TYPE_PNG),
            (CACHE_CONTROL, CACHE_CONTROL_IMMUTABLE),
        ],
        bytes,
    )
}
