#![allow(non_snake_case)]

use crate::adapter::todo_client::get_todo;
use crate::components::divider::Divider;
use crate::layouts::menu::Menu;
use dioxus::prelude::*;

#[component]
pub fn TodoDetailPage(id: String) -> Element {
    let todo_future = use_resource(move || {
        let id_cloned = id.clone();
        async move { get_todo(id_cloned).await }
    });
    let todo = match &*todo_future.read_unchecked() {
        Some(Ok(response)) => {
            let todo = &response.todo;
            rsx! {
                div {
                    class: "flex flex-row gap-2 items-center",
                    p { b { "{todo.title}" } }
                }
                div {
                    class: "mt-2",
                    div { "{todo.description}" }
                }
            }
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
                class: "mt-2",
                { todo }
            }
        }
    }
}
