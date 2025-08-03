use tauri_plugin_shell::ShellExt;

// Example Tauri command - you can add more native commands here
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! Greetings from Tauri!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}