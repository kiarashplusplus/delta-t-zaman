#![allow(dead_code)]

use serde::{Deserialize, Serialize};

pub use delta_t_zaman_shared::{
    Alarm, AlarmAttention, AlarmState, ExportResult, ImportResult, Recurrence, ScheduleResult,
    SystemInfo, TimeZoneEntry, UserPreferences,
};

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
