use axum::response::Html;
use http::StatusCode;
use maud::html;

use crate::error::Fallible;
use crate::templates::page_template;

pub async fn not_found_handler() -> Fallible<(StatusCode, Html<String>)> {
    let body = html! {
        h1 {
            "404 Not Found!"
        }
    };
    let html = page_template("Not Found", None, body);
    Ok((StatusCode::NOT_FOUND, Html(html.into_string())))
}
