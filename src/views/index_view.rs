use axum::Router;
use axum::extract::State;
use axum::response::Html;
use axum::routing::get;
use http::StatusCode;
use maud::html;
use rusqlite::Connection;
use rusqlite::Transaction;
use tokio::sync::MutexGuard;

use crate::error::Fallible;
use crate::server::ServerState;
use crate::templates::page_template;

pub struct IndexView {}

impl IndexView {
    pub fn route(router: Router<ServerState>) -> Router<ServerState> {
        router.route(Self::url(), get(Self::handler))
    }

    pub fn url() -> &'static str {
        "/"
    }

    async fn handler(State(state): State<ServerState>) -> Fallible<(StatusCode, Html<String>)> {
        let mut db: MutexGuard<'_, Connection> = state.db.lock().await;
        let _tx: Transaction<'_> = db.transaction()?;
        let body = html! {
            h1 {
                "Hello, world!"
            }
        };
        let html = page_template("Hello, world!", body);
        Ok((StatusCode::OK, Html(html.into_string())))
    }
}
