use yew::{function_component, html, Html};

#[function_component]
pub fn NavMenuButton() -> Html {
    html! {
        <button
            class="navbar-toggler"
            type="button"
            data-bs-toggle="collapse"
            data-bs-target="#navbarNav"
            aria-controls="navbarNav"
            aria-expanded="false"
            aria-label="Toggle navigation"
        >
            <span class="navbar-toggler-icon"></span>
        </button>
    }
}

#[derive(Clone)]
struct MenuItem {
    text: String,
    href: String,
}

impl MenuItem {
    fn to_html(self) -> Html {
        html! {
            <li class="nav-item p-2">
                <a class="nav-link" href={ self.href }>{ self.text }</a>
            </li>
        }
    }
}

#[function_component]
pub fn NavMenu() -> Html {
    let menu_items = vec![
        MenuItem {
            text: "Home".to_string(),
            href: "#".to_string(),
        },
        MenuItem {
            text: "Reset".to_string(),
            href: "#".to_string(),
        },
    ]
    .iter()
    .map(|item| item.clone().to_html())
    .collect::<Html>();

    html! {
        <div class="collapse navbar-collapse" id="navbarNav">
            <ul class="navbar-nav">
              { menu_items }
            </ul>
        </div>
    }
}
