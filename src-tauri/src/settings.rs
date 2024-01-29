use serde::{Deserialize, Serialize};
use std::sync::{OnceLock, RwLock};

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

#[cfg(debug_assertions)] // n'incluez ce code que sur les versions de débogage
static SETTINGS_PATH: &'static str = "../target/DofusSwitcher.toml";

#[cfg(not(debug_assertions))] // n'incluez ce code que sur les versions de débogage
static settings_path: &'static str = "/etc/dofus-switcher/Config.toml";

impl Settings {
    pub fn new() -> Self {
        match std::fs::read_to_string(SETTINGS_PATH) {
            Ok(content) => toml::from_str(content.as_str()).expect("content to be toml"),
            Err(_) => Default::default(),
        }
    }

    pub fn save(&self) {
        toml::to_string(self)
            .map_err(|e| e.to_string())
            .and_then(|content| {
                std::fs::write(SETTINGS_PATH, content).map_err(|e| e.to_string())
            })
            .expect("settings to be saved at DofusSwitcher.toml")
    }
}

pub static SETTINGS: OnceLock<RwLock<Settings>> = OnceLock::new();

#[tauri::command]
pub fn get_settings() -> &'static RwLock<Settings> {
    SETTINGS.get_or_init(|| Settings::new().into())
}