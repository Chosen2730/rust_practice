use crate::pages::about::About;
use crate::pages::home::Home;
use crate::pages::secure::Secure;
use yew::prelude::*;
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

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Secure => html! {
            <Secure />
        },
        Route::About { id } => html! {<About id={id} />},
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
