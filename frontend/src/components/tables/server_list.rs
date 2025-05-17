use crate::components::buttons::{start_server_button, stop_server_button};

struct ServerData {
    pub name: String,
    pub endpoint: String,
    pub running: bool,
}

async fn get_servers() -> Vec<ServerData> {
    // TODO: ask backend
    vec![
        ServerData {
            name: "mcserver".into(),
            endpoint: "http://foo.bar:1234".into(),
            running: true,
        },
        ServerData {
            name: "gmod".into(),
            endpoint: "http://foo.bar:1234".into(),
            running: true,
        },
        ServerData {
            name: "l4d2server".into(),
            endpoint: "http://foo.bar:1234".into(),
            running: false,
        },
    ]
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
                <td>{server_status}</td>
                <td>{server_button}</td>
            </tr>
            "#,
            server_name = server.name,
            server_endpoint = server.endpoint,
            server_status = match server.running {
                true => "YES",
                false => "NO",
            },
            server_button = match server.running {
                true => stop_server_button(server.name.clone()),
                false => start_server_button(server.name.clone()),
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
                    <th>Running</th>
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
