use dioxus::prelude::*;

pub fn NavBar() -> Element {
    let menu = vec!["Home", "About", "Menu", "Review", "Contact"];
    let mut menu_hidden = use_signal(|| "hidden".to_string());
    let mut selected_menu = use_signal(|| 0);
    let mut dark_state = use_signal(|| false);

    //Dark state eval
    // add dark class to html tag for tailwind css theme
    use_effect(move || {
        spawn({
            to_owned![dark_state];
            let dark = if dark_state() { "dark" } else { " " };
            async move {
                web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .document_element()
                    .unwrap()
                    .set_attribute("class", &(format!("{dark}")))
                    .expect("error")
            }
        });
    });

    // eval for hidden and visible border for header when scroll
    // let eval_border = use_eval(cx);
    let mut header_border_visible = use_signal(|| "");
    let _ = use_resource(move || async move {
        let mut eval = eval(
            r#"
                let header_border = "";
                window.addEventListener('scroll', () => {
                  if (window.pageYOffset < 50 ) {
                    header_border = "hidden";
                  } else {
                    header_border = "visible";
                  }
                  dioxus.send(header_border);
                });
                "#,
        );

        while let Ok(res) = eval.recv().await {
            if res == "hidden" {
                header_border_visible.set("");
            } else {
                header_border_visible.set("border-b border-secondaryColor");
            }
        }
    });

    rsx!(
        // Header ----------------------------------
        header { class: "bg-primaryColor dark:bg-darkColor fixed top-0 left-0 w-full z-50 {header_border_visible}",
            nav { class: "container relative h-14 flex justify-between items-center",
                div {
                    a { href: "#", class: "text-2xl uppercase font-oswald",
                        "Crabs"
                        span { class: "text-2xl uppercase text-secondaryColor", "Burger" }
                    }
                }

                div { class: "{menu_hidden} absolute top-0 left-0 w-full py-14 bg-primaryColor dark:bg-darkColor border-b border-secondaryColor md:block md:static md:py-0 md:border-none md:w-auto md:ml-auto",
                    ul { class: "flex flex-col text-center gap-5 md:flex-row",
                        { menu.iter().enumerate().map(|(id, _)| {
                        let selected = selected_menu == id;

                        let bg_selected = match selected {
                            true => "text-secondaryColor ease-in duration-200",
                            false => "hover:text-secondaryColor ease-in duration-200",
                        };

                        rsx! {
                            li {
                            onclick: move |_| {
                                selected_menu.set(id);
                                // hidden open menu from mobile
                                menu_hidden.set("hidden".to_string())},
                            a {
                                class: "{bg_selected}",
                                href: "#{menu[id].to_lowercase()}",
                                "{menu[id]}"
                            }

                        }
                        }})}
                    }
                    div {
                        class: "absolute top-[0.7rem] right-4 cursor-pointer md:hidden",
                        onclick: move |_| { menu_hidden.set("hidden".to_string()) },
                        svg {
                            class: " fill-current text-white",
                            xmlns: "http://www.w3.org/2000/svg",
                            view_box: "0 0 24 24",
                            path { d: "m12 10.586 4.95-4.95 1.415 1.415-4.95 4.95 4.95 4.95-1.415 1.414-4.95-4.95-4.95 4.95-1.413-1.415 4.95-4.95-4.95-4.95L7.05 5.638l4.95 4.95Z" }
                        }
                    }
                }
                div { class: "flex flex-row items-center gap-5",
                    div {
                        onclick: move |_| {
                            dark_state.set(!dark_state());
                        },

                        {if dark_state == true {

                            rsx!{
                                svg {
                                    class: "cursor-pointer ml-4 h-6 w-6 fill-current text-white",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    view_box: "0 -960 960 960",
                                    // sun icon
                                    path {d: "M480-360q50 0 85-35t35-85q0-50-35-85t-85-35q-50 0-85 35t-35 85q0 50 35 85t85 35Zm0 80q-83 0-141.5-58.5T280-480q0-83 58.5-141.5T480-680q83 0 141.5 58.5T680-480q0 83-58.5 141.5T480-280ZM200-440H40v-80h160v80Zm720 0H760v-80h160v80ZM440-760v-160h80v160h-80Zm0 720v-160h80v160h-80ZM256-650l-101-97 57-59 96 100-52 56Zm492 496-97-101 53-55 101 97-57 59Zm-98-550 97-101 59 57-100 96-56-52ZM154-212l101-97 55 53-97 101-59-57Zm326-268Z"}}
                            }
                        } else {

                            rsx!{
                                svg {
                                class: "cursor-pointer ml-4 h-6 w-6 fill-current text-white",
                                xmlns: "http://www.w3.org/2000/svg",
                                view_box: "0 0 24 24",
                                // moon icon
                                path { d: "M10 7a7 7 0 0 0 12 4.9v.1c0 5.523-4.477 10-10 10S2 17.523 2 12 6.477 2 12 2h.1A6.98 6.98 0 0 0 10 7Zm-6 5a8 8 0 0 0 15.062 3.762A9 9 0 0 1 8.238 4.938 7.999 7.999 0 0 0 4 12Z" }
                                }
                            }
                        }}
                    }
                    div { onclick: move |_| { menu_hidden.set("".to_string()) },
                        svg {
                            class: "cursor-pointer ml-4 h-6 w-6 fill-current text-white md:hidden",
                            xmlns: "http://www.w3.org/2000/svg",
                            view_box: "0 0 24 24",
                            // hamburger icon
                            path { d: "M3 4h18v2H3V4Zm0 7h12v2H3v-2Zm0 7h18v2H3v-2Z" }
                        }
                    }
                }
            }
        }
    )
}
