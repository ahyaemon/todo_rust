use crate::Route::{AboutPage, TodoListPage};
use dioxus::prelude::*;

pub fn Menu() -> Element {
    rsx! {
        div {
            class: "flex flex-row gap-2",
            Link {
                class: "text-blue-800",
                to: TodoListPage {},
                "Home"
            }
            p { "/" }
            Link {
                class: "text-blue-800",
                to: AboutPage {},
                "About"
            }
        }
    }
}
