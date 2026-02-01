use crate::services::ws_service::WsService;


#[derive(Debug, Clone)]
pub struct AppState {
    pub ws_service: WsService,
}

impl AppState {
    pub fn new() -> Self {
        Self { ws_service: WsService::new() }
    }
}
