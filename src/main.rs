use axum::Router;
use axum::extract::Path;
use axum::routing::get;
use chrono::Local;
use game_server::{GameServer, ServerAction, build_command, run_command};
use strum::IntoEnumIterator;

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
    let output = run_command(command).await;
    output
}

async fn help() -> String {
    log_call("help");
    let mut output = String::new();

    output.push_str("Available Servers:\n");
    for game_server in GameServer::iter() {
        output.push_str(&format!("- {}\n", game_server));
    }
    output.push_str("\n");

    output.push_str("Available Actions:\n");
    for action in ServerAction::iter() {
        output.push_str(&format!("- {}\n", action));
    }
    output.push_str("\n");

    output.push_str("Sample request:\n");
    output.push_str(&format!(
        "/{}/{}\n",
        GameServer::PZomboid,
        ServerAction::Monitor
    ));
    output
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/{server_name}/{endpoint}", get(server_action))
        .route("/", get(help));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5180").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
