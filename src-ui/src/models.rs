#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct WindowState {
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub maximized: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TimeZoneEntry {
    pub id: String,
    pub iana_id: String,
    pub display_label: String,
    pub sort_order: usize,
    pub pinned_to_tray: bool,
    pub created_at: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AlarmState {
    Active,
    Snoozed,
    Completed,
    Dismissed,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Recurrence {
    Once,
    Daily,
    Weekdays,
    Weekends,
    Custom,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Alarm {
    pub id: String,
    pub title: String,
    pub target_time: String,
    pub timezone_id: String,
    pub recurrence: Recurrence,
    pub custom_days: Vec<usize>,
    pub enabled: bool,
    pub state: AlarmState,
    pub snooze_count: usize,
    pub sound_enabled: bool,
    pub created_at: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TimezoneMetadata {
    pub id: String,
    pub city: Option<String>,
    pub country: Option<String>,
    pub country_code: Option<String>,
    pub utc_offset_minutes: i32,
    pub latitude: f64,
    pub longitude: f64,
    pub aliases: Vec<String>,
}
