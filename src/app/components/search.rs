use yew::{function_component, html, Html};
use yew_icons::{Icon, IconId};

#[function_component]
pub fn Search() -> Html {
    html! {
        <div class="container-fluid d-flex justify-content-between p-2">
            <div class="d-col p-1">
                <Icon icon_id={IconId::BootstrapSearch}
                    class="text-second"
                />
            </div>
            <div class="container-fluid d-col">
                <input
                    type="text"
                    class="form-control"
                    placeholder="Search"
                    aria-describedby="basic-addon2"
                />
            </div>
        </div>
    }
}
