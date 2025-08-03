use dioxus::prelude::*;

const HERO_CSS: Asset = asset!("/assets/styling/hero.css");

#[component]
pub fn Hero() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: HERO_CSS }
        section { class: "hero",
            div { class: "hero-content",
                h1 { class: "hero-title", "Welcome to {{project-name}}" }
                p { class: "hero-subtitle", 
                    "A fullstack Dioxus application with Tauri desktop support" 
                }
                div { class: "hero-features",
                    div { class: "feature",
                        h3 { "🚀 Fullstack" }
                        p { "Server functions with client-side rendering" }
                    }
                    div { class: "feature",
                        h3 { "🦀 Rust Everywhere" }
                        p { "One language for all platforms" }
                    }
                    div { class: "feature",
                        h3 { "⚡ Fast & Native" }
                        p { "Native performance with Tauri" }
                    }
                }
            }
        }
    }
}