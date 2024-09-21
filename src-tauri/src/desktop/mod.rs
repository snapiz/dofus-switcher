use crate::database::{get_database, Character};
use std::{
    collections::HashMap,
    sync::{OnceLock, RwLock},
    thread::{self, sleep},
    time::Duration,
};

mod x11;

pub use x11::X11Desktop as Desktop;

static CHARACTER_WINDOWS: OnceLock<RwLock<Option<Vec<(u32, Character)>>>> = OnceLock::new();

pub fn get_character_windows() -> &'static RwLock<Option<Vec<(u32, Character)>>> {
    CHARACTER_WINDOWS.get_or_init(|| Default::default())
}

pub fn watch() {
    thread::spawn(|| {
        let Ok(desktop) = Desktop::connect() else {
            return;
        };

        loop {
            sleep(Duration::from_secs(1));

            let Ok(mut windows) = desktop.get_windows() else {
                break;
            };

            let group_wins = {
                let Ok(db) = get_database().read() else {
                    continue;
                };

                let mut groups = db.groups.clone();

                groups.sort_by(|a, b| {
                    let a_count = a
                        .characters
                        .iter()
                        .filter(|c| windows.contains_key(&c.name))
                        .count();
                    let a_percent = if a.characters.is_empty() {
                        0
                    } else {
                        a_count * 100 / a.characters.len()
                    };

                    let b_count = b
                        .characters
                        .iter()
                        .filter(|c| windows.contains_key(&c.name))
                        .count();
                    let b_percent = if b.characters.is_empty() {
                        0
                    } else {
                        b_count * 100 / b.characters.len()
                    };

                    a_percent.cmp(&b_percent)
                });

                groups.last().cloned().map(|g| {
                    g.characters
                        .iter()
                        .filter_map(|c| windows.get(&c.name).map(|win| (win.to_owned(), c.clone())))
                        .collect::<Vec<_>>()
                })
            };

            let Ok(mut character_windows) = get_character_windows().write() else {
                continue;
            };

            *character_windows = group_wins;

            let Ok(mut db) = get_database().write() else {
                continue;
            };

            for (key, _) in db.characters.iter() {
                windows.remove(key.as_str());
            }

            if windows.is_empty() {
                continue;
            }

            let characters = windows
                .iter()
                .map(|(key, _)| (key.to_owned(), Character::new(key)))
                .collect::<HashMap<_, _>>();

            db.characters.extend(characters);
            db.save();
        }
    });
}
