use maud::DOCTYPE;
use maud::Markup;
use maud::html;

use crate::types::Date;
use crate::views::CssView;
use crate::views::GetEntryView;
use crate::views::TodayIconView;
use crate::views::YesterdayIconView;

pub fn page_template(title: &str, body: Markup) -> Markup {
    let today: Date = Date::today();
    let yesterday: Date = today.previous();
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
                .app {
                    .box {
                        .box-row {
                            .sidebar {
                                nav {
                                    ul {
                                        li {
                                            a href=(GetEntryView::url(today)) {
                                                img src=(TodayIconView::url());
                                                .text {
                                                    "Today"
                                                }
                                            }
                                        }
                                        li {
                                            a href=(GetEntryView::url(yesterday)) {
                                                img src=(YesterdayIconView::url());
                                                .text {
                                                    "Yesterday"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            .content {
                                (body)
                            }
                        }
                    }
                }
            }
        }
    }
}
