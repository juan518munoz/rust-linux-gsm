use std::env;

use serde::Deserialize;

use crate::components::buttons::{start_server_button, stop_server_button};

#[derive(Deserialize)]
pub struct ServerStatus {
    pub server_name: String,
    pub endpoint: String,
    pub players: Vec<String>,
    pub running: bool,
}

async fn get_servers() -> Vec<ServerStatus> {
    let backend_port = match env::var("BACKEND_PORT") {
        Ok(port) => port,
        Err(_) => {
            log::error!("BACKEND_PORT not set");
            return Vec::new();
        }
    };

    let backend_endpoint = format!("http://0.0.0.0:{}/status", backend_port);
    let response = match reqwest::get(backend_endpoint).await {
        Ok(response) => response,
        Err(err) => {
            log::error!("Error fetching server status: {}", err);
            return Vec::new();
        }
    };

    match response.json::<Vec<ServerStatus>>().await {
        Ok(servers) => servers,
        Err(err) => {
            log::error!("Error parsing server status: {}", err);
            Vec::new()
        }
    }
}

fn players_list(players: Vec<String>) -> String {
    if players.is_empty() {
        return "".to_string();
    }

    let mut player_list = String::new();
    for player in players {
        player_list.push_str(&format!("<p>{}</p>", player));
    }
    player_list
}

async fn server_rows() -> String {
    let servers = get_servers().await;

    let mut table = String::new();
    for server in servers {
        table.push_str(&format!(
            r#"
            <tr>
                <td>{server_name}</td>
                <td>{server_endpoint}</td>
                <td>{players_list}</td>
                <td>{server_button}</td>
            </tr>
            "#,
            server_name = server.server_name,
            server_endpoint = server.endpoint,
            players_list = players_list(server.players),
            server_button = match server.running {
                true => stop_server_button(server.server_name.clone()),
                false => start_server_button(server.server_name.clone()),
            }
        ));
    }

    table
}

pub async fn server_list() -> String {
    format!(
        r#"
        <table class="table table-dark table-striped table-bordered">
            <thead>
                <tr>
                    <th>Server</th>
                    <th>Endpoint</th>
                    <th>Players</th>
                    <th>Action</th>
                </tr>
            </thead>
            <tbody>
                {server_rows}
            </tbody>
        </table>
        "#,
        server_rows = server_rows().await,
    )
}
