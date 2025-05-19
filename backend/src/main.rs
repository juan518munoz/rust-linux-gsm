use std::env;

use axum::routing::post;
use axum::{Router, routing::get};
use game_server::GameServer;
use routes::{list_servers, server_action};

pub(crate) mod action;
mod command;
mod game_server;
mod routes;

#[tokio::main]
async fn main() {
    dotenv::from_filename("../.env").ok();
    env_logger::init();
    log::info!("Starting backend server");

    let app = Router::new()
        .route("/{server_name}/{endpoint}", post(server_action))
        .route("/status", get(list_servers));

    let port = env::var("BACKEND_PORT").expect("BACKEND_PORT must be set");
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    log::info!("Backend listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}
