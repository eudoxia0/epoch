use axum::Form;
use axum::Router;
use axum::extract::Path;
use axum::extract::State;
use axum::response::Html;
use axum::response::Redirect;
use axum::routing::get;
use axum::routing::post;
use http::StatusCode;
use maud::Markup;
use maud::html;
use rusqlite::Connection;
use rusqlite::Transaction;
use serde::Deserialize;
use tokio::sync::MutexGuard;

use crate::db::DayEntry;
use crate::db::create_day_entry;
use crate::db::get_day_entry;
use crate::db::update_day_entry;
use crate::error::Fallible;
use crate::server::ServerState;
use crate::templates::page_template;
use crate::types::Date;
use crate::types::EntryText;
use crate::views::GetDayView;

pub struct EditView {}

impl EditView {
    pub fn route(router: Router<ServerState>) -> Router<ServerState> {
        router
            .route("/day/{date}/edit", get(Self::get_handler))
            .route("/day/{date}/edit", post(Self::post_handler))
    }

    pub fn url(date: Date) -> String {
        format!("/day/{date}/edit")
    }

    async fn get_handler(
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

    async fn post_handler(
        State(state): State<ServerState>,
        Path(date): Path<Date>,
        Form(form): Form<FormData>,
    ) -> Fallible<Redirect> {
        let content: EntryText = EntryText::new(form.content)?;
        let mut db: MutexGuard<'_, Connection> = state.db.lock().await;
        let tx: Transaction<'_> = db.transaction()?;
        match get_day_entry(&tx, date)? {
            Some(_) => update_day_entry(&tx, date, &content)?,
            None => create_day_entry(&tx, date, &content)?,
        };
        tx.commit()?;
        Ok(Redirect::to(&GetDayView::url(date)))
    }
}

#[derive(Deserialize)]
struct FormData {
    content: String,
}
