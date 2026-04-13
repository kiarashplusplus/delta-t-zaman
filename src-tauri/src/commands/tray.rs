use serde::Deserialize;
use tauri::{AppHandle, Manager, WebviewWindow};

#[derive(Deserialize)]
pub struct TrayZone {
    pub label: String,
    pub time: String,
    pub is_pinned: bool,
}

#[tauri::command]
#[cfg(desktop)]
pub fn update_tray_display(app: AppHandle, zones: Vec<TrayZone>) -> Result<(), String> {
    if let Some(tray) = app.tray_by_id("main") {
        let pinned: Vec<&TrayZone> = zones.iter().filter(|z| z.is_pinned).collect();
        if pinned.is_empty() {
            let _ = tray.set_tooltip(Some("Delta-t"));
        } else {
            let tooltip = pinned
                .iter()
                .map(|z| format!("{}: {}", z.label, z.time))
                .collect::<Vec<String>>()
                .join("\n");
            let _ = tray.set_tooltip(Some(tooltip));
        }
    }
    Ok(())
}

#[tauri::command]
#[cfg(mobile)]
pub fn update_tray_display(_app: AppHandle, _zones: Vec<TrayZone>) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
#[cfg(desktop)]
pub fn hide_to_tray(window: WebviewWindow) -> Result<(), String> {
    window.hide().map_err(|e| e.to_string())
}

#[tauri::command]
#[cfg(mobile)]
pub fn hide_to_tray(_window: WebviewWindow) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
#[cfg(desktop)]
pub fn show_from_tray(window: WebviewWindow) -> Result<(), String> {
    window.show().map_err(|e| e.to_string())?;
    window.set_focus().map_err(|e| e.to_string())
}

#[tauri::command]
#[cfg(mobile)]
pub fn show_from_tray(_window: WebviewWindow) -> Result<(), String> {
    Ok(())
}
