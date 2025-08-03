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

### Desktop Development

```bash
cd desktop
npm install
npm run dev
```

This will:
1. Start the Tailwind CSS watcher
2. Launch the Dioxus dev server
3. Open the Tauri desktop app

### Web Development

```bash
cd web
dx serve --platform web
```

### Building for Production

Desktop:
```bash
cd desktop
npm run build
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

## Troubleshooting

### dx serve hangs
The desktop app uses the `-i false` flag to prevent hanging. This is already configured in `tauri.conf.json`.

### Cargo.lock version issues
If you see errors about Cargo.lock version, change the version from 4 to 3.

### Missing WASM target
```bash
rustup target add wasm32-unknown-unknown
```

## License

This project is licensed under the MIT License.