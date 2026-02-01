use axum::Router;

use crate::state::AppState;

pub mod audit;
pub mod strategies;
pub mod auth;



pub fn create_api_router(state: AppState) -> Router<AppState> {
    Router::new()
        .with_state(state)
        .nest("/auth", auth::router())
}