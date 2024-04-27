use dioxus::prelude::*;

use model::PostShoppingItem;

use crate::controllers;

#[derive(Debug, Clone, PartialEq)]
pub struct ListChange;

#[component]
pub fn shopping_list(change_signal: Signal<ListChange>) -> Element {
    let items_request = use_resource(move || async move {
        change_signal.read();
        controllers::get_items().await
    });
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

#[component]
pub fn item_input(change_signal: Signal<ListChange>) -> Element {
    let mut item = use_signal(|| "".to_string());
    let mut author = use_signal(|| "".to_string());

    let onsubmit = move |_| {
        spawn(async move {
            let title = item.read().to_string();
            let posted_by = author.read().to_string();
            let response = controllers::post_item(PostShoppingItem { title, posted_by }).await;
            if response.is_ok() {
                change_signal.write();
            }
        });
    };

    rsx! {
        div {
            class: "w-300 m-4 mt-16 rounded",
            form { class: "grid grid-cols-3 gap-2",
                onsubmit: onsubmit,
                div {
                    input {
                        value: "{item}",
                        class: "input input-bordered input-primary w-full",
                        placeholder: "next item..",
                        r#type: "text",
                        id: "item_name",
                        name: "item_name",
                        oninput: move |e| item.set(e.data.value().clone())
                    }
                }
                div {
                    input {
                        value: "{author}",
                        class: "input input-bordered input-primary w-full",
                        placeholder: "wanted by..",
                        r#type: "text",
                        id: "author",
                        name: "author",
                        oninput: move |e| author.set(e.data.value().clone())
                    }
                }
                button {
                    class: "btn btn-primary w-full",
                    r#type: "submit",
                    "Add Item"
                }
            }
        }
    }
}
