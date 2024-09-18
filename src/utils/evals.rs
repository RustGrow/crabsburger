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
            if (localStorage.getItem("mode") == "dark") {
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
    rsx! {}
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
    rsx! {}
}

#[component]
pub fn ScrollButtonVisible(mut visible: Signal<String>) -> Element {
    // let data = use_context::<ApplicationData>();
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
                visible.set("hidden".to_string());
            } else {
                visible.set("visible".to_string());
            }
        }
    });
    rsx! {}
}

#[component]
pub fn NavBarToggle() -> Element {
    // let data = use_context::<ApplicationData>();
    // let dark_theme = *data.dark.read();
    let _ = use_resource(move || async move {
        let eval = eval(
            r#"
               let dark = await dioxus.recv();
               if (dark == false) {
                html.classList.remove("dark");
               localStorage.setItem("mode", "light");
               } else {
               html.classList.add("dark");
               localStorage.setItem("mode", "dark");                                        
               } 
                "#,
        );
        eval.send(true.into()).unwrap();
    });
    rsx! {}
}
