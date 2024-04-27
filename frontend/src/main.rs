use dioxus::prelude::*;

use crate::components::ListChange;

mod components;
mod controllers;

const _STYLE: &str = manganis::mg!(file("public/tailwind.css"));

#[derive(Debug, Clone, PartialEq, Routable)]
enum Route {
    #[layout(crate::components::layout)]
    #[route("/", crate::home)]
    Home,

    #[route("/profile", crate::profile)]
    Profile,
}

fn main() {
    launch(application)
}

fn application() -> Element {
    rsx! { Router::<Route> {} }
}

fn home() -> Element {
    let change_signal = use_signal(|| ListChange);
    rsx!(
        p { class: "h-16" }
        components::item_input{change_signal}
        p { class: "h-16" }
        components::shopping_list{change_signal}
    )
}

pub fn profile() -> Element {
    rsx! {
        div {
            div {
                class: "flex flex-col gap-4 w-full",
                div {
                    class: "flex gap-4 items-center",
                    div { class: "skeleton w-16 h-16 rounded-full shrink-0" }
                    div {
                        class: "flex flex-col hap-4",
                        div { class: "skeleton h-4 w-20" }
                        div { class: "skeleton h-4 w-28" }
                    }
                }
                div { class: "skeleton h-32 w-full" }
                div { class: "skeleton h-32 w-full" }
            }
        }
    }
}
