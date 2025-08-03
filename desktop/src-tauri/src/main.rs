// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use {{crate_name | snake_case}}_tauri_lib::run;

fn main() {
    run();
}