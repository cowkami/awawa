use yew::{function_component, html, Html};

use super::Search;

#[function_component]
pub fn GameConsole() -> Html {
    html! {
        <div class="container rounded bg-main p-2">
            <Search />
        </div>
    }
}
