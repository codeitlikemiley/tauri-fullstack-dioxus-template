use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

#[component]
pub fn Navbar(children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }
        nav { class: "navbar",
            div { class: "navbar-container",
                h2 { class: "navbar-title", "{{project-name}}" }
                div { class: "navbar-links",
                    {children}
                }
            }
        }
    }
}