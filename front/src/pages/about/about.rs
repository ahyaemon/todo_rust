#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::Route::Home;

#[component]
pub fn About() -> Element {
    rsx! {
        Link { to: Home {}, "Go to home" }
        h1 { "About" }
        hr {}
        p { "I am ahyaemon." }
        "Blog post"
    }
}
