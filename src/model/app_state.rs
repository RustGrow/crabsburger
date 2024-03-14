use dioxus::prelude::*;

#[derive(Clone, Copy, Default)]
pub struct ApplicationData {
    pub selected_menu: Signal<usize>,
}

// impl ApplicationData {
//     pub fn new() -> Self {
//         Self {
//             selected_menu: use_signal(|| 0 as usize),
//         }
//     }
// }
