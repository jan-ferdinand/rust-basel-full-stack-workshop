use axum::extract::Path;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::routing::post;
use axum::Json;
use axum::Router;
use serde::Deserialize;
use serde::Serialize;
use tower_http::cors::CorsLayer;

use model::ShoppingListItem;

#[derive(Serialize, Deserialize)]
struct Workshop {
    num_attendees: i32,
    people_like_it: bool,
}

async fn hello_world() -> impl IntoResponse {
    "Hello World"
}

async fn hello_name(Path(name): Path<String>) -> impl IntoResponse {
    format!("Hello {name}")
}

async fn workshop_echo(json: Json<Workshop>) -> impl IntoResponse {
    json
}

async fn shopping_list_items() -> impl IntoResponse {
    let items = ["bread", "hummus", "cucumber"]
        .map(String::from)
        .map(|title| ShoppingListItem {
            title,
            posted_by: "jfs".to_string(),
            uuid: "jfs_list".to_string(),
        })
        .to_vec();

    Json(items)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/:name", get(hello_name))
        .route("/echo", post(workshop_echo))
        .route("/items", get(shopping_list_items))
        .layer(CorsLayer::permissive()); // never use “CorsLayer::permissive()” in production!

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
