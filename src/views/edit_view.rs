use axum::Router;
use axum::extract::Path;
use axum::extract::State;
use axum::response::Html;
use axum::routing::get;
use http::StatusCode;
use maud::Markup;
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
use crate::views::GetDayView;

pub struct EditView {}

impl EditView {
    pub fn route(router: Router<ServerState>) -> Router<ServerState> {
        router.route("/day/{date}/edit", get(Self::handler))
    }

    pub fn url(date: Date) -> String {
        format!("/day/{date}/edit")
    }

    async fn handler(
        State(state): State<ServerState>,
        Path(date): Path<Date>,
    ) -> Fallible<(StatusCode, Html<String>)> {
        let mut db: MutexGuard<'_, Connection> = state.db.lock().await;
        let tx: Transaction<'_> = db.transaction()?;
        let maybe_day_entry: Option<DayEntry> = get_day_entry(&tx, date)?;
        let text: String = match maybe_day_entry {
            Some(de) => de.content.as_str().to_string(),
            None => String::new(),
        };
        let day = date.day();
        let month = date.month_name();
        let year = date.year();
        let weekday = date.weekday();
        let body: Markup = html! {
            .entry {
                .entry-header {
                    .entry-title {
                        h1 {
                            (day) " " (month) ", " (year)
                        }
                    }
                    .entry-subtitle {
                        (weekday)
                    }
                }
                .entry-controls {
                    a .button href=(GetDayView::url(date)) {
                        .icon { "🔙" }
                        "Back"
                    }
                }
                .entry-body .entry-edit {
                    form .entry-form method="POST" {
                        textarea name="content" {
                            (text)
                        }
                        button .button {
                            .icon { "💾" }
                            "Save"
                        }
                    }
                }
            }
        };
        let title: String = format!("{day} {month}, {year}");
        let html = page_template(&title, body);
        Ok((StatusCode::OK, Html(html.into_string())))
    }
}
