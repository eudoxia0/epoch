use maud::DOCTYPE;
use maud::Markup;
use maud::html;

use crate::types::Date;
use crate::views::AllEntriesView;
use crate::views::CssView;
use crate::views::GetDayView;
use crate::views::SearchView;
use crate::views::StatsView;

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
                title { (title) " | Epoch" }
            }
            body {
                .app {
                    .box {
                        .sidebar {
                            nav {
                                ul {
                                    li {
                                        a href=(GetDayView::url(today)) {
                                            .icon {
                                                "📆"
                                            }
                                            .text {
                                                "Today"
                                            }
                                        }
                                    }
                                    li {
                                        a href=(GetDayView::url(yesterday)) {
                                            .icon {
                                                "🗓️"
                                            }
                                            .text {
                                                "Yesterday"
                                            }
                                        }
                                    }
                                    li {
                                        a href=(AllEntriesView::url()) {
                                            .icon {
                                                "🗓️"
                                            }
                                            .text {
                                                "All Entries"
                                            }
                                        }
                                    }
                                    li {
                                        a href=(SearchView::url()) {
                                            .icon {
                                                "🔍"
                                            }
                                            .text {
                                                "Search"
                                            }
                                        }
                                    }
                                    li {
                                        a href=(StatsView::url()) {
                                            .icon {
                                                "📊"
                                            }
                                            .text {
                                                "Stats"
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
