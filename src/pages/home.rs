use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Home)]
pub fn secure() -> Html {
    let navigator: Navigator = use_navigator().unwrap();
    let onclick: Callback<MouseEvent> = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div style="color: red;">
            <h1>{ "Home" }</h1>
            <button {onclick}>{ "We are home Home" }</button>
        </div>
    }
}
