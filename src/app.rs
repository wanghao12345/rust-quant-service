use axum::routing::any;
use axum::{Router, routing::get};

use crate::api::create_api_router;
use crate::api::ws::ws_handler;
use crate::state::AppState;

pub async fn run_app() -> Result<Router, Box<dyn std::error::Error>> {
    let app_state = AppState::new();
    //  构建路由
    let api_router = create_api_router(app_state.clone());
    let app = Router::new()
        .route("/health", get(health_check_handler))
        .route("/ws", any(ws_handler))
        .nest("/api", api_router)
        .with_state(app_state);
    Ok(app)
}

async fn health_check_handler() -> &'static str {
    "OK"
}
