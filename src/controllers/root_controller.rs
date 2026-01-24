use askama::Template;
use axum::Router;
use axum::response::Html;
use axum::response::Redirect;
use axum::routing::get;
use http::StatusCode;

use crate::controllers::DayController;
use crate::error::Fallible;
use crate::server::ServerState;
use crate::types::Date;

pub struct RootController {}

impl RootController {
    pub fn route(router: Router<ServerState>) -> Router<ServerState> {
        router.route("/", get(Self::index))
    }

    async fn index() -> Redirect {
        let today: Date = Date::today();
        Redirect::to(&DayController::url_show(today))
    }

    pub async fn not_found() -> Fallible<(StatusCode, Html<String>)> {
        let tmpl = NotFoundTemplate { title: "Not Found" };
        Ok((StatusCode::NOT_FOUND, Html(tmpl.render()?)))
    }
}

#[derive(Template)]
#[template(path = "not_found.jinja2")]
struct NotFoundTemplate<'a> {
    title: &'a str,
}
