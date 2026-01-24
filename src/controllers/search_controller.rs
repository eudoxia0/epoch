use askama::Template;
use axum::Router;
use axum::extract::Query;
use axum::extract::State;
use axum::response::Html;
use axum::routing::get;
use http::StatusCode;
use rusqlite::Connection;
use rusqlite::Transaction;
use rusqlite::params;
use serde::Deserialize;
use tokio::sync::MutexGuard;

use crate::controllers::DayController;
use crate::error::Fallible;
use crate::server::ServerState;
use crate::types::Date;

pub struct SearchController {}

impl SearchController {
    pub fn route(router: Router<ServerState>) -> Router<ServerState> {
        router.route("/search", get(Self::index))
    }

    async fn index(
        State(state): State<ServerState>,
        Query(params): Query<SearchParams>,
    ) -> Fallible<(StatusCode, Html<String>)> {
        let SearchParams { q } = params;
        let results: Vec<SearchResult> = match &q {
            Some(query) => {
                let mut db: MutexGuard<'_, Connection> = state.db.lock().await;
                let tx: Transaction<'_> = db.transaction()?;
                let dates = search_day_entries(&tx, query)?;
                dates
                    .into_iter()
                    .map(|date| SearchResult {
                        url: DayController::url_show(date),
                        display: date.humanize(),
                    })
                    .collect()
            }
            None => Vec::new(),
        };
        let tmpl = SearchIndexTemplate {
            title: "Search",
            query: q.as_deref().unwrap_or(""),
            results,
        };
        Ok((StatusCode::OK, Html(tmpl.render()?)))
    }
}

#[derive(Deserialize)]
struct SearchParams {
    q: Option<String>,
}

struct SearchResult {
    url: String,
    display: String,
}

#[derive(Template)]
#[template(path = "search_index.jinja2")]
struct SearchIndexTemplate<'a> {
    title: &'a str,
    query: &'a str,
    results: Vec<SearchResult>,
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
