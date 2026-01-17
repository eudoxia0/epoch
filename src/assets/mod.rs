use axum::Router;
use axum::response::IntoResponse;
use axum::routing::get;
use http::StatusCode;
use http::header::CACHE_CONTROL;
use http::header::CONTENT_TYPE;

use crate::server::CACHE_CONTROL_NONE;
use crate::server::CONTENT_TYPE_CSS;
use crate::server::ServerState;

pub const CSS_URL: &str = "/static/style.css";

pub fn route_assets(router: Router<ServerState>) -> Router<ServerState> {
    router.route(CSS_URL, get(async || serve_css(include_str!("style.css"))))
}

fn serve_css(text: &'static str) -> impl IntoResponse {
    (
        StatusCode::OK,
        [
            (CONTENT_TYPE, CONTENT_TYPE_CSS),
            (CACHE_CONTROL, CACHE_CONTROL_NONE),
        ],
        text,
    )
}
