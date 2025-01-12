// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::process::Command;
use std::str;
use serde::Serialize;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![load, run])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn load() -> String{
    
    let dir: String = std::env::current_exe().unwrap().parent().unwrap().to_string_lossy().to_string();
    let rs_file: String = format!("{}/code.rs", dir);

    match fs::read_to_string(rs_file){
        Ok(code) => {
            code
        },
        Err(_e) => {
            "".to_string()
        }
    }
}

#[tauri::command]
fn run(code: String) -> OutputResponse {

    let dir: String = std::env::current_exe().unwrap().parent().unwrap().to_string_lossy().to_string();
    let rs_file: String = format!("{}/code.rs", dir);

    match fs::write(&rs_file, code.as_bytes()){
        Ok(()) => {

            let bin_file: String = format!("{}/code", dir);

            let output = Command::new("rustc")
                .arg(&rs_file)
                .args(["-o", &bin_file])
                .output()
                .unwrap();
    
            let standard_error: String = String::from(str::from_utf8(&output.stderr).unwrap());
        
            let output = Command::new(&bin_file).output().unwrap();
        
            let standard_output: String = String::from(str::from_utf8(&output.stdout).unwrap());
        
            OutputResponse {
                standard_error,
                standard_output,
            }
        },
        Err(e) => {
            OutputResponse {
                standard_error : e.to_string(),
                standard_output : "".to_string(),
            }
        }
    }
}

#[derive(Serialize)]
struct OutputResponse {
    standard_error: String,
    standard_output: String,
}