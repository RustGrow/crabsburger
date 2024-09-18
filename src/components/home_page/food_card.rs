use crate::model::card::Food;
use dioxus::prelude::*;

#[component]
pub fn FoodCard(card: Food) -> Element {
    rsx! {
        li { class: "item_wrap",
            div { class: "h-56 grid place-items-center bg-primaryColorLight dark:bg-darkColorLight rounded-3xl hover:bg-secondaryColor ease-linear duration-200 lg:h-40 card-shadow",
                img {
                    class: "w-40 hover:scale-110 ease-linear duration-200 md:w-48 lg:w-24",
                    src: card.img_path,
                    alt: card.alt
                }
            }
            div { class: "pt-5",
                div { class: "mb-2",
                    h4 { class: "card-title", "{card.title}" }
                    p { class: "paragraph", "{card.description}" }
                }
                p { class: " text-black dark:text-secondaryColor", "{card.price}" }
            }
        }
    }
}
