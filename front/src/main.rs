#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;

mod pages;
mod utils;

use crate::pages::{
    home::home::Home,
    about::about::About,
};

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/about")]
    About {},
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
