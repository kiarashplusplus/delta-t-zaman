use leptos::*;
use crate::models::{AlarmAttention, TimeZoneEntry};
use chrono::Utc;

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

    provide_context(ClockState { zones });
}

#[derive(Clone, Copy)]
pub struct UserPreferencesState {
    pub prefs: RwSignal<crate::models::UserPreferences>,
}

pub fn provide_user_preferences() {
    let prefs = create_rw_signal(crate::models::UserPreferences::default());

    provide_context(UserPreferencesState { prefs });
}

#[derive(Clone, Copy)]
pub struct AlarmState {
    pub alarms: RwSignal<Vec<crate::models::Alarm>>,
}

#[derive(Clone, Copy)]
pub struct AlarmAttentionState {
    pub attention: RwSignal<AlarmAttention>,
}

pub fn provide_alarm_state() {
    let alarms = create_rw_signal(Vec::new());
    let attention = create_rw_signal(AlarmAttention::default());

    #[cfg(target_arch = "wasm32")]
    {
        let alarm_state = AlarmState { alarms };
        let attention_state = AlarmAttentionState { attention };
        crate::ipc::listen_for_event("alarms-updated", move || {
            let alarm_state = alarm_state;
            let attention_state = attention_state;
            spawn_local(async move {
                let _ = crate::client::reload_alarm_state(alarm_state, attention_state).await;
            });
        });

        let attention_signal = attention;
        crate::ipc::listen_for_alarm_attention(move |payload| {
            attention_signal.set(payload);
        });
    }

    provide_context(AlarmState { alarms });
    provide_context(AlarmAttentionState { attention });
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
