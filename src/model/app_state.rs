use dioxus::prelude::*;
#[derive(PartialEq)]
pub enum Visibility {
    Visible,
    Hidden,
}
#[derive(PartialEq)]
pub enum Theme {
    Light,
    Dark,
}

#[derive(Clone, Copy)]
pub struct ApplicationData {
    pub selected_menu: Signal<usize>,
    pub theme_state: Signal<String>,
    pub header_border_style_on_scroll: Signal<Visibility>,
    pub scroll_button_visibility: Signal<Visibility>,
    pub hidden_menu: Signal<String>,
}

impl ApplicationData {
    pub fn new() -> Self {
        Self {
            selected_menu: Signal::new(0 as usize),
            theme_state: Signal::new("".to_string()),
            header_border_style_on_scroll: Signal::new(Visibility::Hidden),
            scroll_button_visibility: Signal::new(Visibility::Hidden),
            hidden_menu: Signal::new("hidden".to_string()),
        }
    }
}
