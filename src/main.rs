#![allow(non_snake_case)]
mod components;
mod model;
mod repository;
use crate::route::Route;
mod route;
use dioxus::prelude::*;

const _STYLE: &str = manganis::mg!(file("public/tailwind.css"));
fn main() {
    launch(|| {
        rsx! { Router::<Route> {} }
    });
}
