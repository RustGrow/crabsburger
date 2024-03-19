use crate::model::app_state::ApplicationData;
use crate::route::Route;
use crate::utils::evals::InitThemeColorState;
use dioxus::prelude::*;

pub fn App() -> Element {
    use_context_provider(ApplicationData::default);

    InitThemeColorState();
    rsx! { Router::<Route> {} }
}
