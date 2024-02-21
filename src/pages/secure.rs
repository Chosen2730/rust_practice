use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Secure)]
pub fn secure() -> Html {
    let navigator: Navigator = use_navigator().unwrap();
    let onclick: Callback<MouseEvent> = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div style="text-red-500">
            <h1 class="bg-red-500  text-blue-500">{ "Secures" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}
