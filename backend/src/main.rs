use action::ServerAction;
use axum::Router;
use axum::extract::Path;
use axum::routing::post;
use chrono::Local;
use command::{build_command, run_command};
use game_server::GameServer;

pub(crate) mod action;
mod command;
mod game_server;

fn log_call(endpoint: &str) {
    let now = Local::now();
    println!("[{}] {}", now.format("%Y-%m-%d %H:%M:%S"), endpoint)
}

async fn server_action(Path((server_name, action)): Path<(String, String)>) -> String {
    log_call("server_action");
    let game_server = match GameServer::try_from(server_name) {
        Ok(game_server) => game_server,
        Err(_) => return "Invalid Server\n".to_string(),
    };

    let action = match ServerAction::try_from(action) {
        Ok(action) => action,
        Err(_) => return "Invalid Action\n".to_string(),
    };

    let command = build_command(game_server, action);
    run_command(command).await
}

#[tokio::main]
async fn main() {
    dotenv::from_filename("../.env").ok();
    let app = Router::new().route("/{server_name}/{endpoint}", post(server_action));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5180").await.unwrap();
    println!("Listening on 0.0.0.0:5180");
    axum::serve(listener, app).await.unwrap();
}
