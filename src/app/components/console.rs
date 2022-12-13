use yew::{function_component, html, Html};

use super::Search;

#[function_component]
pub fn GameConsole() -> Html {
    html! {
        <div class="container rounded bg-main p-2">
            // <div class="container bg-dark">
            <Search />
            // </div>
            <div class="container rounded bg-dark p-1">
              { "description" }
            </div>
        </div>
    }
}
