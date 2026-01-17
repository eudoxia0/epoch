use axum::Router;
use axum::extract::Path;
use axum::extract::State;
use axum::response::Html;
use axum::routing::get;
use http::StatusCode;
use maud::Markup;
use maud::PreEscaped;
use maud::html;
use rusqlite::Connection;
use rusqlite::Transaction;
use tokio::sync::MutexGuard;

use crate::db::DayEntry;
use crate::db::get_day_entry;
use crate::error::Fallible;
use crate::server::ServerState;
use crate::templates::page_template;
use crate::types::Date;
use crate::views::EditView;

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
        let body: Markup = match maybe_day_entry {
            Some(day_entry) => {
                html! {
                    (PreEscaped(day_entry.content.as_html()?))
                }
            }
            None => {
                html! {
                    p {
                        i {
                            "No entry for this date."
                        }
                    }
                }
            }
        };
        let weekday = date.weekday();
        let body: Markup = html! {
            .entry {
                .entry-header {
                    .entry-title {
                        h1 {
                            (date.humanize())
                        }
                    }
                }
                .entry-controls {
                    a .button href=(EditView::url(date)) {
                        .icon { "✏️" }
                        "Edit"
                    }
                }
                .entry-body {
                    p {
                        i {
                            (weekday)
                        }
                    }
                    (body)
                }
            }
        };
        let html = page_template(&date.humanize(), body);
        Ok((StatusCode::OK, Html(html.into_string())))
    }
}
