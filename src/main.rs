#![allow(non_snake_case)]
mod app;
mod components;
mod model;
mod repository;
mod route;
use dioxus::prelude::*;

const _STYLE: &str = manganis::mg!(file("public/tailwind.css"));
fn main() {
    launch(app::App);
}
