// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn find_duplicates(handle: tauri::AppHandle, path: &str) -> String {
    let resource_path = handle
        .path_resolver()
        .resolve_resource("resources/find_duplicates.py")
        .expect("failed to resolve resource");

    let output = Command::new(resource_path)
        .arg(path)
        .output()
        .expect("command failed");

    format!("{}", String::from_utf8_lossy(&output.stdout))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![find_duplicates])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
