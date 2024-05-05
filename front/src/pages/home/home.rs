#![allow(non_snake_case)]

use dioxus::prelude::*;
use serde::Deserialize;
use tracing::Level;
use crate::Route::About;
use crate::utils::log::log;

#[derive(Deserialize)]
struct ApiResponse {
    message: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
struct Todo {
    id: String,
    title: String,
}

#[derive(Deserialize, Debug)]
struct ListTodosResponse {
    todos: Vec<Todo>
}

#[component]
fn TodoCard(todo: Todo) -> Element {
    rsx! {
        div { "todo card" }
    }
}

#[component]
pub fn Home() -> Element {
    let mut count = use_signal(|| 0);
    let future = use_resource(|| async move {
        // reqwest::get("https://dog.ceo/api/breeds/image/random")
        reqwest::get("http://localhost:18080/todos")
            .await
            .unwrap()
            .json::<ListTodosResponse>()
            .await
    });

    let todos = match &*future.read_unchecked() {
        Some(Ok(response)) => {
            let todos = &response.todos;
            let todo_items = todos.iter().map( |r| {
                rsx! {
                    div {
                        span {{ r.id.clone() }}
                        span { ": " }
                        span {{ r.title.clone() }}
                    }
                }
            });

            log(&format!("ok: {:?}", response.todos));
            rsx! {
                div { { todo_items } }
            }
        }
        Some(Err(e)) => {
            log(&format!("err: {}", e));
            rsx! {
                p { "Error occurred" }
            }
        }
        None => {
            log("none");
            rsx! {
                p { "Loading..." }
            }
        }
    };

    let a = rsx! {
        div { "div dayo" }
    };

    rsx! {
        Link { to: About {}, "Go to about" }
        div {
            h1 { "TODO List" }
            hr {}
            div {

            }
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
            { todos }
        }
    }
}
