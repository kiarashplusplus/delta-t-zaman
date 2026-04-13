use leptos::*;
use crate::models::{Alarm, AlarmState, Recurrence};
use crate::state::UserPreferencesState;
use crate::utils::get_timezone_metadata;
use delta_t_zaman_shared::{AlarmAction, AlarmActionArgs};

#[cfg(test)]
fn recurrence_matches(alarm: &Alarm, occurrence_key: &str, weekday: u32) -> bool {
    match alarm.recurrence {
        Recurrence::Once => alarm
            .target_date
            .as_deref()
            .map(|target_date| target_date == occurrence_key)
            .unwrap_or(true),
        Recurrence::Daily => true,
        Recurrence::Weekdays => weekday != 0 && weekday != 6,
        Recurrence::Weekends => weekday == 0 || weekday == 6,
        Recurrence::Custom => alarm.custom_days.contains(&(weekday as usize)),
    }
}

#[cfg(test)]
fn triggered_in_same_minute(last_triggered_at: Option<u64>, timestamp: i64) -> bool {
    last_triggered_at
        .map(|last| last / 60_000 == (timestamp as u64) / 60_000)
        .unwrap_or(false)
}

fn allows_alarm_action(state: &AlarmState) -> bool {
    matches!(state, AlarmState::Triggered | AlarmState::Snoozed)
}

fn status_label(state: &AlarmState) -> &'static str {
    match state {
        AlarmState::Active => "Active",
        AlarmState::Triggered => "Ringing",
        AlarmState::Snoozed => "Snoozed",
        AlarmState::Completed => "Completed",
        AlarmState::Dismissed => "Dismissed",
    }
}

fn weekday_short_label(day: usize) -> &'static str {
    match day {
        0 => "Sun",
        1 => "Mon",
        2 => "Tue",
        3 => "Wed",
        4 => "Thu",
        5 => "Fri",
        6 => "Sat",
        _ => "?",
    }
}

fn recurrence_label(alarm: &Alarm) -> String {
    match alarm.recurrence {
        Recurrence::Once => "Once".to_string(),
        Recurrence::Daily => "Daily".to_string(),
        Recurrence::Weekdays => "Weekdays".to_string(),
        Recurrence::Weekends => "Weekends".to_string(),
        Recurrence::Custom => {
            if alarm.custom_days.is_empty() {
                "Custom".to_string()
            } else {
                alarm.custom_days
                    .iter()
                    .map(|day| weekday_short_label(*day))
                    .collect::<Vec<_>>()
                    .join(", ")
            }
        }
    }
}

fn attention_banner_text(attention: &crate::models::AlarmAttention) -> Option<String> {
    crate::workflow::attention_banner(attention).map(|banner| banner.body)
}

#[component]
pub fn AlarmCard(
    alarm: Alarm,
    set_feedback: WriteSignal<Option<crate::product::BannerCopy>>,
) -> impl IntoView {
    let alarm_for_edit = alarm.clone();
    let prefs_ctx = use_context::<UserPreferencesState>().expect("UserPreferencesState missing");
    let alarm_state_ctx = use_context::<crate::state::AlarmState>().expect("AlarmState missing");
    let alarm_attention_ctx =
        use_context::<crate::state::AlarmAttentionState>().expect("AlarmAttentionState missing");
    let modal_state = use_context::<crate::state::ModalState>().expect("ModalState missing");
    let is_12h = move || prefs_ctx.prefs.get().time_format == "12h";
    let display_target_time = alarm.target_time.clone();

    let time_str = move || {
        if is_12h() {
            let parts: Vec<&str> = display_target_time.split(':').collect();
            if parts.len() == 2 {
                let h: u32 = parts[0].parse().unwrap_or(0);
                let m = parts[1];
                let period = if h >= 12 { "PM" } else { "AM" };
                let h12 = if h.is_multiple_of(12) { 12 } else { h % 12 };
                format!("{}:{} {}", h12, m, period)
            } else {
                display_target_time.clone()
            }
        } else {
            display_target_time.clone()
        }
    };

    let metadata = get_timezone_metadata(&alarm.timezone_id);
    let tz_display = metadata
        .and_then(|m| m.city)
        .map(|c| format!("in {}", c))
        .unwrap_or_else(|| format!("in {}", alarm.timezone_id));

    let recurrence_str = recurrence_label(&alarm);

    let state_str = status_label(&alarm.state);

    let state_class = match alarm.state {
        AlarmState::Active => "alarm-state-active",
        AlarmState::Triggered => "alarm-state-triggered",
        AlarmState::Snoozed => "alarm-state-snoozed",
        AlarmState::Completed => "alarm-state-completed",
        AlarmState::Dismissed => "alarm-state-dismissed",
    };

    let badge_style = match alarm.state {
        AlarmState::Completed | AlarmState::Dismissed => {
            "background: var(--color-gray-200); color: var(--color-gray-600);"
        }
        AlarmState::Triggered => {
            "background: rgba(239, 68, 68, 0.14); color: #b91c1c;"
        }
        AlarmState::Snoozed => {
            "background: rgba(245, 158, 11, 0.15); color: var(--color-warning);"
        }
        _ => "background: rgba(16, 185, 129, 0.15); color: var(--color-success);",
    };

    let time_opacity = match alarm.state {
        AlarmState::Completed | AlarmState::Dismissed => "0.5",
        _ => "1.0",
    };

    let action_hint = match alarm.state {
        AlarmState::Triggered => Some("Needs action"),
        AlarmState::Snoozed => Some("Snoozed until next reminder"),
        _ => None,
    };

    let alarm_id_for_toggle = alarm.id.clone();
    let alarm_id_for_delete = alarm.id.clone();
    let alarm_id_for_snooze = alarm.id.clone();
    let alarm_id_for_dismiss = alarm.id.clone();
    let alarm_title_for_delete = alarm.title.clone();
    let alarm_title_for_snooze = alarm.title.clone();
    let alarm_title_for_dismiss = alarm.title.clone();

    view! {
        <div class=format!("alarm-card {}", state_class)>
            <div class="alarm-card__header">
                <div class="alarm-card__title">{alarm.title.clone()}</div>
                <crate::components::ui::Toggle
                    checked=Signal::derive(move || alarm.enabled)
                    on_change={
                        let alarm_state_ctx = alarm_state_ctx;
                        let alarm_attention_ctx = alarm_attention_ctx;
                        let alarm_id_for_toggle = alarm_id_for_toggle.clone();
                        move |checked| {
                            let mut alarms = alarm_state_ctx.alarms.get_untracked();
                            if let Some(a) = alarms.iter_mut().find(|a| a.id == alarm_id_for_toggle) {
                                a.enabled = checked;
                                if checked {
                                    a.state = AlarmState::Active;
                                    a.snooze_until = None;
                                }
                                alarm_attention_ctx
                                    .attention
                                    .set(crate::store::attention_from_alarms(&alarms));
                                alarm_state_ctx.alarms.set(alarms.clone());
                                spawn_local(async move {
                                    crate::store::save_alarms(alarms).await;
                                });
                            }
                        }
                    }
                />
            </div>
            <div class="alarm-card__body">
                <div class="alarm-card__time-group">
                    <div class="alarm-card__time" style=format!("opacity: {}", time_opacity)>
                        {time_str}
                    </div>
                    <div class="alarm-card__timezone" style="color: var(--theme-muted); font-weight: var(--font-weight-medium);">
                        {tz_display}
                    </div>
                </div>
                <div class="alarm-card__recurrence" style="color: var(--theme-muted); font-size: var(--font-size-sm);">
                    {recurrence_str}
                </div>
                <div style="margin-top: 0.35rem; color: var(--theme-muted); font-size: var(--font-size-sm);">
                    {format!(
                        "Default snooze: {}m · Sound {}",
                        alarm.default_snooze_minutes,
                        if alarm.sound_enabled { "on" } else { "off" }
                    )}
                </div>
                <Show when=move || action_hint.is_some()>
                    <div style="margin-top: 0.4rem; color: var(--theme-muted); font-size: var(--font-size-sm);">
                        {action_hint.unwrap_or_default()}
                    </div>
                </Show>
            </div>
            <div class="alarm-card__footer">
                <div class="alarm-card__badge" style=badge_style>{state_str}</div>
                <div class="alarm-card__actions" style="display: flex; gap: var(--spacing-sm);">
                    <Show when=move || allows_alarm_action(&alarm.state)>
                        <button
                            class="alarm-card__snooze"
                            aria-label="Snooze Alarm"
                            on:click={
                                let alarm_state_ctx = alarm_state_ctx;
                                let alarm_attention_ctx = alarm_attention_ctx;
                                let alarm_id_for_snooze = alarm_id_for_snooze.clone();
                                let alarm_title_for_snooze = alarm_title_for_snooze.clone();
                                let set_feedback = set_feedback;
                                move |_| {
                                    let alarm_id = alarm_id_for_snooze.clone();
                                    let alarm_title = alarm_title_for_snooze.clone();
                                    spawn_local(async move {
                                        if crate::client::perform_alarm_action_and_refresh(
                                            alarm_state_ctx,
                                            alarm_attention_ctx,
                                            AlarmActionArgs {
                                            alarm_id: alarm_id.clone(),
                                            action: AlarmAction::Snooze,
                                            snooze_minutes: Some(alarm.default_snooze_minutes),
                                        },
                                        )
                                        .await
                                        .is_ok()
                                        {
                                            set_feedback.set(Some(
                                                crate::workflow::alarm_action_notice(
                                                    AlarmAction::Snooze,
                                                    &alarm_title,
                                                    &alarm_attention_ctx.attention.get_untracked(),
                                                ),
                                            ));
                                        }
                                    });
                                }
                            }
                        >
                            {format!("Snooze {}m", alarm.default_snooze_minutes)}
                        </button>
                        <button
                            class="alarm-card__dismiss"
                            aria-label="Dismiss Alarm"
                            on:click={
                                let alarm_state_ctx = alarm_state_ctx;
                                let alarm_attention_ctx = alarm_attention_ctx;
                                let alarm_id_for_dismiss = alarm_id_for_dismiss.clone();
                                let alarm_title_for_dismiss = alarm_title_for_dismiss.clone();
                                let set_feedback = set_feedback;
                                move |_| {
                                    let alarm_id = alarm_id_for_dismiss.clone();
                                    let alarm_title = alarm_title_for_dismiss.clone();
                                    spawn_local(async move {
                                        if crate::client::perform_alarm_action_and_refresh(
                                            alarm_state_ctx,
                                            alarm_attention_ctx,
                                            AlarmActionArgs {
                                            alarm_id: alarm_id.clone(),
                                            action: AlarmAction::Dismiss,
                                            snooze_minutes: None,
                                        },
                                        )
                                        .await
                                        .is_ok()
                                        {
                                            set_feedback.set(Some(
                                                crate::workflow::alarm_action_notice(
                                                    AlarmAction::Dismiss,
                                                    &alarm_title,
                                                    &alarm_attention_ctx.attention.get_untracked(),
                                                ),
                                            ));
                                        }
                                    });
                                }
                            }
                        >
                            "Dismiss"
                        </button>
                    </Show>
                    <button
                        class="alarm-card__edit"
                        aria-label="Edit Alarm"
                        on:click=move |_| {
                            modal_state.editing_alarm.set(Some(alarm_for_edit.clone()));
                            modal_state.show_alarm_form.set(true);
                        }
                    >
                        "✏️"
                    </button>
                    <button
                        class="alarm-card__delete"
                        aria-label="Delete Alarm"
                        on:click={
                            let alarm_id_for_delete = alarm_id_for_delete.clone();
                            let alarm_title_for_delete = alarm_title_for_delete.clone();
                            let set_feedback = set_feedback;
                            move |_| {
                            if web_sys::window().unwrap().confirm_with_message(
                                &crate::product::confirm_delete_alarm_message(&alarm.title)
                            ).unwrap_or(false) {
                                let mut alarms = alarm_state_ctx.alarms.get_untracked();
                                alarms.retain(|a| a.id != alarm_id_for_delete);
                                alarm_attention_ctx
                                    .attention
                                    .set(crate::store::attention_from_alarms(&alarms));
                                alarm_state_ctx.alarms.set(alarms.clone());
                                set_feedback.set(Some(
                                    crate::workflow::alarm_delete_notice(
                                        &alarm_title_for_delete,
                                        alarms.len(),
                                    ),
                                ));
                                spawn_local(async move {
                                    crate::store::save_alarms(alarms).await;
                                });
                            }
                        }}
                    >
                        "×"
                    </button>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn AlarmList() -> impl IntoView {
    let alarm_state = use_context::<crate::state::AlarmState>().expect("AlarmState missing");
    let modal_state = use_context::<crate::state::ModalState>().expect("ModalState missing");
    let attention_state = use_context::<crate::state::AlarmAttentionState>().expect("AlarmAttentionState missing");
    let empty_copy = crate::product::alarms_empty_copy();
    let add_label = empty_copy.action_label.clone().unwrap_or_else(|| "+ Add Alarm".into());
    let (action_feedback, set_action_feedback) =
        create_signal(None::<crate::product::BannerCopy>);

    view! {
        <div class="alarm-list-view">
            <div class="alarm-list-header" style="display: flex; justify-content: space-between; align-items: center; margin-bottom: var(--spacing-lg);">
                <h2 style="margin: 0; font-size: 2rem; letter-spacing: -0.02em;">"Alarms"</h2>
                <button
                    aria-label="Add Alarm"
                    style="margin: 0;"
                    on:click=move |_| {
                        set_action_feedback.set(None);
                        modal_state.editing_alarm.set(None);
                        modal_state.show_alarm_form.set(true);
                    }
                >
                    "+ Add Alarm"
                </button>
            </div>

            <Show when=move || action_feedback.get().is_some()>
                {move || {
                    action_feedback.get().map(|feedback| {
                        view! {
                            <div
                                style=move || format!(
                                    "display: flex; align-items: flex-start; justify-content: space-between; gap: 1rem; margin-bottom: var(--spacing-lg); padding: 0.8rem 0.95rem; border-radius: 1rem; line-height: 1.45; {}",
                                    crate::product::banner_style(&feedback.tone)
                                )
                            >
                                <div>
                                    <div style="font-weight: 700; margin-bottom: 0.2rem;">{feedback.title}</div>
                                    <div>{feedback.body}</div>
                                </div>
                                <button
                                    class="btn-secondary"
                                    style="margin: 0; white-space: nowrap;"
                                    on:click=move |_| set_action_feedback.set(None)
                                >
                                    "Clear"
                                </button>
                            </div>
                        }
                    })
                }}
            </Show>

            <Show when=move || attention_banner_text(&attention_state.attention.get()).is_some()>
                <div style="margin-bottom: var(--spacing-lg); padding: 0.8rem 0.95rem; border-radius: 1rem; background: rgba(239, 68, 68, 0.08); color: #991b1b; line-height: 1.45;">
                    {move || {
                        attention_banner_text(&attention_state.attention.get()).unwrap_or_default()
                    }}
                </div>
            </Show>

            <Show
                when=move || !alarm_state.alarms.get().is_empty()
                fallback=move || view! {
                    <div class="alarm-list__empty" style="display: grid; gap: 0.85rem; padding: 1.25rem; border: 1px dashed rgba(148, 163, 184, 0.45); border-radius: 1rem; background: rgba(148, 163, 184, 0.08);">
                        <div>
                            <div style="font-size: 1rem; font-weight: 700; margin-bottom: 0.35rem;">{empty_copy.title.clone()}</div>
                            <div style="color: var(--theme-muted); line-height: 1.5;">{empty_copy.body.clone()}</div>
                        </div>
                        <div>
                            <button
                                aria-label="Add Alarm"
                                style="margin: 0;"
                                on:click=move |_| {
                                    modal_state.editing_alarm.set(None);
                                    modal_state.show_alarm_form.set(true);
                                    set_action_feedback.set(None);
                                }
                            >
                                {add_label.clone()}
                            </button>
                        </div>
                    </div>
                }
            >
                <div class="alarm-list" role="list">
                    <For
                        each=move || alarm_state.alarms.get()
                        key=|a| a.id.clone()
                        children=move |alarm| view! { <AlarmCard alarm=alarm set_feedback=set_action_feedback/> }
                    />
                </div>
            </Show>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_alarm(recurrence: Recurrence) -> Alarm {
        Alarm {
            id: "alarm-1".into(),
            title: "Test".into(),
            target_time: "08:00".into(),
            target_date: Some("2026-04-13".into()),
            timezone_id: "UTC".into(),
            recurrence,
            custom_days: vec![1, 3, 5],
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
    fn once_recurrence_matches_only_target_date() {
        let alarm = sample_alarm(Recurrence::Once);
        assert!(recurrence_matches(&alarm, "2026-04-13", 1));
        assert!(!recurrence_matches(&alarm, "2026-04-14", 2));
    }

    #[test]
    fn weekday_and_weekend_rules_are_applied() {
        assert!(recurrence_matches(&sample_alarm(Recurrence::Weekdays), "2026-04-13", 1));
        assert!(!recurrence_matches(&sample_alarm(Recurrence::Weekdays), "2026-04-12", 0));
        assert!(recurrence_matches(&sample_alarm(Recurrence::Weekends), "2026-04-12", 0));
        assert!(!recurrence_matches(&sample_alarm(Recurrence::Weekends), "2026-04-14", 2));
    }

    #[test]
    fn custom_days_and_minute_debouncing_work() {
        let alarm = sample_alarm(Recurrence::Custom);
        assert!(recurrence_matches(&alarm, "2026-04-14", 3));
        assert!(!recurrence_matches(&alarm, "2026-04-12", 0));
        assert!(triggered_in_same_minute(Some(120_000), 179_999));
        assert!(!triggered_in_same_minute(Some(120_000), 180_000));
    }

    #[test]
    fn triggered_and_snoozed_states_allow_quick_actions() {
        assert!(allows_alarm_action(&AlarmState::Triggered));
        assert!(allows_alarm_action(&AlarmState::Snoozed));
        assert!(!allows_alarm_action(&AlarmState::Active));
        assert_eq!(status_label(&AlarmState::Triggered), "Ringing");
    }

    #[test]
    fn custom_recurrence_label_lists_selected_days() {
        let mut alarm = sample_alarm(Recurrence::Custom);
        alarm.custom_days = vec![1, 3, 5];

        assert_eq!(recurrence_label(&alarm), "Mon, Wed, Fri");
    }

    #[test]
    fn recovered_attention_produces_banner_text() {
        let attention = crate::models::AlarmAttention {
            triggered_alarm_ids: vec!["alarm-1".into()],
            snoozed_alarm_ids: vec!["alarm-2".into()],
        };

        assert_eq!(
            attention_banner_text(&attention).as_deref(),
            Some("1 ringing, 1 snoozed. Open alarms here to snooze or dismiss them.")
        );
        assert_eq!(
            attention_banner_text(&crate::models::AlarmAttention::default()),
            None
        );
    }

    #[test]
    fn action_feedback_style_matches_product_tone() {
        assert!(crate::product::banner_style(&crate::product::BannerTone::Info).contains("#1d4ed8"));
        assert!(crate::product::banner_style(&crate::product::BannerTone::Success).contains("#047857"));
        assert!(crate::product::banner_style(&crate::product::BannerTone::Warning).contains("#b45309"));
    }
}
