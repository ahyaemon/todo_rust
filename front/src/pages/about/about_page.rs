#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::components::divider::Divider;
use crate::Route::HomePage;

#[component]
pub fn AboutPage() -> Element {
    rsx! {
        Link { to: HomePage {}, "Go to home" }
        h1 { class: "text-xl mt-2", "About" }
        Divider { class: "mt-2" }
        p { "I am ahyaemon." }
        "Blog post"
    }
}
