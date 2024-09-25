#![allow(non_snake_case)]
mod components;
mod constants;
mod model;
mod repository;
mod route;
mod utils;
use crate::model::app_state::ApplicationData;
use crate::route::Route;
use crate::utils::evals::InitThemeColorState;
use constants::STYLE;
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use utils::evals::LangSettings;

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    use_context_provider(ApplicationData::new);
    InitThemeColorState().expect("Fail to init color theme");
    rsx! {
        LangSettings {}
        head::Link { rel: "stylesheet", href: STYLE }
        Router::<Route> {}
    }
}
