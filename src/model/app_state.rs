use dioxus::prelude::*;

#[derive(Clone, Copy, Default)]
pub struct ApplicationData {
    pub selected_menu: Signal<usize>,
    pub dark: Signal<bool>,
    pub header_border_style_on_scroll: Signal<bool>,
    pub scroll_button_visibility: Signal<bool>,
    pub show_hidden_menu: Signal<bool>,
    pub lang_code: Signal<String>,
    pub show_lang_menu: Signal<bool>,
    pub title_menu: Signal<String>,
}

impl ApplicationData {
    pub fn new() -> Self {
        Self {
            selected_menu: Signal::new(0 as usize),
            dark: Signal::new(false),
            header_border_style_on_scroll: Signal::new(false),
            scroll_button_visibility: Signal::new(false),
            show_hidden_menu: Signal::new(false), // hidden
            lang_code: Signal::new("en".to_string()),
            show_lang_menu: Signal::new(false),
            title_menu: Signal::new("Home".to_string()),
        }
    }
}
