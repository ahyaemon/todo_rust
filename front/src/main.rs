#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;

mod adapter;
mod components;
mod domain;
mod layouts;
mod pages;
mod utils;

use crate::pages::about::about_page::AboutPage;
use crate::pages::todo::add::todo_add_page::TodoAddPage;
use crate::pages::todo::detail::todo_detail_page::TodoDetailPage;
use crate::pages::todo::list::todo_list_page::TodoListPage;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    TodoListPage {},
    #[route("/about")]
    AboutPage {},
    #[route("/todos/:id")]
    TodoDetailPage { id: String },
    #[route("/todos/new")]
    TodoAddPage {},
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
