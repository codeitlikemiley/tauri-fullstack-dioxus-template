# {{ project-name | title_case}} App

A fullstack Dioxus app with Tauri

## Overview

This is a fullstack Dioxus application with:
- **Tauri** for native desktop apps
- **Server functions** for backend logic
- **Shared UI components** across all platforms
- **Tailwind CSS** for styling
- **Hot-reloading** development experience

## Project Structure

```
{{ project-name }}/
├── api/          # Server functions (shared backend logic)
├── ui/           # Shared UI components
├── desktop/      # Tauri desktop app
├── web/          # Web application
├── mobile/       # Mobile app (iOS/Android)
└── Cargo.toml    # Workspace configuration
```

## Prerequisites

- Rust 1.70 or higher
- Node.js 16 or higher
- cargo-generate: `cargo install cargo-generate`
- Dioxus CLI: `cargo install dioxus-cli`
- Tauri CLI: `cargo install tauri-cli`

## Quick Start

### Clone the Repo

```bash
cargo generate --git https://github.com/codeitlikemiley/tauri-fullstack-dioxus-template.git
cd {{ project-name }}

```

### Desktop Development

```bash
cd desktop

# Use your own icon (recommended)
cargo tauri icon path/to/your/icon.png

# Generate Tailwind Css Classes
tailwindcss -i input.css -o assets/tailwind.css --watch

cargo tauri dev
```


### Web Development

```bash
cd web
dx serve --platform web
```

### Mobile Development

Desktop:
```bash
cd mobile
dx serve --platform android
dx serve --platform ios
```

## Architecture

### Server Functions (api/)

The `api` crate contains all server functions that can be called from any platform:

```rust
#[server(Echo)]
pub async fn echo(input: String) -> Result<String, ServerFnError> {
    Ok(format!("Server received: {}", input))
}
```

### Shared UI Components (ui/)

Reusable components that work across all platforms:
- `Navbar` - Navigation component
- `Hero` - Landing page hero section
- `Echo` - Server function demo component

### Platform-Specific Apps

Each platform has its own crate with platform-specific routing and features:
- `desktop/` - Tauri desktop app with native features
- `web/` - Pure web application
- `mobile/` - iOS/Android apps


## License

This project is licensed under the MIT License.