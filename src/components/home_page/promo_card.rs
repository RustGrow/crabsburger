use crate::model::card::Promo;
use dioxus::prelude::*;

#[component]
pub fn PromoCard(card: Promo) -> Element {
    rsx! {
        div { class: "bg-primaryColorLight dark:bg-darkColorLight flex flex-col p-5 rounded-lg md:flex-row md:items-center lg:flex-row-reverse lg:flex-1 card-shadow",
            img {
                class: "w-40 mx-auto hover:animate-movingY md:mx-5",
                src: "{card.img.src}",
                alt: "{card.img.alt}"
            }
            div { class: "space-y-2 pt-5 md:pt-0",
                p { class: "text-base text-redColor dark:text-secondaryColor", "{card.promo_type}" }
                h3 { class: "card-title", "{card.title}" }
                p { class: "paragraph", "{card.description}" }
                a {
                    class: "text-base text-redColor dark:text-secondaryColor",
                    href: "#",
                    "Buy online"
                }
            }
        }
    }
}
