// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod error;
mod group;
mod settings;
mod window;

use arboard::Clipboard;
use group::{get_windows, update_windows};
use rdev::{listen, Button, Event, EventType, Key};
use settings::get_settings;
use std::{
    sync::{OnceLock, RwLock},
    thread,
    time::Duration,
};
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};
use window::send;

static SHIFT_PRESSED: OnceLock<RwLock<bool>> = OnceLock::new();

pub fn is_shift_pressed() -> &'static RwLock<bool> {
    SHIFT_PRESSED.get_or_init(|| false.into())
}

fn pre_typing() {
    send(&EventType::KeyPress(Key::Space));
    send(&EventType::KeyRelease(Key::Space));
    send(&EventType::KeyPress(Key::ControlLeft));
    send(&EventType::KeyPress(Key::KeyA));
    send(&EventType::KeyRelease(Key::ControlLeft));
    send(&EventType::KeyRelease(Key::KeyA));
    send(&EventType::KeyPress(Key::Delete));
    send(&EventType::KeyRelease(Key::Delete));
}

fn callback(event: Event) {
    let Ok(Some(active_window)) = window::get_active_window() else {
        return;
    };

    let wins = get_windows().read().unwrap();

    // travel
    if let EventType::KeyPress(Key::PageUp) = event.event_type {
        let Some((_, leader)) = wins.first() else {
            return;
        };

        for (_, win) in wins.iter() {
            if &active_window != leader {
                let _ = window::focus(win, true);
            }

            thread::sleep(Duration::from_millis(150));

            pre_typing();

            thread::sleep(Duration::from_millis(100));

            send(&EventType::KeyPress(Key::ControlLeft));

            thread::sleep(Duration::from_millis(20));

            send(&EventType::KeyPress(Key::KeyV));
            send(&EventType::KeyRelease(Key::ControlLeft));
            send(&EventType::KeyRelease(Key::KeyV));

            thread::sleep(Duration::from_millis(20));

            send(&EventType::KeyPress(Key::Return));
            send(&EventType::KeyRelease(Key::Return));

            thread::sleep(Duration::from_millis(200));

            send(&EventType::KeyPress(Key::Return));
            send(&EventType::KeyRelease(Key::Return));
        }
    }

    // invite to group
    if let EventType::KeyPress(Key::Home) = event.event_type {
        let Some((_, leader)) = wins.first() else {
            return;
        };

        if &active_window != leader {
            let _ = window::focus(leader, true);
        }

        pre_typing();

        for (name, _) in wins.iter().skip(1) {
            thread::sleep(Duration::from_millis(150));

            let mut clipboard = Clipboard::new().unwrap();
            clipboard.set_text(format!("/invite {name}")).unwrap();
            thread::sleep(Duration::from_millis(100));

            send(&EventType::KeyPress(Key::ControlLeft));

            thread::sleep(Duration::from_millis(20));

            send(&EventType::KeyPress(Key::KeyV));
            send(&EventType::KeyRelease(Key::ControlLeft));
            send(&EventType::KeyRelease(Key::KeyV));

            thread::sleep(Duration::from_millis(20));

            send(&EventType::KeyPress(Key::Return));
            send(&EventType::KeyRelease(Key::Return));
        }
    }

    // right click with all
    if let EventType::KeyPress(Key::End) = event.event_type {
        for (_, win) in wins.iter() {
            let _ = window::focus(win, false);
            send(&EventType::ButtonPress(Button::Right));
            send(&EventType::ButtonRelease(Button::Right));
        }
    }

    // double left click all
    if let EventType::KeyPress(Key::PageDown) = event.event_type {
        for (_, win) in wins.iter() {
            let _ = window::focus(win, false);
            send(&EventType::ButtonPress(Button::Left));
            send(&EventType::ButtonRelease(Button::Left));
            send(&EventType::ButtonPress(Button::Left));
            send(&EventType::ButtonRelease(Button::Left));
        }
    }

    // left click all
    if let EventType::KeyPress(Key::BackQuote) = event.event_type {
        let shift_pressed = is_shift_pressed().read().unwrap();
        let skin_n = if shift_pressed.to_owned() { 1 } else { 0 };

        for (_, win) in wins.iter().skip(skin_n) {
            let _ = window::focus(win, false);
            send(&EventType::ButtonPress(Button::Left));
            send(&EventType::ButtonRelease(Button::Left));
        }
    }

    if let EventType::KeyPress(Key::ShiftLeft) = event.event_type {
        let mut shift_pressed = is_shift_pressed().write().unwrap();
        *shift_pressed = true;
    }

    if let EventType::KeyRelease(Key::ShiftLeft) = event.event_type {
        let mut shift_pressed = is_shift_pressed().write().unwrap();
        *shift_pressed = false;
    }

    // Go to next or previous
    if let EventType::KeyPress(Key::Tab) = event.event_type {
        let Some(pos) = wins.iter().position(|(_, win)| win == &active_window) else {
            return;
        };

        let shift_pressed = is_shift_pressed().read().unwrap();

        let i = if shift_pressed.to_owned() {
            if pos == 0 {
                wins.len() - 1
            } else {
                pos - 1
            }
        } else {
            if pos == wins.len() - 1 {
                0
            } else {
                pos + 1
            }
        };

        if let Some((_, win)) = wins.get(i) {
            let _ = window::focus(win, true);
        }
    }

    // Select window 1
    if let (EventType::KeyPress(Key::F1), Some((_, win))) = (event.event_type, wins.get(0)) {
        let _ = window::focus(win, true);
    }

    // Select window 2
    if let (EventType::KeyPress(Key::F2), Some((_, win))) = (event.event_type, wins.get(1)) {
        let _ = window::focus(win, true);
    }

    // Select window 3
    if let (EventType::KeyPress(Key::F3), Some((_, win))) = (event.event_type, wins.get(2)) {
        let _ = window::focus(win, true);
    }

    // Select window 4
    if let (EventType::KeyPress(Key::F4), Some((_, win))) = (event.event_type, wins.get(3)) {
        let _ = window::focus(win, true);
    }

    // Select window 5
    if let (EventType::KeyPress(Key::F5), Some((_, win))) = (event.event_type, wins.get(4)) {
        let _ = window::focus(win, true);
    }

    // Select window 6
    if let (EventType::KeyPress(Key::F6), Some((_, win))) = (event.event_type, wins.get(5)) {
        let _ = window::focus(win, true);
    }

    // Select window 7
    if let (EventType::KeyPress(Key::F7), Some((_, win))) = (event.event_type, wins.get(6)) {
        let _ = window::focus(win, true);
    }

    // Select window 8
    if let (EventType::KeyPress(Key::F8), Some((_, win))) = (event.event_type, wins.get(6)) {
        let _ = window::focus(win, true);
    }
}

fn main() {
    thread::spawn(|| listen(callback));

    let refresh = CustomMenuItem::new("refresh".to_string(), "Refresh");
    let show = CustomMenuItem::new("settings".to_string(), "Settings");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_item(refresh)
        .add_item(quit); // insert the menu items here
    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)] // n'incluez ce code que sur les versions de débogage
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            settings::get_settings,
            group::create_group,
            group::set_current_group,
            group::delete_group,
            group::refresh_group,
            group::delete_group_character,
            group::swap_group_character
        ])
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => app.exit(0),
                "refresh" => {
                    let settings = get_settings().read().unwrap();
                    update_windows(&settings.clone());
                }
                "settings" => {
                    let window = app.get_window("main").expect("failed to get main window");

                    window
                        .show()
                        .expect("failed to show main window at MenuItemClick.settings");

                    window.set_focus().expect("window to focus");
                }
                _ => {}
            },
            _ => {}
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event
                    .window()
                    .hide()
                    .expect("failed to hide main window at WindowEvent.CloseRequested");
                api.prevent_close();
            }
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => api.prevent_exit(),
            tauri::RunEvent::Ready => {
                let window = app_handle
                    .get_window("main")
                    .expect("failed to get main window");

                window.hide().expect("failed to hide main window at ready");
            }
            _ => {}
        });
}
