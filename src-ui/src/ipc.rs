#![allow(dead_code)]

use serde::Serialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::models::*;
use delta_t_zaman_shared::{
    AlarmActionArgs, AlarmActionResult, AlarmAttention, AlarmNotificationConfig, AlarmsStore,
    EnabledArg, ImportConfig, PreferencesStore, ZonesStore,
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "event"])]
    async fn listen(event: &str, handler: &Closure<dyn FnMut(JsValue)>) -> Result<JsValue, JsValue>;
}

pub async fn get_system_info() -> Result<SystemInfo, JsValue> {
    let res = invoke("get_system_info", JsValue::NULL).await?;
    serde_wasm_bindgen::from_value(res).map_err(|e| e.into())
}

#[derive(Serialize)]
struct ScheduleAlarmNotificationArgs {
    config: AlarmNotificationConfig,
}

#[derive(Serialize)]
struct PerformAlarmActionArgs {
    payload: AlarmActionArgs,
}

#[derive(Serialize)]
struct ExportUserDataArgs {
    file_path: String,
}

#[derive(Serialize)]
struct PickExportDestinationArgs {
    default_file_name: String,
}

#[derive(Serialize)]
struct ImportUserDataArgs {
    config: ImportConfig,
}

#[derive(Serialize)]
struct SaveZonesStoreArgs {
    store: ZonesStore,
}

#[derive(Serialize)]
struct SaveAlarmsStoreArgs {
    store: AlarmsStore,
}

#[derive(Serialize)]
struct SavePreferencesStoreArgs {
    store: PreferencesStore,
}

pub async fn set_always_on_top(enabled: bool) -> Result<(), JsValue> {
    let args = serde_wasm_bindgen::to_value(&EnabledArg { enabled }).unwrap();
    invoke("set_always_on_top", args).await?;
    Ok(())
}

pub async fn set_autostart(enabled: bool) -> Result<(), JsValue> {
    let args = serde_wasm_bindgen::to_value(&EnabledArg { enabled }).unwrap();
    invoke("set_autostart", args).await?;
    Ok(())
}

pub async fn get_notification_permission() -> Result<String, JsValue> {
    let res = invoke("get_notification_permission", JsValue::NULL).await?;
    serde_wasm_bindgen::from_value(res).map_err(|e| e.into())
}

pub async fn request_notification_permission() -> Result<String, JsValue> {
    let res = invoke("request_notification_permission", JsValue::NULL).await?;
    serde_wasm_bindgen::from_value(res).map_err(|e| e.into())
}

pub async fn schedule_alarm_notification(
    alarm_id: String,
    title: String,
    body: String,
    schedule_at: String,
    sound_enabled: bool,
    action_type: String,
) -> Result<ScheduleResult, JsValue> {
    let args = serde_wasm_bindgen::to_value(&ScheduleAlarmNotificationArgs {
        config: AlarmNotificationConfig {
            alarm_id,
            title,
            body,
            schedule_at,
            sound_enabled,
            action_type,
        },
    })
    .unwrap();
    let res = invoke("schedule_alarm_notification", args).await?;
    serde_wasm_bindgen::from_value(res).map_err(|e| e.into())
}

pub async fn perform_alarm_action(payload: AlarmActionArgs) -> Result<AlarmActionResult, JsValue> {
    let args = serde_wasm_bindgen::to_value(&PerformAlarmActionArgs { payload }).unwrap();
    let res = invoke("perform_alarm_action", args).await?;
    serde_wasm_bindgen::from_value(res).map_err(|e| e.into())
}

pub async fn export_user_data(file_path: String) -> Result<ExportResult, JsValue> {
    let args = serde_wasm_bindgen::to_value(&ExportUserDataArgs { file_path }).unwrap();
    let res = invoke("export_user_data", args).await?;
    serde_wasm_bindgen::from_value(res).map_err(|e| e.into())
}

pub async fn pick_export_destination(default_file_name: String) -> Result<Option<String>, JsValue> {
    let args = serde_wasm_bindgen::to_value(&PickExportDestinationArgs { default_file_name }).unwrap();
    let res = invoke("pick_export_destination", args).await?;
    serde_wasm_bindgen::from_value(res).map_err(|e| e.into())
}

pub async fn pick_import_source() -> Result<Option<String>, JsValue> {
    let res = invoke("pick_import_source", JsValue::NULL).await?;
    serde_wasm_bindgen::from_value(res).map_err(|e| e.into())
}

pub async fn pick_import_mode() -> Result<Option<String>, JsValue> {
    let res = invoke("pick_import_mode", JsValue::NULL).await?;
    serde_wasm_bindgen::from_value(res).map_err(|e| e.into())
}

pub async fn import_user_data(file_path: String, mode: String) -> Result<ImportResult, JsValue> {
    let args = serde_wasm_bindgen::to_value(&ImportUserDataArgs {
        config: ImportConfig { file_path, mode },
    })
    .unwrap();
    let res = invoke("import_user_data", args).await?;
    serde_wasm_bindgen::from_value(res).map_err(|e| e.into())
}

pub async fn load_zones_store() -> Result<ZonesStore, JsValue> {
    let res = invoke("load_zones_store", JsValue::NULL).await?;
    serde_wasm_bindgen::from_value(res).map_err(|e| e.into())
}

pub async fn save_zones_store(store: ZonesStore) -> Result<(), JsValue> {
    let args = serde_wasm_bindgen::to_value(&SaveZonesStoreArgs { store }).unwrap();
    invoke("save_zones_store", args).await?;
    Ok(())
}

pub async fn load_alarms_store() -> Result<AlarmsStore, JsValue> {
    let res = invoke("load_alarms_store", JsValue::NULL).await?;
    serde_wasm_bindgen::from_value(res).map_err(|e| e.into())
}

pub async fn save_alarms_store(store: AlarmsStore) -> Result<(), JsValue> {
    let args = serde_wasm_bindgen::to_value(&SaveAlarmsStoreArgs { store }).unwrap();
    invoke("save_alarms_store", args).await?;
    Ok(())
}

pub async fn load_preferences_store() -> Result<PreferencesStore, JsValue> {
    let res = invoke("load_preferences_store", JsValue::NULL).await?;
    serde_wasm_bindgen::from_value(res).map_err(|e| e.into())
}

pub async fn save_preferences_store(store: PreferencesStore) -> Result<(), JsValue> {
    let args = serde_wasm_bindgen::to_value(&SavePreferencesStoreArgs { store }).unwrap();
    invoke("save_preferences_store", args).await?;
    Ok(())
}

pub async fn load_zones() -> Option<Vec<TimeZoneEntry>> {
    load_zones_store().await.ok().map(|store| store.zones)
}

pub async fn save_zones(zones: Vec<TimeZoneEntry>) {
    let _ = save_zones_store(ZonesStore { zones }).await;
}

pub async fn load_alarms() -> Option<Vec<Alarm>> {
    load_alarms_store().await.ok().map(|store| store.alarms)
}

pub async fn save_alarms(alarms: Vec<Alarm>) {
    let _ = save_alarms_store(AlarmsStore { alarms }).await;
}

pub async fn load_preferences() -> Option<UserPreferences> {
    load_preferences_store()
        .await
        .ok()
        .map(|store| store.preferences)
}

pub async fn save_preferences(preferences: UserPreferences) {
    let _ = save_preferences_store(PreferencesStore { preferences }).await;
}

pub fn listen_for_event(event: &'static str, mut on_event: impl FnMut() + 'static) {
    let handler = Closure::wrap(Box::new(move |_payload: JsValue| {
        on_event();
    }) as Box<dyn FnMut(JsValue)>);

    spawn_local(async move {
        let _ = listen(event, &handler).await;
        handler.forget();
    });
}

pub fn listen_for_alarm_attention(mut on_event: impl FnMut(AlarmAttention) + 'static) {
    let handler = Closure::wrap(Box::new(move |payload: JsValue| {
        if let Ok(attention) = serde_wasm_bindgen::from_value::<AlarmAttention>(payload) {
            on_event(attention);
        }
    }) as Box<dyn FnMut(JsValue)>);

    spawn_local(async move {
        let _ = listen("alarm-attention", &handler).await;
        handler.forget();
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enabled_arg_matches_backend_field_name() {
        let json = serde_json::to_value(EnabledArg { enabled: true }).unwrap();
        assert_eq!(json, serde_json::json!({ "enabled": true }));
    }

    #[test]
    fn alarm_payload_contract_serializes_expected_shape() {
        let payload = PerformAlarmActionArgs {
            payload: AlarmActionArgs {
                alarm_id: "alarm-1".into(),
                action: delta_t_zaman_shared::AlarmAction::Snooze,
                snooze_minutes: Some(10),
            },
        };

        let json = serde_json::to_value(payload).unwrap();
        assert_eq!(
            json,
            serde_json::json!({
                "payload": {
                    "alarm_id": "alarm-1",
                    "action": "snooze",
                    "snooze_minutes": 10
                }
            })
        );
    }

    #[test]
    fn system_info_contract_defaults_missing_fields() {
        let info: SystemInfo = serde_json::from_value(serde_json::json!({
            "platform": "linux",
            "timezone": "UTC"
        }))
        .unwrap();

        assert_eq!(info.platform, "linux");
        assert_eq!(info.timezone, "UTC");
        assert!(info.arch.is_empty());
        assert!(!info.supports_tray);
    }
}
