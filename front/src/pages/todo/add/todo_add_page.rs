#![allow(non_snake_case)]

use crate::adapter::todo_client::add_todo;
use crate::components::divider::Divider;
use crate::layouts::menu::Menu;
use crate::Route::TodoDetailPage;
use dioxus::prelude::*;

#[component]
pub fn TodoAddPage() -> Element {
    let nav = navigator();
    let mut title = use_signal(|| "".to_string());
    let mut description = use_signal(|| "".to_string());

    rsx! {
        Menu {}
        div {
            class: "flex flex-col items-center",
            h1 { class: "text-xl mt-2", "New TODO" }

            Divider { class: "mt-2" }

            div {
                class: "mt-2 flex flex-col gap-2",
                div {
                    class: "flex flex-col items-left",
                    p { "Title:" }
                    input {
                        class: "border-solid border-slate-500 border-2 rounded px-2",
                        placeholder: "Title",
                        value: "{title}",
                        oninput: move |event| title.set(event.value())
                    }
                }
                div {
                    class: "flex flex-col items-left",
                    p { "Description:" }
                    textarea {
                        class: "border-solid border-slate-500 border-2 rounded px-2 w-80 h-40",
                        placeholder: "Description",
                        value: "{description}",
                        oninput: move |event| description.set(event.value())
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
