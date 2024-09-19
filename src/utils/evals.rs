use crate::model::app_state::ApplicationData;
use dioxus::prelude::*;

// Get colour from local storage and set to html when page loads or reloads
#[component]
pub fn InitThemeColorState() -> Element {
    let mut data = use_context::<ApplicationData>();
    let _ = use_resource(move || async move {
        let mut theme_state = eval(
            r#"            
            const html = document.querySelector("html");

            // Check the theme in localStorage
            let mode = localStorage.getItem("mode");

            // If the theme is not set, get it from system or browser settings
            if (!mode) {
                mode = window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
            }

            // Apply the theme
            if (mode === "dark") {
                darkMode();
                dioxus.send(true);
            } else {
                lightMode();
                dioxus.send(false);
            }

            function darkMode() {
                html.classList.add("dark");  
                localStorage.setItem("mode", "dark");
            }

            function lightMode() {
                html.classList.remove("dark");  
                localStorage.setItem("mode", "light");
            }
            
            "#,
        );
        if theme_state.recv().await.unwrap() == true {
            data.dark.set(true);
        } else {
            data.dark.set(false);
        }
    });
    rsx! {  }
}

// Change the style of the navigation menu when scrolling
#[component]
pub fn toggle_navbar_style_on_scroll(mut navbar_style: Signal<bool>) -> Element {
    let _ = use_resource(move || async move {
        let mut eval = eval(
            r#"
                let header_border = "";
                window.addEventListener('scroll', () => {
                  if (window.pageYOffset < 50 ) {
                    header_border = false;
                  } else {
                    header_border = true;
                  }
                  dioxus.send(header_border);
                });
                "#,
        );

        while let Ok(res) = eval.recv().await {
            if res == false {
                navbar_style.set(false);
            } else {
                navbar_style.set(true);
            }
        }
    });
    rsx! {  }
}

#[component]
pub fn ScrollButtonVisible() -> Element {
    let mut data = use_context::<ApplicationData>();
    let _ = use_resource(move || async move {
        let mut eval = eval(
            r#"
                let button = false;
                window.addEventListener('scroll', () => {
                  if (window.pageYOffset < 600 ) {
                    // hidden button
                    button = false;
                  } else {
                    // show button
                    button = true;
                  }
                  dioxus.send(button);
                });
                "#,
        );

        while let Ok(show) = eval.recv().await {
            if show == true {
                data.scroll_button_visibility.set(true);
            } else {
                data.scroll_button_visibility.set(false);
            }
        }
    });
    rsx! {  }
}

pub fn NavBarToggle() {
    let eval = eval(
        r#"
        var htmlElement = document.documentElement;
        let dark = localStorage.getItem("mode");
        htmlElement.classList.toggle('dark');
        if (dark === "dark") { 
        localStorage.setItem("mode", "light");
        } else {
        localStorage.setItem("mode", "dark");                                        
        } 
        "#,
    );
    eval.send(true.into()).unwrap();
}
