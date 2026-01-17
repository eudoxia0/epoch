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
use rusqlite::params;
use serde::Deserialize;
use tokio::sync::MutexGuard;

use crate::db::DayEntry;
use crate::db::get_day_entry;
use crate::error::Fallible;
use crate::server::ServerState;
use crate::templates::page_template;
use crate::types::Date;
use crate::types::EntryText;
use crate::views::GetDayView;

pub struct EditDayView {}

impl EditDayView {
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
        let body: Markup = html! {
            .edit-entry-page {
                form .entry-form method="POST" {
                    textarea name="content" autofocus {
                        (text)
                    }
                    button .button {
                        .icon { "💾" }
                        "Save"
                    }
                }
            }
        };
        let controls = html! {
            a .button href=(GetDayView::url(date)) {
                .icon { "🔙" }
                "Back"
            }
        };
        let html = page_template(&date.humanize(), Some(controls), body);
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

/// Create a day entry.
fn create_day_entry(tx: &Transaction<'_>, date: Date, content: &EntryText) -> Fallible<()> {
    let word_count: i32 = content.word_count();
    let sql = "
        insert into day_entries
            (date, content, word_count)
        values
            (?1, ?2, ?3);
    ";
    tx.execute(sql, params![date, content, word_count])?;
    Ok(())
}

/// Update a day entry.
fn update_day_entry(tx: &Transaction<'_>, date: Date, content: &EntryText) -> Fallible<()> {
    let word_count: i32 = content.word_count();
    let sql = "
        update day_entries set
            content    = ?2,
            word_count = ?3
        where
            date = ?1;
    ";
    tx.execute(sql, params![date, content, word_count])?;
    Ok(())
}
