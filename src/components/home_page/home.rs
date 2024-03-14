#![allow(non_snake_case)]
use super::category_card_ui::CategoryCard;
use super::food_card_ui::FoodCard;
use super::home_card_icon_ui::HomeCardIcon;
use super::promo_card_ui::PromoCard;
use super::reviewers_card_ui::ReviewersCard;
// use crate::components::nav_bar::NavBar;
use crate::model::app_state::ApplicationData;
use crate::repository::category_card_repo::CATEGORY_CARDS;
use crate::repository::food_card_repo::{BEVERAGE, BURGERS, SNACKS};
use crate::repository::home_card_icon_repo::HOME_CARD_ICONS;
use crate::repository::promo_card_repo::PROMO_CARDS;
use crate::repository::review_card_repo::REVIEWERS;
use chrono::Datelike;
use dioxus::prelude::*;

pub fn Home() -> Element {
    let mut data = use_context::<ApplicationData>();
    let tabs = vec!["All", "Food", "Snack", "Beverage"];
    let mut selected_snippet = use_signal(|| 0);

    // eval for hidden and visible button when scroll
    let mut button_visible = use_signal(|| "hidden");
    let _ = use_resource(move || async move {
        // Don't using tokio
        // tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        let mut eval = eval(
            r#"
                let button = "";
                window.addEventListener('scroll', () => {
                  if (window.pageYOffset < 600 ) {
                    button = "hidden";
                  } else {
                    button = "visible";
                  }
                  dioxus.send(button);
                });
                "#,
        );

        while let Ok(res) = eval.recv().await {
            if res == "hidden" {
                button_visible.set("hidden");
            } else {
                button_visible.set("visible");
            }
        }
    });

    // add dark mode to localstorage
    let local_storage = use_signal(|| "light");
    // use_future(cx, (), |_| {
    //     to_owned![local_storage, dark_state];
    //     async move {
    //         if *dark_state.get() {
    //             local_storage(
    //                 r#"
    //              localStorage.setItem("mode", "dark");
    //             "#,
    //             )
    //             .unwrap();
    //         } else {
    //             local_storage(
    //                 r#"
    //              localStorage.setItem("mode", "light");
    //             "#,
    //             )
    //             .unwrap();
    //         }
    //     }
    // });
    ////////////////////////////////////////////

    rsx! {
        // NavBar {}
        main {
            // Home ----------------------------------------------
            section { id: "home",
                div { class: "container flex flex-col items-center gap-10 md:flex-row",
                    div { class: "mx-auto md:basis-1/2 lg:basis-2/5 animate-movingY", img {
                        class: "w-60 md:w-full",
                        src: "images/home-image.png",
                        alt: "home image"
                    } }
                    div { class: "text-center md:basis-1/2 md:text-start lg:basis-3/5",
                        h1 { class: "home__title", "HAPPY TUMMY WITH TASTY CRABSBURGER." }
                        div { class: "separator mx-auto md:mx-0" }
                        p { class: "paragraph",
                            "The ultimate destination for burger fans who want to indulge in mouth-watering and satisfying burgers. We use only fresh and quality ingredients to make our burgers, and we offer a variety of options to suit your taste. Come and visit us today, or order online and get a free drink. You will love our burgers."
                        }

                        // Home card icons
                        div { class: "text-base flex items-center justify-center gap-4 py-10 md:justify-start md:gap-20",
                            {
                                HOME_CARD_ICONS.iter().enumerate().map(|(_, card)| {
                                    rsx!{
                                        HomeCardIcon  {
                                            card: *card
                                        }
                                    }
                                })
                            }
                        }
                        a { class: "btn btn-primary", href: "#", "lern more" }
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
            section { id: "about",
                div { class: "container flex flex-col gap-10 md:flex-row",
                    div { class: "flex-1", img {
                        class: "rounded-lg",
                        src: "images/about.jpg",
                        alt: "about image"
                    } }
                    div { class: "flex-1",
                        h2 { class: "section__title",
                            "FIND FOOD AND DRINKS, ALL-IN-ONE PLACE FOR YOUR BEST TASTE."
                        }
                        div { class: "separator" }
                        p { class: "paragraph",
                            "Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Aenean commodo ligula eget dolor. Aenean massa. Cum sociis natoque penatibus et magnis dis parturient montes."
                        }
                        ul { class: "grid grid-cols-2 py-5 space-y-1",
                            li { class: "flex flex-row items-center gap-1 text-xs text-paragraphColor",
                                svg {
                                    class: "h-4 w-4 fill-current text-secondaryColor",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    view_box: "0 0 448 512",
                                    path { d: "M438.6 105.4c12.5 12.5 12.5 32.8 0 45.3l-256 256c-12.5 12.5-32.8 12.5-45.3 0l-128-128c-12.5-12.5-12.5-32.8 0-45.3s32.8-12.5 45.3 0L160 338.7 393.4 105.4c12.5-12.5 32.8-12.5 45.3 0z" }
                                }
                                p { "Best Price" }
                            }
                            li { class: "flex flex-row items-center gap-1 text-xs text-paragraphColor",
                                svg {
                                    class: "h-4 w-4 fill-current text-secondaryColor",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    view_box: "0 0 448 512",
                                    path { d: "M438.6 105.4c12.5 12.5 12.5 32.8 0 45.3l-256 256c-12.5 12.5-32.8 12.5-45.3 0l-128-128c-12.5-12.5-12.5-32.8 0-45.3s32.8-12.5 45.3 0L160 338.7 393.4 105.4c12.5-12.5 32.8-12.5 45.3 0z" }
                                }
                                "Fresh Ingredient"
                            }
                            li { class: "flex flex-row items-center gap-1 text-xs text-paragraphColor",
                                svg {
                                    class: "h-4 w-4 fill-current text-secondaryColor",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    view_box: "0 0 448 512",
                                    path { d: "M438.6 105.4c12.5 12.5 12.5 32.8 0 45.3l-256 256c-12.5 12.5-32.8 12.5-45.3 0l-128-128c-12.5-12.5-12.5-32.8 0-45.3s32.8-12.5 45.3 0L160 338.7 393.4 105.4c12.5-12.5 32.8-12.5 45.3 0z" }
                                }
                                "Best Service"
                            }
                            li { class: "flex flex-row items-center gap-1 text-xs text-paragraphColor",
                                svg {
                                    class: "h-4 w-4 fill-current text-secondaryColor",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    view_box: "0 0 448 512",
                                    path { d: "M438.6 105.4c12.5 12.5 12.5 32.8 0 45.3l-256 256c-12.5 12.5-32.8 12.5-45.3 0l-128-128c-12.5-12.5-12.5-32.8 0-45.3s32.8-12.5 45.3 0L160 338.7 393.4 105.4c12.5-12.5 32.8-12.5 45.3 0z" }
                                }
                                "Health Protocol"
                            }
                        }
                        a { class: "btn btn-primary", href: "", "About us" }
                    }
                }
            }
            // Menu
            section { id: "menu",
                div { class: "container",
                    div { class: "max-w-md mx-auto text-center",
                        h2 { class: "section__title", "OUR BEST MENU" }
                        div { class: "separator mx-auto" }
                        p { class: "paragraph",
                            "Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Aenean commodo ligula eget dolor. Aenean massa."
                        }
                        div { class: "tabs_wrap",
                            ul { class: "flex flex-wrap justify-center gap-3 py-10",
                                { tabs.iter().enumerate().map(|(id, _)| {
                                    let selected = selected_snippet == id;

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
                    div { class: "menu__items",
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
                id: "review",
                // Review
                class: "bg-primaryColorLight dark:bg-darkColorLight py-20",
                div { class: "container",
                    div { class: "max-w-md mx-auto text-center",
                        h2 { class: "section__title", "CUSTOMER REVIEW" }
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
            section { class: "bg-secondaryColor py-16", id: "contact",
                div { class: "container flex flex-col gap-5 md:items-center md:flex-row",
                    div { class: "space-y-4 md:flex-1",
                        h2 { class: "section__title text-blackColor", "GET EXCLUSIVE UPDATE" }
                        p { class: "text-sm", "Lorem ipsum dolor sit amet, consectetur adipiscing elit." }
                    }
                    div { class: "flex flex-col gap-3 md:flex-row md:flex-1",
                        input {
                            class: "p-2 text-blackColor rounded-lg outline-none md:w-full",
                            r#type: "text",
                            placeholder: "Email address"
                        }
                        a {
                            class: "flex items-center justify-center gap-2 btn bg-blackColor hover:opacity-75",
                            href: "",
                            svg {
                                class: "fill-current text-white",
                                xmlns: "http://www.w3.org/2000/svg",
                                height: "16",
                                width: "16",
                                view_box: "0 0 512 512",
                                path { d: "M16.1 260.2c-22.6 12.9-20.5 47.3 3.6 57.3L160 376V479.3c0 18.1 14.6 32.7 32.7 32.7c9.7 0 18.9-4.3 25.1-11.8l62-74.3 123.9 51.6c18.9 7.9 40.8-4.5 43.9-24.7l64-416c1.9-12.1-3.4-24.3-13.5-31.2s-23.3-7.5-34-1.4l-448 256zm52.1 25.5L409.7 90.6 190.1 336l1.2 1L68.2 285.7zM403.3 425.4L236.7 355.9 450.8 116.6 403.3 425.4z" }
                            }
                            "Subscribe"
                        }
                    }
                }
            }
        }
        footer {
            section { class: "footer",
                div { class: "container",
                    ul { class: "grid grid-cols-1 items-start gap-5 pb-5 md:grid-cols-2 lg:grid-cols-4",
                        li {
                            div { class: "space-y-3",
                                a { class: "text-4xl font-oswald uppercase", href: "",
                                    "Crabs"
                                    span { class: "text-secondaryColor", "Burger" }
                                }
                                p { class: "text-sm",
                                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Ut elit tellus, luctus nec ullamcorper mattis, pulvinar dapibus leo."
                                }
                            }
                        }
                        li {
                            div { class: "flex flex-col gap-3",
                                h3 { class: "text-lg uppercase font-oswald", "SUPPORT" }
                                a {
                                    class: "text-xs hover:text-secondaryColor",
                                    href: "",
                                    "FAQ's"
                                }
                                a {
                                    class: "text-xs hover:text-secondaryColor",
                                    href: "",
                                    "Privacy Policy"
                                }
                                a {
                                    class: "text-xs hover:text-secondaryColor",
                                    href: "",
                                    "Term & Condition"
                                }
                                a {
                                    class: "text-xs hover:text-secondaryColor",
                                    href: "",
                                    "Contact"
                                }
                            }
                        }
                        li { class: "space-y-8",
                            div { class: "space-y-2",
                                h3 { class: "text-lg uppercase font-oswald", "phone" }
                                p { class: "flex items-center gap-2 text-xs",
                                    svg {
                                        class: "fill-current h-5 w-5 text-secondaryColor",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        view_box: "0 0 512 512",
                                        path { d: "M164.9 24.6c-7.7-18.6-28-28.5-47.4-23.2l-88 24C12.1 30.2 0 46 0 64C0 311.4 200.6 512 448 512c18 0 33.8-12.1 38.6-29.5l24-88c5.3-19.4-4.6-39.7-23.2-47.4l-96-40c-16.3-6.8-35.2-2.1-46.3 11.6L304.7 368C234.3 334.7 177.3 277.7 144 207.3L193.3 167c13.7-11.2 18.4-30 11.6-46.3l-40-96z" }
                                    }
                                    "+1 000 0000000"
                                }
                            }
                            div { class: "space-y-2",
                                h3 { class: "text-lg uppercase font-oswald", "email" }
                                p { class: "flex items-center gap-2 text-xs",
                                    svg {
                                        class: "fill-current h-5 w-5 text-secondaryColor",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        view_box: "0 0 512 512",
                                        path { d: "M48 64C21.5 64 0 85.5 0 112c0 15.1 7.1 29.3 19.2 38.4L236.8 313.6c11.4 8.5 27 8.5 38.4 0L492.8 150.4c12.1-9.1 19.2-23.3 19.2-38.4c0-26.5-21.5-48-48-48H48zM0 176V384c0 35.3 28.7 64 64 64H448c35.3 0 64-28.7 64-64V176L294.4 339.2c-22.8 17.1-54 17.1-76.8 0L0 176z" }
                                    }
                                    "crab.info@email.com"
                                }
                            }
                        }
                        li { class: "space-y-8",
                            div { class: "space-y-2",
                                h3 { class: "text-lg uppercase font-oswald", "address" }
                                p { class: "flex items-center gap-2 text-xs",
                                    svg {
                                        class: "fill-current h-5 w-5 cursor-pointer text-secondaryColor",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        view_box: "0 0 384 512",
                                        path { d: "M215.7 499.2C267 435 384 279.4 384 192C384 86 298 0 192 0S0 86 0 192c0 87.4 117 243 168.3 307.2c12.3 15.3 35.1 15.3 47.4 0zM192 128a64 64 0 1 1 0 128 64 64 0 1 1 0-128z" }
                                    }
                                    "Address goes here"
                                }
                            }
                            div { class: "space-y-2",
                                h3 { class: "text-lg uppercase font-oswald", "follow us" }
                                div { class: "space-x-3 flex flex-row",
                                    svg {
                                        class: "fill-current h-5 w-5 cursor-pointer text-secondaryColor hover:-translate-y-1 ease-in duration-200",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        view_box: "0 0 320 512",
                                        path { d: "M80 299.3V512H196V299.3h86.5l18-97.8H196V166.9c0-51.7 20.3-71.5 72.7-71.5c16.3 0 29.4 .4 37 1.2V7.9C291.4 4 256.4 0 236.2 0C129.3 0 80 50.5 80 159.4v42.1H14v97.8H80z" }
                                    }
                                    svg {
                                        class: "fill-current h-5 w-5 cursor-pointer text-secondaryColor hover:-translate-y-1 ease-in duration-200",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        height: "16",
                                        width: "16",
                                        view_box: "0 0 512 512",
                                        path { d: "M389.2 48h70.6L305.6 224.2 487 464H345L233.7 318.6 106.5 464H35.8L200.7 275.5 26.8 48H172.4L272.9 180.9 389.2 48zM364.4 421.8h39.1L151.1 88h-42L364.4 421.8z" }
                                    }
                                    svg {
                                        class: "fill-current h-5 w-5 cursor-pointer text-secondaryColor hover:-translate-y-1 ease-in duration-200",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        height: "16",
                                        width: "14",
                                        view_box: "0 0 448 512",
                                        path { d: "M224.1 141c-63.6 0-114.9 51.3-114.9 114.9s51.3 114.9 114.9 114.9S339 319.5 339 255.9 287.7 141 224.1 141zm0 189.6c-41.1 0-74.7-33.5-74.7-74.7s33.5-74.7 74.7-74.7 74.7 33.5 74.7 74.7-33.6 74.7-74.7 74.7zm146.4-194.3c0 14.9-12 26.8-26.8 26.8-14.9 0-26.8-12-26.8-26.8s12-26.8 26.8-26.8 26.8 12 26.8 26.8zm76.1 27.2c-1.7-35.9-9.9-67.7-36.2-93.9-26.2-26.2-58-34.4-93.9-36.2-37-2.1-147.9-2.1-184.9 0-35.8 1.7-67.6 9.9-93.9 36.1s-34.4 58-36.2 93.9c-2.1 37-2.1 147.9 0 184.9 1.7 35.9 9.9 67.7 36.2 93.9s58 34.4 93.9 36.2c37 2.1 147.9 2.1 184.9 0 35.9-1.7 67.7-9.9 93.9-36.2 26.2-26.2 34.4-58 36.2-93.9 2.1-37 2.1-147.8 0-184.8zM398.8 388c-7.8 19.6-22.9 34.7-42.6 42.6-29.5 11.7-99.5 9-132.1 9s-102.7 2.6-132.1-9c-19.6-7.8-34.7-22.9-42.6-42.6-11.7-29.5-9-99.5-9-132.1s-2.6-102.7 9-132.1c7.8-19.6 22.9-34.7 42.6-42.6 29.5-11.7 99.5-9 132.1-9s102.7-2.6 132.1 9c19.6 7.8 34.7 22.9 42.6 42.6 11.7 29.5 9 99.5 9 132.1s2.7 102.7-9 132.1z" }
                                    }
                                }
                            }
                        }
                    }
                    div { class: "flex flex-col items-center border-t border-primaryColorLight dark:border-darkColorLight py-5 md:flex-row md:justify-between",
                        p { class: "paragraph",
                            "CrabsBurger Template Kit with ❤️ to "
                            a { href: "https://dioxuslabs.com/", alt: "Dioxus labs", "Dioxus" }
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
            class: "fixed {button_visible} right-4 bottom-4 h-11 w-11 bg-secondaryColor shadow-sm flex rounded-full text-lg text-blackColor z-50 hover:-translate-y-1 ease-in duration-200 items-center justify-center",
            onclick: move |_| { data.selected_menu.set(0) },
            href: "#",
            svg {
                class: "fill-current h-6 w-6 text-blackColor",
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 384 512",
                path { d: "M214.6 41.4c-12.5-12.5-32.8-12.5-45.3 0l-160 160c-12.5 12.5-12.5 32.8 0 45.3s32.8 12.5 45.3 0L160 141.2V448c0 17.7 14.3 32 32 32s32-14.3 32-32V141.2L329.4 246.6c12.5 12.5 32.8 12.5 45.3 0s12.5-32.8 0-45.3l-160-160z" }
            }
        }
    }
}

// fn change_local_storage(cx: &Scoped<'_>, state: &UseState<bool>) {
//     // add dark mode to localstorage
//     let local_storage = use_eval(cx);
//     use_future(cx, (), |_| {
//         to_owned![local_storage, state];
//         async move {
//             if *state.get() {
//                 local_storage(
//                     r#"
//                  localStorage.setItem("node", "dark");
//                 "#,
//                 )
//                 .unwrap();
//             } else {
//                 local_storage(
//                     r#"
//                  localStorage.setItem("mode", "light");
//                 "#,
//                 )
//                 .unwrap();
//             }
//         }
//     });
// }
