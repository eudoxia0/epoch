use askama::Template;
use axum::Form;
use axum::Router;
use axum::extract::Path;
use axum::extract::Query;
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
            .route("/achievements/new", get(Self::create_get))
            .route("/achievements/new", post(Self::create_post))
            .route("/achievements/{achievement_id}/edit", get(Self::edit_get))
            .route("/achievements/{achievement_id}/edit", post(Self::edit_post))
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

    async fn create_get(
        State(_state): State<ServerState>,
        Query(params): Query<NewQueryParams>,
    ) -> Fallible<(StatusCode, Html<String>)> {
        let date = params.date.unwrap_or_else(Date::today);
        let return_url = params.return_url.as_deref();
        let tmpl = AchievementsEditTemplate {
            title: "New Achievement",
            date: &date.to_string(),
            achievement_type: None,
            content: None,
            return_url,
        };
        Ok((StatusCode::OK, Html(tmpl.render()?)))
    }

    async fn create_post(
        State(state): State<ServerState>,
        Form(form): Form<CreateForm>,
    ) -> Fallible<Redirect> {
        let CreateForm {
            date,
            achievement_type,
            content,
            return_url,
        } = form;
        let date: Date = Date::try_from(date)?;
        let achievement_type: AchievementType =
            AchievementType::try_from(achievement_type.as_str())?;
        let content: RichText = RichText::new(content);
        let mut db: MutexGuard<'_, Connection> = state.db.lock().await;
        let tx: Transaction<'_> = db.transaction()?;
        Achievement::create(&tx, date, achievement_type, content)?;
        tx.commit()?;
        let redirect_url = return_url.as_deref().unwrap_or("/achievements");
        Ok(Redirect::to(redirect_url))
    }

    async fn edit_get(
        State(state): State<ServerState>,
        Path(achievement_id): Path<AchievementId>,
    ) -> Fallible<(StatusCode, Html<String>)> {
        let mut db: MutexGuard<'_, Connection> = state.db.lock().await;
        let tx: Transaction<'_> = db.transaction()?;
        let achievement = Achievement::get(&tx, achievement_id)?;
        let date = achievement.date.to_string();
        let achievement_type = achievement.achievement_type.as_str();
        let content = achievement.content.as_str();
        let tmpl = AchievementsEditTemplate {
            title: "Edit Achievement",
            date: &date,
            achievement_type: Some(achievement_type),
            content: Some(content),
            return_url: None,
        };
        Ok((StatusCode::OK, Html(tmpl.render()?)))
    }

    async fn edit_post(
        State(state): State<ServerState>,
        Path(achievement_id): Path<AchievementId>,
        Form(form): Form<UpdateForm>,
    ) -> Fallible<Redirect> {
        let UpdateForm {
            date,
            achievement_type,
            content,
        } = form;
        let date: Date = Date::try_from(date)?;
        let achievement_type: AchievementType =
            AchievementType::try_from(achievement_type.as_str())?;
        let content: RichText = RichText::new(content);
        let mut db: MutexGuard<'_, Connection> = state.db.lock().await;
        let tx: Transaction<'_> = db.transaction()?;
        Achievement::update(&tx, achievement_id, date, achievement_type, content)?;
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
struct NewQueryParams {
    date: Option<Date>,
    return_url: Option<String>,
}

#[derive(Deserialize)]
struct CreateForm {
    date: String,
    achievement_type: String,
    content: String,
    return_url: Option<String>,
}

#[derive(Deserialize)]
struct UpdateForm {
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
    achievement_type: Option<&'a str>,
    content: Option<&'a str>,
    return_url: Option<&'a str>,
}
