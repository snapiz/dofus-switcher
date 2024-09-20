use std::{
    sync::{OnceLock, RwLock},
    thread,
};

use anyhow::Result;
use enigo::{Enigo, Keyboard, Mouse};
use rdev::{listen, EventType};

use crate::desktop::Desktop;

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

            if let EventType::KeyPress(rdev::Key::F1) = event.event_type {
                let wins = desktop.get_windows().unwrap();
                let win = wins.get("Yuniz").unwrap();
                let _ = select_window(&desktop, &mut enigo, win.to_owned());
                enigo
                    .key(enigo::Key::Space, enigo::Direction::Press)
                    .unwrap();
                enigo
                    .key(enigo::Key::Space, enigo::Direction::Release)
                    .unwrap();
            }
        });
    });
}

fn select_window(desktop: &Desktop, enigo: &mut Enigo, id: u32) -> Result<()> {
    desktop.show_window(id)?;
    enigo.button(enigo::Button::Middle, enigo::Direction::Click)?;

    Ok(())
}
