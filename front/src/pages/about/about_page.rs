#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::components::divider::Divider;
use crate::layouts::menu::Menu;

#[component]
pub fn AboutPage() -> Element {
    rsx! {
        Menu {}
        h1 { class: "text-xl mt-2", "About" }
        Divider { class: "mt-2" }
        p { "Hello, I am Ahyaemon." }
        a {
            href: "https://github.com/ahyaemon/todo_rust",
            target: "_blank",
            "https://github.com/ahyaemon/todo_rust"
        }
    }
}
