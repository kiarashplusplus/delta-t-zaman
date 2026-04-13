use delta_t_zaman_shared::SystemInfo;

fn detect_system_info() -> SystemInfo {
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
    let locale = std::env::var("LANG")
        .ok()
        .and_then(|value| value.split('.').next().map(|part| part.replace('_', "-")))
        .unwrap_or_else(|| "en-US".to_string());
    let timezone = std::env::var("TZ").unwrap_or_else(|_| "UTC".to_string());

    SystemInfo {
        platform: platform.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        locale,
        timezone,
        app_version: env!("CARGO_PKG_VERSION").to_string(),
        is_mobile,
        supports_tray,
        os_version: std::env::consts::OS.to_string(),
    }
}

#[tauri::command]
pub fn get_system_info() -> SystemInfo {
    detect_system_info()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn system_info_populates_shared_contract_fields() {
        let info = detect_system_info();

        assert!(!info.platform.is_empty());
        assert!(!info.arch.is_empty());
        assert!(!info.app_version.is_empty());
        assert!(!info.os_version.is_empty());
    }

    #[test]
    fn system_info_contract_round_trips_through_json() {
        let info = detect_system_info();
        let json = serde_json::to_value(&info).unwrap();
        let reparsed: SystemInfo = serde_json::from_value(json).unwrap();

        assert_eq!(reparsed, info);
    }
}
