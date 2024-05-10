#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::adapter::todo_client::add_todo;
use crate::components::divider::Divider;
use crate::layouts::menu::Menu;
use crate::Route::{TodoDetailPage};

#[component]
pub fn TodoAddPage() -> Element {

    let nav = navigator();
    let mut title = use_signal(|| "".to_string());

    rsx! {
        Menu {}
        div {
            class: "flex flex-col items-center",
            h1 { class: "text-xl mt-2", "New TODO" }

            Divider { class: "mt-2" }

            div {
                class: "mt-2 flex flex-col gap-2",
                div {
                    class: "flex flex-row gap-2 items-center",
                    p { "Title:" }
                    input {
                        class: "border-solid border-slate-500 border-2 rounded px-2",
                        placeholder: "Title",
                        value: "{title}",
                        oninput: move |event| title.set(event.value())
                    }
                }
                button {
                    class: "bg-green-700 text-white px-4 py-3 rounded",
                    onclick: move |_event| async move {
                        let response = add_todo(&title.cloned()).await.unwrap();
                        nav.push(TodoDetailPage { id: response.todo.id });
                    },
                    "Add Todo"
                }
            }
        }
    }
}
