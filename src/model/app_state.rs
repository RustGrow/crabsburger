use dioxus::prelude::*;
// pub enum Visibility {}
// pub enum Theme {}

#[derive(Clone, Copy, Default)]
pub struct ApplicationData {
    pub selected_menu: Signal<usize>,
    pub theme_state: Signal<String>,
    pub header_border_visible: Signal<String>,
    pub scroll_button_visible: Signal<String>,
    pub hidden_menu: Signal<String>,
}

// impl ApplicationData {
//     pub fn new() -> Self {
//         Self {
//             selected_menu: Signal::new(0 as usize),
//             theme_state: Signal::new("".to_string()),
//             header_border_visible: Signal::new("hidden".to_string()),
//             scroll_button_visible: Signal::new("hidden".to_string()),
//             hidden_menu: Signal::new("hidden".to_string()),
//         }
//     }
// }
