use serde::Deserialize;
use std::collections::HashMap;
use tauri::AppHandle;

#[derive(Deserialize)]
pub struct TelemetryEvent {
    pub event_type: String,
    pub metadata: Option<HashMap<String, String>>,
}

#[tauri::command]
pub async fn send_telemetry_event(_app: AppHandle, _event: TelemetryEvent) {
    // fire and forget
}
