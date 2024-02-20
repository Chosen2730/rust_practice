mod pages {
    pub mod about;
    pub mod home;
    pub mod secure;
}

mod app_router {
    pub mod route;
}

use app_router::route::Route;

use pages::about::About;
use pages::home::Home;
use pages::secure::Secure;
use yew::prelude::*;
use yew_router::prelude::*;

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Secure => html! {
            <Secure />
        },
        Route::About { id } => html! {<About id={id} />},
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
