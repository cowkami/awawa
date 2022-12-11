use super::menu::{NavMenu, NavMenuButton};
use yew::{function_component, html, Html};

#[function_component]
pub fn Navbar() -> Html {
    html! {
        <nav class="navbar navbar-dark bg-dark">
            <div class="container-fluid d-flex flex-row">
                <div class="d-flex mx-auto">
                    <a class="navbar-brand h1">{ "awawa" }</a>
                </div>
                <div class="d-flex">
                    <NavMenuButton />
                </div>
            </div>
            <NavMenu />
        </nav>
    }
}
