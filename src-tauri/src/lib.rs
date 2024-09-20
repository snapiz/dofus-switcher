mod database;
mod desktop;
mod shortcut;

use database::get_database;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    desktop::watch();
    shortcut::watch();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_database])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
