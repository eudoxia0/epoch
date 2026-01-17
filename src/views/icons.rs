use axum::Router;
use axum::routing::get;
use http::HeaderName;
use http::StatusCode;
use http::header::CACHE_CONTROL;
use http::header::CONTENT_TYPE;

use crate::server::CACHE_CONTROL_IMMUTABLE;
use crate::server::CONTENT_TYPE_PNG;
use crate::server::ServerState;

pub struct TodayIconView {}

impl TodayIconView {
    pub fn route(router: Router<ServerState>) -> Router<ServerState> {
        router.route(Self::url(), get(Self::handler))
    }

    pub fn url() -> &'static str {
        "/static/today.png"
    }

    async fn handler() -> (StatusCode, [(HeaderName, &'static str); 2], &'static [u8]) {
        let bytes = include_bytes!("../../res/calendar-day.png");
        (
            StatusCode::OK,
            [
                (CONTENT_TYPE, CONTENT_TYPE_PNG),
                (CACHE_CONTROL, CACHE_CONTROL_IMMUTABLE),
            ],
            bytes,
        )
    }
}

pub struct YesterdayIconView {}

impl YesterdayIconView {
    pub fn route(router: Router<ServerState>) -> Router<ServerState> {
        router.route(Self::url(), get(Self::handler))
    }

    pub fn url() -> &'static str {
        "/static/yesterday.png"
    }

    async fn handler() -> (StatusCode, [(HeaderName, &'static str); 2], &'static [u8]) {
        let bytes = include_bytes!("../../res/calendar-month.png");
        (
            StatusCode::OK,
            [
                (CONTENT_TYPE, CONTENT_TYPE_PNG),
                (CACHE_CONTROL, CACHE_CONTROL_IMMUTABLE),
            ],
            bytes,
        )
    }
}
