use dioxus::prelude::*;
use crate::domain::todo::Todo;

#[derive(PartialEq, Props, Clone)]
pub struct TodoCardProps {
    todo: Todo,
}

pub fn TodoCard(props: TodoCardProps) -> Element {
    let todo = props.todo;
    rsx! {
        div {
            span { "{todo.id}" }
            span { ": " }
            span { "{todo.title}" }
        }
    }
}
