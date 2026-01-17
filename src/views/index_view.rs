use axum::Router;
use axum::response::Redirect;
use axum::routing::get;

use crate::server::ServerState;
use crate::types::Date;
use crate::views::GetEntryView;

pub struct IndexView {}

impl IndexView {
    pub fn route(router: Router<ServerState>) -> Router<ServerState> {
        router.route(Self::url(), get(Self::handler))
    }

    pub fn url() -> &'static str {
        "/"
    }

    async fn handler() -> Redirect {
        let today: Date = Date::today();
        Redirect::to(&GetEntryView::url(today))
    }
}
