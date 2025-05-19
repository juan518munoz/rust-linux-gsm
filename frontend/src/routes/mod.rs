use std::env;

use axum::Router;
use axum::response::Html;
use axum::routing::{get, post};

mod start_server;
mod stop_server;

pub use start_server::start_server_clicked;
pub use stop_server::stop_server_clicked;

use crate::BACKEND_URL;
use crate::components::tables::server_list;

pub(crate) fn backend_url() -> String {
    let backend_port = env::var("BACKEND_PORT").unwrap_or_else(|_| "8080".to_string());
    format!("{}:{}", BACKEND_URL, backend_port)
}

async fn index() -> Html<String> {
    let html = format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <script src="https://unpkg.com/htmx.org@2.0.4"></script>
            <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.6/dist/css/bootstrap.min.css" rel="stylesheet" integrity="sha384-4Q6Gf2aSP4eDXB8Miphtr37CMZZQ5oXLH2yaXMJ2w8e2ZtHTl7GptT4jmndRuHDT" crossorigin="anonymous">        </head>
        <body class="bg-dark text-light m-5">
            <h1 class="text-center mb-4">Simple lgsm</h1>
            {server_list}
            <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.6/dist/js/bootstrap.bundle.min.js" integrity="sha384-j1CDi7MgGQ12Z7Qab0qlWQ/Qqz24Gc6BM0thvEMVjHnfYGF0rmFCozFSxQBxwHKO" crossorigin="anonymous"></script>
            <div class="mt-3>
                <label for="api_token" class="form-label">API Token</label>
                <input type="text" class="form-control bg-dark text-light" id="api_token" aria-label="API Token" aria-describedby="apiToken" name="api_token">
            </div>
        </body>
        </html>
        "#,
        server_list = server_list().await
    );

    Html(html)
}

pub fn build_app() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/start_server_clicked", post(start_server_clicked))
        .route("/stop_server_clicked", post(stop_server_clicked))
}
