use crate::components::icon::CheckMark;
use crate::constants::LOCALES;
use crate::model::app_state::ApplicationData;
use dioxus::prelude::*;
use fluent_templates::Loader;
use std::str::FromStr;
use unic_langid::LanguageIdentifier;

#[component]
pub fn Check() -> Element {
    let data = use_context::<ApplicationData>();
    let lang_id = &LanguageIdentifier::from_str(&(data.lang_code)() as &str).unwrap();
    let card_name: [String; 4] = [
        { LOCALES.lookup(lang_id, "check-price") },
        { LOCALES.lookup(lang_id, "check-ingredient") },
        { LOCALES.lookup(lang_id, "check-service") },
        { LOCALES.lookup(lang_id, "check-protocol") },
    ];
    rsx! {
        for check in card_name.iter() {
            li { class: "flex flex-row items-center gap-1 text-base text-paragraphColor dark:text-white",
                // Check mark icon
                CheckMark {}
                "{check}"
            }
        }
    }
}
