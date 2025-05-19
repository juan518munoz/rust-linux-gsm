use std::time::Duration;

use axum::{Form, response::Html};
use reqwest::StatusCode;
use serde::Deserialize;

use crate::{
    components::buttons::{start_server_button, stop_server_button},
    routes::backend_url,
};

#[derive(Deserialize)]
pub struct StartRequest {
    server: String,
    api_token: String,
}

pub async fn start_server_clicked(Form(payload): Form<StartRequest>) -> Html<String> {
    log::info!("Start clicked for server: {}", payload.server);

    let client = reqwest::Client::new();
    let response = match client
        .post(format!("http://{}/{}/start", backend_url(), payload.server))
        .header("Authorization", format!("Bearer {}", payload.api_token))
        .timeout(Duration::from_secs(5))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => {
            log::error!("Error sending request: {}", err);
            return Html(format!(
                r#"
                    {}
                    <script>
                        alert("Error: {}", err);
                    </script>
                    "#,
                err,
                start_server_button(payload.server.clone()),
            ));
        }
    };

    match response.status() {
        StatusCode::OK => {
            log::debug!("Server started successfully");
            Html(stop_server_button(payload.server.clone()))
        }
        _ => {
            log::debug!("Unauthorized: Invalid API token");
            Html(format!(
                r#"
                {}
                <script>
                    alert("Unauthorized: Invalid API token");
                </script>
                "#,
                start_server_button(payload.server.clone())
            ))
        }
    }
}
