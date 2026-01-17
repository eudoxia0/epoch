use maud::DOCTYPE;
use maud::Markup;
use maud::html;

use crate::views::CssView;

pub fn page_template(title: &str, body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                link rel="stylesheet" href=(CssView::url());
                title { (title) }
            }
            body {
                (body)
            }
        }
    }
}
