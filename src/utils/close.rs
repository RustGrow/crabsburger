use crate::model::app_state::ApplicationData;
use dioxus::prelude::*;

pub fn close_dropdown() {
    let mut data = consume_context::<ApplicationData>();
    (data.show_hidden_menu).set(false);
    (data.show_lang_menu).set(false);
}
