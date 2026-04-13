use serde::Serialize;
use tauri::AppHandle;

#[derive(Serialize)]
pub struct UpdateCheckResult {
    pub update_available: bool,
    pub version: Option<String>,
    pub release_notes: Option<String>,
    pub download_url: Option<String>,
}

#[derive(Serialize)]
pub struct InstallResult {
    pub status: String,
    pub progress: Option<f64>,
}

#[tauri::command]
pub async fn check_for_update(_app: AppHandle) -> Result<UpdateCheckResult, String> {
    Ok(UpdateCheckResult {
        update_available: false,
        version: None,
        release_notes: None,
        download_url: None,
    })
}

#[tauri::command]
pub async fn install_update(_app: AppHandle) -> Result<InstallResult, String> {
    Err("No update pending".to_string())
}
