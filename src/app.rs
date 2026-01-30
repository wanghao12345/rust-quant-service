use axum::{Router, routing::get};



pub async fn run_app() -> Result<Router, Box<dyn std::error::Error>> {
    //  构建路由
    let app = Router::new()
    .route("/health", get(health_check_handler));
    Ok(app)
}

async fn health_check_handler() -> &'static str {
    "OK"
}