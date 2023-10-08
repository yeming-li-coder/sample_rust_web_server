use std::collections::HashMap;

use axum::extract::{Path, Query};
use axum::{routing::get, Router};
async fn greet() -> String {
    "Hello World!".to_owned()
}

async fn handler(Path(path): Path<HashMap<String, String>>) -> String {
    match path.get("name") {
        Some(path) => format!("Hello {}", path),
        None => format!("Hello World"),
    }
}


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(greet))
        .route("/:name", get(handler))
        .route("/say/hello", get(|| async { format!("Hello Again!") }));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
