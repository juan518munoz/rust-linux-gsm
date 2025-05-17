use axum::{Form, response::Html};
use serde::Deserialize;

use crate::components::buttons::start_server_button;

#[derive(Deserialize)]
pub struct StopRequest {
    server: String,
}

pub async fn stop_server_clicked(Form(payload): Form<StopRequest>) -> Html<String> {
    println!("Stop clicked for server: {}", payload.server);

    let client = reqwest::Client::new();
    let response = client
        .post(format!("http://0.0.0.0:5180/{}/stop", payload.server))
        .send()
        .await
        .unwrap();
    println!("Response from backend: {:?}", response);

    Html(start_server_button(payload.server.clone()))
}
