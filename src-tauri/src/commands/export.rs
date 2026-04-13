use delta_t_zaman_shared::{
    Alarm, AlarmsStore, ExportBundle, ExportResult, ImportConfig, ImportResult,
    PreferencesStore, TimeZoneEntry, UserPreferences, ZonesStore, ALARMS_STORE_NAME,
    PREFERENCES_STORE_NAME, ZONES_STORE_NAME,
};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};
use tauri::{AppHandle, Manager};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind, MessageDialogResult};

fn store_file(base_dir: &Path, store_name: &str) -> Result<PathBuf, String> {
    if !base_dir.exists() {
        fs::create_dir_all(base_dir).map_err(|e| e.to_string())?;
    }
    Ok(base_dir.join(store_name))
}

fn read_json_file<T: DeserializeOwned>(path: &Path) -> Result<Option<T>, String> {
    if !path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    serde_json::from_str(&content).map(Some).map_err(|e| e.to_string())
}

fn write_json_file<T: Serialize>(path: &Path, value: &T) -> Result<(), String> {
    fs::write(path, serde_json::to_vec_pretty(value).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())
}

fn read_store_file<T: DeserializeOwned>(
    base_dir: &Path,
    store_name: &str,
) -> Result<Option<T>, String> {
    let path = store_file(base_dir, store_name)?;
    read_json_file(&path)
}

fn write_store_file<T: Serialize>(
    base_dir: &Path,
    store_name: &str,
    value: &T,
) -> Result<(), String> {
    let path = store_file(base_dir, store_name)?;
    write_json_file(&path, value)
}

fn build_export_bundle_from_dir(base_dir: &Path) -> Result<ExportBundle, String> {
    Ok(ExportBundle {
        version: env!("CARGO_PKG_VERSION").to_string(),
        exported_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| e.to_string())?
            .as_secs(),
        zones: read_store_file::<ZonesStore>(base_dir, ZONES_STORE_NAME)?.map(|store| store.zones),
        alarms: read_store_file::<AlarmsStore>(base_dir, ALARMS_STORE_NAME)?.map(|store| store.alarms),
        preferences: read_store_file::<PreferencesStore>(base_dir, PREFERENCES_STORE_NAME)?
            .map(|store| store.preferences),
    })
}

fn export_bundle_to_path(bundle: &ExportBundle, file_path: &str) -> Result<ExportResult, String> {
    let target_path = PathBuf::from(file_path);
    if let Some(parent) = target_path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Unable to create export folder: {e}"))?;
        }
    }

    let bytes = serde_json::to_vec_pretty(bundle).map_err(|e| e.to_string())?;
    fs::write(&target_path, &bytes).map_err(|e| format!("Unable to write export file: {e}"))?;

    Ok(ExportResult {
        success: true,
        file_path: file_path.to_string(),
        bytes_written: bytes.len() as u64,
    })
}

fn merge_timezones(
    existing: Vec<TimeZoneEntry>,
    incoming: Vec<TimeZoneEntry>,
) -> (Vec<TimeZoneEntry>, u32, u32, Vec<String>) {
    let mut merged = existing;
    let mut known_ids: HashSet<String> = merged.iter().map(|item| item.id.clone()).collect();
    let mut known_iana_ids: HashSet<String> =
        merged.iter().map(|item| item.iana_id.clone()).collect();

    let mut added = 0u32;
    let mut conflicts = 0u32;
    for zone in incoming {
        if known_ids.contains(&zone.id) || known_iana_ids.contains(&zone.iana_id) {
            conflicts += 1;
            continue;
        }

        known_ids.insert(zone.id.clone());
        known_iana_ids.insert(zone.iana_id.clone());
        merged.push(zone);
        added += 1;
    }

    let warnings = if conflicts > 0 {
        vec![format!(
            "Skipped {conflicts} conflicting clock entr{} during merge.",
            if conflicts == 1 { "y" } else { "ies" }
        )]
    } else {
        Vec::new()
    };

    (merged, added, conflicts, warnings)
}

fn merge_alarms(existing: Vec<Alarm>, incoming: Vec<Alarm>) -> (Vec<Alarm>, u32, u32, Vec<String>) {
    let mut merged = existing;
    let mut known_ids: HashSet<String> = merged.iter().map(|item| item.id.clone()).collect();

    let mut added = 0u32;
    let mut conflicts = 0u32;
    for alarm in incoming {
        if known_ids.contains(&alarm.id) {
            conflicts += 1;
            continue;
        }

        known_ids.insert(alarm.id.clone());
        merged.push(alarm);
        added += 1;
    }

    let warnings = if conflicts > 0 {
        vec![format!(
            "Skipped {conflicts} conflicting alarm entr{} during merge.",
            if conflicts == 1 { "y" } else { "ies" }
        )]
    } else {
        Vec::new()
    };

    (merged, added, conflicts, warnings)
}

fn merge_preferences(
    existing: Option<UserPreferences>,
    incoming: UserPreferences,
) -> (UserPreferences, bool, u32, Vec<String>) {
    match existing {
        None => (incoming, true, 0, Vec::new()),
        Some(current) if current == incoming => (current, false, 0, Vec::new()),
        Some(current) => (
            current,
            false,
            1,
            vec![
                "Skipped imported preferences during merge because local preferences already exist."
                    .to_string(),
            ],
        ),
    }
}

fn apply_import_bundle_to_dir(
    base_dir: &Path,
    bundle: ExportBundle,
    mode: &str,
) -> Result<ImportResult, String> {
    let mut result = ImportResult {
        success: true,
        ..ImportResult::default()
    };

    if let Some(zones) = bundle.zones {
        if mode == "merge" {
            let existing = read_store_file::<ZonesStore>(base_dir, ZONES_STORE_NAME)?
                .map(|store| store.zones)
                .unwrap_or_default();
            let (merged, imported, conflicts, warnings) = merge_timezones(existing, zones);
            result.zones_imported = imported;
            result.conflicts_detected += conflicts;
            result.warnings.extend(warnings);
            write_store_file(base_dir, ZONES_STORE_NAME, &ZonesStore { zones: merged })?;
        } else {
            result.zones_imported = zones.len() as u32;
            write_store_file(base_dir, ZONES_STORE_NAME, &ZonesStore { zones })?;
        }
    }

    if let Some(alarms) = bundle.alarms {
        if mode == "merge" {
            let existing = read_store_file::<AlarmsStore>(base_dir, ALARMS_STORE_NAME)?
                .map(|store| store.alarms)
                .unwrap_or_default();
            let (merged, imported, conflicts, warnings) = merge_alarms(existing, alarms);
            result.alarms_imported = imported;
            result.conflicts_detected += conflicts;
            result.warnings.extend(warnings);
            write_store_file(base_dir, ALARMS_STORE_NAME, &AlarmsStore { alarms: merged })?;
        } else {
            result.alarms_imported = alarms.len() as u32;
            write_store_file(base_dir, ALARMS_STORE_NAME, &AlarmsStore { alarms })?;
        }
    }

    if let Some(preferences) = bundle.preferences {
        if mode == "merge" {
            let existing = read_store_file::<PreferencesStore>(base_dir, PREFERENCES_STORE_NAME)?
                .map(|store| store.preferences);
            let (merged, updated, conflicts, warnings) = merge_preferences(existing, preferences);
            result.preferences_updated = updated;
            result.conflicts_detected += conflicts;
            result.warnings.extend(warnings);
            write_store_file(
                base_dir,
                PREFERENCES_STORE_NAME,
                &PreferencesStore {
                    preferences: merged,
                },
            )?;
        } else {
            result.preferences_updated = true;
            write_store_file(
                base_dir,
                PREFERENCES_STORE_NAME,
                &PreferencesStore { preferences },
            )?;
        }
    }

    Ok(result)
}

fn import_bundle_from_path(file_path: &str) -> Result<ExportBundle, String> {
    let content =
        fs::read_to_string(file_path).map_err(|e| format!("Unable to read import file: {e}"))?;
    serde_json::from_str(&content)
        .map_err(|_| "The selected file is not a valid Delta-T Zaman export.".to_string())
}

#[tauri::command]
pub fn pick_export_destination(
    app: AppHandle,
    default_file_name: String,
) -> Result<Option<String>, String> {
    Ok(app
        .dialog()
        .file()
        .set_title("Export Delta-T Zaman Data")
        .add_filter("JSON", &["json"])
        .set_file_name(default_file_name)
        .blocking_save_file()
        .map(|path| path.to_string()))
}

#[tauri::command]
pub fn pick_import_source(app: AppHandle) -> Result<Option<String>, String> {
    Ok(app
        .dialog()
        .file()
        .set_title("Import Delta-T Zaman Data")
        .add_filter("JSON", &["json"])
        .blocking_pick_file()
        .map(|path| path.to_string()))
}

#[tauri::command]
pub fn pick_import_mode(app: AppHandle) -> Result<Option<String>, String> {
    let result = app
        .dialog()
        .message("How should imported data be applied?")
        .kind(MessageDialogKind::Info)
        .buttons(MessageDialogButtons::YesNoCancelCustom(
            "Replace".to_string(),
            "Merge".to_string(),
            "Cancel".to_string(),
        ))
        .blocking_show_with_result();

    let mode = match result {
        MessageDialogResult::Yes => Some("replace".to_string()),
        MessageDialogResult::No => Some("merge".to_string()),
        MessageDialogResult::Custom(choice) if choice == "Replace" => Some("replace".to_string()),
        MessageDialogResult::Custom(choice) if choice == "Merge" => Some("merge".to_string()),
        _ => None,
    };

    Ok(mode)
}

#[tauri::command]
pub async fn export_user_data(app: AppHandle, file_path: String) -> Result<ExportResult, String> {
    let app_data = app
        .path()
        .app_data_dir()
        .map_err(|_| "No app data dir".to_string())?;
    let bundle = build_export_bundle_from_dir(&app_data)?;
    export_bundle_to_path(&bundle, &file_path)
}

#[tauri::command]
pub async fn import_user_data(
    app: AppHandle,
    config: ImportConfig,
) -> Result<ImportResult, String> {
    let bundle = import_bundle_from_path(&config.file_path)?;
    let mode = if config.mode == "merge" { "merge" } else { "replace" };
    let app_data = app
        .path()
        .app_data_dir()
        .map_err(|_| "No app data dir".to_string())?;

    apply_import_bundle_to_dir(&app_data, bundle, mode)
}

#[cfg(test)]
mod tests {
    use super::*;
    use delta_t_zaman_shared::{AlarmState, Recurrence};

    fn temp_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "delta-t-zaman-tests-{}-{}",
            name,
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn sample_zone(id: &str, iana_id: &str) -> TimeZoneEntry {
        TimeZoneEntry {
            id: id.into(),
            iana_id: iana_id.into(),
            display_label: iana_id.into(),
            sort_order: 0,
            pinned_to_tray: false,
            created_at: 0,
        }
    }

    fn sample_alarm(id: &str) -> Alarm {
        Alarm {
            id: id.into(),
            title: "Wake".into(),
            target_time: "08:00".into(),
            target_date: None,
            timezone_id: "UTC".into(),
            recurrence: Recurrence::Daily,
            custom_days: vec![],
            enabled: true,
            state: AlarmState::Active,
            snooze_count: 0,
            default_snooze_minutes: 10,
            snooze_until: None,
            sound_enabled: true,
            last_triggered_at: None,
            created_at: 0,
        }
    }

    #[test]
    fn write_store_replace_creates_typed_payload() {
        let dir = temp_dir("replace");

        write_store_file(
            &dir,
            ZONES_STORE_NAME,
            &ZonesStore {
                zones: vec![sample_zone("utc", "UTC")],
            },
        )
        .unwrap();

        let stored = read_store_file::<ZonesStore>(&dir, ZONES_STORE_NAME).unwrap().unwrap();
        assert_eq!(stored.zones, vec![sample_zone("utc", "UTC")]);

        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn merge_import_skips_conflicting_items_and_reports_warnings() {
        let dir = temp_dir("merge-import");
        write_store_file(
            &dir,
            ZONES_STORE_NAME,
            &ZonesStore {
                zones: vec![sample_zone("utc-1", "UTC")],
            },
        )
        .unwrap();
        write_store_file(
            &dir,
            ALARMS_STORE_NAME,
            &AlarmsStore {
                alarms: vec![sample_alarm("alarm-1")],
            },
        )
        .unwrap();
        write_store_file(
            &dir,
            PREFERENCES_STORE_NAME,
            &PreferencesStore {
                preferences: UserPreferences {
                    theme: "dark".into(),
                    ..UserPreferences::default()
                },
            },
        )
        .unwrap();

        let result = apply_import_bundle_to_dir(
            &dir,
            ExportBundle {
                version: "1.0.0".into(),
                exported_at: 0,
                zones: Some(vec![
                    sample_zone("utc-1", "UTC"),
                    sample_zone("paris-1", "Europe/Paris"),
                ]),
                alarms: Some(vec![sample_alarm("alarm-1"), sample_alarm("alarm-2")]),
                preferences: Some(UserPreferences {
                    theme: "light".into(),
                    ..UserPreferences::default()
                }),
            },
            "merge",
        )
        .unwrap();

        assert_eq!(result.zones_imported, 1);
        assert_eq!(result.alarms_imported, 1);
        assert!(!result.preferences_updated);
        assert_eq!(result.conflicts_detected, 3);
        assert_eq!(result.warnings.len(), 3);

        let zones = read_store_file::<ZonesStore>(&dir, ZONES_STORE_NAME).unwrap().unwrap();
        assert_eq!(zones.zones.len(), 2);

        let alarms = read_store_file::<AlarmsStore>(&dir, ALARMS_STORE_NAME).unwrap().unwrap();
        assert_eq!(alarms.alarms.len(), 2);

        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn export_and_import_round_trip_restores_typed_saved_data() {
        let source_dir = temp_dir("roundtrip-source");
        let target_dir = temp_dir("roundtrip-target");
        let export_path = source_dir.join("backup.json");

        write_store_file(
            &source_dir,
            ZONES_STORE_NAME,
            &ZonesStore {
                zones: vec![sample_zone("utc-1", "UTC")],
            },
        )
        .unwrap();
        write_store_file(
            &source_dir,
            ALARMS_STORE_NAME,
            &AlarmsStore {
                alarms: vec![sample_alarm("alarm-1")],
            },
        )
        .unwrap();
        write_store_file(
            &source_dir,
            PREFERENCES_STORE_NAME,
            &PreferencesStore {
                preferences: UserPreferences {
                    theme: "dark".into(),
                    ..UserPreferences::default()
                },
            },
        )
        .unwrap();

        let bundle = build_export_bundle_from_dir(&source_dir).unwrap();
        let export_result = export_bundle_to_path(&bundle, export_path.to_str().unwrap()).unwrap();
        assert!(export_result.bytes_written > 0);

        let imported_bundle = import_bundle_from_path(export_path.to_str().unwrap()).unwrap();
        let import_result =
            apply_import_bundle_to_dir(&target_dir, imported_bundle, "replace").unwrap();

        assert_eq!(import_result.zones_imported, 1);
        assert_eq!(import_result.alarms_imported, 1);
        assert!(import_result.preferences_updated);

        let restored_prefs = read_store_file::<PreferencesStore>(&target_dir, PREFERENCES_STORE_NAME)
            .unwrap()
            .unwrap();
        assert_eq!(restored_prefs.preferences.theme, "dark");

        let _ = fs::remove_dir_all(source_dir);
        let _ = fs::remove_dir_all(target_dir);
    }

    #[test]
    fn invalid_import_file_reports_friendly_error() {
        let dir = temp_dir("invalid-import");
        let bad_file = dir.join("bad.json");
        fs::write(&bad_file, "{ this is not valid json").unwrap();

        let error = import_bundle_from_path(bad_file.to_str().unwrap()).unwrap_err();
        assert_eq!(error, "The selected file is not a valid Delta-T Zaman export.");

        let _ = fs::remove_dir_all(dir);
    }
}
