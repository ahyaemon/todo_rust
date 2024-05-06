#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::adapter::todo_client::{get_todo};
use crate::Route::HomePage;

#[component]
pub fn TodoPage(id: String) -> Element {
    let todo_future = use_resource(move || {
        let id_cloned = id.clone();
        async move {
            get_todo(id_cloned).await
        }
    });
    let todo = match &*todo_future.read_unchecked() {
        Some(Ok(response)) => {
            let todo = &response.todo;
            rsx! { p { "{todo.title}" } }
        }
        Some(Err(_e)) => {
            rsx! { p { "Error occurred: {_e}" } }
        }
        None => {
            rsx! { p { "Loading..." } }
        }
    };

    rsx! {
        Link { to: HomePage {}, "Go to home" }
        div {
            class: "flex flex-col items-center",
            h1 {
                class: "text-xl mt-2",
                "TODO"
            }
            hr {
                class: "border-1 border-slate-500 w-dvw"
            }
            p {{todo}}
        }
    }
}
