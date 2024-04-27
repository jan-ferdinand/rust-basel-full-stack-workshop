use dioxus::prelude::*;

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
    )
}
