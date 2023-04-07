// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    println!("Hello, {}! You've been greeted from Rust!", name);
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// コマンドを追加する
#[tauri::command]
async fn say_hello_command() -> String {
    println!("こんにちは世界! Rustからこんにちは!");
    format!("こんにちは世界! Rustからこんにちは!")
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, say_hello_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
