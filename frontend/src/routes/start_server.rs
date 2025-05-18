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
    println!("Start clicked for server: {}", payload.server);

    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/{}/start", backend_url(), payload.server))
        .header("Authorization", format!("Bearer {}", payload.api_token))
        .send()
        .await
        .unwrap();

    match response.status() {
        StatusCode::OK => Html(stop_server_button(payload.server.clone())),
        _ => {
            let html = format!(
                r#"
                {}
                <script>
                    alert("Unauthorized: Invalid API token");
                </script>
                "#,
                start_server_button(payload.server.clone())
            );

            Html(html)
        }
    }
}
