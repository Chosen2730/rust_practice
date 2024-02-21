mod components;
mod pages;

mod app_router {
    pub mod route;
}

use app_router::route::switch;
use app_router::route::Route;

use yew::prelude::*;
use yew_router::prelude::*;

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
