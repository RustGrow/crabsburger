use dioxus::prelude::*;
// pub enum Visibility {
//     Visible,
//     Hidden,
// }
// pub enum Theme {
//     Light,
//     Dark,
// }

#[derive(Clone, Copy, Default)]
pub struct ApplicationData {
    pub selected_menu: Signal<usize>,
    pub dark: Signal<bool>,
    pub header_border_style_on_scroll: Signal<bool>,
    pub scroll_button_visibility: Signal<String>,
    pub hidden_menu: Signal<String>,
}

impl ApplicationData {
    pub fn new() -> Self {
        Self {
            selected_menu: Signal::new(0 as usize),
            dark: Signal::new(false),
            header_border_style_on_scroll: Signal::new(false),
            scroll_button_visibility: Signal::new("hidden".to_string()),
            hidden_menu: Signal::new("hidden".to_string()),
        }
    }
}
