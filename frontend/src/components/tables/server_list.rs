use serde::Deserialize;

use crate::components::buttons::{start_server_button, stop_server_button};

#[derive(Deserialize)]
pub struct ServerStatus {
    pub server_name: String,
    pub endpoint: String,
    pub running: bool,
}

async fn get_servers() -> Vec<ServerStatus> {
    let backend_port = std::env::var("BACKEND_PORT").expect("BACKEND_PORT must be set");
    let backend_endpoint = format!("http://localhost:{}/status", backend_port);
    let response = reqwest::get(backend_endpoint)
        .await
        .expect("Failed to fetch server status");
    let servers = response
        .json::<Vec<ServerStatus>>()
        .await
        .expect("Failed to parse server status");
    servers
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
                <td>{server_button}</td>
            </tr>
            "#,
            server_name = server.server_name,
            server_endpoint = server.endpoint,
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
