use dioxus::prelude::*;

use crate::components::ListChange;

mod components;
mod controllers;

const _STYLE: &str = manganis::mg!(file("public/tailwind.css"));

fn main() {
    launch(application)
}

fn application() -> Element {
    let change_signal = use_signal(|| ListChange);
    rsx!(
        p { class: "h-16" }
        components::item_input{change_signal}
        p { class: "h-16" }
        components::shopping_list{change_signal}
    )
}
