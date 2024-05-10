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
        p { "I am ahyaemon." }
        "Blog post"
    }
}
