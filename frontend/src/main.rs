use axum::Router;
use axum::routing::{get, post};
use axum::response::Html;

async fn index() -> Html<String> {
    Html(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <script src="https://unpkg.com/htmx.org@2.0.4"></script>
        </head>
        <body>
            <button hx-post="/clicked" hx-swap="outerHTML">
                Click Me
            </button>
        </body>
        </html>
    "#.to_string())
}

async fn clicked() -> Html<String> {
    Html(r#"<p>✅ You clicked the button!</p>"#.to_string())
}

#[tokio::main]
async fn main() {
    // dotenv::from_filename("../.env").ok();
    let app = Router::new()
        .route("/", get(index))
        .route("/clicked", post(clicked));    let listener = tokio::net::TcpListener::bind("0.0.0.0:5179").await.unwrap();
    println!("Listening on 0.0.0.0:5179");
    axum::serve(listener, app).await.unwrap();
}
