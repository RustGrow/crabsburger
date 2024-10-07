use crate::constants::LOCALES;
use crate::model::app_state::ApplicationData;
use crate::model::card::{Category, Image};
use dioxus::prelude::*;
use fluent_templates::Loader;
use std::str::FromStr;
use unic_langid::LanguageIdentifier;

#[component]
pub fn CategoryCard() -> Element {
    let data = use_context::<ApplicationData>();
    let lang_id = &LanguageIdentifier::from_str(&(data.lang_code)() as &str).unwrap();
    let categories: [Category; 3] = [
        Category {
            title: LOCALES.lookup(lang_id, "burger"),
            text: LOCALES.lookup(lang_id, "cat-bur-text"),
            img: Image {
                class: "",
                src: "/images/burger-1.png",
                alt: LOCALES.lookup(lang_id, "cat-bur-alt"),
            },
            bg_color: "bg-secondaryColor",
        },
        Category {
            title: LOCALES.lookup(lang_id, "snack"),
            text: LOCALES.lookup(lang_id, "cat-sn-text"),
            img: Image {
                class: "",
                src: "/images/snack-1.png",
                alt: LOCALES.lookup(lang_id, "cat-sn-alt"),
            },
            bg_color: "bg-redColor",
        },
        Category {
            title: LOCALES.lookup(lang_id, "beverage"),
            text: LOCALES.lookup(lang_id, "cat-bev-text"),
            img: Image {
                class: "",
                src: "/images/beverage-2.png",
                alt: LOCALES.lookup(lang_id, "cat-bev-alt"),
            },
            bg_color: "bg-greenColor",
        },
    ];
    rsx! {
        for card in categories.iter() {
            div { class: "{card.bg_color} flex py-3 rounded-lg overflow-hidden md:flex-1",
                div { class: "basis-1/3 relative",
                    img {
                        class: "absolute w-28 -bottom-4 -left-4",
                        src: "{card.img.src}",
                        alt: "{card.img.alt}",
                    }
                }
                div {
                    div { class: "mb-2",
                        h4 { class: "card-title", "{card.title}" }
                        p { class: "text-xs", "{card.text}" }
                    }
                    a { class: "text-blackColor cursor-pointer", href: "#",
                        "{LOCALES.lookup(lang_id, \"buy-online\")}"
                    }
                }
            }
        }
    }
}
