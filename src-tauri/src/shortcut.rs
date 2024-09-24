use std::{
    sync::{OnceLock, RwLock},
    thread::{self, sleep},
    time::Duration,
};

use anyhow::Result;
use arboard::Clipboard;
use enigo::{Enigo, Keyboard, Mouse};
use lazy_regex::regex_captures;
use rdev::{listen, EventType};

use crate::desktop::{get_character_windows, Desktop};

static BACKSLASH_PRESSED: OnceLock<RwLock<bool>> = OnceLock::new();

fn is_backslash_pressed() -> &'static RwLock<bool> {
    BACKSLASH_PRESSED.get_or_init(|| false.into())
}

pub fn watch() {
    thread::spawn(|| {
        let _ = listen(|event| {
            let Ok(desktop) = Desktop::connect() else {
                return;
            };

            let Ok(Some(active_window)) = desktop.get_active_window() else {
                return;
            };

            let Ok(wins) = get_character_windows().read() else {
                return;
            };

            let Some(wins) = wins.to_owned() else {
                return;
            };

            let wins = wins.iter().filter(|(_, c)| c.enabled).collect::<Vec<_>>();

            let Ok(mut enigo) = Enigo::new(&enigo::Settings::default()) else {
                return;
            };

            if let EventType::KeyPress(rdev::Key::IntlBackslash) = event.event_type {
                let Ok(mut pressed) = is_backslash_pressed().write() else {
                    return;
                };

                *pressed = true;
            }

            if let EventType::KeyRelease(rdev::Key::IntlBackslash) = event.event_type {
                let Ok(mut pressed) = is_backslash_pressed().write() else {
                    return;
                };

                *pressed = false;
            }

            // Go to window 1
            if let (EventType::KeyPress(rdev::Key::F1), Some((id, _))) =
                (event.event_type, wins.get(0))
            {
                let _ = select_window(&desktop, &mut enigo, id.to_owned());
            }

            // Go to window 2
            if let (EventType::KeyPress(rdev::Key::F2), Some((id, _))) =
                (event.event_type, wins.get(1))
            {
                let _ = select_window(&desktop, &mut enigo, id.to_owned());
            }

            // Go to window 3
            if let (EventType::KeyPress(rdev::Key::F3), Some((id, _))) =
                (event.event_type, wins.get(2))
            {
                let _ = select_window(&desktop, &mut enigo, id.to_owned());
            }

            // Go to window 4
            if let (EventType::KeyPress(rdev::Key::F4), Some((id, _))) =
                (event.event_type, wins.get(3))
            {
                let _ = select_window(&desktop, &mut enigo, id.to_owned());
            }

            // Go to window 5
            if let (EventType::KeyPress(rdev::Key::F5), Some((id, _))) =
                (event.event_type, wins.get(4))
            {
                let _ = select_window(&desktop, &mut enigo, id.to_owned());
            }

            // Go to window 6
            if let (EventType::KeyPress(rdev::Key::F6), Some((id, _))) =
                (event.event_type, wins.get(5))
            {
                let _ = select_window(&desktop, &mut enigo, id.to_owned());
            }

            // Go to window 7
            if let (EventType::KeyPress(rdev::Key::F7), Some((id, _))) =
                (event.event_type, wins.get(6))
            {
                let _ = select_window(&desktop, &mut enigo, id.to_owned());
            }

            // Go to window 8
            if let (EventType::KeyPress(rdev::Key::F8), Some((id, _))) =
                (event.event_type, wins.get(7))
            {
                let _ = select_window(&desktop, &mut enigo, id.to_owned());
            }

            // Travel
            if let EventType::KeyPress(rdev::Key::PageUp) = event.event_type {
                let Ok(pressed) = is_backslash_pressed().read() else {
                    return;
                };

                let skip = if pressed.to_owned() { 1 } else { 0 };

                let Ok(mut clipboard) = Clipboard::new() else {
                    return;
                };

                let Ok(selection) = clipboard.get_text() else {
                    return;
                };

                if !selection.starts_with("/travel ") {
                    let Some((_, x, y)) = regex_captures!(r#"(-?\d+),(-?\d+)"#, &selection) else {
                        return;
                    };
                    if clipboard.set_text(&format!("/travel {x},{y}")).is_err() {
                        return;
                    };
                };

                for (id, _) in wins.iter().skip(skip) {
                    let _ = select_window(&desktop, &mut enigo, id.to_owned());
                    sleep(Duration::from_millis(40));
                    let _ = enigo.key(enigo::Key::Space, enigo::Direction::Click);
                    sleep(Duration::from_millis(40));
                    let _ = enigo.key(enigo::Key::Control, enigo::Direction::Press);
                    sleep(Duration::from_millis(40));
                    let _ = enigo.key(enigo::Key::Unicode('a'), enigo::Direction::Click);
                    sleep(Duration::from_millis(40));
                    let _ = enigo.key(enigo::Key::Unicode('v'), enigo::Direction::Click);
                    sleep(Duration::from_millis(40));
                    let _ = enigo.key(enigo::Key::Control, enigo::Direction::Release);

                    sleep(Duration::from_millis(40));
                    let _ = enigo.key(enigo::Key::Return, enigo::Direction::Click);

                    sleep(Duration::from_millis(200));

                    let _ = enigo.key(enigo::Key::Return, enigo::Direction::Click);

                    sleep(Duration::from_millis(100));
                }

                if let Some((id, _)) = wins.first() {
                    let _ = select_window(&desktop, &mut enigo, id.to_owned());
                }
            }

            // Send group invit
            if let EventType::KeyPress(rdev::Key::Home) = event.event_type {
                if let Some((id, _)) = wins.first() {
                    let _ = select_window(&desktop, &mut enigo, id.to_owned());
                } else {
                    return;
                }
                let Ok(mut clipboard) = Clipboard::new() else {
                    return;
                };

                sleep(Duration::from_millis(40));
                let _ = enigo.key(enigo::Key::Space, enigo::Direction::Click);

                for (_, character) in wins.iter().skip(1) {
                    let _ = clipboard.set_text(&format!("/invite {}", character.name));
                    sleep(Duration::from_millis(40));
                    let _ = enigo.key(enigo::Key::Control, enigo::Direction::Press);
                    sleep(Duration::from_millis(40));
                    let _ = enigo.key(enigo::Key::Unicode('a'), enigo::Direction::Click);
                    sleep(Duration::from_millis(40));
                    let _ = enigo.key(enigo::Key::Unicode('v'), enigo::Direction::Click);
                    sleep(Duration::from_millis(40));
                    let _ = enigo.key(enigo::Key::Control, enigo::Direction::Release);
                    sleep(Duration::from_millis(40));
                    let _ = enigo.key(enigo::Key::Return, enigo::Direction::Click);
                    sleep(Duration::from_millis(100));
                }
            }

            // Go right click to all with or without leader
            if let EventType::KeyPress(rdev::Key::End) = event.event_type {
                let Ok(pressed) = is_backslash_pressed().read() else {
                    return;
                };

                let skip = if pressed.to_owned() { 1 } else { 0 };

                for (id, _) in wins.iter().skip(skip) {
                    let _ = select_window(&desktop, &mut enigo, id.to_owned());
                    let _ = enigo.button(enigo::Button::Right, enigo::Direction::Click);
                    sleep(Duration::from_millis(150));
                }

                if let Some((id, _)) = wins.first() {
                    let _ = select_window(&desktop, &mut enigo, id.to_owned());
                }
            }

            // Go left click to all with or without leader
            if let EventType::KeyPress(rdev::Key::Delete) = event.event_type {
                let Ok(pressed) = is_backslash_pressed().read() else {
                    return;
                };

                let skip = if pressed.to_owned() { 1 } else { 0 };

                for (id, _) in wins.iter().skip(skip) {
                    let _ = select_window(&desktop, &mut enigo, id.to_owned());
                    let _ = enigo.button(enigo::Button::Left, enigo::Direction::Click);
                    sleep(Duration::from_millis(150));
                }

                if let Some((id, _)) = wins.first() {
                    let _ = select_window(&desktop, &mut enigo, id.to_owned());
                }
            }

            // Go left double click to all with or without leader
            if let EventType::KeyPress(rdev::Key::PageDown) = event.event_type {
                let Ok(pressed) = is_backslash_pressed().read() else {
                    return;
                };

                let skip = if pressed.to_owned() { 1 } else { 0 };

                for (id, _) in wins.iter().skip(skip) {
                    let _ = select_window(&desktop, &mut enigo, id.to_owned());
                    let _ = enigo.button(enigo::Button::Left, enigo::Direction::Click);
                    let _ = enigo.button(enigo::Button::Left, enigo::Direction::Click);
                    sleep(Duration::from_millis(150));
                }

                if let Some((id, _)) = wins.first() {
                    let _ = select_window(&desktop, &mut enigo, id.to_owned());
                }
            }

            // Go to previous or next
            if let EventType::KeyPress(rdev::Key::Tab) = event.event_type {
                let Some(active_pos) = wins.iter().position(|(win, _)| win == &active_window)
                else {
                    return;
                };

                let Ok(pressed) = is_backslash_pressed().read() else {
                    return;
                };

                let next_pos = if pressed.to_owned() {
                    (active_pos + wins.len() - 1) % wins.len()
                } else {
                    (active_pos + 1) % wins.len()
                };

                if let Some((id, _)) = wins.get(next_pos) {
                    let _ = select_window(&desktop, &mut enigo, id.to_owned());
                }
            }
        });
    });
}

fn select_window(desktop: &Desktop, enigo: &mut Enigo, id: u32) -> Result<()> {
    desktop.show_window(id)?;
    enigo.button(enigo::Button::Middle, enigo::Direction::Click)?;

    Ok(())
}
