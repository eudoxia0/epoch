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

use crate::db::DayEntryData;
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
            .route("/day/{date}/edit", get(Self::edit_get))
            .route("/day/{date}/edit", post(Self::edit_post))
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
        let maybe_day_entry: Option<DayEntryData> = state.db2.get_day_entry(date).await?;
        let has_entry: bool = maybe_day_entry.is_some();
        let (content, achievements) = maybe_day_entry
            .map(|de| (de.content, de.achievements))
            .unwrap_or((RichText::new(""), vec![]));
        let weekday = date.weekday();
        let prev_url = DayController::url_show(date.previous());
        let next_url = DayController::url_show(date.next());
        let tmpl = DayViewTemplate {
            title: &date.humanize(),
            weekday,
            has_entry,
            content: &content.as_html()?,
            edit_url: &DayController::url_edit(date),
            prev_url,
            next_url,
            achievements: &achievements,
            date: &date.to_string(),
        };
        Ok((StatusCode::OK, Html(tmpl.render()?)))
    }

    async fn edit_get(
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

    async fn edit_post(
        State(state): State<ServerState>,
        Path(date): Path<Date>,
        Form(form): Form<UpdateForm>,
    ) -> Fallible<Redirect> {
        let content: RichText = RichText::new(form.content);
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
    prev_url: String,
    next_url: String,
    achievements: &'a [Achievement],
    date: &'a str,
}

#[derive(Template)]
#[template(path = "day_edit.jinja2")]
struct DayEditTemplate<'a> {
    title: &'a str,
    text: &'a str,
    back_url: &'a str,
}
