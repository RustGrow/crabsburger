use crate::components::err_page::Err404;
use crate::components::home_page::home::Home;
use crate::components::nav_bar::NavBar;
use dioxus::prelude::*;

#[derive(Routable, Clone, Debug, PartialEq)]
#[rustfmt::skip]
// pub enum Route {    
//     #[layout(NavBar)]        
//         #[route("/")]
//         Home {},
//     #[end_layout]
//     #[route("/:..route")]
//     Err404 {
//         route: Vec<String>,
//     },
// }
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/:..route")]
    Err404 {
        route: Vec<String>,
    },
}
