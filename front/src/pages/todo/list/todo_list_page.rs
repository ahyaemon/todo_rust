#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::adapter::todo_client::{list_todo_summaries};
use crate::components::divider::Divider;
use crate::layouts::menu::Menu;
use crate::Route::{TodoDetailPage, TodoAddPage};
use crate::pages::todo::list::todo_card::TodoCard;

#[component]
pub fn TodoListPage() -> Element {
    let future = use_resource(list_todo_summaries);

    let todo_ul = match &*future.read_unchecked() {
        Some(Ok(response)) => {
            let todos = &response.todo_summaries;
            let todo_items = todos.iter().map( |todo_summary| {
                rsx! {
                    Link {
                        to: TodoDetailPage { id: todo_summary.id.clone() },
                        li { TodoCard { todo_summary: todo_summary.clone() } }
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
        Menu {}
        div {
            class: "flex flex-col items-center",
            h1 { class: "text-xl mt-2", "TODO List" }
            Divider { class: "mt-2" }
            div {
                class: "mt-4",
                Link {
                    class: "bg-green-700 text-white px-4 py-3 rounded",
                    to: TodoAddPage {},
                    "NEW Todo"
                }
            }
            div {
                class: "mt-4 px-4",
                { todo_ul }
            }
        }
    }
}
