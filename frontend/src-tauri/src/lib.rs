// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[cfg_attr(mobile, tauri::mobile_entry_point)]

use std::path::PathBuf;
use acore::dir::main_and_inst::*;

#[tauri::command]
fn create_command(instance_name: String) -> Result<PathBuf, String> {
    setup_instance(&instance_name).map_err(|e| e.to_string())

}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![create_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
