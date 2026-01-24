use askama::Template;

mod page;

pub use page::page_template;

#[derive(Template)]
#[template(path = "stats_index.jinja2")]
pub struct StatsIndexTemplate {
    pub title: String,
    pub entries: i32,
    pub words: i32,
}
