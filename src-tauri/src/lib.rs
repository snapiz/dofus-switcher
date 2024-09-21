mod database;
mod desktop;
mod group;
mod shortcut;

use group::{
    add_character_to_group, add_character_to_group_at, create_group, delete_group,
    get_available_characters, get_groups, remove_character_from_group, set_character_breed,
    set_character_enabled,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    desktop::watch();
    shortcut::watch();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_groups,
            create_group,
            delete_group,
            get_available_characters,
            remove_character_from_group,
            add_character_to_group,
            add_character_to_group_at,
            set_character_enabled,
            set_character_breed,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
