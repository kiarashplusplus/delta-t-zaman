use delta_t_zaman_shared::{
    AlarmsStore, PreferencesStore, ZonesStore, ALARMS_STORE_NAME, PREFERENCES_STORE_NAME,
    ZONES_STORE_NAME,
};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};
use tauri::{command, AppHandle, Manager};

pub fn app_data_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let app_data = app.path().app_data_dir().map_err(|_| "No app data dir".to_string())?;
    if !app_data.exists() {
        let _ = fs::create_dir_all(&app_data);
    }
    Ok(app_data)
}

fn store_file(base_dir: &Path, store_name: &str) -> Result<PathBuf, String> {
    if !base_dir.exists() {
        fs::create_dir_all(base_dir).map_err(|e| e.to_string())?;
    }
    Ok(base_dir.join(store_name))
}

fn load_store_file_from_dir<T: DeserializeOwned>(
    base_dir: &Path,
    store_name: &str,
) -> Result<Option<T>, String> {
    let path = store_file(base_dir, store_name)?;
    if !path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    serde_json::from_str(&content).map(Some).map_err(|e| e.to_string())
}

fn save_store_file_to_dir<T: Serialize>(
    base_dir: &Path,
    store_name: &str,
    value: &T,
) -> Result<(), String> {
    let path = store_file(base_dir, store_name)?;
    let content = serde_json::to_string_pretty(value).map_err(|e| e.to_string())?;
    fs::write(&path, content).map_err(|e| e.to_string())
}

pub fn load_store_file<T: DeserializeOwned>(
    app: &AppHandle,
    store_name: &str,
) -> Result<Option<T>, String> {
    let base_dir = app_data_dir(app)?;
    load_store_file_from_dir(&base_dir, store_name)
}

pub fn save_store_file<T: Serialize>(
    app: &AppHandle,
    store_name: &str,
    value: &T,
) -> Result<(), String> {
    let base_dir = app_data_dir(app)?;
    save_store_file_to_dir(&base_dir, store_name, value)
}

pub fn load_zones_store_state(app: &AppHandle) -> Result<ZonesStore, String> {
    Ok(load_store_file(app, ZONES_STORE_NAME)?.unwrap_or_default())
}

pub fn save_zones_store_state(app: &AppHandle, store: &ZonesStore) -> Result<(), String> {
    save_store_file(app, ZONES_STORE_NAME, store)
}

pub fn load_alarms_store_state(app: &AppHandle) -> Result<AlarmsStore, String> {
    Ok(load_store_file(app, ALARMS_STORE_NAME)?.unwrap_or_default())
}

pub fn save_alarms_store_state(app: &AppHandle, store: &AlarmsStore) -> Result<(), String> {
    save_store_file(app, ALARMS_STORE_NAME, store)
}

pub fn load_preferences_store_state(app: &AppHandle) -> Result<PreferencesStore, String> {
    Ok(load_store_file(app, PREFERENCES_STORE_NAME)?.unwrap_or_default())
}

pub fn save_preferences_store_state(
    app: &AppHandle,
    store: &PreferencesStore,
) -> Result<(), String> {
    save_store_file(app, PREFERENCES_STORE_NAME, store)
}

#[command]
pub fn load_zones_store(app: AppHandle) -> Result<ZonesStore, String> {
    load_zones_store_state(&app)
}

#[command]
pub fn save_zones_store(app: AppHandle, store: ZonesStore) -> Result<(), String> {
    save_zones_store_state(&app, &store)
}

#[command]
pub fn load_alarms_store(app: AppHandle) -> Result<AlarmsStore, String> {
    load_alarms_store_state(&app)
}

#[command]
pub fn save_alarms_store(app: AppHandle, store: AlarmsStore) -> Result<(), String> {
    save_alarms_store_state(&app, &store)
}

#[command]
pub fn load_preferences_store(app: AppHandle) -> Result<PreferencesStore, String> {
    load_preferences_store_state(&app)
}

#[command]
pub fn save_preferences_store(app: AppHandle, store: PreferencesStore) -> Result<(), String> {
    save_preferences_store_state(&app, &store)
}

#[cfg(test)]
mod tests {
    use super::*;
    use delta_t_zaman_shared::{TimeZoneEntry, UserPreferences};

    fn temp_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "delta-t-zaman-store-tests-{}-{}",
            name,
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn typed_store_helpers_round_trip_zones_store() {
        let dir = temp_dir("zones");
        let store = ZonesStore {
            zones: vec![TimeZoneEntry {
                id: "utc-1".into(),
                iana_id: "UTC".into(),
                display_label: "UTC".into(),
                sort_order: 0,
                pinned_to_tray: false,
                created_at: 1,
            }],
        };

        save_store_file_to_dir(&dir, ZONES_STORE_NAME, &store).unwrap();
        let loaded = load_store_file_from_dir::<ZonesStore>(&dir, ZONES_STORE_NAME)
            .unwrap()
            .unwrap();

        assert_eq!(loaded, store);
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn missing_preferences_store_defaults_cleanly() {
        let dir = temp_dir("prefs");
        let loaded = load_store_file_from_dir::<PreferencesStore>(&dir, PREFERENCES_STORE_NAME)
            .unwrap()
            .unwrap_or_default();

        assert_eq!(loaded.preferences, UserPreferences::default());
        let _ = fs::remove_dir_all(dir);
    }

}
