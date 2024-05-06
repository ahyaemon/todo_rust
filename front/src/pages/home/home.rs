#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::adapter::todo_client::list_todos;
use crate::Route::About;
use crate::pages::home::todo_card::TodoCard;

#[component]
pub fn Home() -> Element {
    let future = use_resource(list_todos);

    let todos = match &*future.read_unchecked() {
        Some(Ok(response)) => {
            let todos = &response.todos;
            let todo_items = todos.iter().map( |r| {
                rsx! { li { TodoCard { todo: r.clone() } } }
            });
            rsx! {
                ul { { todo_items } }
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
        Link { to: About {}, "Go to about" }
        div {
            h1 { "TODO List" }
            hr {}
            { todos }
        }
    }
}
