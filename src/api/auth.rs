use axum::{Json, Router, extract::State, routing::post};

use crate::{models::user::LoginRequest, services::auth_service::AuthService, state::AppState, utils::{error::ServiceError, response::ApiResponse}};

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<ApiResponse<String>>, ServiceError> {
    let auth_service = AuthService::new(state);
    let result = auth_service.login(&payload.email, &payload.code).await;

    match result {
        Ok(token) => Ok(Json(ApiResponse::success(token))),
        Err(e) => Err(ServiceError::AuthorizationError(e)),
    }
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/login", post(login))
}
