use crate::models::{Alarm, AlarmAttention, TimeZoneEntry, UserPreferences};
use crate::state::{AlarmAttentionState, AlarmState, ClockState, UserPreferencesState};
use chrono::Utc;
use leptos::SignalSet;

#[derive(Clone, Debug, PartialEq)]
pub struct AppStoreSnapshot {
    pub zones: Vec<TimeZoneEntry>,
    pub prefs: UserPreferences,
    pub alarms: Vec<Alarm>,
    pub attention: AlarmAttention,
}

pub fn default_zone_entry(created_at: u64) -> TimeZoneEntry {
    TimeZoneEntry {
        id: "utc-1".into(),
        iana_id: "UTC".into(),
        display_label: "Universal Time".into(),
        sort_order: 0,
        pinned_to_tray: false,
        created_at,
    }
}

pub fn attention_from_alarms(alarms: &[Alarm]) -> AlarmAttention {
    AlarmAttention::from_alarms(alarms)
}

pub fn snapshot_from_parts(
    zones: Option<Vec<TimeZoneEntry>>,
    prefs: Option<UserPreferences>,
    alarms: Option<Vec<Alarm>>,
    now_ms: u64,
) -> AppStoreSnapshot {
    let zones = zones.unwrap_or_else(|| vec![default_zone_entry(now_ms)]);
    let prefs = prefs.unwrap_or_default();
    let alarms = alarms.unwrap_or_default();
    let attention = attention_from_alarms(&alarms);

    AppStoreSnapshot {
        zones,
        prefs,
        alarms,
        attention,
    }
}

pub fn apply_zones(clock_state: ClockState, zones: Vec<TimeZoneEntry>) {
    clock_state.zones.set(zones);
}

pub fn apply_preferences(prefs_state: UserPreferencesState, prefs: UserPreferences) {
    prefs_state.prefs.set(prefs);
}

pub fn apply_alarms(
    alarm_state: AlarmState,
    attention_state: AlarmAttentionState,
    alarms: Vec<Alarm>,
) {
    attention_state.attention.set(attention_from_alarms(&alarms));
    alarm_state.alarms.set(alarms);
}

pub fn apply_snapshot(
    snapshot: AppStoreSnapshot,
    clock_state: ClockState,
    prefs_state: UserPreferencesState,
    alarm_state: AlarmState,
    attention_state: AlarmAttentionState,
) {
    apply_zones(clock_state, snapshot.zones);
    apply_preferences(prefs_state, snapshot.prefs);
    apply_alarms(alarm_state, attention_state, snapshot.alarms);
}

pub async fn load_snapshot() -> AppStoreSnapshot {
    let now_ms = Utc::now().timestamp_millis() as u64;
    let zones = crate::ipc::load_zones().await;
    let prefs = crate::ipc::load_preferences().await;
    let alarms = crate::ipc::load_alarms().await;

    let snapshot = snapshot_from_parts(zones, prefs, alarms, now_ms);
    if snapshot.zones.len() == 1 && snapshot.zones[0].id == "utc-1" {
        // Ensure the default zone exists on disk for first-run flows.
        crate::ipc::save_zones(snapshot.zones.clone()).await;
    }
    snapshot
}

pub async fn load_alarms_or_default() -> Vec<Alarm> {
    crate::ipc::load_alarms().await.unwrap_or_default()
}

pub async fn refresh_snapshot_into_states(
    clock_state: ClockState,
    prefs_state: UserPreferencesState,
    alarm_state: AlarmState,
    attention_state: AlarmAttentionState,
) {
    let snapshot = load_snapshot().await;
    apply_snapshot(snapshot, clock_state, prefs_state, alarm_state, attention_state);
}

pub async fn save_zones(zones: Vec<TimeZoneEntry>) {
    crate::ipc::save_zones(zones).await;
}

pub async fn save_preferences(preferences: UserPreferences) {
    crate::ipc::save_preferences(preferences).await;
}

pub async fn save_alarms(alarms: Vec<Alarm>) {
    crate::ipc::save_alarms(alarms).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{AlarmState as AlarmLifecycleState, Recurrence};
    use leptos::{create_runtime, create_rw_signal, SignalGetUntracked};

    fn sample_alarm(state: AlarmLifecycleState) -> Alarm {
        Alarm {
            id: "alarm-1".into(),
            title: "Wake".into(),
            target_time: "08:00".into(),
            target_date: None,
            timezone_id: "UTC".into(),
            recurrence: Recurrence::Daily,
            custom_days: vec![],
            enabled: true,
            state,
            snooze_count: 0,
            default_snooze_minutes: 10,
            snooze_until: None,
            sound_enabled: true,
            last_triggered_at: None,
            created_at: 0,
        }
    }

    #[test]
    fn snapshot_recovery_derives_default_zone_and_attention() {
        let snapshot = snapshot_from_parts(
            None,
            None,
            Some(vec![
                sample_alarm(AlarmLifecycleState::Triggered),
                sample_alarm(AlarmLifecycleState::Snoozed),
            ]),
            42,
        );

        assert_eq!(snapshot.zones.len(), 1);
        assert_eq!(snapshot.zones[0].iana_id, "UTC");
        assert_eq!(snapshot.attention.attention_count(), 2);
    }

    #[test]
    fn apply_snapshot_keeps_alarm_attention_in_sync() {
        let runtime = create_runtime();
        let clock_state = ClockState {
            zones: create_rw_signal(Vec::new()),
        };
        let prefs_state = UserPreferencesState {
            prefs: create_rw_signal(UserPreferences::default()),
        };
        let alarm_state = crate::state::AlarmState {
            alarms: create_rw_signal(Vec::new()),
        };
        let attention_state = AlarmAttentionState {
            attention: create_rw_signal(AlarmAttention::default()),
        };

        let snapshot = snapshot_from_parts(
            None,
            Some(UserPreferences {
                theme: "dark".into(),
                ..UserPreferences::default()
            }),
            Some(vec![sample_alarm(AlarmLifecycleState::Triggered)]),
            42,
        );

        apply_snapshot(
            snapshot,
            clock_state,
            prefs_state,
            alarm_state,
            attention_state,
        );

        assert_eq!(clock_state.zones.get_untracked().len(), 1);
        assert_eq!(prefs_state.prefs.get_untracked().theme, "dark");
        assert_eq!(alarm_state.alarms.get_untracked().len(), 1);
        assert_eq!(attention_state.attention.get_untracked().triggered_count(), 1);

        runtime.dispose();
    }
}
