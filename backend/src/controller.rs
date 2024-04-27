use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use uuid::Uuid;

use model::PostShoppingItem;
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

pub async fn add_item(
    State(state): State<Database>,
    Json(post_request): Json<PostShoppingItem>,
) -> impl IntoResponse {
    let PostShoppingItem { title, posted_by } = post_request;
    let item = ShoppingItem {
        title: title.clone(),
        creator: posted_by.clone(),
    };
    let uuid = Uuid::new_v4().to_string();

    // scope off unique access to `RwLock`
    {
        let Ok(mut db) = state.write() else {
            return StatusCode::SERVICE_UNAVAILABLE.into_response();
        };
        db.create(uuid.clone(), item);
    }

    let item = ShoppingListItem {
        title,
        posted_by,
        uuid,
    };

    (StatusCode::OK, Json(item)).into_response()
}

pub async fn delete_item(
    State(state): State<Database>,
    Path(uuid): Path<String>,
) -> impl IntoResponse {
    let Ok(mut db) = state.write() else {
        return StatusCode::SERVICE_UNAVAILABLE;
    };

    db.delete(&uuid);
    StatusCode::OK
}
