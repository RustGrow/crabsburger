use crate::components::icon::{flags, Lang};
use crate::constants::{LANG_CODES, LANG_NAMES, LOCALES};
use crate::model::app_state::ApplicationData;
use crate::utils::evals::ButtonLang;
use crate::Route;
use dioxus::prelude::*;
use fluent_templates::Loader;
use std::str::FromStr;
use unic_langid::LanguageIdentifier;

#[component]
pub fn LangDropDown() -> Element {
    let mut data = use_context::<ApplicationData>();
    let lang_id = &LanguageIdentifier::from_str(&(data.lang_code)() as &str).unwrap();
    let rtl = use_memo(move || {
        if (data.lang_code)() == "ar" {
            true
        } else {
            false
        }
    });
    rsx! {
        div { class: "relative ml-3",
            div {
                button {
                    class: "relative flex rounded-full bg-white text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2",
                    r#type: "button",
                    id: "user-menu-button",
                    aria_expanded: "false",
                    aria_haspopup: "true",
                    onclick: move |_| (data.show_lang_menu).toggle(),
                    // "Up high!"
                    span { class: "absolute -inset-1.5" }
                    span { class: "sr-only", "Open user menu" }

                    Lang {}
                }
            }
            div {
                class: "absolute z-10 mt-2 w-40 origin-top-right rounded-md bg-white py-1 shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none",
                class: if !(data.show_lang_menu)() { "hidden" },
                class: if !rtl() { "right-0" } else { "left-0" },
                role: "menu",
                aria_orientation: "vertical",
                aria_labelledby: "user-menu-button",
                ul { class: "flex flex-col",
                    for ((code , name) , flag) in LANG_CODES.iter().zip(LANG_NAMES.iter()).zip(flags().iter()) {
                        match *code {
                            "en" => rsx!{
                                Link { class: "grid grid-cols-3 gap-4 text-sm text-gray-700 hover:bg-slate-300 cursor-pointer hover:ring-1 items-center px-2 py-1",
                                    onclick: move |_| {
                                        (data.lang_code).set(code.to_string());
                                        let eval = ButtonLang();
                                        eval.send((*code).into()).unwrap();
                                        (data.show_lang_menu).toggle();
                                    },
                                    to: Route::Home {},
                                        div{ class: "col-span-1 ", {flag} },
                                        div { class: "col-span-2 text-base", {LOCALES.lookup(lang_id, name)} }
                                },
                            },
                            _ => rsx!{
                                Link { class: "grid grid-cols-3 gap-4 text-sm text-gray-700 hover:bg-slate-300 cursor-pointer hover:ring-1 items-center px-2 py-1",
                                    onclick: move |_| {
                                        (data.lang_code).set(code.to_string());
                                        let eval = ButtonLang();
                                        eval.send((*code).into()).unwrap();
                                        (data.show_lang_menu).toggle();
                                    },
                                    to: Route::HomeLang {
                                        lang: code.to_string(),
                                    },
                                    div{ class: "col-span-1 ", {flag} },
                                    div { class: "col-span-2 text-base", {LOCALES.lookup(lang_id, name)} }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
