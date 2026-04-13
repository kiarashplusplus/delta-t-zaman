use serde::{Deserialize, Serialize};
fn default_snooze_minutes() -> u32 {
    10
}

pub const ZONES_STORE_NAME: &str = "zones.dat";
pub const ZONES_STORE_KEY: &str = "zones";
pub const ALARMS_STORE_NAME: &str = "alarms.dat";
pub const ALARMS_STORE_KEY: &str = "alarms";
pub const PREFERENCES_STORE_NAME: &str = "preferences.dat";
pub const PREFERENCES_STORE_KEY: &str = "preferences";

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct SystemInfo {
    pub platform: String,
    pub arch: String,
    pub locale: String,
    pub timezone: String,
    pub app_version: String,
    pub is_mobile: bool,
    pub supports_tray: bool,
    pub os_version: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct EnabledArg {
    pub enabled: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AlarmNotificationConfig {
    pub alarm_id: String,
    pub title: String,
    pub body: String,
    pub schedule_at: String,
    pub sound_enabled: bool,
    pub action_type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AlarmAction {
    Snooze,
    Dismiss,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AlarmActionArgs {
    pub alarm_id: String,
    pub action: AlarmAction,
    pub snooze_minutes: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AlarmActionResult {
    pub alarm_id: String,
    pub state: String,
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct AlarmAttention {
    pub triggered_alarm_ids: Vec<String>,
    pub snoozed_alarm_ids: Vec<String>,
}

impl AlarmAttention {
    pub fn from_alarms(alarms: &[Alarm]) -> Self {
        let mut attention = Self::default();
        for alarm in alarms {
            match alarm.state {
                AlarmState::Triggered => attention.triggered_alarm_ids.push(alarm.id.clone()),
                AlarmState::Snoozed => attention.snoozed_alarm_ids.push(alarm.id.clone()),
                _ => {}
            }
        }
        attention
    }

    pub fn triggered_count(&self) -> usize {
        self.triggered_alarm_ids.len()
    }

    pub fn snoozed_count(&self) -> usize {
        self.snoozed_alarm_ids.len()
    }

    pub fn attention_count(&self) -> usize {
        self.triggered_count() + self.snoozed_count()
    }

    pub fn has_attention(&self) -> bool {
        self.attention_count() > 0
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct WindowState {
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub maximized: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserPreferences {
    pub time_format: String,
    pub locale: String,
    pub theme: String,
    pub high_contrast: String,
    pub default_timezone: String,
    pub always_on_top: bool,
    pub autostart: bool,
    pub telemetry_opt_in: Option<bool>,
    pub auto_update: bool,
    pub window_state: Option<WindowState>,
    pub compact_mode: bool,
    pub compact_window_state: Option<WindowState>,
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            time_format: "12h".into(),
            locale: "en-US".into(),
            theme: "system".into(),
            high_contrast: "system".into(),
            default_timezone: "UTC".into(),
            always_on_top: false,
            autostart: false,
            telemetry_opt_in: None,
            auto_update: true,
            window_state: None,
            compact_mode: false,
            compact_window_state: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TimeZoneEntry {
    pub id: String,
    pub iana_id: String,
    pub display_label: String,
    pub sort_order: usize,
    pub pinned_to_tray: bool,
    pub created_at: u64,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct ZonesStore {
    pub zones: Vec<TimeZoneEntry>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AlarmState {
    Active,
    Triggered,
    Snoozed,
    Completed,
    Dismissed,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Recurrence {
    Once,
    Daily,
    Weekdays,
    Weekends,
    Custom,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Alarm {
    pub id: String,
    pub title: String,
    pub target_time: String,
    #[serde(default)]
    pub target_date: Option<String>,
    pub timezone_id: String,
    pub recurrence: Recurrence,
    #[serde(default)]
    pub custom_days: Vec<usize>,
    pub enabled: bool,
    pub state: AlarmState,
    #[serde(default)]
    pub snooze_count: usize,
    #[serde(default = "default_snooze_minutes")]
    pub default_snooze_minutes: u32,
    #[serde(default)]
    pub snooze_until: Option<u64>,
    pub sound_enabled: bool,
    #[serde(default)]
    pub last_triggered_at: Option<u64>,
    pub created_at: u64,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct AlarmsStore {
    pub alarms: Vec<Alarm>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ScheduleResult {
    pub notification_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExportResult {
    pub success: bool,
    pub file_path: String,
    pub bytes_written: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImportConfig {
    pub file_path: String,
    pub mode: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct ImportResult {
    pub success: bool,
    pub zones_imported: u32,
    pub alarms_imported: u32,
    pub preferences_updated: bool,
    pub conflicts_detected: u32,
    pub warnings: Vec<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct PreferencesStore {
    pub preferences: UserPreferences,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExportBundle {
    pub version: String,
    pub exported_at: u64,
    pub zones: Option<Vec<TimeZoneEntry>>,
    pub alarms: Option<Vec<Alarm>>,
    pub preferences: Option<UserPreferences>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alarm_defaults_make_old_payloads_backward_compatible() {
        let alarm: Alarm = serde_json::from_value(serde_json::json!({
            "id": "alarm-1",
            "title": "Wake",
            "target_time": "08:00",
            "timezone_id": "UTC",
            "recurrence": "daily",
            "enabled": true,
            "state": "active",
            "sound_enabled": true,
            "created_at": 1
        }))
        .unwrap();

        assert_eq!(alarm.custom_days, Vec::<usize>::new());
        assert_eq!(alarm.default_snooze_minutes, 10);
        assert_eq!(alarm.snooze_count, 0);
        assert_eq!(alarm.snooze_until, None);
        assert_eq!(alarm.last_triggered_at, None);
    }

    #[test]
    fn attention_is_derived_from_triggered_and_snoozed_alarms() {
        let alarms = vec![
            Alarm {
                id: "a1".into(),
                title: "One".into(),
                target_time: "08:00".into(),
                target_date: None,
                timezone_id: "UTC".into(),
                recurrence: Recurrence::Daily,
                custom_days: vec![],
                enabled: true,
                state: AlarmState::Triggered,
                snooze_count: 0,
                default_snooze_minutes: 10,
                snooze_until: None,
                sound_enabled: true,
                last_triggered_at: None,
                created_at: 0,
            },
            Alarm {
                id: "a2".into(),
                title: "Two".into(),
                target_time: "08:30".into(),
                target_date: None,
                timezone_id: "UTC".into(),
                recurrence: Recurrence::Daily,
                custom_days: vec![],
                enabled: true,
                state: AlarmState::Snoozed,
                snooze_count: 1,
                default_snooze_minutes: 10,
                snooze_until: Some(10),
                sound_enabled: true,
                last_triggered_at: None,
                created_at: 0,
            },
        ];

        let attention = AlarmAttention::from_alarms(&alarms);
        assert_eq!(attention.triggered_alarm_ids, vec!["a1"]);
        assert_eq!(attention.snoozed_alarm_ids, vec!["a2"]);
        assert_eq!(attention.attention_count(), 2);
        assert!(attention.has_attention());
    }

    #[test]
    fn user_preferences_default_matches_expected_store_defaults() {
        let prefs = UserPreferences::default();
        assert_eq!(prefs.theme, "system");
        assert_eq!(prefs.default_timezone, "UTC");
        assert!(prefs.auto_update);
        assert!(!prefs.always_on_top);
    }

    #[test]
    fn export_bundle_round_trips_typed_data() {
        let bundle = ExportBundle {
            version: "1.0.0".into(),
            exported_at: 42,
            zones: Some(vec![TimeZoneEntry {
                id: "utc-1".into(),
                iana_id: "UTC".into(),
                display_label: "UTC".into(),
                sort_order: 0,
                pinned_to_tray: false,
                created_at: 1,
            }]),
            alarms: Some(vec![Alarm {
                id: "alarm-1".into(),
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
                created_at: 1,
            }]),
            preferences: Some(UserPreferences::default()),
        };

        let json = serde_json::to_value(&bundle).unwrap();
        let reparsed: ExportBundle = serde_json::from_value(json).unwrap();
        assert_eq!(reparsed, bundle);
    }
}
