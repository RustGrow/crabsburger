use crate::constants::LOCALES;
use crate::model::app_state::ApplicationData;
use crate::model::card::Food;
use dioxus::prelude::*;
use fluent_templates::Loader;
use std::str::FromStr;
use unic_langid::LanguageIdentifier;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Menu {
    Burger,
    Snack,
    Beverage,
    All,
}

#[component]
pub fn FoodCard(card: Menu) -> Element {
    let data = use_context::<ApplicationData>();
    let lang_id = &LanguageIdentifier::from_str(&(data.lang_code)() as &str).unwrap();
    let burgers = vec![
        Food {
            img_path: asset!("/assets/images/burger-1.png").input,
            alt: LOCALES.lookup(lang_id, "bur-Cheesy-alt"),
            title: LOCALES.lookup(lang_id, "bur-Cheesy-title"),
            description: LOCALES.lookup(lang_id, "bur-Cheesy-desc"),
            price: LOCALES.lookup(lang_id, "bur-Cheesy-price"),
        },
        Food {
            img_path: asset!("/assets/images/burger-2.png").input,
            alt: LOCALES.lookup(lang_id, "bur-Chicken-alt"),
            title: LOCALES.lookup(lang_id, "bur-Chicken-title"),
            description: LOCALES.lookup(lang_id, "bur-Chicken-desc"),
            price: LOCALES.lookup(lang_id, "bur-Chicken-price"),
        },
        Food {
            img_path: asset!("/assets/images/burger-3.png").input,
            alt: LOCALES.lookup(lang_id, "bur-Mushroom-alt"),
            title: LOCALES.lookup(lang_id, "bur-Mushroom-title"),
            description: LOCALES.lookup(lang_id, "bur-Mushroom-desc"),
            price: LOCALES.lookup(lang_id, "bur-Mushroom-price"),
        },
        Food {
            img_path: asset!("/assets/images/burger-4.png").input,
            alt: LOCALES.lookup(lang_id, "bur-Salmon-alt"),
            title: LOCALES.lookup(lang_id, "bur-Salmon-title"),
            description: LOCALES.lookup(lang_id, "bur-Salmon-desc"),
            price: LOCALES.lookup(lang_id, "bur-Salmon-price"),
        },
        Food {
            img_path: asset!("/assets/images/burger-5.png").input,
            alt: LOCALES.lookup(lang_id, "bur-Black-alt"),
            title: LOCALES.lookup(lang_id, "bur-Black-title"),
            description: LOCALES.lookup(lang_id, "bur-Black-desc"),
            price: LOCALES.lookup(lang_id, "bur-Black-price"),
        },
        Food {
            img_path: asset!("/assets/images/burger-6.png").input,
            alt: LOCALES.lookup(lang_id, "bur-Greek-alt"),
            title: LOCALES.lookup(lang_id, "bur-Greek-title"),
            description: LOCALES.lookup(lang_id, "bur-Greek-desc"),
            price: LOCALES.lookup(lang_id, "bur-Greek-price"),
        },
    ];
    let snacks = vec![
        Food {
            img_path: asset!("/assets/images/snack-popcorn.png").input,
            alt: LOCALES.lookup(lang_id, "sn-Popcorn-alt"),
            title: LOCALES.lookup(lang_id, "sn-Popcorn-title"),
            description: LOCALES.lookup(lang_id, "sn-Popcorn-desc"),
            price: LOCALES.lookup(lang_id, "sn-Popcorn-price"),
        },
        Food {
            img_path: asset!("/assets/images/snack-nachos.png").input,
            alt: LOCALES.lookup(lang_id, "sn-Nachos-alt"),
            title: LOCALES.lookup(lang_id, "sn-Nachos-title"),
            description: LOCALES.lookup(lang_id, "sn-Nachos-desc"),
            price: LOCALES.lookup(lang_id, "sn-Nachos-price"),
        },
        Food {
            img_path: asset!("/assets/images/snack-pretzel.png").input,
            alt: LOCALES.lookup(lang_id, "sn-Pretzels-alt"),
            title: LOCALES.lookup(lang_id, "sn-Pretzels-title"),
            description: LOCALES.lookup(lang_id, "sn-Pretzels-desc"),
            price: LOCALES.lookup(lang_id, "sn-Pretzels-price"),
        },
        Food {
            img_path: asset!("/assets/images/snack-mini-pizzas.png").input,
            alt: LOCALES.lookup(lang_id, "sn-Pizzas-alt"),
            title: LOCALES.lookup(lang_id, "sn-Pizzas-title"),
            description: LOCALES.lookup(lang_id, "sn-Pizzas-desc"),
            price: LOCALES.lookup(lang_id, "sn-Pizzas-price"),
        },
        Food {
            img_path: asset!("/assets/images/snack-trail-mix.png").input,
            alt: LOCALES.lookup(lang_id, "sn-Trail-alt"),
            title: LOCALES.lookup(lang_id, "sn-Trail-title"),
            description: LOCALES.lookup(lang_id, "sn-Trail-desc"),
            price: LOCALES.lookup(lang_id, "sn-Trail-price"),
        },
        Food {
            img_path: asset!("/assets/images/snack-chips.png").input,
            alt: LOCALES.lookup(lang_id, "sn-Chips-alt"),
            title: LOCALES.lookup(lang_id, "sn-Chips-title"),
            description: LOCALES.lookup(lang_id, "sn-Chips-desc"),
            price: LOCALES.lookup(lang_id, "sn-Chips-price"),
        },
    ];

    let beverage = vec![
        Food {
            img_path: asset!("/assets/images/beverage-orange-juice.png").input,
            alt: LOCALES.lookup(lang_id, "bev-Orange-alt"),
            title: LOCALES.lookup(lang_id, "bev-Orange-title"),
            description: LOCALES.lookup(lang_id, "bev-Orange-desc"),
            price: LOCALES.lookup(lang_id, "bev-Orange-price"),
        },
        Food {
            img_path: asset!("/assets/images/beverage-coffee.png").input,
            alt: LOCALES.lookup(lang_id, "bev-Coffee-alt"),
            title: LOCALES.lookup(lang_id, "bev-Coffee-title"),
            description: LOCALES.lookup(lang_id, "bev-Coffee-desc"),
            price: LOCALES.lookup(lang_id, "bev-Coffee-price"),
        },
        Food {
            img_path: asset!("/assets/images/beverage-beer.png").input,
            alt: LOCALES.lookup(lang_id, "bev-Beer-alt"),
            title: LOCALES.lookup(lang_id, "bev-Beer-title"),
            description: LOCALES.lookup(lang_id, "bev-Beer-desc"),
            price: LOCALES.lookup(lang_id, "bev-Beer-price"),
        },
        Food {
            img_path: asset!("/assets/images/beverage-milk.png").input,
            alt: LOCALES.lookup(lang_id, "bev-Milk-alt"),
            title: LOCALES.lookup(lang_id, "bev-Milk-title"),
            description: LOCALES.lookup(lang_id, "bev-Milk-desc"),
            price: LOCALES.lookup(lang_id, "bev-Milk-price"),
        },
        Food {
            img_path: asset!("/assets/images/beverage-hot-chocolate.png").input,
            alt: LOCALES.lookup(lang_id, "bev-Chocolate-alt"),
            title: LOCALES.lookup(lang_id, "bev-Chocolate-title"),
            description: LOCALES.lookup(lang_id, "bev-Chocolate-desc"),
            price: LOCALES.lookup(lang_id, "bev-Chocolate-price"),
        },
    ];
    match card {
        Menu::Burger => rsx! {
            FoodList { cards: burgers }
        },
        Menu::Snack => rsx! {
            FoodList { cards: snacks }
        },
        Menu::Beverage => rsx! {
            FoodList { cards: beverage }
        },
        Menu::All => rsx! {
            FoodList { cards: burgers }
            FoodList { cards: snacks }
            FoodList { cards: beverage }
        },
    }
}

#[component]
pub fn FoodList(cards: Vec<Food>) -> Element {
    rsx! {
        for card in cards.iter() {
            li { class: "item_wrap",
                div { class: "h-56 grid place-items-center bg-primaryColorLight dark:bg-darkColorLight rounded-3xl hover:bg-secondaryColor ease-linear duration-200 lg:h-40 card-shadow",
                    img {
                        class: "w-40 hover:scale-110 ease-linear duration-200 md:w-48 lg:w-24",
                        src: card.img_path,
                        alt: card.alt.clone()
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
}
