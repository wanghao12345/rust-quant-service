use tracing::info;

mod api;
mod app;
mod config;
mod core;
mod exchange;
mod models;
mod services;



#[tokio::main]
async fn main() {
    // 设置日志
    tracing_subscriber::fmt::init();
    // 运行应用
    let app = app::run_app().await;
    match app {
        Ok(app) => {
            // 绑定监听器
            let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
            // 启动服务器
            info!("🚀 服务器启动在 http://localhost:3000");
            axum::serve(listener, app).await.unwrap();
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
}
