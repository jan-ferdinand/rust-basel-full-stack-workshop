use dioxus::prelude::*;

mod components;
mod controllers;

const _STYLE: &str = manganis::mg!(file("public/tailwind.css"));

fn main() {
    launch(application)
}

fn application() -> Element {
    rsx!(
        p { class: "h-16" }
        components::shopping_list{}
        components::item_input{}
    )
}
