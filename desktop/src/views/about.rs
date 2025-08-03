use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        div { class: "about-page",
            h1 { class: "text-4xl font-bold mb-6", "About {{project-name}}" }
            div { class: "prose lg:prose-xl dark:prose-invert mx-auto",
                p { "{{description}}" }
                
                h2 { class: "mt-8", "Features" }
                ul {
                    li { "Fullstack Dioxus with server functions" }
                    li { "Tauri for native desktop performance" }
                    li { "Tailwind CSS for styling" }
                    li { "Shared UI components across platforms" }
                    li { "Hot-reloading development experience" }
                }
                
                h2 { class: "mt-8", "Architecture" }
                p { 
                    "This application uses a monorepo structure with shared components "
                    "and platform-specific implementations."
                }
            }
        }
    }
}