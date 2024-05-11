#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::adapter::todo_client::{get_todo};
use crate::components::divider::Divider;
use crate::layouts::menu::Menu;

#[component]
pub fn TodoDetailPage(id: String) -> Element {
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
        Menu {}
        div {
            class: "flex flex-col items-center",
            h1 { class: "text-xl mt-2", "TODO" }

            Divider { class: "mt-2" }

            div {
                class: "flex flex-row gap-2 items-center",
                p { "Title:" }
                p { {todo} }
            }
        }
    }
}