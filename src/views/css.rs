use axum::Router;
use axum::routing::get;
use http::HeaderName;
use http::StatusCode;
use http::header::CACHE_CONTROL;
use http::header::CONTENT_TYPE;

use crate::server::CACHE_CONTROL_NONE;
use crate::server::CONTENT_TYPE_CSS;
use crate::server::ServerState;

pub struct CssView {}

impl CssView {
    pub fn route(router: Router<ServerState>) -> Router<ServerState> {
        router.route(Self::url(), get(Self::handler))
    }

    pub fn url() -> &'static str {
        "/static/style.css"
    }

    async fn handler() -> (StatusCode, [(HeaderName, &'static str); 2], &'static [u8]) {
        let bytes = include_bytes!("../../res/style.css");
        (
            StatusCode::OK,
            [
                (CONTENT_TYPE, CONTENT_TYPE_CSS),
                (CACHE_CONTROL, CACHE_CONTROL_NONE),
            ],
            bytes,
        )
    }
}
