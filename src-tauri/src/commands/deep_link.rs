use serde::Serialize;

#[derive(Serialize)]
pub struct DeepLinkResult {
    pub action: String,
    pub timezone: Option<String>,
    pub label: Option<String>,
    pub alarm_time: Option<String>,
    pub valid: bool,
    pub error_message: Option<String>,
}

#[tauri::command]
pub fn handle_deep_link(_url: String) -> Result<DeepLinkResult, String> {
    Ok(DeepLinkResult {
        action: "unknown".to_string(),
        timezone: None,
        label: None,
        alarm_time: None,
        valid: false,
        error_message: Some("Not implemented".to_string()),
    })
}
