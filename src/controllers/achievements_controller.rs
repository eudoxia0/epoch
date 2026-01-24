use axum::Form;
use axum::Router;
use axum::extract::Path;
use axum::extract::State;
use axum::response::Html;
use axum::response::Redirect;
use axum::routing::get;
use axum::routing::post;
use http::StatusCode;
use serde::Deserialize;

use crate::error::Fallible;
use crate::server::ServerState;
use crate::types::AchievementId;

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
        todo!()
    }

    async fn new(State(state): State<ServerState>) -> Fallible<(StatusCode, Html<String>)> {
        todo!()
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
        todo!()
    }

    async fn delete(
        State(state): State<ServerState>,
        Path(achievement_id): Path<AchievementId>,
    ) -> Fallible<Redirect> {
        todo!()
    }
}

#[derive(Deserialize)]
struct CreateForm {
    date: String,
    achievement_type: String,
    content: String,
}
