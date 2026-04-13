use leptos::*;
use chrono::Utc;
use crate::models::TimeZoneEntry;

#[derive(Clone, Copy)]
pub struct GlobalTime(pub RwSignal<i64>);

pub fn provide_global_time() {
    let time = create_rw_signal(Utc::now().timestamp_millis());

    // Use interval to update every second
    #[cfg(target_arch = "wasm32")]
    {
        use leptos::set_interval_with_handle;
        use std::time::Duration;
        set_interval_with_handle(
            move || {
                time.set(Utc::now().timestamp_millis());
            },
            Duration::from_millis(1000),
        ).expect("Failed to set interval");
    }

    provide_context(GlobalTime(time));
}

#[derive(Clone, Copy)]
pub struct ClockState {
    pub zones: RwSignal<Vec<TimeZoneEntry>>,
}

pub fn provide_clock_state() {
    let zones = create_rw_signal(Vec::new());

    spawn_local(async move {
        if let Some(saved) = crate::ipc::get_store_value::<Vec<TimeZoneEntry>>("zones.dat", "zones").await {
            zones.set(saved);
        } else {
            // Default fallback
            let default_zone = TimeZoneEntry {
                id: "utc-1".into(),
                iana_id: "UTC".into(),
                display_label: "Universal Time".into(),
                sort_order: 0,
                pinned_to_tray: false,
                created_at: Utc::now().timestamp_millis() as u64,
            };
            zones.set(vec![default_zone.clone()]);
            crate::ipc::set_store_value("zones.dat", "zones", vec![default_zone]).await;
        }
    });

    provide_context(ClockState { zones });
}

#[derive(Clone, Copy)]
pub struct UserPreferencesState {
    pub prefs: RwSignal<crate::models::UserPreferences>,
}

pub fn provide_user_preferences() {
    let prefs = create_rw_signal(crate::models::UserPreferences::default());

    spawn_local(async move {
        if let Some(saved) = crate::ipc::get_store_value::<crate::models::UserPreferences>("preferences.dat", "preferences").await {
            prefs.set(saved);
        }
    });

    provide_context(UserPreferencesState { prefs });
}

#[derive(Clone, Copy)]
pub struct AlarmState {
    pub alarms: RwSignal<Vec<crate::models::Alarm>>,
}

pub fn provide_alarm_state() {
    let alarms = create_rw_signal(Vec::new());

    spawn_local(async move {
        if let Some(saved) = crate::ipc::get_store_value::<Vec<crate::models::Alarm>>("alarms.dat", "alarms").await {
            alarms.set(saved);
        }
    });

    provide_context(AlarmState { alarms });
}

#[derive(Clone, Copy)]
pub struct ModalState {
    pub show_timezone_search: RwSignal<bool>,
    pub show_alarm_form: RwSignal<bool>,
    pub editing_alarm: RwSignal<Option<crate::models::Alarm>>,
}

pub fn provide_modal_state() {
    let show_timezone_search = create_rw_signal(false);
    let show_alarm_form = create_rw_signal(false);
    let editing_alarm = create_rw_signal(None);
    provide_context(ModalState { show_timezone_search, show_alarm_form, editing_alarm });
}
