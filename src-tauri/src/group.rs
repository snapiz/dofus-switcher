use std::sync::{OnceLock, RwLock};

use crate::{
    settings::{get_settings, Breed, Character, Group, Settings},
    window::get_visible_windows,
};

static WINDOWS: OnceLock<RwLock<Vec<(String, u32)>>> = OnceLock::new();

pub fn get_windows() -> &'static RwLock<Vec<(String, u32)>> {
    WINDOWS.get_or_init(|| {
        let settings = get_settings().read().unwrap().clone();
        get_windows_from_settings(&settings).into()
    })
}

fn get_windows_from_settings(settings: &Settings) -> Vec<(String, u32)> {
    let visible_windows = get_visible_windows().expect("failed to get visible windows");
    let mut windows = Vec::new();

    if settings.groups.is_empty() {
        return windows;
    }

    for character in settings.groups[settings.current_group].characters.iter() {
        if let Some(w) = visible_windows.get(&character.name) {
            windows.push((character.name.to_owned(), w.to_owned()));
        }
    }

    windows
}

pub fn update_windows(settings: &Settings) {
    let mut windows = get_windows().write().unwrap();

    *windows = get_windows_from_settings(settings);
}

#[tauri::command]
pub fn create_group(name: &str) -> Settings {
    let settings = {
        let mut settings = get_settings().write().unwrap();

        let Ok(windows) = get_visible_windows() else {
            return settings.clone();
        };

        let characters = windows
            .into_iter()
            .map(|(name, _)| Character {
                name,
                breed: Breed::Iop,
            })
            .collect::<Vec<_>>();

        settings.groups.push(Group {
            name: name.to_owned(),
            characters,
        });

        settings.current_group = settings.groups.len() - 1;
        settings.save();
        settings.clone()
    };

    update_windows(&settings);

    settings
}

#[tauri::command]
pub fn set_current_group(id: usize) -> Settings {
    let settings = {
        let mut settings = get_settings().write().unwrap();

        settings.current_group = id;
        settings.save();
        settings.clone()
    };

    update_windows(&settings);
    settings
}

#[tauri::command]
pub fn delete_group(id: usize) -> Settings {
    let settings = {
        let mut settings = get_settings().write().unwrap();

        settings.groups.remove(id);

        if settings.current_group > id {
            settings.current_group = settings.current_group - 1
        }

        settings.clone()
    };

    settings.save();

    update_windows(&settings);

    settings
}

#[tauri::command]
pub fn refresh_group(id: usize) -> Settings {
    let settings = {
        let mut settings = get_settings().write().unwrap();

        let Ok(windows) = get_visible_windows() else {
            return settings.clone();
        };

        let characters = windows
            .into_iter()
            .map(|(name, _)| Character {
                name,
                breed: Breed::Iop,
            })
            .collect::<Vec<_>>();

        settings.groups[id].characters = characters;
        settings.current_group = id;
        settings.save();
        settings.clone()
    };

    update_windows(&settings);

    settings
}

#[tauri::command]
pub fn delete_group_character(group: usize, character: usize) -> Settings {
    let settings = {
        let mut settings = get_settings().write().unwrap();

        settings.groups[group].characters.remove(character);
        settings.save();
        settings.clone()
    };

    update_windows(&settings);

    settings
}
