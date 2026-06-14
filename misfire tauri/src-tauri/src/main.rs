// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use std::fs;

// --- COMANDO DE CIERRE ---
// Modificado para destruir la ventana de forma limpia desde el backend
#[tauri::command]
fn close_app(window: tauri::WebviewWindow) {
    let _ = window.destroy(); 
}

// --- TUS COMANDOS DE DATOS ---
fn get_data_dir(app: &tauri::AppHandle) -> std::path::PathBuf {
    app.path().app_data_dir().expect("no se pudo encontrar la ruta")
}

#[tauri::command]
fn read_data(app: tauri::AppHandle) -> Result<String, String> {
    let mut path = get_data_dir(&app);
    path.push("data.json");
    if path.exists() {
        fs::read_to_string(path).map_err(|e| e.to_string())
    } else {
        Ok("{}".to_string())
    }
}

#[tauri::command]
fn write_data(key: String, value: String, app: tauri::AppHandle) -> Result<(), String> {
    let mut path = get_data_dir(&app);
    fs::create_dir_all(&path).map_err(|e| e.to_string())?;
    path.push("data.json");
    let mut current_data: serde_json::Value = if path.exists() {
        let content = fs::read_to_string(&path).unwrap_or_else(|_| "{}".to_string());
        serde_json::from_str(&content).unwrap_or(serde_json::json!({}))
    } else {
        serde_json::json!({})
    };
    current_data[key] = serde_json::Value::String(value);
    fs::write(path, current_data.to_string()).map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        // Añadimos close_app y tus otros comandos a los handlers disponibles
        .invoke_handler(tauri::generate_handler![
            read_data, 
            write_data, 
            close_app
        ])
        .run(tauri::generate_context!())
        .expect("error al ejecutar la app");
}