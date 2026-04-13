use serde::Serialize;

#[derive(Serialize)]
pub struct SystemInfo {
    pub platform: String,
    pub locale: String,
    pub timezone: String,
    pub is_mobile: bool,
    pub supports_tray: bool,
}

#[tauri::command]
pub fn get_system_info() -> SystemInfo {
    let platform = if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "ios") {
        "ios"
    } else if cfg!(target_os = "android") {
        "android"
    } else {
        "unknown"
    };

    let is_mobile = cfg!(target_os = "ios") || cfg!(target_os = "android");
    let supports_tray = !is_mobile;

    SystemInfo {
        platform: platform.to_string(),
        locale: "en-US".to_string(),
        timezone: "UTC".to_string(),
        is_mobile,
        supports_tray,
    }
}
