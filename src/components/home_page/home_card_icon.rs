use crate::components::icon::home_icons;
use crate::constants::LOCALES;
use crate::model::app_state::ApplicationData;
use dioxus::prelude::*;
use fluent_templates::Loader;
use std::str::FromStr;
use unic_langid::LanguageIdentifier;

#[component]
pub fn HomeCardIcon() -> Element {
    let data = use_context::<ApplicationData>();
    let lang_id = &LanguageIdentifier::from_str(&(data.lang_code)() as &str).unwrap();
    let card_name: [String; 3] = [
        { LOCALES.lookup(lang_id, "Delicious") },
        { LOCALES.lookup(lang_id, "Fresh") },
        { LOCALES.lookup(lang_id, "Organic") },
    ];

    rsx! {
        for (name , svg_icon) in card_name.iter().zip(home_icons().iter()) {
            div { class: "text-center flex flex-col items-center justify-center",
                {svg_icon}
                br {}
                "{name}"
            }
        }
    }
}
