#![allow(non_snake_case)]
use super::category_card::CategoryCard;
use super::food_card::FoodCard;
use super::home_card_icon::HomeCardIcon;
use super::promo_card::PromoCard;
use super::reviewers_card::ReviewersCard;
use crate::components::icon::*;
use crate::constants::*;
use crate::model::app_state::ApplicationData;
use crate::repository::category_repo::CATEGORY_CARDS;
use crate::repository::food_repo::{BEVERAGE, BURGERS, SNACKS};
use crate::repository::promo_repo::PROMO_CARDS;
use crate::repository::review_repo::REVIEWERS;
use crate::utils::evals::ScrollButtonVisible;
use crate::Route;
use chrono::Datelike;
use dioxus::prelude::*;
use fluent_templates::Loader;
use std::str::FromStr;
use unic_langid::LanguageIdentifier;

#[component]
pub fn Home() -> Element {
    // let lang: Signal<String> = use_context();
    let data = use_context::<ApplicationData>();

    let nav = navigator();
    if &(data.lang_code)() as &str != "en" {
        nav.push(Route::HomeLang {
            lang: (data.lang_code)(),
        });
    }
    rsx! {
        HomeContent {}
    }
}

#[component]
pub fn HomeLang(lang: String) -> Element {
    // let lang: Signal<String> = use_context();
    let data = use_context::<ApplicationData>();
    let nav = navigator();
    if &(data.lang_code)() as &str == "en" {
        nav.push(Route::Home {});
    }
    rsx! {
        HomeContent {}
    }
}

pub fn HomeContent() -> Element {
    let mut data = use_context::<ApplicationData>();
    let lang_id = &LanguageIdentifier::from_str(&(data.lang_code)() as &str).unwrap();
    let tabs = vec!["All", "Burger", "Snack", "Beverage"];
    let mut selected_snippet = use_signal(|| 0);

    rsx! {
        ScrollButtonVisible {}
        main {
            // Home ----------------------------------------------
            section { id: "{LOCALES.lookup(lang_id, \"Home\").to_lowercase()}",
                div { class: "container flex flex-col items-center gap-10 md:flex-row",
                    div { class: "mx-auto md:basis-1/2 lg:basis-2/5 animate-movingY",
                        img {
                            class: " w-60 md:w-full",
                            src: "/images/home-image.png",
                            alt: "home image"
                        }
                    }
                    div { class: "text-center md:basis-1/2 md:text-start lg:basis-3/5",
                        h1 { class: "home-title drop-shadow shadow-black dark:shadow-white",
                            "{LOCALES.lookup(lang_id, \"home-title\")}"
                            span { class: "gradient", "CRABSBURGER" }
                        }
                        div { class: "separator mx-auto md:mx-0" }
                        p { class: "paragraph", "{LOCALES.lookup(lang_id, \"home-desc\")}" }

                        // Home card icons
                        div { class: "text-base flex items-center justify-center gap-4 py-10 md:justify-start md:gap-20",
                            HomeCardIcon {}
                        }
                        a { class: "btn btn-primary", href: "#",
                            "{LOCALES.lookup(lang_id, \"lern-more\")}"
                        }
                    }
                }
            }
            // Category --------------------------------------------
            section { id: "category",
                div { class: "container flex flex-col gap-5 md:flex-row",
                    {CATEGORY_CARDS.iter().enumerate().map(|(_, card)| {
                        rsx!{
                            CategoryCard  {
                                card: *card
                            }
                        }
                    })}
                }
            }
            // Promo ---------------------------------------------
            section { id: "promo",
                div { class: "container flex flex-col gap-5 lg:gap-10 lg:flex-row",
                    {PROMO_CARDS.iter().enumerate().map(|(_, card)| {
                    rsx!{
                        PromoCard  {
                            card: *card
                        }
                    }
                    })}
                }
            }
            // About
            section { id: "{LOCALES.lookup(lang_id, \"About\").to_lowercase()}",
                div { class: "container flex flex-col gap-10 md:flex-row",
                    div { class: "flex-1",
                        img {
                            class: "rounded-lg",
                            src: "/images/about.jpg",
                            alt: "about image"
                        }
                    }
                    div { class: "flex-1",
                        h2 { class: "section-title",
                            "FIND FOOD AND DRINKS, ALL-IN-ONE PLACE FOR YOUR BEST TASTE."
                        }
                        div { class: "separator" }
                        p { class: "paragraph",
                            "Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Aenean commodo ligula eget dolor. Aenean massa. Cum sociis natoque penatibus et magnis dis parturient montes."
                        }
                        ul { class: "grid grid-cols-2 py-5 space-y-1",
                            for check in CHECK.iter() {
                                li { class: "flex flex-row items-center gap-1 text-base text-paragraphColor dark:text-white",
                                    // Check mark icon
                                    CheckMark {}
                                    "{check}"
                                }
                            }
                        }
                        a { class: "btn btn-primary", href: "", "About us" }
                    }
                }
            }
            // Menu
            section { id: "{LOCALES.lookup(lang_id, \"Menu\").to_lowercase()}",
                div { class: "container",
                    div { class: "max-w-md mx-auto text-center",
                        h2 { class: "section-title", "OUR BEST MENU" }
                        div { class: "separator mx-auto" }
                        p { class: "paragraph",
                            "Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Aenean commodo ligula eget dolor. Aenean massa."
                        }
                        div { class: "tabs_wrap",
                            ul { class: "flex flex-wrap justify-center gap-3 py-10",
                                { tabs.iter().enumerate().map(|(id, _)| {
                                    let selected = *selected_snippet.read() == id;

                                    let bg_selected = match selected {
                                        true => "btn bg-secondaryColorLight dark:bg-darkColorLight active",
                                        false => "btn bg-primaryColorLight dark:bg-darkColorLight",
                                    };
                                    rsx! {
                                        li { class: "{bg_selected}",
                                        onclick: move |_| selected_snippet.set(id),
                                        "{tabs[id]}",
                                    }
                                    }
                                })}
                            }
                        }
                    }
                    div { class: "menu-items",
                        ul { class: "grid grid-cols-1 gap-5 md:grid-cols-2 lg:grid-cols-4 lg:gap-12",
                            match selected_snippet() {
                            1 => {
                            rsx!{
                                {BURGERS.iter().enumerate().map(|(_, card)| {
                                    rsx!{
                                        FoodCard  {
                                            card: *card
                                        }
                                    }
                                })}
                            }
                            },
                            2 => {
                            rsx!{
                                {SNACKS.iter().enumerate().map(|(_, card)| {
                                    rsx!{
                                        FoodCard  {
                                            card: *card
                                        }
                                    }
                                })}
                            }

                            },
                            3 => {
                            rsx!{
                                {BEVERAGE.iter().enumerate().map(|(_, card)| {
                                    rsx!{
                                        FoodCard  {
                                            card: *card
                                        }
                                    }
                                })}
                            }
                            },
                            _ => {
                            rsx!{
                                {BURGERS.iter().enumerate().map(|(_, card)| {
                                    rsx!{
                                        FoodCard  {
                                            card: *card
                                        }
                                    }
                                })}
                                {SNACKS.iter().enumerate().map(|(_, card)| {
                                    rsx!{
                                        FoodCard  {
                                            card: *card
                                        }
                                    }
                                })}
                                {BEVERAGE.iter().enumerate().map(|(_, card)| {
                                    rsx!{
                                        FoodCard  {
                                            card: *card
                                        }
                                    }
                                })}
                            }
                            }
                            }
                        }
                    }
                }
            }
            // Review
            section {
                // Review
                id: "{LOCALES.lookup(lang_id, \"Review\").to_lowercase()}",
                // Review
                class: "bg-primaryColorLight dark:bg-darkColorLight py-20",
                div { class: "container",
                    div { class: "max-w-md mx-auto text-center",
                        h2 { class: "section-title", "CUSTOMER REVIEW" }
                        div { class: "separator mx-auto" }
                        p { class: "paragraph",
                            "Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Aenean commodo ligula eget dolor. Aenean massa."
                        }
                    }
                    div { class: "swiper py-10",
                        ul { class: "grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3",
                            {
                                REVIEWERS.iter().enumerate().map(|(_, card)| {
                            rsx!{
                                ReviewersCard  {
                                    card: *card
                                }
                            }
                            })
                            }
                        }
                    }
                }
            }
            section {
                class: "bg-secondaryColor py-16",
                id: "{LOCALES.lookup(lang_id, \"Contact\").to_lowercase()}",
                div { class: "container flex flex-col gap-5 md:items-center md:flex-row",
                    div { class: "space-y-4 md:flex-1",
                        h2 { class: "section-title text-blackColor", "GET EXCLUSIVE UPDATE" }
                        p { class: "text-sm",
                            "Lorem ipsum dolor sit amet, consectetur adipiscing elit."
                        }
                    }
                    div { class: "flex flex-col gap-3 md:flex-row md:flex-1",
                        input {
                            class: "p-2 text-blackColor rounded-lg outline-none md:w-full",
                            r#type: "text",
                            placeholder: "Email address"
                        }
                        a {
                            class: "flex items-center justify-center gap-2 btn text-white bg-blackColor hover:opacity-75",
                            href: "",
                            // Subscribe icon
                            Subscribe {}
                            "Subscribe"
                        }
                    }
                }
            }
        }
        footer {
            div { class: "flex flex-row justify-center items-center w-full h-full",
                a {
                    class: " py-8  text-4xl uppercase font-poppinsRegular font-bold gradient drop-shadow shadow-black dark:shadow-white",
                    href: "",
                    "CrabsBurger"
                }
            }
            div { class: "footer",
                div { class: "container",
                    ul { class: "grid grid-cols-1 items-start gap-5 pb-5 md:grid-cols-2 lg:grid-cols-4",
                        li {
                            div { class: "space-y-3",
                                p { class: "text-sm",
                                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Ut elit tellus, luctus nec ullamcorper mattis, pulvinar dapibus leo."
                                }
                            }
                        }
                        li {
                            div { class: "flex flex-col gap-3",
                                h3 { class: "text-lg uppercase font-oswald", "SUPPORT" }
                                for name in SUPPORT.iter() {
                                    a {
                                        class: "text-xs hover:text-secondaryColor",
                                        href: "",
                                        "{name}"
                                    }
                                }
                            }
                        }
                        li { class: "space-y-8",
                            div { class: "space-y-2",
                                h3 { class: "text-lg uppercase font-oswald", "phone" }
                                p { class: "flex items-center gap-2 text-xs",
                                    // Phone icon
                                    Phone {}
                                    "+1 000 0000000"
                                }
                            }
                            div { class: "space-y-2",
                                h3 { class: "text-lg uppercase font-oswald", "email" }
                                p { class: "flex items-center gap-2 text-xs",
                                    // Email icon
                                    Email {}
                                    "crab.info@email.com"
                                }
                            }
                        }
                        li { class: "space-y-8",
                            div { class: "space-y-2",
                                h3 { class: "text-lg uppercase font-oswald", "address" }
                                p { class: "flex items-center gap-2 text-xs",
                                    // Address icon
                                    Address {}
                                    "Address goes here"
                                }
                            }
                            div { class: "space-y-2",
                                h3 { class: "text-lg uppercase font-oswald", "follow us" }
                                div { class: "space-x-3 flex flex-row",
                                    // Facebook icon
                                    Facebook {}
                                    // X icon
                                    X {}
                                    // Instagram icon
                                    Instagram {}
                                }
                            }
                        }
                    }
                    div { class: "flex flex-col items-center border-t border-primaryColorLight dark:border-darkColorLight py-5 md:flex-row md:justify-between",
                        p { class: "paragraph",
                            span { class: "uppercase gradient", "CrabsBurger" }
                            "Template Kit with ❤️ to "
                            a {
                                href: "https://dioxuslabs.com/",
                                alt: "Dioxus labs",
                                "Dioxus"
                            }
                        }
                        p { class: "paragraph",
                            "Copyright © {chrono::Utc::now().year()}. All rights reserved."
                        }
                    }
                }
            }
        }
        // Scroll button
        a {
            class: "fixed {data.scroll_button_visibility} right-4 bottom-4 h-11 w-11 bg-secondaryColor shadow-sm flex rounded-full text-lg text-blackColor z-50 hover:-translate-y-1 ease-in duration-200 items-center justify-center",
            class: if !*data.scroll_button_visibility.read() { "hidden" },
            onclick: move |_| { data.selected_menu.set(0) },
            href: "#",
            // ArrowUp icon
            ArrowUp {}
        }
    }
}
