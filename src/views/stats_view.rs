use axum::Router;
use axum::extract::State;
use axum::response::Html;
use axum::routing::get;
use http::StatusCode;
use maud::Markup;
use maud::html;
use rusqlite::Connection;
use rusqlite::Transaction;
use tokio::sync::MutexGuard;

use crate::error::Fallible;
use crate::server::ServerState;
use crate::templates::page_template;

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
        let body: Markup = html! {
            .stats-page {
                p {
                    "Entries: " (count_entries(&tx)?)
                }
                p {
                    "Words: " (count_words(&tx)?)
                }
            }
        };
        let html = page_template(&"Stats", body);
        Ok((StatusCode::OK, Html(html.into_string())))
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
            sum(word_count)
        from
            day_entries;
    ";
    let result: i32 = tx.query_one(sql, [], |row| row.get(0))?;
    Ok(result)
}
