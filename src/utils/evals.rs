use crate::model::app_state::ApplicationData;
use dioxus::prelude::*;

// Get colour from local storage and set to html when page loads or reloads
pub fn InitThemeColorState() {
    let mut data = use_context::<ApplicationData>();
    let _ = use_resource(move || async move {
        let mut theme_state = eval(
            r#"
            const html = document.querySelector("html");
            if (localStorage.getItem("mode") == "dark") {
                darkMode();
                dioxus.send("dark");
            } else {
                lightMode();
                dioxus.send("light");
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
        if theme_state.recv().await.unwrap() == "dark" {
            data.theme_state.set("dark".to_string());
        } else {
            data.theme_state.set("light".to_string());
        }
    });
}

// Change the style of the navigation menu when scrolling
pub fn toggle_navbar_style_on_scroll(mut navbar_style: Signal<String>) {
    let _ = use_resource(move || async move {
        let mut eval = eval(
            r#"
                let header_border = "";
                window.addEventListener('scroll', () => {
                  if (window.pageYOffset < 50 ) {
                    header_border = "hidden";
                  } else {
                    header_border = "";
                  }
                  dioxus.send(header_border);
                });
                "#,
        );

        while let Ok(res) = eval.recv().await {
            if res == "hidden" {
                navbar_style.set("".to_string());
            } else {
                navbar_style.set("border-b border-secondaryColor card-shadow".to_string());
            }
        }
    });
}

pub fn ScrollButtonVisible(mut visible: Signal<String>) {
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
}

pub fn NavBarToggle(theme: Signal<String>) {
    let _ = use_resource(move || async move {
        let eval = eval(
            r#"
               let color = await dioxus.recv();
               if (color == "light") {
                html.classList.remove("dark");
               localStorage.setItem("mode", color);
               } else {
               html.classList.add("dark");
               localStorage.setItem("mode", color);                                        
               } 
                "#,
        );
        eval.send((theme)().into()).unwrap();
    });
}
