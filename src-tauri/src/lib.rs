mod database;
mod desktop;
mod group;
mod shortcut;

#[cfg(desktop)]
mod tray;

use group::{
    add_character_to_group, add_character_to_group_at, create_group, delete_group,
    get_active_characters, get_available_characters, get_groups, remove_character_from_group,
    set_character_breed, set_character_enabled,
};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    desktop::watch();
    shortcut::watch();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            #[cfg(all(desktop))]
            {
                let handle = app.handle();
                tray::create_tray(handle)?;
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_groups,
            create_group,
            delete_group,
            get_available_characters,
            remove_character_from_group,
            add_character_to_group,
            add_character_to_group_at,
            set_character_enabled,
            get_active_characters,
            set_character_breed,
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app_handle, event| match event {
            tauri::RunEvent::Ready => {
                let window = app_handle
                    .get_webview_window("main")
                    .expect("failed to get main window");

                window.hide().expect("failed to hide main window at ready");

                let sidebar = app_handle.get_webview_window("sidebar").unwrap();
                sidebar.set_always_on_top(true).unwrap();
            }
            _ => {}
        });
}
