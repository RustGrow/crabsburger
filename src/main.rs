#![allow(non_snake_case)]
mod app;
mod components;
mod model;
mod repository;
mod route;
use dioxus::prelude::*;
mod utils;

const _STYLE: &str = manganis::mg!(file("public/tailwind.css"));
fn main() {
    launch(app::App);
}
