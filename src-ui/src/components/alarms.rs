use leptos::*;
use crate::models::{Alarm, AlarmState, Recurrence};
use crate::state::UserPreferencesState;
use crate::utils::get_timezone_metadata;

#[component]
pub fn AlarmCard(alarm: Alarm) -> impl IntoView {
    let alarm_for_edit = alarm.clone();
    let prefs_ctx = use_context::<UserPreferencesState>().expect("UserPreferencesState missing");
    let is_12h = move || prefs_ctx.prefs.get().time_format == "12h";

    let time_str = move || {
        if is_12h() {
            let parts: Vec<&str> = alarm.target_time.split(':').collect();
            if parts.len() == 2 {
                let h: u32 = parts[0].parse().unwrap_or(0);
                let m = parts[1];
                let period = if h >= 12 { "PM" } else { "AM" };
                let h12 = if h.is_multiple_of(12) { 12 } else { h % 12 };
                format!("{}:{} {}", h12, m, period)
            } else {
                alarm.target_time.clone()
            }
        } else {
            alarm.target_time.clone()
        }
    };

    let metadata = get_timezone_metadata(&alarm.timezone_id);
    let tz_display = metadata.and_then(|m| m.city).map(|c| format!("in {}", c)).unwrap_or_else(|| format!("in {}", alarm.timezone_id));

    let recurrence_str = match alarm.recurrence {
        Recurrence::Once => "Once",
        Recurrence::Daily => "Daily",
        Recurrence::Weekdays => "Weekdays",
        Recurrence::Weekends => "Weekends",
        Recurrence::Custom => {
            "Custom"
        }
    };

    let state_str = match alarm.state {
        AlarmState::Active => "Active",
        AlarmState::Snoozed => "Snoozed",
        AlarmState::Completed => "Completed",
        AlarmState::Dismissed => "Dismissed",
    };

    let state_class = match alarm.state {
        AlarmState::Active => "alarm-state-active",
        AlarmState::Snoozed => "alarm-state-snoozed",
        AlarmState::Completed => "alarm-state-completed",
        AlarmState::Dismissed => "alarm-state-dismissed",
    };

    let badge_style = match alarm.state {
        AlarmState::Completed | AlarmState::Dismissed => "background: var(--color-gray-200); color: var(--color-gray-600);",
        AlarmState::Snoozed => "background: rgba(245, 158, 11, 0.15); color: var(--color-warning);",
        _ => "background: rgba(16, 185, 129, 0.15); color: var(--color-success);",
    };

    let time_opacity = match alarm.state {
        AlarmState::Completed | AlarmState::Dismissed => "0.5",
        _ => "1.0",
    };

    let alarm_id_for_delete = alarm.id.clone();

    view! {
        <div class=format!("alarm-card {}", state_class)>
            <div class="alarm-card__header">
                <div class="alarm-card__title">{alarm.title.clone()}</div>
                <crate::components::ui::Toggle 
                    checked=Signal::derive(move || alarm.enabled)
                    on_change={
                        let alarm_id = alarm.id.clone();
                        move |checked| {
                            let alarm_state = use_context::<crate::state::AlarmState>().expect("missing");
                            let mut alarms = alarm_state.alarms.get_untracked();
                            if let Some(a) = alarms.iter_mut().find(|a| a.id == alarm_id) {
                                a.enabled = checked;
                                alarm_state.alarms.set(alarms.clone());
                                spawn_local(async move {
                                    crate::ipc::set_store_value("alarms.dat", "alarms", alarms).await;
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
            </div>
            <div class="alarm-card__footer">
                <div class="alarm-card__badge" style=badge_style>{state_str}</div>
                <div class="alarm-card__actions" style="display: flex; gap: var(--spacing-sm);">
                    <button 
                        class="alarm-card__edit" 
                        aria-label="Edit Alarm"
                        on:click=move |_| {
                            let modal_state = use_context::<crate::state::ModalState>().expect("missing");
                            modal_state.editing_alarm.set(Some(alarm_for_edit.clone()));
                            modal_state.show_alarm_form.set(true);
                        }
                    >
                        "✏️"
                    </button>
                    <button 
                        class="alarm-card__delete" 
                        aria-label="Delete Alarm"
                        on:click=move |_| {
                            if web_sys::window().unwrap().confirm_with_message("Delete this alarm?").unwrap_or(false) {
                                let alarm_state = use_context::<crate::state::AlarmState>().expect("missing");
                                let mut alarms = alarm_state.alarms.get_untracked();
                                alarms.retain(|a| a.id != alarm_id_for_delete);
                                alarm_state.alarms.set(alarms.clone());
                                spawn_local(async move {
                                    crate::ipc::set_store_value("alarms.dat", "alarms", alarms).await;
                                });
                            }
                        }
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

    view! {
        <div class="alarm-list-view">
            <div class="alarm-list-header" style="display: flex; justify-content: space-between; align-items: center; margin-bottom: var(--spacing-lg);">
                <h2 style="margin: 0; font-size: 2rem; letter-spacing: -0.02em;">"Alarms"</h2>
                <button 
                    aria-label="Add Alarm" 
                    style="margin: 0;"
                    on:click=move |_| {
                        modal_state.editing_alarm.set(None);
                        modal_state.show_alarm_form.set(true);
                    }
                >
                    "+ Add Alarm"
                </button>
            </div>
            
            <Show 
                when=move || !alarm_state.alarms.get().is_empty() 
                fallback=|| view! { 
                    <div class="alarm-list__empty">
                        <p style="font-weight: 500; margin: 0;">"No alarms yet"</p>
                    </div> 
                }
            >
                <div class="alarm-list" role="list">
                    <For
                        each=move || alarm_state.alarms.get()
                        key=|a| a.id.clone()
                        children=move |alarm| view! { <AlarmCard alarm=alarm/> }
                    />
                </div>
            </Show>
        </div>
    }
}