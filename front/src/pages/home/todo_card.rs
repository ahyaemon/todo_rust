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
            class: "border-2 p-4 min-w-80",
            span { "{todo.title}" }
        }
    }
}
