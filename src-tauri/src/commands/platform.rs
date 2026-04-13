use serde::Serialize;
use tauri::AppHandle;

#[derive(Serialize)]
pub struct BatteryExemptionResult {
    pub granted: bool,
    pub already_exempt: bool,
}

#[derive(Serialize)]
pub struct ExactAlarmPermission {
    pub granted: bool,
    pub should_show_rationale: bool,
}

#[derive(Serialize)]
pub struct DndStatus {
    pub dnd_active: bool,
    pub platform_mode_name: Option<String>,
}

#[tauri::command]
pub async fn request_battery_exemption(_app: AppHandle) -> BatteryExemptionResult {
    BatteryExemptionResult {
        granted: true,
        already_exempt: true,
    }
}

#[tauri::command]
pub fn request_exact_alarm_permission() -> Result<ExactAlarmPermission, String> {
    Ok(ExactAlarmPermission {
        granted: true,
        should_show_rationale: false,
    })
}

#[tauri::command]
pub fn check_dnd_status() -> DndStatus {
    DndStatus {
        dnd_active: false,
        platform_mode_name: None,
    }
}
