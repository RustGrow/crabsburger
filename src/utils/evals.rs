use crate::document::*;
use crate::model::app_state::ApplicationData;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;

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
        if theme_state.recv::<bool>().await.unwrap() == true {
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

        while let Ok(res) = eval.recv::<bool>().await {
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

        while let Ok(show) = eval.recv::<bool>().await {
            if show == true {
                data.scroll_button_visibility.set(true);
            } else {
                data.scroll_button_visibility.set(false);
            }
        }
    });
    rsx! {}
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
    eval.send(true).unwrap();
}

#[component]
pub fn LangSettings() -> Element {
    let mut data = use_context::<ApplicationData>();
    let _ = use_resource(move || async move {
        let mut eval = eval(
            r#"
            // Function to set lang and dir attributes for the <html> tag
            function setHtmlLanguageAndDirection(lang, dir) {
                var htmlElement = document.documentElement;
                htmlElement.lang = lang;
                if (dir) {
                    htmlElement.dir = dir;
                } else {
                    htmlElement.removeAttribute('dir');
                }
            }

            let lang = localStorage.getItem("lang");
            let browserLang = navigator.language || navigator.userLanguage;
            let langCode = browserLang.substring(0, 2);

            const supportedLangs = {
                "en": true,
                "de": true,
                "es": true,
                "ar": true
            };

            // The code checks if the browser language is in the list of supported languages,
            // and if not, sets English as the default language.
            if (!supportedLangs[langCode]) {
                langCode = "en";
            }

            if (langCode === "zh") {
                if (browserLang === "zh-CN" || browserLang === "zh-SG") {
                    langCode = "zh-Hans"; // Simplified Chinese
                } else if (browserLang === "zh-TW" || browserLang === "zh-HK" || browserLang === "zh-MO") {
                    langCode = "zh-Hant"; // Traditional Chinese
                }
            }

            // If there is no language value in local storage, set it
            if (!lang) {
                localStorage.setItem("lang", langCode);
                lang = langCode;
            }

            // Set language and direction in HTML
            if (lang === "ar" || lang === "he") {
                setHtmlLanguageAndDirection(lang, 'rtl');
            } else {
                setHtmlLanguageAndDirection(lang, null);
            }

            dioxus.send(lang);

            // How to get arr from JS to Dioxus
            // let arr = [lang, langCode];
            // dioxus.send(arr);

            "#,
        );
        let js_lang: String = eval.recv().await.unwrap();
        *(data.lang_code).write() = js_lang;

        // Working with array from JS
        // if js_lang[0] == Value::Null {
        //     // Get lang from browser lang
        //     // *lang.write() = String::from(js_lang[1].as_str().unwrap());
        //     // *lang.write() = String::from("de");
        // } else {
        //     // Get lang from browser storage
        //     *lang.write() = String::from(js_lang[0].as_str().unwrap());
        // }
    });
    info!("Lang is {}", (data.lang_code)());
    rsx! {}
}

pub fn ButtonLang() -> Eval {
    eval(
        r#"
        // Function to set lang and dir attributes for the <html> tag
        function setHtmlLanguageAndDirection(lang, dir) {
            var htmlElement = document.documentElement;
            htmlElement.lang = lang;
            if (dir) {
                htmlElement.dir = dir;
            } else {
                htmlElement.removeAttribute('dir');
            }
        }

        let lang = await dioxus.recv();
        localStorage.setItem("lang", lang);

        // Set language and direction in HTML
        if (lang === "ar" || lang === "he") {
            setHtmlLanguageAndDirection(lang, 'rtl');
        } else {
            setHtmlLanguageAndDirection(lang, null);
        }

        "#,
    )
}
