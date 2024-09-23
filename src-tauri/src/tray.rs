use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager, Runtime,
};

pub fn create_tray<R: Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<()> {
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let settings_i = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
    let sidebar_i = MenuItem::with_id(app, "sidebar", "Sidebar", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&settings_i, &sidebar_i, &quit_i])?;

    let _ = TrayIconBuilder::with_id("tray")
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .menu_on_left_click(false)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "quit" => app.exit(0),
            "settings" => {
                let main = app
                    .get_webview_window("main")
                    .expect("failed to get main window from settings menu event");
                main.show()
                    .expect("faild to show main window from setting menu event");
                main.set_focus()
                    .expect("failed to focus main window from settings menu event");

                let sidebar = app.get_webview_window("sidebar").unwrap();
                sidebar.hide().unwrap();
            }
            "sidebar" => {
                let sidebar = app
                    .get_webview_window("sidebar")
                    .expect("failed to get main window from settings menu event");
                sidebar
                    .show()
                    .expect("faild to show main window from setting menu event");
                sidebar
                    .set_focus()
                    .expect("failed to focus main window from settings menu event");
                sidebar.set_always_on_top(true).unwrap();

                let main = app.get_webview_window("main").unwrap();
                main.hide().unwrap();
            }
            _ => {}
        })
        .build(app);

    Ok(())
}
