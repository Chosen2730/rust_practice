use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/secure")]
    Secure,
    #[at("/about/:id")]
    About { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}
