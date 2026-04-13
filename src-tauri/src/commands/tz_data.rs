use serde::Serialize;
use tauri::AppHandle;

#[derive(Serialize)]
pub struct TzDataUpdateResult {
    pub updated: bool,
    pub patch_version: Option<String>,
    pub zones_affected: u32,
}

#[tauri::command]
pub async fn check_timezone_data_update(_app: AppHandle) -> Result<TzDataUpdateResult, String> {
    Ok(TzDataUpdateResult {
        updated: false,
        patch_version: None,
        zones_affected: 0,
    })
}
