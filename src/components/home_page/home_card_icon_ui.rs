use crate::model::home_card_icon::HomeCardIcon;
use dioxus::prelude::*;

#[component]
pub fn HomeCardIcon(card: HomeCardIcon) -> Element {
    rsx! {
        div { class: "text-center flex flex-col items-center justify-center",
            svg {
                class: "fa-solid h-10 w-10 fill-current text-redColor dark:text-secondaryColor",
                xmlns: "{card.svg.xmlns}",
                view_box: "{card.svg.view_box}",
                path { d: "{card.svg.path.d}" }
            }
            br {}
            "{card.title}"
        }
    }
}
