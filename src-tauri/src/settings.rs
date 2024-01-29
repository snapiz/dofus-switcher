use serde::{Deserialize, Serialize};
use std::{
    path::Path,
    sync::{OnceLock, RwLock},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Breed {
    Ecaflip,
    Eniripsa,
    Iop,
    Cra,
    Feca,
    Sacrieur,
    Sadida,
    Osamodas,
    Enutrof,
    Sram,
    Xélor,
    Pandawa,
    Roublard,
    Zobal,
    Steamer,
    Eliotrope,
    Huppermage,
    Ouginak,
    Forgelance,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Character {
    pub name: String,
    pub breed: Breed,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Group {
    pub name: String,
    pub characters: Vec<Character>,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Settings {
    pub groups: Vec<Group>,
    pub current_group: usize,
}

static SETTINGS_PATH: &'static str = "~/.config/dofus-switcher/Config.toml";

fn get_settings_path() -> String {
    shellexpand::tilde(SETTINGS_PATH).to_string()
}

impl Settings {
    pub fn new() -> Self {
        let path = get_settings_path();

        match std::fs::read_to_string(path) {
            Ok(content) => toml::from_str(content.as_str()).expect("content to be toml"),
            Err(_) => Default::default(),
        }
    }

    pub fn save(&self) {
        let path = get_settings_path();

        let Some(dir) = Path::new(path.as_str()).parent() else {
            return;
        };

        if let Err(_) = std::fs::create_dir_all(dir) {
            return;
        }

        toml::to_string(self)
            .map_err(|e| e.to_string())
            .and_then(|content| std::fs::write(path, content).map_err(|e| e.to_string()))
            .expect("settings to be saved at DofusSwitcher.toml")
    }
}

pub static SETTINGS: OnceLock<RwLock<Settings>> = OnceLock::new();

#[tauri::command]
pub fn get_settings() -> &'static RwLock<Settings> {
    SETTINGS.get_or_init(|| Settings::new().into())
}
