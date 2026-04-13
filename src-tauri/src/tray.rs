#[cfg(desktop)]
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter,
};

#[cfg(desktop)]
pub fn setup_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let show_i = MenuItem::with_id(app, "show", "Show Window", true, None::<&str>)?;
    let toggle_compact_i = MenuItem::with_id(app, "toggle_compact", "Toggle Compact Mode", true, None::<&str>)?;
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&show_i, &toggle_compact_i, &quit_i])?;

    TrayIconBuilder::with_id("main")
        .menu(&menu)
        .icon(app.default_window_icon().unwrap().clone())
        .on_menu_event(|app: &tauri::AppHandle, event| {
            let id = event.id.as_ref();
            if id == "show" {
                let _ = app.emit("tray-action", "show");
            } else if id == "toggle_compact" {
                let _ = app.emit("tray-action", "toggle_compact");
            } else if id == "quit" {
                app.exit(0);
            }
        })
        .on_tray_icon_event(|tray: &tauri::tray::TrayIcon, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                let _ = app.emit("tray-action", "show");
            }
        })
        .build(app)?;

    Ok(())
}

#[cfg(mobile)]
pub fn setup_tray(_app: &tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
