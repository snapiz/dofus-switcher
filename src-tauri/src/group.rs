use crate::{
    database::{get_database, Breed, Character, Group},
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
            db.characters
                .get(name)
                .cloned()
                .unwrap_or_else(|| Character::new(name))
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

    if db.groups[id].characters.iter().any(|c| c.name == name) {
        return db.groups.clone();
    }

    let character = db
        .characters
        .get(&name)
        .cloned()
        .unwrap_or_else(|| Character::new(name));

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

    let character = db
        .characters
        .get(&name)
        .cloned()
        .unwrap_or_else(|| Character::new(name));

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

#[tauri::command]
pub fn set_character_enabled(id: usize, character_id: usize, value: bool) -> Vec<Group> {
    let Ok(mut db) = get_database().write() else {
        return vec![];
    };

    if db.groups.get(id).is_none() {
        return db.groups.clone();
    };

    if db.groups[id].characters.get(character_id).is_none() {
        return db.groups.clone();
    };

    db.groups[id].characters[character_id].enabled = value;
    db.save();
    db.groups.clone()
}

#[tauri::command]
pub fn set_character_breed(name: String, breed: Breed) -> Vec<Group> {
    let Ok(mut db) = get_database().write() else {
        return vec![];
    };

    if let Some(character) = db.characters.get_mut(&name) {
        character.breed = Some(breed.clone());
    }

    for group in db.groups.iter_mut() {
        if let Some(character) = group.characters.iter_mut().find(|c| c.name == name) {
            character.breed = Some(breed.clone());
        }
    }

    db.save();
    db.groups.clone()
}
