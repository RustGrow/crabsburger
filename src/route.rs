use crate::components::err::PageNotFound;
use crate::components::home_page::home::Home;
use crate::components::nav_bar::NavBar;
use dioxus::prelude::*;

#[derive(Routable, Clone, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {        
    #[layout(NavBar)]        
        #[route("/")]
        Home {},
    #[end_layout]
    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}
