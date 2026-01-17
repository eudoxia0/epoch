use axum::Router;
use axum::extract::Query;
use axum::extract::State;
use axum::response::Html;
use axum::routing::get;
use http::StatusCode;
use maud::Markup;
use maud::html;
use rusqlite::Connection;
use rusqlite::Transaction;
use rusqlite::params;
use serde::Deserialize;
use tokio::sync::MutexGuard;

use crate::error::Fallible;
use crate::server::ServerState;
use crate::templates::page_template;
use crate::types::Date;
use crate::views::GetDayView;

pub struct SearchView {}

impl SearchView {
    pub fn route(router: Router<ServerState>) -> Router<ServerState> {
        router.route(Self::url(), get(Self::handler))
    }

    pub fn url() -> &'static str {
        "/search"
    }

    async fn handler(
        State(state): State<ServerState>,
        Query(params): Query<SearchParams>,
    ) -> Fallible<(StatusCode, Html<String>)> {
        let SearchParams { q } = params;
        let results: Markup = match &q {
            Some(q) => {
                let mut db: MutexGuard<'_, Connection> = state.db.lock().await;
                let tx: Transaction<'_> = db.transaction()?;
                let results = search_day_entries(&tx, q)?;
                html! {
                    .results {
                        @for date in results {
                            .result {
                                a href=(GetDayView::url(date)) {
                                    (date.humanize())
                                }
                            }
                        }
                    }
                }
            }
            None => html! {},
        };
        let body: Markup = html! {
            .search-page {
                form .search-form method="GET" {
                    input type="text" name="q" value=(q.unwrap_or("".to_string())) autofocus;
                    button .button type="Submit" {
                        .icon {
                            "🔍"
                        }
                        "Search"
                    }
                }
                (results)
            }
        };
        let html = page_template("Search", None, body);
        Ok((StatusCode::OK, Html(html.into_string())))
    }
}

#[derive(Deserialize)]
struct SearchParams {
    q: Option<String>,
}

pub fn search_day_entries(tx: &Transaction<'_>, query: &str) -> Fallible<Vec<Date>> {
    let sql = "
        select
            date
        from
            day_entries
        where
            lower(content) like lower(?1)
        order by
            date asc;
    ";
    let mut stmt = tx.prepare(sql)?;
    let rows = stmt.query_map(params![format!("%{}%", query)], |row| row.get(0))?;
    let mut entries = Vec::new();
    for entry in rows {
        entries.push(entry?);
    }
    Ok(entries)
}
