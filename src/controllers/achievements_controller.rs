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
use crate::server::ServerState;
use crate::types::AchievementId;
use crate::types::AchievementType;
use crate::types::Date;
use crate::types::RichText;

pub struct AchievementsController {}

impl AchievementsController {
    pub fn route(router: Router<ServerState>) -> Router<ServerState> {
        router
            .route("/achievements", get(Self::index))
            .route("/achievements/new", get(Self::new))
            .route("/achievements/new", post(Self::create))
            .route("/achievements/{achievement_id}/delete", post(Self::delete))
    }

    async fn index(State(state): State<ServerState>) -> Fallible<(StatusCode, Html<String>)> {
        let mut db: MutexGuard<'_, Connection> = state.db.lock().await;
        let tx: Transaction<'_> = db.transaction()?;
        let achievements = Achievement::list(&tx)?;
        let tmpl = AchievementsIndexTemplate {
            title: "Achievements",
            achievements: &achievements,
        };
        Ok((StatusCode::OK, Html(tmpl.render()?)))
    }

    async fn new(State(_state): State<ServerState>) -> Fallible<(StatusCode, Html<String>)> {
        let today = Date::today();
        let tmpl = AchievementsEditTemplate {
            title: "New Achievement",
            date: &today.to_string(),
        };
        Ok((StatusCode::OK, Html(tmpl.render()?)))
    }

    async fn create(
        State(state): State<ServerState>,
        Form(form): Form<CreateForm>,
    ) -> Fallible<Redirect> {
        let CreateForm {
            date,
            achievement_type,
            content,
        } = form;
        let date: Date = Date::try_from(date)?;
        let achievement_type: AchievementType = AchievementType::try_from(achievement_type.as_str())?;
        let content: RichText = RichText::new(content)?;
        let mut db: MutexGuard<'_, Connection> = state.db.lock().await;
        let tx: Transaction<'_> = db.transaction()?;
        Achievement::create(&tx, date, achievement_type, content)?;
        tx.commit()?;
        Ok(Redirect::to("/achievements"))
    }

    async fn delete(
        State(state): State<ServerState>,
        Path(achievement_id): Path<AchievementId>,
    ) -> Fallible<Redirect> {
        let mut db: MutexGuard<'_, Connection> = state.db.lock().await;
        let tx: Transaction<'_> = db.transaction()?;
        Achievement::delete(&tx, achievement_id)?;
        tx.commit()?;
        Ok(Redirect::to("/achievements"))
    }
}

#[derive(Deserialize)]
struct CreateForm {
    date: String,
    achievement_type: String,
    content: String,
}

#[derive(Template)]
#[template(path = "achievements_index.jinja2")]
struct AchievementsIndexTemplate<'a> {
    title: &'a str,
    achievements: &'a [Achievement],
}

#[derive(Template)]
#[template(path = "achievements_edit.jinja2")]
struct AchievementsEditTemplate<'a> {
    title: &'a str,
    date: &'a str,
}
