#![allow(non_snake_case)]

use dioxus::prelude::*;
use serde::Deserialize;
use crate::Route::About;
use crate::utils::log::log;

#[derive(Deserialize)]
struct ApiResponse {
    message: String,
}

#[component]
pub fn Home() -> Element {
    log("test");

    let mut count = use_signal(|| 0);
    let future = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<ApiResponse>()
            .await
    });

    match &*future.read_unchecked() {
        Some(Ok(response)) => {
            log(&format!("ok: {}", response.message));
        }
        Some(Err(e)) => {
            log(&format!("err: {}", e));
        }
        None => {
            log("none");
        }
    };

    rsx! {
        Link { to: About {}, "Go to about" }
        div {
            h2 { "test11" }
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    }
}
