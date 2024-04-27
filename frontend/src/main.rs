use dioxus::prelude::*;

fn main() {
    launch(application)
}

fn application() -> Element {
    rsx!("Hello, Basel.")
}
