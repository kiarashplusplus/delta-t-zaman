use chrono::{DateTime, Datelike, Duration, TimeZone, Timelike, Utc, Weekday};
use chrono_tz::Tz;
use delta_t_zaman_shared::{
    Alarm as StoredAlarm, AlarmAction, AlarmActionArgs, AlarmActionResult, AlarmAttention,
    AlarmNotificationConfig, AlarmState as StoredAlarmState, AlarmsStore,
    Recurrence as StoredRecurrence, ScheduleResult,
};
use std::str::FromStr;
use tauri::{AppHandle, Emitter};
use tauri_plugin_notification::{NotificationExt, PermissionState};
const DEFAULT_SNOOZE_MINUTES: u32 = 10;

#[derive(Clone, Debug)]
struct DueNotification {
    title: String,
    body: String,
}

fn load_alarms(app: &AppHandle) -> Result<Vec<StoredAlarm>, String> {
    Ok(super::store_api::load_alarms_store_state(app)?.alarms)
}

fn save_alarms(app: &AppHandle, alarms: &[StoredAlarm]) -> Result<(), String> {
    super::store_api::save_alarms_store_state(
        app,
        &AlarmsStore {
            alarms: alarms.to_vec(),
        },
    )
}

fn parse_alarm_time(target_time: &str) -> Option<(u32, u32)> {
    let parts: Vec<&str> = target_time.split(':').collect();
    if parts.len() != 2 {
        return None;
    }

    Some((parts[0].parse().ok()?, parts[1].parse().ok()?))
}

fn weekday_index(weekday: Weekday) -> usize {
    weekday.num_days_from_sunday() as usize
}

fn recurrence_matches(alarm: &StoredAlarm, occurrence_key: &str, weekday: Weekday) -> bool {
    match alarm.recurrence {
        StoredRecurrence::Once => alarm
            .target_date
            .as_deref()
            .map(|target_date| target_date == occurrence_key)
            .unwrap_or(true),
        StoredRecurrence::Daily => true,
        StoredRecurrence::Weekdays => !matches!(weekday, Weekday::Sat | Weekday::Sun),
        StoredRecurrence::Weekends => matches!(weekday, Weekday::Sat | Weekday::Sun),
        StoredRecurrence::Custom => alarm.custom_days.contains(&weekday_index(weekday)),
    }
}

fn triggered_in_same_minute(last_triggered_at: Option<u64>, timestamp_ms: u64) -> bool {
    last_triggered_at
        .map(|last| last / 60_000 == timestamp_ms / 60_000)
        .unwrap_or(false)
}

fn next_once_target_date(
    now_local: DateTime<Tz>,
    target_hour: u32,
    target_minute: u32,
) -> String {
    let target_today_or_tomorrow = if (now_local.hour(), now_local.minute()) <= (target_hour, target_minute) {
        now_local
    } else {
        now_local + Duration::days(1)
    };

    target_today_or_tomorrow.format("%F").to_string()
}

fn alarm_display_title(alarm: &StoredAlarm) -> String {
    if alarm.title.trim().is_empty() {
        "Alarm".to_string()
    } else {
        alarm.title.clone()
    }
}

fn trigger_alarm(alarm: &mut StoredAlarm, now_ms: u64) -> DueNotification {
    alarm.state = StoredAlarmState::Triggered;
    alarm.snooze_until = None;
    alarm.last_triggered_at = Some(now_ms);

    DueNotification {
        title: alarm_display_title(alarm),
        body: format!("It's {} in {}", alarm.target_time, alarm.timezone_id),
    }
}

fn dismiss_alarm_for_now(alarm: &mut StoredAlarm, now_ms: u64) {
    alarm.snooze_until = None;
    alarm.last_triggered_at = Some(now_ms);

    if alarm.recurrence == StoredRecurrence::Once {
        alarm.enabled = false;
        alarm.state = StoredAlarmState::Completed;
    } else {
        alarm.state = StoredAlarmState::Active;
        alarm.snooze_count = 0;
    }
}

fn snooze_alarm_for_minutes(alarm: &mut StoredAlarm, now_ms: u64, minutes: u32) {
    let snooze_ms = u64::from(minutes.max(1)) * 60_000;
    alarm.state = StoredAlarmState::Snoozed;
    alarm.snooze_count += 1;
    alarm.snooze_until = Some(now_ms + snooze_ms);
    alarm.enabled = true;
}

fn apply_alarm_action_to_alarm(
    alarm: &mut StoredAlarm,
    action: AlarmAction,
    now_ms: u64,
    snooze_minutes: Option<u32>,
) {
    match action {
        AlarmAction::Snooze => snooze_alarm_for_minutes(
            alarm,
            now_ms,
            snooze_minutes.unwrap_or(if alarm.default_snooze_minutes == 0 {
                DEFAULT_SNOOZE_MINUTES
            } else {
                alarm.default_snooze_minutes
            }),
        ),
        AlarmAction::Dismiss => dismiss_alarm_for_now(alarm, now_ms),
    }
}

fn state_label(state: &StoredAlarmState) -> &'static str {
    match state {
        StoredAlarmState::Active => "active",
        StoredAlarmState::Triggered => "triggered",
        StoredAlarmState::Snoozed => "snoozed",
        StoredAlarmState::Completed => "completed",
        StoredAlarmState::Dismissed => "dismissed",
    }
}

fn process_alarms(now_ms: u64, alarms: &mut [StoredAlarm]) -> Vec<DueNotification> {
    let mut due_notifications = Vec::new();
    let now_utc = Utc
        .timestamp_millis_opt(now_ms as i64)
        .single()
        .unwrap_or_else(Utc::now);

    for alarm in alarms.iter_mut() {
        if !alarm.enabled
            || matches!(
                alarm.state,
                StoredAlarmState::Completed | StoredAlarmState::Dismissed | StoredAlarmState::Triggered
            )
        {
            continue;
        }

        let Some((target_hour, target_minute)) = parse_alarm_time(&alarm.target_time) else {
            continue;
        };

        let timezone = Tz::from_str(&alarm.timezone_id).unwrap_or(chrono_tz::UTC);
        let local_now = now_utc.with_timezone(&timezone);
        let occurrence_key = local_now.format("%F").to_string();

        if alarm.recurrence == StoredRecurrence::Once && alarm.target_date.is_none() {
            alarm.target_date = Some(next_once_target_date(local_now, target_hour, target_minute));
        }

        if alarm.state == StoredAlarmState::Snoozed {
            if alarm.snooze_until.is_some_and(|snooze_until| now_ms >= snooze_until) {
                due_notifications.push(trigger_alarm(alarm, now_ms));
            }
            continue;
        }

        if local_now.hour() != target_hour || local_now.minute() != target_minute {
            continue;
        }

        if triggered_in_same_minute(alarm.last_triggered_at, now_ms) {
            continue;
        }

        if !recurrence_matches(alarm, &occurrence_key, local_now.weekday()) {
            continue;
        }

        due_notifications.push(trigger_alarm(alarm, now_ms));
    }

    due_notifications
}

fn show_due_notification(app: &AppHandle, notification: DueNotification) -> Result<(), String> {
    app.notification()
        .builder()
        .title(notification.title)
        .body(notification.body)
        .show()
        .map_err(|e| e.to_string())
}

fn current_time_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

fn alarm_attention(alarms: &[StoredAlarm]) -> AlarmAttention {
    AlarmAttention::from_alarms(alarms)
}

fn emit_alarm_update(app: &AppHandle, alarms: &[StoredAlarm]) {
    let _ = app.emit("alarms-updated", ());
    let _ = app.emit("alarm-attention", alarm_attention(alarms));
}

fn poll_and_fire_alarms(app: &AppHandle, now_ms: u64) -> Result<(), String> {
    let mut alarms = load_alarms(app)?;
    if alarms.is_empty() {
        return Ok(());
    }

    let before = serde_json::to_string(&alarms).map_err(|e| e.to_string())?;
    let due_notifications = process_alarms(now_ms, &mut alarms);
    let after = serde_json::to_string(&alarms).map_err(|e| e.to_string())?;

    if due_notifications.is_empty() && before == after {
        return Ok(());
    }

    if !due_notifications.is_empty() {
        let permission_granted = matches!(
            app.notification().permission_state(),
            Ok(PermissionState::Granted)
        );

        if permission_granted {
            for notification in due_notifications {
                let _ = show_due_notification(app, notification);
            }
        }
    }

    save_alarms(app, &alarms)?;
    emit_alarm_update(app, &alarms);
    Ok(())
}

pub fn start_alarm_monitor(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        if let Ok(alarms) = load_alarms(&app) {
            emit_alarm_update(&app, &alarms);
        }

        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(15));
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

        loop {
            interval.tick().await;
            let _ = poll_and_fire_alarms(&app, current_time_ms());
        }
    });
}

#[tauri::command]
pub async fn schedule_alarm_notification(
    app: AppHandle,
    config: AlarmNotificationConfig,
) -> Result<ScheduleResult, String> {
    let _ = &config.schedule_at;
    let _ = config.sound_enabled;
    let _ = &config.action_type;

    app.notification()
        .builder()
        .title(config.title)
        .body(config.body)
        .show()
        .map_err(|e| e.to_string())?;

    Ok(ScheduleResult {
        notification_id: format!("alarm_{}", config.alarm_id),
    })
}

#[tauri::command]
pub async fn perform_alarm_action(
    app: AppHandle,
    payload: AlarmActionArgs,
) -> Result<AlarmActionResult, String> {
    let mut alarms = load_alarms(&app)?;
    let Some(alarm) = alarms.iter_mut().find(|alarm| alarm.id == payload.alarm_id) else {
        return Err("Alarm not found.".to_string());
    };

    apply_alarm_action_to_alarm(alarm, payload.action, current_time_ms(), payload.snooze_minutes);
    let result = AlarmActionResult {
        alarm_id: alarm.id.clone(),
        state: state_label(&alarm.state).to_string(),
        enabled: alarm.enabled,
    };

    save_alarms(&app, &alarms)?;
    emit_alarm_update(&app, &alarms);
    Ok(result)
}

#[tauri::command]
pub async fn cancel_alarm_notification(
    _app: AppHandle,
    _notification_id: String,
) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub async fn get_notification_permission(app: AppHandle) -> Result<String, String> {
    match app.notification().permission_state() {
        Ok(PermissionState::Granted) => Ok("granted".to_string()),
        Ok(PermissionState::Denied) => Ok("denied".to_string()),
        Ok(PermissionState::Prompt) => Ok("default".to_string()),
        Ok(PermissionState::PromptWithRationale) => Ok("default".to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn request_notification_permission(app: AppHandle) -> Result<String, String> {
    match app.notification().request_permission() {
        Ok(PermissionState::Granted) => Ok("granted".to_string()),
        Ok(PermissionState::Denied) => Ok("denied".to_string()),
        Ok(PermissionState::Prompt) => Ok("default".to_string()),
        Ok(PermissionState::PromptWithRationale) => Ok("default".to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_alarm(recurrence: StoredRecurrence) -> StoredAlarm {
        StoredAlarm {
            id: "alarm-1".into(),
            title: "Wake up".into(),
            target_time: "08:00".into(),
            target_date: Some("2026-04-13".into()),
            timezone_id: "UTC".into(),
            recurrence,
            custom_days: vec![1, 3, 5],
            enabled: true,
            state: StoredAlarmState::Active,
            snooze_count: 0,
            default_snooze_minutes: 10,
            snooze_until: None,
            sound_enabled: true,
            last_triggered_at: None,
            created_at: 0,
        }
    }

    #[test]
    fn once_alarm_fires_and_waits_for_action() {
        let mut alarms = vec![sample_alarm(StoredRecurrence::Once)];
        let now_ms = Utc
            .with_ymd_and_hms(2026, 4, 13, 8, 0, 0)
            .single()
            .unwrap()
            .timestamp_millis() as u64;

        let due = process_alarms(now_ms, &mut alarms);

        assert_eq!(due.len(), 1);
        assert!(alarms[0].enabled);
        assert_eq!(alarms[0].state, StoredAlarmState::Triggered);
        assert_eq!(alarms[0].last_triggered_at, Some(now_ms));
    }

    #[test]
    fn once_alarm_without_target_date_gets_initialized() {
        let mut alarm = sample_alarm(StoredRecurrence::Once);
        alarm.target_date = None;
        let now_ms = Utc
            .with_ymd_and_hms(2026, 4, 13, 7, 30, 0)
            .single()
            .unwrap()
            .timestamp_millis() as u64;

        let due = process_alarms(now_ms, std::slice::from_mut(&mut alarm));

        assert!(due.is_empty());
        assert_eq!(alarm.target_date.as_deref(), Some("2026-04-13"));
    }

    #[test]
    fn recurring_alarm_is_debounced_within_same_minute() {
        let mut alarm = sample_alarm(StoredRecurrence::Daily);
        let now_ms = Utc
            .with_ymd_and_hms(2026, 4, 13, 8, 0, 30)
            .single()
            .unwrap()
            .timestamp_millis() as u64;
        alarm.last_triggered_at = Some(now_ms - 10_000);

        let due = process_alarms(now_ms, std::slice::from_mut(&mut alarm));

        assert!(due.is_empty());
    }

    #[test]
    fn custom_recurrence_honors_weekday_list() {
        let mut alarm = sample_alarm(StoredRecurrence::Custom);
        let monday_ms = Utc
            .with_ymd_and_hms(2026, 4, 13, 8, 0, 0)
            .single()
            .unwrap()
            .timestamp_millis() as u64;
        let sunday_ms = Utc
            .with_ymd_and_hms(2026, 4, 12, 8, 0, 0)
            .single()
            .unwrap()
            .timestamp_millis() as u64;

        assert_eq!(process_alarms(monday_ms, std::slice::from_mut(&mut alarm)).len(), 1);

        let mut sunday_alarm = sample_alarm(StoredRecurrence::Custom);
        sunday_alarm.target_date = None;
        assert!(process_alarms(sunday_ms, std::slice::from_mut(&mut sunday_alarm)).is_empty());
    }

    #[test]
    fn snoozed_alarm_retriggers_after_deadline() {
        let mut alarm = sample_alarm(StoredRecurrence::Daily);
        alarm.state = StoredAlarmState::Snoozed;
        alarm.snooze_until = Some(600_000);

        let due = process_alarms(600_000, std::slice::from_mut(&mut alarm));

        assert_eq!(due.len(), 1);
        assert_eq!(alarm.state, StoredAlarmState::Triggered);
        assert_eq!(alarm.snooze_until, None);
    }

    #[test]
    fn dismissing_once_alarm_completes_it() {
        let mut alarm = sample_alarm(StoredRecurrence::Once);
        alarm.state = StoredAlarmState::Triggered;

        apply_alarm_action_to_alarm(&mut alarm, AlarmAction::Dismiss, 1_000, None);

        assert_eq!(alarm.state, StoredAlarmState::Completed);
        assert!(!alarm.enabled);
    }

    #[test]
    fn dismissing_recurring_alarm_reschedules_next_occurrence() {
        let mut alarm = sample_alarm(StoredRecurrence::Daily);
        alarm.state = StoredAlarmState::Triggered;
        alarm.snooze_count = 2;

        apply_alarm_action_to_alarm(&mut alarm, AlarmAction::Dismiss, 1_000, None);

        assert_eq!(alarm.state, StoredAlarmState::Active);
        assert!(alarm.enabled);
        assert_eq!(alarm.snooze_count, 0);
    }

    #[test]
    fn snooze_action_uses_alarm_default_duration_when_payload_omits_one() {
        let mut alarm = sample_alarm(StoredRecurrence::Daily);
        alarm.default_snooze_minutes = 15;
        alarm.state = StoredAlarmState::Triggered;

        apply_alarm_action_to_alarm(&mut alarm, AlarmAction::Snooze, 60_000, None);

        assert_eq!(alarm.state, StoredAlarmState::Snoozed);
        assert_eq!(alarm.snooze_until, Some(960_000));
    }

    #[test]
    fn attention_payload_reflects_triggered_and_snoozed_alarms() {
        let mut triggered = sample_alarm(StoredRecurrence::Daily);
        triggered.id = "triggered".into();
        triggered.state = StoredAlarmState::Triggered;

        let mut snoozed = sample_alarm(StoredRecurrence::Daily);
        snoozed.id = "snoozed".into();
        snoozed.state = StoredAlarmState::Snoozed;
        snoozed.snooze_until = Some(1);

        let attention = alarm_attention(&[triggered, snoozed]);
        assert_eq!(attention.triggered_alarm_ids, vec!["triggered"]);
        assert_eq!(attention.snoozed_alarm_ids, vec!["snoozed"]);
    }

    #[test]
    fn restart_recovery_keeps_attention_without_refiring_alarm() {
        let mut triggered = sample_alarm(StoredRecurrence::Daily);
        triggered.id = "triggered".into();
        triggered.state = StoredAlarmState::Triggered;
        triggered.last_triggered_at = Some(
            Utc.with_ymd_and_hms(2026, 4, 13, 8, 0, 0)
                .single()
                .unwrap()
                .timestamp_millis() as u64,
        );

        let attention_before = alarm_attention(&[triggered.clone()]);
        assert_eq!(attention_before.triggered_alarm_ids, vec!["triggered"]);

        let now_ms = Utc
            .with_ymd_and_hms(2026, 4, 13, 8, 0, 30)
            .single()
            .unwrap()
            .timestamp_millis() as u64;

        let due = process_alarms(now_ms, std::slice::from_mut(&mut triggered));
        assert!(due.is_empty());

        let attention_after = alarm_attention(&[triggered]);
        assert_eq!(attention_after.triggered_alarm_ids, vec!["triggered"]);
    }

    #[test]
    fn triggered_alarm_survives_restart_without_duplicate_fire() {
        let mut alarm = sample_alarm(StoredRecurrence::Daily);
        alarm.state = StoredAlarmState::Triggered;
        alarm.last_triggered_at = Some(
            Utc.with_ymd_and_hms(2026, 4, 13, 8, 0, 0)
                .single()
                .unwrap()
                .timestamp_millis() as u64,
        );

        let now_ms = Utc
            .with_ymd_and_hms(2026, 4, 13, 8, 0, 30)
            .single()
            .unwrap()
            .timestamp_millis() as u64;

        let due = process_alarms(now_ms, std::slice::from_mut(&mut alarm));

        assert!(due.is_empty());
        assert_eq!(alarm.state, StoredAlarmState::Triggered);
    }
}
