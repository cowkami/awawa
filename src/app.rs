use crate::components::Navbar;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <Navbar />
    }
}
