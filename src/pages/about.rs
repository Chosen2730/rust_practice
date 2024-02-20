use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct AboutProps {
    pub id: String,
}

#[function_component(About)]
pub fn about(props: &AboutProps) -> Html {
    html! {
         <h1>{"About" } {&props.id}</h1>
    }
}
