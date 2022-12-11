use super::menu::{NavMenu, NavMenuButton};
use yew::{function_component, html, Html};

#[function_component]
pub fn Navbar() -> Html {
    html! {
        <nav class="navbar navbar-dark bg-dark">
            <div classs="container-fluid">
              <a class="navbar-brand mb-0 h1">{ "awawa" }</a>
              <NavMenuButton />
              <NavMenu />
            </div>
        </nav>
    }
}
