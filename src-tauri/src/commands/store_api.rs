use tauri::{AppHandle, Manager, command};
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

fn get_file_path(app: &AppHandle, store_name: &str) -> Result<PathBuf, String> {
    let app_data = app.path().app_data_dir().map_err(|_| "No app data dir".to_string())?;
    if !app_data.exists() {
        let _ = fs::create_dir_all(&app_data);
    }
    Ok(app_data.join(store_name))
}

#[command]
pub fn load_store_value(app: AppHandle, store_name: String, key: String) -> Result<Option<Value>, String> {
    let path = get_file_path(&app, &store_name)?;
    if !path.exists() {
        return Ok(None);
    }
    let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let json: Value = serde_json::from_str(&content).unwrap_or(Value::Null);
    Ok(json.get(&key).cloned())
}

#[command]
pub fn save_store_value(app: AppHandle, store_name: String, key: String, value: Value) -> Result<(), String> {
    let path = get_file_path(&app, &store_name)?;
    let mut json = if path.exists() {
        let content = fs::read_to_string(&path).unwrap_or_default();
        serde_json::from_str(&content).unwrap_or(Value::Object(serde_json::Map::new()))
    } else {
        Value::Object(serde_json::Map::new())
    };

    if let Value::Object(ref mut map) = json {
        map.insert(key, value);
    }

    let content = serde_json::to_string_pretty(&json).map_err(|e| e.to_string())?;
    fs::write(&path, content).map_err(|e| e.to_string())?;
    Ok(())
}
