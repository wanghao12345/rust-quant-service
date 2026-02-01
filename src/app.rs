use axum::{
    Router,
    extract::{State, WebSocketUpgrade, ws::WebSocket},
    routing::get,
};

use crate::{api::create_api_router, services::ws_service::WsService};
use crate::state::AppState;

pub async fn run_app() -> Result<Router, Box<dyn std::error::Error>> {
    let app_state = AppState::new();
    //  构建路由
    let api_router = create_api_router(app_state.clone());
    let app = Router::new()
        .route("/health", get(health_check_handler))
        .nest("/api", api_router)
        .with_state(app_state);
    Ok(app)
}

async fn health_check_handler() -> &'static str {
    "OK"
}
async fn ws_handler(
    ws: WebSocketUpgrade,
    State(app_state): State<AppState>,
) -> &'static str {
    ws.on_upgrade(|socket| handle_socket(socket, app_state.ws_service));
    "OK"
}
async fn handle_socket(socket: WebSocket, ws_service: WsService) {
    let (id, mut outgoing_rx) = ws_service.register();
    let mut bcast_rx = ws_service.subscribe();
    let mut socket = socket;
}
