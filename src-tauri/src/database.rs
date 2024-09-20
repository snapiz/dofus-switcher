use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    hash::Hash,
    path::Path,
    sync::{OnceLock, RwLock},
};

static PATH: &'static str = "~/.config/fusdo/data.toml";

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
    pub breed: Option<Breed>,
}

impl Character {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            breed: None,
        }
    }
}

impl Hash for Character {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Group {
    pub name: String,
    pub characters: HashSet<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Database {
    pub groups: Vec<Group>,
    pub characters: Vec<Character>,
}

impl Database {
    pub fn new() -> Self {
        let path = shellexpand::tilde(PATH).to_string();

        match std::fs::read_to_string(path) {
            Ok(data) => toml::from_str(data.as_str()).unwrap_or_default(),
            Err(_) => Default::default(),
        }
    }

    pub fn save(&self) {
        let path = shellexpand::tilde(PATH).to_string();

        let Some(dir) = Path::new(path.as_str()).parent() else {
            return;
        };

        if std::fs::create_dir_all(dir).is_err() {
            return;
        }

        let _ = toml::to_string(self)
            .map_err(|e| e.to_string())
            .and_then(|data| std::fs::write(path, data).map_err(|e| e.to_string()));
    }
}

pub static DATABASE: OnceLock<RwLock<Database>> = OnceLock::new();

#[tauri::command]
pub fn get_database() -> &'static RwLock<Database> {
    DATABASE.get_or_init(|| Database::new().into())
}
