// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rand::Rng;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, random])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, my {name}!")
}

#[tauri::command]
fn random() -> String {
    let mut rng = rand::thread_rng();
    let n1: u8 = rng.gen();
    format!("Random number: {}", n1)
}
