use routes::build_app;
use std::env;

mod components;
mod routes;

// The backend might eventually be moved to a different server,
// so we should not hardcode the URL here.
pub const BACKEND_URL: &str = "0.0.0.0";

#[tokio::main]
async fn main() {
    dotenv::from_filename("../.env").ok();
    env_logger::init();
    log::info!("Starting frontend server");

    let app = build_app();

    let port = env::var("FRONTEND_PORT").expect("FRONTEND_PORT must be set");
    let addr = format!("{}:{}", BACKEND_URL, port);
    let listener = tokio::net::TcpListener::bind(addr.clone()).await.unwrap();

    log::info!("Frontend listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}
