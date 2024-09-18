#![allow(non_snake_case)]
mod components;
mod model;
mod repository;
mod route;
mod utils;
use crate::model::app_state::ApplicationData;
use crate::route::Route;
use crate::utils::evals::InitThemeColorState;
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

const STYLE: &str = asset!("./assets/tailwind.css");
// Note: For development use only. Remove before production.
// const TAILWIND_CDN: &str = asset!("https://cdn.tailwindcss.com");

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    use_context_provider(ApplicationData::new);
    InitThemeColorState().expect("Fail to init color theme");
    rsx! {
        head::Link { rel: "stylesheet", href: STYLE }
        // Note: For development use only. Remove before production.
        // script { src: TAILWIND_CDN }
        Router::<Route> {}
    }
}
