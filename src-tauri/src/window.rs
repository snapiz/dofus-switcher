use anyhow::anyhow;
use enigo::{Enigo, MouseControllable};
use rdev::{Button, EventType};
use std::{collections::HashMap, sync::OnceLock, thread};
use x11rb::{
    atom_manager,
    connection::Connection,
    protocol::xproto::{
        AtomEnum, ConfigureWindowAux, ConnectionExt, InputFocus, StackMode, Window,
    },
    rust_connection::RustConnection,
    CURRENT_TIME,
};

use crate::error::CommandResult;

atom_manager! {
    pub AtomCollection: AtomCollectionCookie {
        _NET_WM_NAME,
        _NET_ACTIVE_WINDOW,
        _NET_CLIENT_LIST,
        _NET_WM_STATE,
        _NET_WM_STATE_HIDDEN,
        UTF8_STRING,
    }
}

static CONN_INFO: OnceLock<(RustConnection, usize, AtomCollection)> = OnceLock::new();

pub fn get_conn_info() -> &'static (RustConnection, usize, AtomCollection) {
    CONN_INFO.get_or_init(|| {
        let (conn, screen) = x11rb::connect(None).expect("failed to connect to x11");
        let atoms = AtomCollection::new(&conn)
            .expect("failed to create atoms")
            .reply()
            .expect("failed to reply to atoms");

        (conn, screen, atoms)
    })
}

pub fn get_wm_name(window: u32) -> CommandResult<Option<String>> {
    let (conn, _, atoms) = get_conn_info();
    let property = conn
        .get_property(
            false,
            window,
            atoms._NET_WM_NAME,
            atoms.UTF8_STRING,
            0,
            u32::MAX,
        )?
        .reply()?;

    let wm_name = String::from_utf8(property.value)?;
    let mut wm_name_split = wm_name.split(" - Dofus ");
    let name = wm_name_split.next();

    if wm_name_split.next().is_none() {
        return Ok(None);
    }

    Ok(name.map(|n| n.to_owned()))
}

pub fn get_active_window() -> CommandResult<Option<u32>> {
    let (conn, screen, atoms) = get_conn_info();
    let screen = &conn.setup().roots[screen.to_owned()];
    let active_window = conn
        .get_property(
            false,
            screen.root,
            atoms._NET_ACTIVE_WINDOW,
            AtomEnum::WINDOW,
            0,
            u32::MAX,
        )?
        .reply()?
        .value32()
        .ok_or(anyhow!("_NET_ACTIVE_WINDOW has incorrect format"))?
        .next()
        .ok_or(anyhow!("_NET_ACTIVE_WINDOW is empty"))?;

    Ok(get_wm_name(active_window)?.map(|_| active_window))
}

pub fn get_visible_windows() -> CommandResult<HashMap<String, u32>> {
    let (conn, screen, atoms) = get_conn_info();
    let screen = &conn.setup().roots[screen.to_owned()];
    let windows = conn
        .get_property(
            false,
            screen.root,
            atoms._NET_CLIENT_LIST,
            AtomEnum::WINDOW,
            0,
            u32::MAX,
        )?
        .reply()?
        .value32()
        .ok_or(anyhow!("Wrong property type"))?
        .collect::<Vec<Window>>();

    let mut visible_windows = HashMap::new();

    for window in windows {
        let property = conn
            .get_property(
                false,
                window,
                atoms._NET_WM_STATE,
                AtomEnum::ATOM,
                0,
                u32::MAX,
            )?
            .reply()?;

        if property
            .value32()
            .ok_or(anyhow!("Wrong property type"))?
            .any(|v| v == atoms._NET_WM_STATE_HIDDEN)
        {
            continue;
        }

        if let Some(name) = get_wm_name(window)? {
            visible_windows.insert(name, window);
        }
    }

    Ok(visible_windows)
}

pub fn focus(id: &u32, pre_click: bool) -> CommandResult<()> {
    let (conn, _, _) = get_conn_info();

    conn.set_input_focus(InputFocus::POINTER_ROOT, id.to_owned(), CURRENT_TIME)
        .unwrap()
        .check()?;

    conn.configure_window(
        id.to_owned(),
        &ConfigureWindowAux::new().stack_mode(StackMode::ABOVE),
    )?
    .check()?;

    std::thread::sleep(std::time::Duration::from_millis(100));

    if !pre_click {
        return Ok(());
    }

    let mut enigo = Enigo::new();
    let (x, y) = enigo.mouse_location();
    let attr = conn.get_geometry(id.to_owned())?.reply()?;

    enigo.mouse_move_to(
        (attr.x as u16 + (attr.width as f32 * 0.435).floor() as u16).into(),
        (attr.y as u16 + (attr.height as f32 * 0.98).floor() as u16).into(),
    );

    send(&EventType::ButtonPress(Button::Left));
    send(&EventType::ButtonRelease(Button::Left));

    enigo.mouse_move_to(x, y);

    Ok(())
}

pub fn send(event_type: &EventType) {
    let delay = std::time::Duration::from_millis(20);
    match rdev::simulate(event_type) {
        Ok(()) => (),
        Err(_) => {
            println!("We could not send {:?}", event_type);
        }
    }
    // Let ths OS catchup (at least MacOS)
    thread::sleep(delay);
}
