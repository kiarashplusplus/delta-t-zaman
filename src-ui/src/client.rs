use crate::models::{Alarm, ExportResult, ImportResult, SystemInfo, UserPreferences};
use crate::state::{AlarmAttentionState, AlarmState, ClockState, UserPreferencesState};
use delta_t_zaman_shared::{AlarmActionArgs, AlarmActionResult};
use leptos::{SignalGetUntracked, SignalSet};
use wasm_bindgen::JsValue;

#[derive(Clone, Debug, PartialEq)]
pub enum PickerFlow<T> {
    Completed(T),
    Cancelled { stage: &'static str },
}

pub fn format_js_error(error: JsValue) -> String {
    error
        .as_string()
        .unwrap_or_else(|| format!("{error:?}"))
}

#[cfg(test)]
pub fn cancelled_flow_message(action: &str, stage: &'static str) -> String {
    format!("{action} cancelled before {stage}.")
}

pub async fn hydrate_app_state(
    clock_state: ClockState,
    prefs_state: UserPreferencesState,
    alarm_state: AlarmState,
    attention_state: AlarmAttentionState,
) {
    crate::store::refresh_snapshot_into_states(clock_state, prefs_state, alarm_state, attention_state)
        .await;
}

pub async fn reload_alarm_state(
    alarm_state: AlarmState,
    attention_state: AlarmAttentionState,
) -> Vec<Alarm> {
    let alarms = crate::store::load_alarms_or_default().await;
    crate::store::apply_alarms(alarm_state, attention_state, alarms.clone());
    alarms
}

pub async fn perform_alarm_action_and_refresh(
    alarm_state: AlarmState,
    attention_state: AlarmAttentionState,
    payload: AlarmActionArgs,
) -> Result<AlarmActionResult, JsValue> {
    let result = crate::ipc::perform_alarm_action(payload).await?;
    reload_alarm_state(alarm_state, attention_state).await;
    Ok(result)
}

pub async fn ensure_notification_permission() -> Result<String, JsValue> {
    let permission = crate::ipc::get_notification_permission()
        .await
        .unwrap_or_else(|_| "default".to_string());
    if permission != "granted" {
        crate::ipc::request_notification_permission().await
    } else {
        Ok(permission)
    }
}

pub async fn load_system_info() -> Result<SystemInfo, JsValue> {
    crate::ipc::get_system_info().await
}

pub async fn set_always_on_top_preference(
    prefs_state: UserPreferencesState,
    enabled: bool,
) -> Result<(), JsValue> {
    let previous_prefs = prefs_state.prefs.get_untracked();
    let mut next_prefs = previous_prefs.clone();
    next_prefs.always_on_top = enabled;
    prefs_state.prefs.set(next_prefs.clone());

    match crate::ipc::set_always_on_top(enabled).await {
        Ok(_) => {
            crate::store::save_preferences(next_prefs).await;
            Ok(())
        }
        Err(error) => {
            prefs_state.prefs.set(previous_prefs);
            Err(error)
        }
    }
}

pub async fn set_autostart_preference(
    prefs_state: UserPreferencesState,
    enabled: bool,
) -> Result<(), JsValue> {
    let previous_prefs = prefs_state.prefs.get_untracked();
    let mut next_prefs = previous_prefs.clone();
    next_prefs.autostart = enabled;
    prefs_state.prefs.set(next_prefs.clone());

    match crate::ipc::set_autostart(enabled).await {
        Ok(_) => {
            crate::store::save_preferences(next_prefs).await;
            Ok(())
        }
        Err(error) => {
            prefs_state.prefs.set(previous_prefs);
            Err(error)
        }
    }
}

pub async fn export_with_picker(
    default_file_name: String,
) -> Result<PickerFlow<ExportResult>, JsValue> {
    match crate::ipc::pick_export_destination(default_file_name).await? {
        Some(file_path) => crate::ipc::export_user_data(file_path)
            .await
            .map(PickerFlow::Completed),
        None => Ok(PickerFlow::Cancelled {
            stage: "a file was chosen",
        }),
    }
}

pub async fn import_with_picker_and_refresh(
    clock_state: ClockState,
    prefs_state: UserPreferencesState,
    alarm_state: AlarmState,
    attention_state: AlarmAttentionState,
) -> Result<PickerFlow<ImportResult>, JsValue> {
    let Some(file_path) = crate::ipc::pick_import_source().await? else {
        return Ok(PickerFlow::Cancelled {
            stage: "a file was chosen",
        });
    };

    let Some(mode) = crate::ipc::pick_import_mode().await? else {
        return Ok(PickerFlow::Cancelled {
            stage: "a mode was selected",
        });
    };

    let result = crate::ipc::import_user_data(file_path, mode).await?;
    hydrate_app_state(clock_state, prefs_state, alarm_state, attention_state).await;
    Ok(PickerFlow::Completed(result))
}

pub async fn persist_preferences(prefs: UserPreferences) {
    crate::store::save_preferences(prefs).await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cancelled_flow_message_mentions_action_and_stage() {
        assert_eq!(
            cancelled_flow_message("Import", "a mode was selected"),
            "Import cancelled before a mode was selected."
        );
    }

    #[test]
    fn picker_flow_cancelled_variant_is_distinct() {
        let cancelled = PickerFlow::<ImportResult>::Cancelled {
            stage: "a file was chosen",
        };
        assert!(matches!(
            cancelled,
            PickerFlow::Cancelled {
                stage: "a file was chosen"
            }
        ));
    }

    #[test]
    fn cancelled_flow_message_stays_human_readable() {
        let message = cancelled_flow_message("Export", "a file was chosen");
        assert!(message.contains("Export"));
        assert!(message.contains("a file was chosen"));
    }
}
