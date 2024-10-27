use crate::components::icon::Quotation;
use crate::constants::LOCALES;
use crate::model::app_state::ApplicationData;
use crate::model::card::Review;
use dioxus::prelude::*;
use fluent_templates::Loader;
use std::str::FromStr;
use unic_langid::LanguageIdentifier;

#[component]
pub fn ReviewersCard() -> Element {
    let data = use_context::<ApplicationData>();
    let lang_id = &LanguageIdentifier::from_str(&(data.lang_code)() as &str).unwrap();
    let reviews: [Review; 5] = [
        Review {
            paragraph: LOCALES.lookup(lang_id, "jane-par"),
            img: asset!("/assets/images/review-1.jpg").input,
            img_alt: LOCALES.lookup(lang_id, "jane-alt"),
            name: LOCALES.lookup(lang_id, "jane-name"),
            job: LOCALES.lookup(lang_id, "jane-job"),
        },
        Review {
            paragraph: LOCALES.lookup(lang_id, "mark-par"),
            img: asset!("/assets/images/review-2.jpg").input,
            img_alt: LOCALES.lookup(lang_id, "mark-alt"),
            name: LOCALES.lookup(lang_id, "mark-name"),
            job: LOCALES.lookup(lang_id, "mark-job"),
        },
        Review {
            paragraph: LOCALES.lookup(lang_id, "emily-par"),
            img: asset!("/assets/images/review-3.jpg").input,
            img_alt: LOCALES.lookup(lang_id, "emily-alt"),
            name: LOCALES.lookup(lang_id, "emily-name"),
            job: LOCALES.lookup(lang_id, "emily-job"),
        },
        Review {
            paragraph: LOCALES.lookup(lang_id, "tom-par"),
            img: asset!("/assets/images/review-4.jpg").input,
            img_alt: LOCALES.lookup(lang_id, "tom-alt"),
            name: LOCALES.lookup(lang_id, "tom-name"),
            job: LOCALES.lookup(lang_id, "tom-job"),
        },
        Review {
            paragraph: LOCALES.lookup(lang_id, "michael-par"),
            img: asset!("/assets/images/review-5.jpg").input,
            img_alt: LOCALES.lookup(lang_id, "michael-alt"),
            name: LOCALES.lookup(lang_id, "michael-name"),
            job: LOCALES.lookup(lang_id, "michael-job"),
        },
    ];
    rsx! {
        for card in reviews.iter() {
            li { class: "",
                div { class: "flex flex-col gap-5 bg-primaryColor dark:bg-darkColor rounded-lg p-6 card-shadow",
                    p { class: "paragraph", "{card.paragraph}" }
                    div { class: "flex items-center",
                        img {
                            class: "w-12 h-12 rounded-full",
                            src: card.img,
                            alt: card.img_alt.clone()
                        }
                        div { class: "ml-2",
                            p { class: "font-oswald uppercase", "{card.name}" }
                            p { class: "paragraph", "{card.job}" }
                        }
                        // Quotation marks icon
                        Quotation {}
                    }
                }
            }
        }
    }
}
