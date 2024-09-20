use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;
use x11rb::protocol::xproto::{ConfigureWindowAux, InputFocus, StackMode};
use x11rb::wrapper::ConnectionExt as WConnectionExt;
use x11rb::{
    atom_manager,
    connection::Connection,
    protocol::xproto::{AtomEnum, ConnectionExt, Window},
    rust_connection::RustConnection,
};

atom_manager! {
    pub AtomCollection: AtomCollectionCookie {
        _NET_CLIENT_LIST,
        _NET_WM_NAME,
        _NET_WM_STATE,
        _NET_WM_STATE_HIDDEN,
        _NET_WM_STATE_FOCUSED,
        _NET_ACTIVE_WINDOW,
        WM_CLASS,
        UTF8_STRING,
        STRING,
    }
}

pub struct X11Desktop {
    conn: RustConnection,
    screen: usize,
    atoms: AtomCollection,
}

impl X11Desktop {
    pub fn connect() -> Result<Self> {
        let (conn, screen) = x11rb::connect(None)?;
        let atoms = AtomCollection::new(&conn)?.reply()?;

        Ok(Self {
            conn,
            screen,
            atoms,
        })
    }

    pub fn show_window(&self, id: u32) -> Result<()> {
        self.conn
            .set_input_focus(InputFocus::POINTER_ROOT, id.to_owned(), x11rb::CURRENT_TIME)?;

        self.conn.configure_window(
            id.to_owned(),
            &ConfigureWindowAux::new().stack_mode(StackMode::ABOVE),
        )?;

        self.conn.sync()?;

        let mut i = 0;

        loop {
            sleep(Duration::from_millis(100));

            if self.is_focus(id.to_owned())? {
                break;
            }

            i += 1;
            if i > 300 {
                break;
            }
        }

        Ok(())
    }

    pub fn get_active_window(&self) -> Result<Option<u32>> {
        let screen = &self.conn.setup().roots[self.screen.to_owned()];
        let active_window = self
            .conn
            .get_property(
                false,
                screen.root,
                self.atoms._NET_ACTIVE_WINDOW,
                AtomEnum::WINDOW,
                0,
                u32::MAX,
            )?
            .reply()?
            .value32()
            .ok_or(anyhow!("_NET_ACTIVE_WINDOW has incorrect format"))?
            .next()
            .ok_or(anyhow!("_NET_ACTIVE_WINDOW is empty"))?;

        if !self.get_process_name(active_window)?.contains("dofus.exe") {
            return Ok(None);
        }

        Ok(Some(active_window))
    }

    pub fn get_windows(&self) -> Result<HashMap<String, u32>> {
        let screen = &self.conn.setup().roots[self.screen];
        let windows = self
            .conn
            .get_property(
                false,
                screen.root,
                self.atoms._NET_CLIENT_LIST,
                AtomEnum::WINDOW,
                0,
                u32::MAX,
            )?
            .reply()?
            .value32()
            .ok_or(anyhow!("value32 is none when trying to get client list"))?
            .collect::<Vec<Window>>();

        let mut dofus_windows = HashMap::new();

        for window in windows {
            if !self.get_process_name(window)?.contains("dofus.exe") {
                continue;
            }

            if self.is_hidden(window)? {
                continue;
            }

            let Some(name) = self.get_name(window)? else {
                continue;
            };

            if name == "Dofus" {
                continue;
            }

            dofus_windows.insert(name, window);
        }

        Ok(dofus_windows)
    }

    fn get_name(&self, id: u32) -> Result<Option<String>> {
        let prop = self
            .conn
            .get_property(
                false,
                id,
                self.atoms._NET_WM_NAME,
                self.atoms.UTF8_STRING,
                0,
                u32::MAX,
            )?
            .reply()?;

        let wm_name = String::from_utf8(prop.value)?;
        let name = wm_name.split(" ").next();

        Ok(name.map(|n| n.to_owned()))
    }

    fn get_process_name(&self, id: u32) -> Result<String> {
        let prop = self
            .conn
            .get_property(
                false,
                id,
                self.atoms.WM_CLASS,
                self.atoms.STRING,
                0,
                u32::MAX,
            )?
            .reply()?;

        let wm_class = String::from_utf8(prop.value)?;
        let (name, _) = wm_class.split_at(wm_class.char_indices().count() / 2);

        Ok(name.to_owned())
    }

    fn is_hidden(&self, id: u32) -> Result<bool> {
        let prop = self
            .conn
            .get_property(
                false,
                id,
                self.atoms._NET_WM_STATE,
                AtomEnum::ATOM,
                0,
                u32::MAX,
            )?
            .reply()?;

        let hidden = prop
            .value32()
            .ok_or(anyhow!("value32 is none when trying to get wn state"))?
            .any(|s| s == self.atoms._NET_WM_STATE_HIDDEN);

        Ok(hidden)
    }

    fn is_focus(&self, id: u32) -> Result<bool> {
        let prop = self
            .conn
            .get_property(
                false,
                id,
                self.atoms._NET_WM_STATE,
                AtomEnum::ATOM,
                0,
                u32::MAX,
            )?
            .reply()?;

        let focus = prop
            .value32()
            .ok_or(anyhow!("value32 is none when trying to get wn state"))?
            .any(|s| s == self.atoms._NET_WM_STATE_FOCUSED);

        Ok(focus)
    }
}
