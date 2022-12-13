use yew::{function_component, html, Html};

use super::menu::{NavMenu, NavMenuButton};

#[function_component]
pub fn Navbar() -> Html {
    html! {
        <nav class="navbar fixed-top navbar-dark bg-dark">
            <div class="container-fluid d-flex flex-row">
                <a class="navbar-brand h1 mx-auto">{ "awawa" }</a>
                <NavMenuButton />
            </div>
            <NavMenu />
        </nav>
    }
}
