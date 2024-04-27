use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

use model::ShoppingListItem;

use crate::database::ShoppingItem;
use crate::Database;

pub async fn items(State(state): State<Database>) -> impl IntoResponse {
    let Ok(state) = state.read() else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };

    let items = state
        .to_vec()
        .into_iter()
        .map(|(uuid, shopping_item)| {
            let ShoppingItem {
                title,
                creator: posted_by,
            } = shopping_item;
            ShoppingListItem {
                title,
                posted_by,
                uuid,
            }
        })
        .collect::<Vec<_>>();

    (StatusCode::OK, Json(items)).into_response()
}
