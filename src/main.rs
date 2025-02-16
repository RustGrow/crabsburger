#![allow(non_snake_case)]
mod components;
mod constants;
mod model;
mod route;
mod utils;
use crate::model::app_state::ApplicationData;
use crate::route::Route;
use crate::utils::evals::InitThemeColorState;
use dioxus::document;
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use utils::{close::close_dropdown, evals::LangSettings};

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    dioxus::launch(App);
}

fn App() -> Element {
    use_context_provider(ApplicationData::new);
    InitThemeColorState().expect("Fail to init color theme");
    rsx! {
        LangSettings {}
        document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }
        div { class: "w-full h-full", onclick: move |_| close_dropdown(), Router::<Route> {} }
    }
}
