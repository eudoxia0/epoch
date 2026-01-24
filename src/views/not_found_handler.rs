use askama::Template;
use axum::response::Html;
use http::StatusCode;

use crate::error::Fallible;

pub async fn not_found_handler() -> Fallible<(StatusCode, Html<String>)> {
    let tmpl = NotFoundTemplate { title: "Not Found" };
    Ok((StatusCode::NOT_FOUND, Html(tmpl.render()?)))
}

#[derive(Template)]
#[template(path = "not_found.jinja2")]
struct NotFoundTemplate<'a> {
    title: &'a str,
}
