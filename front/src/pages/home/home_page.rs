#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::adapter::todo_client::list_todos;
use crate::Route::{AboutPage, TodoPage};
use crate::pages::home::todo_card::TodoCard;

#[component]
pub fn HomePage() -> Element {
    let future = use_resource(list_todos);

    let todos = match &*future.read_unchecked() {
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
                    class: "flex flex-col gap-1",
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
            h1 {
                class: "text-xl mt-2",
                "TODO List"
            }
            hr {
                class: "border-1 border-slate-500 w-dvw"
            }
            div {
                class: "mt-2 px-4",
                { todos }
            }
        }
    }
}
