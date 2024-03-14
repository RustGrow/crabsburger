use crate::model::app_state::ApplicationData;
use crate::route::Route;
use dioxus::prelude::*;

pub fn App() -> Element {
    use_context_provider(ApplicationData::default);
    rsx! { Router::<Route> {} }
}
