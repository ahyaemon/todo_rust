#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::adapter::todo_client::{add_todo, list_todos};
use crate::components::divider::Divider;
use crate::Route::{AboutPage, TodoPage};
use crate::pages::home::todo_card::TodoCard;

#[component]
pub fn HomePage() -> Element {
    let future = use_resource(list_todos);

    let todo_ul = match &*future.read_unchecked() {
        Some(Ok(response)) => {
            let todos = &response.todos;
            let todo_items = todos.iter().map( |todo| {
                rsx! {
                    Link {
                        to: TodoPage { id: todo.id.clone() },
                        li { TodoCard { todo: todo.clone() } }
                    }
                }
            });
            rsx! {
                ul {
                    class: "flex flex-col gap-2",
                    { todo_items }
                }
            }
        }
        Some(Err(_e)) => {
            rsx! { p { "Error occurred" } }
        }
        None => {
            rsx! { p { "Loading..." } }
        }
    };

    rsx! {
        Link { to: AboutPage {}, "Go to about" }
        div {
            class: "flex flex-col items-center",
            h1 { class: "text-xl mt-2", "TODO List" }
            Divider { class: "mt-2" }
            div {
                button {
                    class: "bg-green-700 text-white px-4 py-3 mt-2 rounded",
                    onclick: |event| async move {
                        add_todo("test todo").await.unwrap();
                        tracing::info!("Clicked! Event: {event:?}")
                    },
                    "NEW Todo"
                }
            }
            div {
                class: "mt-2 px-4",
                { todo_ul }
            }
        }
    }
}
