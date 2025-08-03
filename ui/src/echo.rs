use dioxus::prelude::*;

const ECHO_CSS: Asset = asset!("/assets/styling/echo.css");

/// Echo component that demonstrates fullstack server functions.
#[component]
pub fn Echo() -> Element {
    let mut response = use_signal(|| String::new());
    let mut input_value = use_signal(|| String::new());

    rsx! {
        document::Link { rel: "stylesheet", href: ECHO_CSS }
        div { class: "echo-container",
            h3 { "Server Function Demo" }
            div { class: "echo-input-group",
                input {
                    class: "echo-input",
                    placeholder: "Type here to echo...",
                    value: "{input_value}",
                    oninput: move |event| {
                        input_value.set(event.value());
                    },
                }
                button {
                    class: "echo-button",
                    onclick: move |_| async move {
                        if !input_value().is_empty() {
                            match api::echo(input_value()).await {
                                Ok(data) => response.set(data),
                                Err(e) => response.set(format!("Error: {}", e)),
                            }
                        }
                    },
                    "Send to Server"
                }
            }

            if !response().is_empty() {
                div { class: "echo-response",
                    p { class: "echo-response-text", "{response}" }
                }
            }
        }
    }
}