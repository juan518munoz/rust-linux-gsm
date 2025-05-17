use axum::{Form, response::Html};
use serde::Deserialize;

use crate::components::buttons::stop_server_button;

#[derive(Deserialize)]
pub struct StartRequest {
    server: String,
}

pub async fn start_server_clicked(Form(payload): Form<StartRequest>) -> Html<String> {
    println!("Start clicked for server: {}", payload.server);

    let client = reqwest::Client::new();
    let response = client
        .post(format!("http://0.0.0.0:5180/{}/start", payload.server))
        .send()
        .await
        .unwrap();
    println!("Response from backend: {:?}", response);

    Html(stop_server_button(payload.server.clone()))
}
