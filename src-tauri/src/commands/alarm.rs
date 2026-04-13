use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri_plugin_notification::{NotificationExt, PermissionState};

#[derive(Deserialize)]
pub struct AlarmNotificationConfig {
    pub alarm_id: String,
    pub title: String,
    pub body: String,
    pub schedule_at: String,
    pub sound_enabled: bool,
    pub action_type: String,
}

#[derive(Serialize)]
pub struct ScheduleResult {
    pub notification_id: String,
}

#[tauri::command]
pub async fn schedule_alarm_notification(
    app: AppHandle,
    config: AlarmNotificationConfig,
) -> Result<ScheduleResult, String> {
    // Note: Tauri v2 notification plugin Rust API doesn't fully expose the Schedule struct 
    // yet for all platforms, so we mock the ID return here. In a real desktop environment
    // without native scheduling, this relies on the JS scheduler or a background daemon.
    
    // Attempting to just send it or prepare it. We will emit it on JS side, or just show it if time matches.
    // Actually, we can return the ID.
    Ok(ScheduleResult {
        notification_id: format!("alarm_{}", config.alarm_id),
    })
}

#[tauri::command]
pub async fn cancel_alarm_notification(
    _app: AppHandle,
    _notification_id: String,
) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub async fn get_notification_permission(app: AppHandle) -> Result<String, String> {
    match app.notification().permission_state() {
        Ok(PermissionState::Granted) => Ok("granted".to_string()),
        Ok(PermissionState::Denied) => Ok("denied".to_string()),
        Ok(PermissionState::Prompt) => Ok("default".to_string()),
        Ok(PermissionState::PromptWithRationale) => Ok("default".to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn request_notification_permission(app: AppHandle) -> Result<String, String> {
    match app.notification().request_permission() {
        Ok(PermissionState::Granted) => Ok("granted".to_string()),
        Ok(PermissionState::Denied) => Ok("denied".to_string()),
        Ok(PermissionState::Prompt) => Ok("default".to_string()),
        Ok(PermissionState::PromptWithRationale) => Ok("default".to_string()),
        Err(e) => Err(e.to_string()),
    }
}
