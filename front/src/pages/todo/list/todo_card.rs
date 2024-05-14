use crate::adapter::todo_client::TodoSummary;
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct TodoCardProps {
    todo_summary: TodoSummary,
}

pub fn TodoCard(props: TodoCardProps) -> Element {
    let todo_summary = props.todo_summary;
    rsx! {
        div {
            class: "border-2 p-4 min-w-80",
            p {
                class: "font-bold",
                "{todo_summary.title}"
            }
            p { "{todo_summary.description_summary}" }
        }
    }
}
