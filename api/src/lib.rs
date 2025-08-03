//! This crate contains all shared fullstack server functions.
use dioxus::prelude::*;

/// Echo the user input on the server.
#[server(Echo)]
pub async fn echo(input: String) -> Result<String, ServerFnError> {
    Ok(format!("Server received: {}", input))
}

/// Example of a more complex server function
#[server(GetData)]
pub async fn get_data(id: i32) -> Result<String, ServerFnError> {
    // Simulate database lookup
    Ok(format!("Data for ID {}: Lorem ipsum dolor sit amet", id))
}