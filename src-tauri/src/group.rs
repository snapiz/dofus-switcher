use crate::{
    database::{get_database, Character, Group},
    desktop::Desktop,
};

#[tauri::command]
pub fn get_available_characters() -> Vec<Character> {
    let Ok(db) = get_database().read() else {
        return vec![];
    };

    let Ok(desktop) = Desktop::connect() else {
        return vec![];
    };

    let Ok(wins) = desktop.get_windows() else {
        return vec![];
    };

    wins.iter()
        .map(|(name, _)| {
            let c = Character::new(name);
            db.characters.get(&c).cloned().unwrap_or(c)
        })
        .collect::<Vec<_>>()
}

#[tauri::command]
pub fn get_groups() -> Vec<Group> {
    let Ok(db) = get_database().read() else {
        return vec![];
    };

    db.groups.clone()
}

#[tauri::command]
pub fn create_group(name: String) -> Vec<Group> {
    let Ok(mut db) = get_database().write() else {
        return vec![];
    };

    db.groups.splice(
        0..0,
        vec![Group {
            name,
            characters: Default::default(),
        }],
    );

    db.save();
    db.groups.clone()
}

#[tauri::command]
pub fn delete_group(id: usize) -> Vec<Group> {
    let Ok(mut db) = get_database().write() else {
        return vec![];
    };

    db.groups.remove(id);

    db.save();
    db.groups.clone()
}

#[tauri::command]
pub fn add_character_to_group(id: usize, name: String) -> Vec<Group> {
    let Ok(mut db) = get_database().write() else {
        return vec![];
    };

    if db.groups.get(id).is_none() {
        return db.groups.clone();
    };

    let character = Character::new(name);
    if db.groups[id].characters.contains(&character) {
        return db.groups.clone();
    }

    let character = db.characters.get(&character).cloned().unwrap_or(character);
    db.groups[id].characters.push(character);

    db.save();
    db.groups.clone()
}

#[tauri::command]
pub fn add_character_to_group_at(
    id: usize,
    name: String,
    target_name: String,
    right: bool,
) -> Vec<Group> {
    let Ok(mut db) = get_database().write() else {
        return vec![];
    };

    if db.groups.get(id).is_none() {
        return db.groups.clone();
    };

    db.groups[id].characters.retain(|c| c.name != name);

    let character = Character::new(name);
    let character = db.characters.get(&character).cloned().unwrap_or(character);

    db.groups[id]
        .characters
        .iter()
        .position(|c| c.name == target_name)
        .map(|index| {
            if right {
                db.groups[id].characters.insert(index + 1, character);
            } else {
                db.groups[id]
                    .characters
                    .splice(index..index, vec![character]);
            }
        });

    db.save();
    db.groups.clone()
}

#[tauri::command]
pub fn remove_character_from_group(id: usize, character_id: usize) -> Vec<Group> {
    let Ok(mut db) = get_database().write() else {
        return vec![];
    };
    if db.groups.get(id).is_none() {
        return db.groups.clone();
    };

    db.groups[id].characters.remove(character_id);
    db.save();
    db.groups.clone()
}
