use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Home)]
pub fn secure() -> Html {
    let navigator: Navigator = use_navigator().unwrap();
    let onclick: Callback<MouseEvent> = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div style="">
            <h1 class="bg-blue-500 text-white p-4 font-bold text-red-600 text-4xl text-center">{ "Home" }</h1>
            <button class="text-center w-full" {onclick}>{ "We are in home Home" }</button>
        </div>
    }
}
