use yew::prelude::*;

use super::components::Navbar;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <Navbar />
    }
}
