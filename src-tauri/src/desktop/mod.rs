use crate::database::{get_database, Character};
use std::{
    collections::HashMap,
    thread::{self, sleep},
    time::Duration,
};

mod x11;

pub use x11::X11Desktop as Desktop;

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
