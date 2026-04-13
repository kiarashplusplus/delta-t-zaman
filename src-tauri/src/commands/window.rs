use serde::{Deserialize, Serialize};
use tauri::{AppHandle, WebviewWindow};

#[cfg(desktop)]
use tauri_plugin_autostart::ManagerExt;

#[derive(Deserialize)]
pub struct WindowStateInput {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub maximized: bool,
}

#[derive(Serialize)]
pub struct WindowStateOutput {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub width: f64,
    pub height: f64,
    pub maximized: bool,
}

#[derive(Serialize)]
pub struct PlatformCloseBehavior {
    pub platform: String,
    pub close_action: String,
    pub tray_available: bool,
    pub quit_shortcut: Option<String>,
}

#[derive(Serialize)]
pub struct ToggleCompactResult {
    pub compact_mode: bool,
}

#[tauri::command]
#[cfg(desktop)]
pub fn set_always_on_top(window: WebviewWindow, enabled: bool) -> Result<(), String> {
    window.set_always_on_top(enabled).map_err(|e| e.to_string())
}

#[tauri::command]
#[cfg(mobile)]
pub fn set_always_on_top(_window: WebviewWindow, _enabled: bool) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
#[cfg(desktop)]
pub fn set_autostart(app: AppHandle, enabled: bool) -> Result<(), String> {
    let autostart = app.autolaunch();
    if enabled {
        autostart.enable().map_err(|e| e.to_string())?;
    } else {
        autostart.disable().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
#[cfg(mobile)]
pub fn set_autostart(_app: AppHandle, _enabled: bool) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn save_window_state(_app: AppHandle, _state: WindowStateInput) -> Result<(), String> {
    // Window state is usually saved via tauri-plugin-window-state, 
    // but we stub it here to fulfill the command signature if manual state saving is expected.
    Ok(())
}

#[tauri::command]
pub fn restore_window_state(_app: AppHandle) -> WindowStateOutput {
    WindowStateOutput {
        x: None,
        y: None,
        width: 420.0,
        height: 680.0,
        maximized: false,
    }
}

#[tauri::command]
#[cfg(desktop)]
pub async fn toggle_compact_mode(window: WebviewWindow, enable: bool) -> ToggleCompactResult {
    if enable {
        let _ = window.set_size(tauri::Size::Logical(tauri::LogicalSize { width: 300.0, height: 400.0 }));
    } else {
        let _ = window.set_size(tauri::Size::Logical(tauri::LogicalSize { width: 420.0, height: 680.0 }));
    }
    ToggleCompactResult { compact_mode: enable }
}

#[tauri::command]
#[cfg(mobile)]
pub async fn toggle_compact_mode(_window: WebviewWindow, enable: bool) -> ToggleCompactResult {
    ToggleCompactResult { compact_mode: enable }
}

#[tauri::command]
pub fn get_platform_close_behavior(_app: AppHandle) -> PlatformCloseBehavior {
    #[cfg(target_os = "macos")]
    let (platform, close_action, quit_shortcut) = ("macos", "suspend", Some("Cmd+Q"));
    
    #[cfg(target_os = "windows")]
    let (platform, close_action, quit_shortcut) = ("windows", "tray", Some("Alt+F4"));
    
    #[cfg(target_os = "linux")]
    let (platform, close_action, quit_shortcut) = ("linux", "tray", Some("Ctrl+Q"));

    #[cfg(mobile)]
    let (platform, close_action, quit_shortcut) = ("mobile", "suspend", None);

    PlatformCloseBehavior {
        platform: platform.to_string(),
        close_action: close_action.to_string(),
        tray_available: cfg!(desktop),
        quit_shortcut: quit_shortcut.map(|s| s.to_string()),
    }
}
