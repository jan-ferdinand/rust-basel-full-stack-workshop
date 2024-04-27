use dioxus::prelude::*;

use model::ShoppingListItem;

const _STYLE: &str = manganis::mg!(file("public/tailwind.css"));

fn main() {
    launch(application)
}

fn application() -> Element {
    rsx!(
        h1 {
            "Hello, Basel."
        }
        button {
            class: "btn",
            "Click me maybe ✨"
        }
        shopping_list{}
    )
}

async fn get_items() -> Result<Vec<ShoppingListItem>, reqwest::Error> {
    let url = "http://localhost:3001/items";
    reqwest::get(url)
        .await?
        .json::<Vec<ShoppingListItem>>()
        .await
}

#[component]
fn shopping_list() -> Element {
    let items_request = use_resource(move || async move { get_items().await });
    let readable = items_request.read();

    match &*readable {
        Some(Ok(list)) => rsx! {
            div { class: "grid place-items-center min-h-500",
                ul {
                    class: "menu bg-base-200 w-200 rounded-box gap-1",
                    for i in list {
                        li {
                            key: "{i.uuid}",
                            shopping_list_item_component {
                                display_name: i.title.clone(),
                                posted_by: i.posted_by.clone()
                            }
                        }
                    }
                }
            }
        },
        Some(Err(err)) => rsx! { p { "Error: {err}" } },
        None => rsx! { p { "Loading items..." } },
    }
}

#[component]
fn shopping_list_item_component(display_name: String, posted_by: String) -> Element {
    rsx! {
        div {
            class: "flex items-center space-x-2",
            p {
                class: "grow text-2xl",
                "{display_name}"
            }
            span { "posted by {posted_by}" }
        }
    }
}
