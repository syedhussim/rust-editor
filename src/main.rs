// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::process::Command;
use std::str;
use std::sync::Mutex;

use serde::Serialize;
use tauri::{State, AppHandle, Emitter, Manager};
use tauri_plugin_dialog::DialogExt;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app: &mut tauri::App| {
            let monitor: tauri::Monitor = app.primary_monitor().unwrap().unwrap();
            let monitor_size: &tauri::PhysicalSize<u32> = monitor.size();

            let app_width: u32 = monitor_size.width - 200;
            let app_height: u32 = monitor_size.height - 200;

            let main_window: tauri::WebviewWindow = app.get_webview_window("main").unwrap();

            let physical_size: tauri::PhysicalSize<u32> =
                tauri::PhysicalSize::new(app_width, app_height);
            let app_size: tauri::Size = tauri::Size::from(physical_size);

            main_window.set_size(app_size).unwrap();

            let pos: tauri::PhysicalPosition<i32> =
                tauri::PhysicalPosition::new(monitor.position().x + 100, 100);

            main_window.set_position(pos).unwrap();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![open, run])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn open(app: AppHandle) {

    app.dialog().file().pick_file(move |file_path| {

        let file = file_path.unwrap().as_path().unwrap().to_string_lossy().to_string();

        let contents: String = fs::read_to_string(&file).unwrap();

        let current_file = CurrentFile {
            file,
            contents
        };

        app.emit("file_opened", serde_json::to_string(&current_file).unwrap()).unwrap();
    });
}

#[tauri::command]
fn run(code: String, file : String) -> OutputResponse {

    fs::write(&file, code.as_bytes()).unwrap();

    let output = Command::new("rustc")
        .arg(&file)
        .args(["-o", ""])
        .output()
        .unwrap();

    let standard_error: String = String::from(str::from_utf8(&output.stderr).unwrap());

    let output = Command::new("").output().unwrap();

    let standard_output: String = String::from(str::from_utf8(&output.stdout).unwrap());

    OutputResponse {
        standard_error,
        standard_output,
    }
}

#[derive(Serialize)]
struct OutputResponse {
    standard_error: String,
    standard_output: String,
}

#[derive(Serialize)]
struct CurrentFile {
    file: String,
    contents: String
}