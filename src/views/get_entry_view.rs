use axum::Router;
use axum::extract::Path;
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
use crate::types::Date;

pub struct GetEntryView {}

impl GetEntryView {
    pub fn route(router: Router<ServerState>) -> Router<ServerState> {
        router.route("/day/{date}", get(Self::handler))
    }

    pub fn url(date: Date) -> String {
        format!("/day/{date}")
    }

    async fn handler(
        State(state): State<ServerState>,
        Path(date): Path<Date>,
    ) -> Fallible<(StatusCode, Html<String>)> {
        let mut db: MutexGuard<'_, Connection> = state.db.lock().await;
        let _tx: Transaction<'_> = db.transaction()?;
        let body = html! {
            h1 {
                (date)
            }
        };
        let html = page_template(&date.to_string(), body);
        Ok((StatusCode::OK, Html(html.into_string())))
    }
}
