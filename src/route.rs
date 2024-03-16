use crate::components::err_page::PageNotFound;
use crate::components::home_page::home::Home;
use crate::components::nav_bar::NavBar;
use dioxus::prelude::*;

#[derive(Routable, Clone, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {    
    // #[nest("/crabsburger")]
    #[layout(NavBar)]        
        #[route("/")]
        Home {},
    #[end_layout]
    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}
// #[derive(Routable, Clone, Debug, PartialEq)]
// #[rustfmt::skip]
// pub enum Route {
//     #[layout(NavBar)]
//         #[route("/")]
//         Home {},
//     #[end_layout]
//     #[route("/:..route")]
//     PageNotFound {
//         route: Vec<String>,
//     },
// }

// #[derive(Routable, Clone, Debug, PartialEq)]
// #[rustfmt::skip]
// enum Route {
//     // Wrap Home in a Navbar Layout
//     #[layout(NavBar)]
//         // The default route is always "/" unless otherwise specified
//         #[route("/")]
//         Home {},
//     #[end_layout]
//     // Finally, we need to handle the 404 page
//     #[route("/:..route")]
//     PageNotFound {
//         route: Vec<String>,
//     },
// }
