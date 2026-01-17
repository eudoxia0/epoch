use maud::DOCTYPE;
use maud::Markup;
use maud::html;

use crate::assets::CSS_RESET_URL;
use crate::assets::CSS_URL;
use crate::types::Date;
use crate::views::AllView;
use crate::views::GetDayView;
use crate::views::SearchView;
use crate::views::StatsView;

pub fn page_template(title: &str, controls: Option<Markup>, body: Markup) -> Markup {
    let today: Date = Date::today();
    let yesterday: Date = today.previous();
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                link rel="stylesheet" href=(CSS_RESET_URL);
                link rel="stylesheet" href=(CSS_URL);
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
                                        a href=(AllView::url()) {
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
                            .header {
                                h1 {
                                    (title)
                                }
                            }
                            @match controls {
                                Some(controls) => {
                                    .page-controls {
                                        (controls)
                                    }
                                }
                                None => {}
                            }
                            .page-content {
                                (body)
                            }
                        }
                    }
                }
            }
        }
    }
}
