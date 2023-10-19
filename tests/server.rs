use std::net::SocketAddr;
use axum::{Json, Router};
use axum::extract::ConnectInfo;
use axum::routing::{get, post};
use tower_http::trace::TraceLayer;

#[tokio::main]
#[test]
async fn hello_world() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route(
            "/json",
            post(|payload: Json<serde_json::Value>| async move {
                let req = payload.0.to_string();
                Json(serde_json::json!({ "data": payload.0 }))
            }),
        )
        .route(
            "/requires-connect-into",
            get(|ConnectInfo(addr): ConnectInfo<SocketAddr>| async move { format!("Hi {addr}") }),
        )
        // We can still add middleware
        .layer(TraceLayer::new_for_http());

    axum::Server::bind(&"127.0.0.1:8888".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}