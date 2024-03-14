use crate::model::review_card::ReviewCard;
use dioxus::prelude::*;

#[component]
pub fn ReviewersCard(card: ReviewCard) -> Element {
    rsx! {
        li { class: "",
            div { class: "flex flex-col gap-5 bg-primaryColor dark:bg-darkColor rounded-lg p-6",
                p { class: "paragraph", "{card.paragraph}" }
                div { class: "flex items-center",
                    img {
                        class: "w-12 h-12 rounded-full",
                        src: "images/{card.img}",
                        alt: card.img_alt
                    }
                    div { class: "ml-2",
                        p { class: "font-oswald uppercase", "{card.name}" }
                        p { class: "paragraph", "{card.job}" }
                    }
                    svg {
                        class: "h-9 w-9 fill-current text-secondaryColor ml-auto",
                        xmlns: "http://www.w3.org/2000/svg",
                        view_box: "0 0 448 512",
                        path {
                            // fill: "#1E3050",
                            d: "M448 296c0 66.3-53.7 120-120 120h-8c-17.7 0-32-14.3-32-32s14.3-32 32-32h8c30.9 0 56-25.1 56-56v-8h-64c-35.3 0-64-28.7-64-64v-64c0-35.3 28.7-64 64-64h64c35.3 0 64 28.7 64 64v136zm-256 0c0 66.3-53.7 120-120 120h-8c-17.7 0-32-14.3-32-32s14.3-32 32-32h8c30.9 0 56-25.1 56-56v-8H64c-35.3 0-64-28.7-64-64v-64c0-35.3 28.7-64 64-64h64c35.3 0 64 28.7 64 64v136z"
                        }
                    }
                }
            }
        }
    }
}
