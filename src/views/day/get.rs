use askama::Template;
use axum::Router;
use axum::extract::Path;
use axum::extract::State;
use axum::response::Html;
use axum::routing::get;
use http::StatusCode;
use rusqlite::Connection;
use rusqlite::Transaction;
use tokio::sync::MutexGuard;

use crate::db::DayEntry;
use crate::db::get_day_entry;
use crate::error::Fallible;
use crate::server::ServerState;
use crate::types::Date;
use crate::views::EditDayView;

pub struct GetDayView {}

impl GetDayView {
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
        let tx: Transaction<'_> = db.transaction()?;
        let maybe_day_entry: Option<DayEntry> = get_day_entry(&tx, date)?;
        let (has_entry, content) = match maybe_day_entry {
            Some(day_entry) => (true, day_entry.content.as_html()?),
            None => (false, String::new()),
        };
        let weekday = date.weekday();
        let tmpl = DayViewTemplate {
            title: &date.humanize(),
            weekday: &weekday.to_string(),
            has_entry,
            content: &content,
            edit_url: &EditDayView::url(date),
        };
        Ok((StatusCode::OK, Html(tmpl.render()?)))
    }
}

#[derive(Template)]
#[template(path = "day_view.jinja2")]
struct DayViewTemplate<'a> {
    title: &'a str,
    weekday: &'a str,
    has_entry: bool,
    content: &'a str,
    edit_url: &'a str,
}
