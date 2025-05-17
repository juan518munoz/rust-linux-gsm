use routes::build_app;
use std::env;

mod components;
mod routes;

#[tokio::main]
async fn main() {
    dotenv::from_filename("../.env").ok();

    let app = build_app();

    let port = env::var("FRONTEND_PORT").expect("FRONTEND_PORT must be set");
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(addr.clone()).await.unwrap();

    println!("Frontend listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}
