use std::sync::Arc;
use std::sync::RwLock;

use axum::routing::delete;
use axum::routing::get;
use axum::routing::post;
use axum::Router;
use tower_http::cors::CorsLayer;

mod controller;
mod database;

type Database = Arc<RwLock<database::DB>>;

#[tokio::main]
async fn main() {
    let db = Database::default();
    let app = Router::new()
        .route("/items", get(controller::items))
        .route("/items", post(controller::add_item))
        .route("/items/:uuid", delete(controller::delete_item))
        .layer(CorsLayer::permissive()) // never use “CorsLayer::permissive()” in production!
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
