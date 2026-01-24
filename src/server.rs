use std::sync::Arc;

use axum::Router;
use axum::routing::IntoMakeService;
use rusqlite::Connection;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

use crate::assets::route_assets;
use crate::config::Config;
use crate::controllers::DayController;
use crate::db::open_db;
use crate::error::Fallible;
use crate::views::AllView;
use crate::views::IndexView;
use crate::views::SearchView;
use crate::views::StatsView;
use crate::views::not_found_handler;

pub const CACHE_CONTROL_NONE: &str = "no-cache";

pub const CONTENT_TYPE_CSS: &str = "text/css";

pub const CACHE_CONTROL_IMMUTABLE: &str = "public, max-age=604800, immutable";

pub const CONTENT_TYPE_PNG: &str = "image/png";

#[derive(Clone)]
pub struct ServerState {
    pub db: Arc<Mutex<Connection>>,
}

pub async fn start_server() -> Fallible<()> {
    let config = Config::from_env()?;
    let db: Connection = open_db(&config.db_path)?;
    let state: ServerState = ServerState {
        db: Arc::new(Mutex::new(db)),
    };

    // Build the router.
    let router: Router<ServerState> = Router::new();

    let router = AllView::route(router);
    let router = DayController::route(router);
    let router = IndexView::route(router);
    let router = SearchView::route(router);
    let router = StatsView::route(router);

    let router = route_assets(router);
    let service: IntoMakeService<Router> = router
        .fallback(not_found_handler)
        .with_state(state)
        .into_make_service();
    let bind: String = format!("127.0.0.1:{}", config.port);
    log::info!("Started server on {bind}.");
    let listener: TcpListener = TcpListener::bind(bind).await?;
    axum::serve(listener, service).await?;
    Ok(())
}
