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
use fluent_templates::{static_loader, Loader};

const STYLE: &str = asset!("./assets/tailwind.css");

static_loader! {
    static LOCALES = {
        locales: "./lang",
        fallback_language: "en-US",
        customise: |bundle| bundle.set_use_isolating(false),
    };
}

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
        Router::<Route> {}
    }
}
