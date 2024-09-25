use crate::components::icon::*;
use crate::components::lang_dd::LangDropDown;
use crate::constants::LOCALES;
use crate::model::app_state::ApplicationData;
use crate::route::Route;
use crate::utils::evals::{toggle_navbar_style_on_scroll, NavBarToggle};
use dioxus::prelude::*;
use fluent_templates::Loader;
use std::str::FromStr;
use unic_langid::LanguageIdentifier;

#[component]
pub fn NavBar() -> Element {
    let mut data = use_context::<ApplicationData>();
    let lang_id = &LanguageIdentifier::from_str(&(data.lang_code)() as &str).unwrap();
    let menu = vec![
        { LOCALES.lookup(lang_id, "Home") },
        { LOCALES.lookup(lang_id, "About") },
        { LOCALES.lookup(lang_id, "Menu") },
        { LOCALES.lookup(lang_id, "Review") },
        { LOCALES.lookup(lang_id, "Contact") },
    ];

    rsx! {
        toggle_navbar_style_on_scroll { navbar_style: data.header_border_style_on_scroll }
        // Header ----------------------------------
        header {
            class: "bg-primaryColor dark:bg-darkColor fixed top-0 left-0 w-full z-50 ",
            class: if *data.header_border_style_on_scroll.read() { "border-b border-secondaryColor card-shadow" },
            nav { class: "container relative h-14 flex justify-between items-center",
                div {
                    a {
                        href: "#",
                        class: "text-2xl uppercase font-poppinsRegular font-bold gradient  drop-shadow shadow-black dark:shadow-white",
                        "CrabsBurger"
                    }
                }

                // Menu
                div {
                    class: "absolute top-0 left-0 w-full py-14 bg-primaryColor dark:bg-darkColor border-b border-secondaryColor md:block md:static md:py-0 md:border-none md:w-auto md:ml-auto",
                    class: if !*data.show_hidden_menu.read() { "hidden" },
                    ul { class: "flex flex-col text-center gap-5 md:flex-row",
                        { menu.iter().enumerate().map(|(id, _)| {
                        let selected = *data.selected_menu.read() == id;

                        let bg_selected = match selected {
                            true => "gradient ease-in duration-200",
                            false => "hover:gradient ease-in duration-200",
                        };

                        rsx! {
                            li {
                            onclick: move |_| {
                                data.selected_menu.set(id);
                                // hidden open menu from mobile
                                data.show_hidden_menu.set(false)},
                            a {
                                class: "{bg_selected}",
                                href: "#{menu[id].to_lowercase()}",
                                "{menu[id]}"
                            }

                        }
                        }})}
                    }
                    div {
                        class: "absolute top-[0.7rem] right-4 cursor-pointer md:hidden",
                        onclick: move |_| { data.show_hidden_menu.set(false) },
                        // Close X icon
                        Close {}
                    }
                }
                div { class: "flex flex-row items-center gap-5",
                    div {
                        onclick: move |_| {
                            data.dark.toggle();
                            NavBarToggle()
                        },

                        {if *data.dark.read() {
                            // Sun icon
                            rsx!{
                                Sun{}
                            }
                        } else {
                            // Moon icon
                            rsx!{ Moon{}}
                        }
                        }
                    }
                    if !(data.show_hidden_menu)() {
                        div { LangDropDown {} }
                    }
                    div { onclick: move |_| { data.show_hidden_menu.set(true) },
                        // Hamburger icon
                        Hamburger {}
                    }
                }
            }
        }
        Outlet::<Route> {}
    }
}
