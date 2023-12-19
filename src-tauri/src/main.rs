// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use image::GenericImageView;
use infer;
use std::fs::File;
use std::io::Read;
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

#[tauri::command]
fn get_file_type(path: &str) -> String {
    let mut buffer = [0; 16];
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return String::new(),
    };

    let mime_type = match file.read(&mut buffer) {
        Ok(_) => {
            let kind = infer::get(&buffer);
            kind.map(|kind| kind.mime_type().to_string())
        }
        Err(_) => None,
    };

    format!("{}", mime_type.unwrap_or(String::new()))
}

#[tauri::command]
fn get_file_size(path: &str) -> u64 {
    let metadata = match std::fs::metadata(path) {
        Ok(metadata) => metadata,
        Err(_) => return 0,
    };

    metadata.len()
}

#[tauri::command]
fn get_image_size(path: &str) -> (u32, u32) {
    let image = match image::open(path) {
        Ok(image) => image,
        Err(_) => return (0, 0),
    };

    image.dimensions()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            find_duplicates,
            get_file_type,
            get_file_size,
            get_image_size
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
