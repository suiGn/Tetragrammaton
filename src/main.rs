use axum::{Router, routing::get, Json};
use serde::Serialize;
use std::net::SocketAddr;

#[derive(Serialize)]
struct Identity {
    id: String,
    public_key: String,
    alias: String,
    created_at: String,
}

// Simulación de respuesta
async fn list_instances() -> Json<Vec<Identity>> {
    let demo = Identity {
        id: "default".to_string(),
        public_key: "abc123...".to_string(),
        alias: "Main Identity".to_string(),
        created_at: "2025-06-03T12:00:00Z".to_string(),
    };

    Json(vec![demo])
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/cleaker/instances", get(list_instances));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("🔮 Tetragrammaton listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}