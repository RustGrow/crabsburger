use crate::constants::LOCALES;
use crate::model::app_state::ApplicationData;
use crate::model::card::{Image, Promo};
use dioxus::prelude::*;
use fluent_templates::Loader;
use std::str::FromStr;
use unic_langid::LanguageIdentifier;

#[component]
pub fn PromoCard() -> Element {
    let data = use_context::<ApplicationData>();
    let lang_id = &LanguageIdentifier::from_str(&(data.lang_code)() as &str).unwrap();
    let promo: [Promo; 2] = [
        Promo {
            promo_type: LOCALES.lookup(lang_id, "pr-bur"),
            title: LOCALES.lookup(lang_id, "pr-bur-title"),
            description: LOCALES.lookup(lang_id, "pr-bur-desc"),
            img: Image {
                class: "",
                src: asset!("/assets/images/promo-1.png").input,
                alt: LOCALES.lookup(lang_id, "pr-bur-alt"),
            },
        },
        Promo {
            promo_type: LOCALES.lookup(lang_id, "pr-bev"),
            title: LOCALES.lookup(lang_id, "pr-bev-title"),
            description: LOCALES.lookup(lang_id, "pr-bev-desc"),
            img: Image {
                class: "",
                src: asset!("/assets/images/promo-2.png").input,
                alt: LOCALES.lookup(lang_id, "pr-bev-alt"),
            },
        },
    ];
    rsx! {
        for card in promo.iter() {
            div { class: "bg-primaryColorLight dark:bg-darkColorLight flex flex-col p-5 rounded-lg md:flex-row md:items-center lg:flex-row-reverse lg:flex-1 card-shadow",
                img {
                    class: "w-40 mx-auto hover:animate-movingY md:mx-5",
                    src: "{card.img.src}",
                    alt: "{card.img.alt}"
                }
                div { class: "space-y-2 pt-5 md:pt-0",
                    p { class: "text-base text-redColor dark:text-secondaryColor",
                        "{card.promo_type}"
                    }
                    h3 { class: "card-title", "{card.title}" }
                    p { class: "paragraph", "{card.description}" }
                    a {
                        class: "text-base text-redColor dark:text-secondaryColor",
                        href: "#",
                        "{LOCALES.lookup(lang_id, \"buy-online\")}"
                    }
                }
            }
        }
    }
}
