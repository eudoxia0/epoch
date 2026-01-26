use askama::Template;
use axum::Form;
use axum::Router;
use axum::extract::Path;
use axum::extract::State;
use axum::response::Html;
use axum::response::Redirect;
use axum::routing::get;
use axum::routing::post;
use http::StatusCode;
use rusqlite::Connection;
use rusqlite::Transaction;
use serde::Deserialize;
use tokio::sync::MutexGuard;

use crate::error::Fallible;
use crate::models::Achievement;
use crate::models::DayEntry;
use crate::server::ServerState;
use crate::types::Date;
use crate::types::RichText;

pub struct DayController {}

impl DayController {
    pub fn route(router: Router<ServerState>) -> Router<ServerState> {
        router
            .route("/day/today", get(Self::today))
            .route("/day/yesterday", get(Self::yesterday))
            .route("/day/{date}", get(Self::show))
            .route("/day/{date}/edit", get(Self::edit))
            .route("/day/{date}/edit", post(Self::update))
    }

    pub fn url_show(date: Date) -> String {
        format!("/day/{date}")
    }

    pub fn url_edit(date: Date) -> String {
        format!("/day/{date}/edit")
    }

    async fn today() -> Redirect {
        let today: Date = Date::today();
        Redirect::to(&DayController::url_show(today))
    }

    async fn yesterday() -> Redirect {
        let today: Date = Date::today();
        Redirect::to(&DayController::url_show(today.previous()))
    }

    async fn show(
        State(state): State<ServerState>,
        Path(date): Path<Date>,
    ) -> Fallible<(StatusCode, Html<String>)> {
        let mut db: MutexGuard<'_, Connection> = state.db.lock().await;
        let tx: Transaction<'_> = db.transaction()?;
        let maybe_day_entry: Option<DayEntry> = DayEntry::get_opt(&tx, date)?;
        let (has_entry, content) = match maybe_day_entry {
            Some(day_entry) => (true, day_entry.content.as_html()?),
            None => (false, String::new()),
        };
        let achievements = Achievement::list_by_date(&tx, date)?;
        let weekday = date.weekday();
        let tmpl = DayViewTemplate {
            title: &date.humanize(),
            weekday: &weekday.to_string(),
            has_entry,
            content: &content,
            edit_url: &DayController::url_edit(date),
            achievements: &achievements,
        };
        Ok((StatusCode::OK, Html(tmpl.render()?)))
    }

    async fn edit(
        State(state): State<ServerState>,
        Path(date): Path<Date>,
    ) -> Fallible<(StatusCode, Html<String>)> {
        let mut db: MutexGuard<'_, Connection> = state.db.lock().await;
        let tx: Transaction<'_> = db.transaction()?;
        let maybe_day_entry: Option<DayEntry> = DayEntry::get_opt(&tx, date)?;
        let text: String = match maybe_day_entry {
            Some(de) => de.content.as_str().to_string(),
            None => String::new(),
        };
        let tmpl = DayEditTemplate {
            title: &date.humanize(),
            text: &text,
            back_url: &DayController::url_show(date),
        };
        Ok((StatusCode::OK, Html(tmpl.render()?)))
    }

    async fn update(
        State(state): State<ServerState>,
        Path(date): Path<Date>,
        Form(form): Form<UpdateForm>,
    ) -> Fallible<Redirect> {
        let content: RichText = RichText::new(form.content)?;
        let mut db: MutexGuard<'_, Connection> = state.db.lock().await;
        let tx: Transaction<'_> = db.transaction()?;
        match DayEntry::get_opt(&tx, date)? {
            Some(_) => DayEntry::update(&tx, date, &content)?,
            None => DayEntry::create(&tx, date, &content)?,
        };
        tx.commit()?;
        Ok(Redirect::to(&DayController::url_show(date)))
    }
}

#[derive(Deserialize)]
struct UpdateForm {
    content: String,
}

#[derive(Template)]
#[template(path = "day_view.jinja2")]
struct DayViewTemplate<'a> {
    title: &'a str,
    weekday: &'a str,
    has_entry: bool,
    content: &'a str,
    edit_url: &'a str,
    achievements: &'a [Achievement],
}

#[derive(Template)]
#[template(path = "day_edit.jinja2")]
struct DayEditTemplate<'a> {
    title: &'a str,
    text: &'a str,
    back_url: &'a str,
}
