use leptos::*;
use crate::models::{Alarm, AlarmState, Recurrence};
use crate::state::{AlarmAttentionState, AlarmState as GlobalAlarmState, ClockState, ModalState};
use chrono::Utc;
use uuid::Uuid;

const WEEKDAY_OPTIONS: [(usize, &str, &str); 7] = [
    (0, "Sun", "Sunday"),
    (1, "Mon", "Monday"),
    (2, "Tue", "Tuesday"),
    (3, "Wed", "Wednesday"),
    (4, "Thu", "Thursday"),
    (5, "Fri", "Friday"),
    (6, "Sat", "Saturday"),
];

fn default_custom_days() -> Vec<usize> {
    vec![1, 2, 3, 4, 5]
}

fn toggle_custom_day(mut days: Vec<usize>, day: usize) -> Vec<usize> {
    if let Some(index) = days.iter().position(|current| *current == day) {
        days.remove(index);
    } else {
        days.push(day);
        days.sort_unstable();
    }
    days
}

#[component]
pub fn AlarmFormModal() -> impl IntoView {
    let modal_state = use_context::<ModalState>().expect("ModalState missing");
    let global_alarms = use_context::<GlobalAlarmState>().expect("GlobalAlarmState missing");
    let alarm_attention = use_context::<AlarmAttentionState>().expect("AlarmAttentionState missing");
    let clock_state = use_context::<ClockState>().expect("ClockState missing");

    let is_open = move || modal_state.show_alarm_form.get();

    let (title, set_title) = create_signal("".to_string());
    let (target_time, set_target_time) = create_signal("08:00".to_string());
    let (timezone_id, set_timezone_id) = create_signal("UTC".to_string());
    let (recurrence, set_recurrence) = create_signal(Recurrence::Once);
    let (custom_days, set_custom_days) = create_signal(default_custom_days());
    let (default_snooze_minutes, set_default_snooze_minutes) = create_signal(10_u32);
    let (sound_enabled, set_sound_enabled) = create_signal(true);
    let (form_error, set_form_error) = create_signal(None::<String>);

    create_effect(move |_| {
        if is_open() {
            set_form_error.set(None);
            if let Some(alarm) = modal_state.editing_alarm.get() {
                set_title.set(alarm.title);
                set_target_time.set(alarm.target_time);
                set_timezone_id.set(alarm.timezone_id);
                set_recurrence.set(alarm.recurrence);
                set_custom_days.set(if alarm.custom_days.is_empty() {
                    default_custom_days()
                } else {
                    alarm.custom_days
                });
                set_default_snooze_minutes.set(alarm.default_snooze_minutes.max(1));
                set_sound_enabled.set(alarm.sound_enabled);
            } else {
                set_title.set("New Alarm".to_string());
                set_target_time.set("08:00".to_string());
                set_timezone_id.set("UTC".to_string());
                set_recurrence.set(Recurrence::Once);
                set_custom_days.set(default_custom_days());
                set_default_snooze_minutes.set(10);
                set_sound_enabled.set(true);
            }
        }
    });

    let close_modal = move || {
        modal_state.show_alarm_form.set(false);
        modal_state.editing_alarm.set(None);
        set_form_error.set(None);
    };

    let save_alarm = move |_| {
        let selected_recurrence = recurrence.get_untracked();
        let selected_custom_days = if selected_recurrence == Recurrence::Custom {
            let chosen_days = custom_days.get_untracked();
            if chosen_days.is_empty() {
                set_form_error.set(Some(
                    "Pick at least one day when using a custom recurrence.".to_string(),
                ));
                return;
            }
            chosen_days
        } else {
            Vec::new()
        };

        let mut alarms = global_alarms.alarms.get_untracked();
        let current_time = Utc::now().timestamp_millis() as u64;
        let selected_time = target_time.get_untracked();
        let selected_timezone = timezone_id.get_untracked();
        let selected_snooze_minutes = default_snooze_minutes.get_untracked().max(1);
        let selected_sound_enabled = sound_enabled.get_untracked();
        let target_date = if selected_recurrence == Recurrence::Once {
            crate::utils::next_alarm_date_key(
                current_time as i64,
                &selected_timezone,
                &selected_time,
            )
        } else {
            None
        };

        if let Some(mut existing) = modal_state.editing_alarm.get_untracked() {
            existing.title = title.get_untracked();
            existing.target_time = selected_time.clone();
            existing.target_date = target_date.clone();
            existing.timezone_id = selected_timezone.clone();
            existing.recurrence = selected_recurrence.clone();
            existing.custom_days = selected_custom_days.clone();
            existing.state = AlarmState::Active;
            existing.snooze_count = 0;
            existing.default_snooze_minutes = selected_snooze_minutes;
            existing.snooze_until = None;
            existing.sound_enabled = selected_sound_enabled;
            existing.last_triggered_at = None;

            if let Some(idx) = alarms.iter().position(|a| a.id == existing.id) {
                alarms[idx] = existing;
            }
        } else {
            let new_alarm = Alarm {
                id: Uuid::new_v4().to_string(),
                title: title.get_untracked(),
                target_time: selected_time,
                target_date,
                timezone_id: selected_timezone,
                recurrence: selected_recurrence,
                custom_days: selected_custom_days,
                enabled: true,
                state: AlarmState::Active,
                snooze_count: 0,
                default_snooze_minutes: selected_snooze_minutes,
                snooze_until: None,
                sound_enabled: selected_sound_enabled,
                last_triggered_at: None,
                created_at: current_time,
            };
            alarms.push(new_alarm);
        }

        crate::store::apply_alarms(global_alarms, alarm_attention, alarms.clone());
        spawn_local(async move {
            crate::store::save_alarms(alarms).await;
            let _ = crate::client::ensure_notification_permission().await;
        });

        close_modal();
    };

    view! {
        <div class="search-modal" style=move || if is_open() { "display: flex;" } else { "display: none;" }>
            <div class="search-modal__content" style="max-width: 460px; padding: var(--spacing-xl);">
                <h2 style="margin-top: 0; margin-bottom: var(--spacing-lg); font-size: var(--font-size-xl); letter-spacing: -0.01em;">
                    {move || if modal_state.editing_alarm.get().is_some() { "Edit Alarm" } else { "Add Alarm" }}
                </h2>

                <div class="form-group">
                    <label>"Title"</label>
                    <input
                        type="text"
                        placeholder="Alarm title..."
                        prop:value=title
                        on:input=move |ev| set_title.set(event_target_value(&ev))
                    />
                </div>

                <div class="form-group">
                    <label>"Time"</label>
                    <input
                        type="time"
                        prop:value=target_time
                        on:input=move |ev| set_target_time.set(event_target_value(&ev))
                    />
                </div>

                <div class="form-group">
                    <label>"Timezone"</label>
                    <select
                        prop:value=timezone_id
                        on:change=move |ev| set_timezone_id.set(event_target_value(&ev))
                    >
                        <optgroup label="Your Clocks">
                            <For
                                each=move || clock_state.zones.get()
                                key=|z| z.id.clone()
                                children=move |zone| {
                                    view! { <option value=zone.iana_id.clone()>{zone.display_label.clone()}</option> }
                                }
                            />
                        </optgroup>
                        <optgroup label="Default">
                            <option value="UTC">"UTC"</option>
                        </optgroup>
                    </select>
                </div>

                <div class="form-group">
                    <label>"Recurrence"</label>
                    <select
                        on:change=move |ev| {
                            let val = event_target_value(&ev);
                            let rec = match val.as_str() {
                                "Once" => Recurrence::Once,
                                "Daily" => Recurrence::Daily,
                                "Weekdays" => Recurrence::Weekdays,
                                "Weekends" => Recurrence::Weekends,
                                "Custom" => Recurrence::Custom,
                                _ => Recurrence::Once,
                            };
                            if rec == Recurrence::Custom && custom_days.get_untracked().is_empty() {
                                set_custom_days.set(default_custom_days());
                            }
                            set_recurrence.set(rec);
                        }
                    >
                        <option value="Once" selected=move || recurrence.get() == Recurrence::Once>"Once"</option>
                        <option value="Daily" selected=move || recurrence.get() == Recurrence::Daily>"Daily"</option>
                        <option value="Weekdays" selected=move || recurrence.get() == Recurrence::Weekdays>"Weekdays"</option>
                        <option value="Weekends" selected=move || recurrence.get() == Recurrence::Weekends>"Weekends"</option>
                        <option value="Custom" selected=move || recurrence.get() == Recurrence::Custom>"Custom"</option>
                    </select>
                </div>

                <Show when=move || recurrence.get() == Recurrence::Custom>
                    <div class="form-group">
                        <label>"Repeat On"</label>
                        <div style="display: grid; grid-template-columns: repeat(7, minmax(0, 1fr)); gap: 0.45rem;">
                            <For
                                each=move || WEEKDAY_OPTIONS.to_vec()
                                key=|(day_idx, _, _)| *day_idx
                                children=move |(day_idx, short_label, long_label)| {
                                    view! {
                                        <button
                                            type="button"
                                            aria-label=long_label
                                            class=move || {
                                                if custom_days.get().contains(&day_idx) {
                                                    "btn-primary"
                                                } else {
                                                    "btn-secondary"
                                                }
                                            }
                                            style="padding: 0.55rem 0.4rem; min-width: 0;"
                                            on:click=move |_| {
                                                set_custom_days.update(|days| {
                                                    let next = toggle_custom_day(days.clone(), day_idx);
                                                    *days = next;
                                                });
                                                set_form_error.set(None);
                                            }
                                        >
                                            {short_label}
                                        </button>
                                    }
                                }
                            />
                        </div>
                    </div>
                </Show>

                <div class="form-group">
                    <label>"Default Snooze"</label>
                    <select
                        prop:value=move || default_snooze_minutes.get().to_string()
                        on:change=move |ev| {
                            let value = event_target_value(&ev).parse::<u32>().unwrap_or(10).max(1);
                            set_default_snooze_minutes.set(value);
                        }
                    >
                        <option value="5">"5 minutes"</option>
                        <option value="10">"10 minutes"</option>
                        <option value="15">"15 minutes"</option>
                        <option value="30">"30 minutes"</option>
                    </select>
                </div>

                <div class="form-group">
                    <label style="display: flex; align-items: center; justify-content: space-between; gap: 1rem;">
                        <span>"Play Sound"</span>
                        <crate::components::ui::Toggle
                            checked=Signal::derive(move || sound_enabled.get())
                            on_change=move |checked| set_sound_enabled.set(checked)
                        />
                    </label>
                </div>

                <Show when=move || form_error.get().is_some()>
                    <div style="margin-bottom: var(--spacing-md); padding: 0.75rem 0.9rem; border-radius: 0.9rem; background: rgba(239, 68, 68, 0.12); color: #b91c1c; font-size: var(--font-size-sm);">
                        {move || form_error.get().unwrap_or_default()}
                    </div>
                </Show>

                <div class="modal-actions">
                    <button
                        class="btn-secondary"
                        on:click=move |_| close_modal()
                    >
                        "Cancel"
                    </button>
                    <button
                        class="btn-primary"
                        on:click=save_alarm
                    >
                        "Save Alarm"
                    </button>
                </div>
            </div>

            <div
                style="position: absolute; top: 0; left: 0; right: 0; bottom: 0; z-index: -1;"
                on:click=move |_| close_modal()
            ></div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toggling_custom_days_adds_and_removes_sorted_values() {
        let days = toggle_custom_day(vec![1, 3, 5], 3);
        assert_eq!(days, vec![1, 5]);

        let days = toggle_custom_day(vec![1, 5], 2);
        assert_eq!(days, vec![1, 2, 5]);
    }

    #[test]
    fn default_custom_days_prefers_weekdays() {
        assert_eq!(default_custom_days(), vec![1, 2, 3, 4, 5]);
    }
}
