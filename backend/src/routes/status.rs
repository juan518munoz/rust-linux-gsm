use crate::GameServer;
use axum::Json;
use serde::Serialize;
use strum::IntoEnumIterator;

#[derive(Serialize)]
pub struct ServerStatus {
    pub server_name: String,
    pub endpoint: String,
    pub players: Vec<String>,
    pub running: bool,
}

pub async fn list_servers() -> Json<Vec<ServerStatus>> {
    log::info!("Received request for server status");
    let mut response = Vec::new();
    for server in GameServer::iter() {
        response.push(ServerStatus {
            server_name: server.to_string(),
            endpoint: server.endpoint(),
            players: match server.is_running() {
                true => server.get_online_players(),
                false => vec![],
            },
            running: server.is_running(),
        });
    }

    Json(response)
}
