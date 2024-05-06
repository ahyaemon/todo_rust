#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::Route::HomePage;

#[component]
pub fn AboutPage() -> Element {
    rsx! {
        Link { to: HomePage {}, "Go to home" }
        h1 { "About" }
        hr {}
        p { "I am ahyaemon." }
        "Blog post"
    }
}
