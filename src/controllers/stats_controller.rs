use askama::Template;
use axum::Router;
use axum::extract::State;
use axum::response::Html;
use axum::routing::get;
use http::StatusCode;
use rusqlite::Connection;
use rusqlite::Transaction;
use tokio::sync::MutexGuard;

use crate::error::Fallible;
use crate::models::DayEntry;
use crate::server::ServerState;

pub struct StatsController {}

impl StatsController {
    pub fn route(router: Router<ServerState>) -> Router<ServerState> {
        router.route("/stats", get(Self::index))
    }

    async fn index(State(state): State<ServerState>) -> Fallible<(StatusCode, Html<String>)> {
        let mut db: MutexGuard<'_, Connection> = state.db.lock().await;
        let tx: Transaction<'_> = db.transaction()?;
        let title: &str = "Stats";
        let entries: i32 = DayEntry::count(&tx)?;
        let words: i32 = DayEntry::count_words(&tx)?;
        let tmpl: StatsIndexTemplate = StatsIndexTemplate {
            title,
            entries,
            words,
        };
        Ok((StatusCode::OK, Html(tmpl.render()?)))
    }
}

#[derive(Template)]
#[template(path = "stats_index.jinja2")]
struct StatsIndexTemplate<'a> {
    title: &'a str,
    entries: i32,
    words: i32,
}
