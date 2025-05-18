use crate::GameServer;
use axum::Json;
use serde::Serialize;
use strum::IntoEnumIterator;

#[derive(Serialize)]
pub struct ServerStatus {
    pub server_name: String,
    pub endpoint: String,
    pub running: bool,
}

pub async fn list_servers() -> Json<Vec<ServerStatus>> {
    let mut response = Vec::new();
    for server in GameServer::iter() {
        response.push(ServerStatus {
            server_name: server.to_string(),
            endpoint: server.endpoint(),
            running: server.is_running(),
        });
    }

    Json(response)
}
