use dioxus::prelude::*;

#[derive(Clone, Copy, Default)]
pub struct ApplicationData {
    pub selected_menu: Signal<usize>,
    pub theme_state: Signal<String>,
    pub header_border_visible: Signal<String>,
    pub scroll_button_visible: Signal<String>,
    pub hidden_menu: Signal<String>,
}

impl ApplicationData {
    pub fn new() -> Self {
        Self {
            selected_menu: use_signal(|| 0 as usize),
            theme_state: use_signal(|| "".to_string()),
            header_border_visible: use_signal(|| "hidden".to_string()),
            scroll_button_visible: use_signal(|| "hidden".to_string()),
            hidden_menu: use_signal(|| "hidden".to_string()),
        }
    }
}
