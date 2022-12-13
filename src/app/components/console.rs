use yew::{function_component, html, Html};

#[function_component]
pub fn GameConsole() -> Html {
    html! {
        <div class="container bg-main">
            <div class="container bg-dark">
              { "display" }
            </div>
            <div class="container bg-dark">
              { "description" }
            </div>
        </div>
    }
}
