use std::sync::Arc;

use axum::Router;
use axum::routing::IntoMakeService;
use rusqlite::Connection;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

use crate::assets::route_assets;
use crate::config::Config;
use crate::controllers::AchievementsController;
use crate::controllers::AllController;
use crate::controllers::DayController;
use crate::controllers::RootController;
use crate::controllers::SearchController;
use crate::controllers::StatsController;
use crate::db::Database;
use crate::error::Fallible;
use crate::models::open_db;

pub const CACHE_CONTROL_NONE: &str = "no-cache";

pub const CONTENT_TYPE_CSS: &str = "text/css";

pub const CACHE_CONTROL_IMMUTABLE: &str = "public, max-age=604800, immutable";

pub const CONTENT_TYPE_PNG: &str = "image/png";

#[derive(Clone)]
pub struct ServerState {
    pub db: Arc<Mutex<Connection>>,
    pub db2: Database,
}

pub async fn start_server() -> Fallible<()> {
    let config = Config::from_env()?;
    let db: Connection = open_db(&config.db_path)?;
    let db2: Database = Database::new(config.db_path.clone())?;
    let state: ServerState = ServerState {
        db: Arc::new(Mutex::new(db)),
        db2,
    };

    // Build the router.
    let router: Router<ServerState> = Router::new();

    let router = AllController::route(router);
    let router = DayController::route(router);
    let router = RootController::route(router);
    let router = SearchController::route(router);
    let router = StatsController::route(router);
    let router = AchievementsController::route(router);

    let router = route_assets(router);
    let service: IntoMakeService<Router> = router
        .fallback(RootController::not_found)
        .with_state(state)
        .into_make_service();
    let bind: String = format!("127.0.0.1:{}", config.port);
    log::info!("Started server on {bind}.");
    let listener: TcpListener = TcpListener::bind(bind).await?;
    axum::serve(listener, service).await?;
    Ok(())
}
