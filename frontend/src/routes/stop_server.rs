use axum::{Form, response::Html};
use reqwest::StatusCode;
use serde::Deserialize;

use crate::{
    components::buttons::{start_server_button, stop_server_button},
    routes::backend_url,
};

#[derive(Deserialize)]
pub struct StopRequest {
    server: String,
    api_token: String,
}

pub async fn stop_server_clicked(Form(payload): Form<StopRequest>) -> Html<String> {
    println!("Stop clicked for server: {}", payload.server);

    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/{}/stop", backend_url(), payload.server))
        .header("Authorization", format!("Bearer {}", payload.api_token))
        .send()
        .await
        .unwrap();

    match response.status() {
        StatusCode::OK => {
            println!("Server started successfully");
            Html(start_server_button(payload.server.clone()))
        }
        _ => {
            println!("Unauthorized: Invalid API token");
            Html(stop_server_button(payload.server.clone()))
        }
    }
}
