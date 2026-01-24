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
use crate::server::ServerState;

pub struct StatsView {}

impl StatsView {
    pub fn route(router: Router<ServerState>) -> Router<ServerState> {
        router.route(Self::url(), get(Self::handler))
    }

    pub fn url() -> &'static str {
        "/stats"
    }

    async fn handler(State(state): State<ServerState>) -> Fallible<(StatusCode, Html<String>)> {
        let mut db: MutexGuard<'_, Connection> = state.db.lock().await;
        let tx: Transaction<'_> = db.transaction()?;
        let title: &str = "Stats";
        let entries: i32 = count_entries(&tx)?;
        let words: i32 = count_words(&tx)?;
        let tmpl: StatsIndexTemplate = StatsIndexTemplate {
            title,
            entries,
            words,
        };
        Ok((StatusCode::OK, Html(tmpl.render()?)))
    }
}

/// Return the total number of entries.
fn count_entries(tx: &Transaction<'_>) -> Fallible<i32> {
    let sql = "
        select
            count(*)
        from
            day_entries;
    ";
    let result: i32 = tx.query_one(sql, [], |row| row.get(0))?;
    Ok(result)
}

/// Return the total number of words across all entries.
fn count_words(tx: &Transaction<'_>) -> Fallible<i32> {
    let sql = "
        select
            coalesce(sum(word_count), 0)
        from
            day_entries;
    ";
    let result: i32 = tx.query_one(sql, [], |row| row.get(0))?;
    Ok(result)
}

#[derive(Template)]
#[template(path = "stats_index.jinja2")]
struct StatsIndexTemplate<'a> {
    title: &'a str,
    entries: i32,
    words: i32,
}
