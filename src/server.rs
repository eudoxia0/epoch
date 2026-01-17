use std::sync::Arc;

use axum::Router;
use axum::routing::IntoMakeService;
use rusqlite::Connection;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

use crate::db::create_db;
use crate::error::Fallible;
use crate::views::CssView;
use crate::views::FaviconView;
use crate::views::GetEntryView;
use crate::views::IndexView;
use crate::views::TodayIconView;
use crate::views::YesterdayIconView;
use crate::views::not_found_handler;

pub const CACHE_CONTROL_NONE: &str = "no-cache";

pub const CONTENT_TYPE_CSS: &str = "text/css";

pub const CACHE_CONTROL_IMMUTABLE: &str = "public, max-age=604800, immutable";

pub const CONTENT_TYPE_PNG: &str = "image/png";

#[derive(Clone)]
pub struct ServerState {
    pub db: Arc<Mutex<Connection>>,
}

impl ServerState {
    pub fn new() -> Fallible<Self> {
        let db: Connection = create_db()?;
        let state: ServerState = ServerState {
            db: Arc::new(Mutex::new(db)),
        };
        Ok(state)
    }
}

pub async fn start_server() -> Fallible<()> {
    let state = ServerState::new()?;

    // Build the router.
    let router: Router<ServerState> = Router::new();

    let router = CssView::route(router);
    let router = FaviconView::route(router);
    let router = GetEntryView::route(router);
    let router = IndexView::route(router);
    let router = TodayIconView::route(router);
    let router = YesterdayIconView::route(router);

    let service: IntoMakeService<Router> = router
        .fallback(not_found_handler)
        .with_state(state)
        .into_make_service();
    let bind: String = format!("127.0.0.1:{}", 8000);
    log::info!("Started server on {bind}.");
    let listener: TcpListener = TcpListener::bind(bind).await?;
    axum::serve(listener, service).await?;
    Ok(())
}
