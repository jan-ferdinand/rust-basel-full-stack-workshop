use model::PostShoppingItem;
use model::ShoppingListItem;

pub async fn get_items() -> Result<Vec<ShoppingListItem>, reqwest::Error> {
    let url = "http://localhost:3001/items";
    reqwest::get(url)
        .await?
        .json::<Vec<ShoppingListItem>>()
        .await
}

pub async fn post_item(item: PostShoppingItem) -> Result<ShoppingListItem, reqwest::Error> {
    let response = reqwest::Client::new()
        .post("http://localhost:3001/items")
        .json(&item)
        .send()
        .await?
        .json::<ShoppingListItem>()
        .await?;

    Ok(response)
}

pub async fn delete_item(item_id: &str) -> Result<(), reqwest::Error> {
    reqwest::Client::new()
        .delete(format!("http://localhost:3001/items/{item_id}"))
        .send()
        .await?;
    Ok(())
}
