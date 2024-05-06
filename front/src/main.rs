#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;

mod pages;
mod utils;
mod domain;
mod adapter;

use crate::pages::home::home_page::HomePage;
use crate::pages::about::about_page::AboutPage;
use crate::pages::todo::todo_page::TodoPage;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    HomePage {},
    #[route("/about")]
    AboutPage {},
    #[route("/todos/:id")]
    TodoPage { id: String }
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
