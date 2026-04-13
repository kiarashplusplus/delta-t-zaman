use leptos::*;
use crate::models::{Alarm, AlarmState, Recurrence};
use crate::state::{AlarmState as GlobalAlarmState, ModalState, ClockState};
use chrono::Utc;
use uuid::Uuid;

#[component]
pub fn AlarmFormModal() -> impl IntoView {
    let modal_state = use_context::<ModalState>().expect("ModalState missing");
    let global_alarms = use_context::<GlobalAlarmState>().expect("GlobalAlarmState missing");
    let clock_state = use_context::<ClockState>().expect("ClockState missing");

    let is_open = move || modal_state.show_alarm_form.get();

    let (title, set_title) = create_signal("".to_string());
    let (target_time, set_target_time) = create_signal("08:00".to_string());
    let (timezone_id, set_timezone_id) = create_signal("UTC".to_string());
    let (recurrence, set_recurrence) = create_signal(Recurrence::Once);

    // Watch for when an editing alarm is set, and populate fields
    create_effect(move |_| {
        if is_open() {
            if let Some(alarm) = modal_state.editing_alarm.get() {
                set_title.set(alarm.title);
                set_target_time.set(alarm.target_time);
                set_timezone_id.set(alarm.timezone_id);
                set_recurrence.set(alarm.recurrence);
            } else {
                set_title.set("New Alarm".to_string());
                set_target_time.set("08:00".to_string());
                set_timezone_id.set("UTC".to_string());
                set_recurrence.set(Recurrence::Once);
            }
        }
    });

    let close_modal = move || {
        modal_state.show_alarm_form.set(false);
        modal_state.editing_alarm.set(None);
    };

    let save_alarm = move |_| {
        let mut alarms = global_alarms.alarms.get_untracked();
        let current_time = Utc::now().timestamp_millis() as u64;

        if let Some(mut existing) = modal_state.editing_alarm.get_untracked() {
            // Update existing
            existing.title = title.get_untracked();
            existing.target_time = target_time.get_untracked();
            existing.timezone_id = timezone_id.get_untracked();
            existing.recurrence = recurrence.get_untracked();
            
            if let Some(idx) = alarms.iter().position(|a| a.id == existing.id) {
                alarms[idx] = existing;
            }
        } else {
            // Create new
            let new_alarm = Alarm {
                id: Uuid::new_v4().to_string(),
                title: title.get_untracked(),
                target_time: target_time.get_untracked(),
                timezone_id: timezone_id.get_untracked(),
                recurrence: recurrence.get_untracked(),
                custom_days: vec![],
                enabled: true,
                state: AlarmState::Active,
                snooze_count: 0,
                sound_enabled: true,
                created_at: current_time,
            };
            alarms.push(new_alarm);
        }

        global_alarms.alarms.set(alarms.clone());
        spawn_local(async move {
            crate::ipc::set_store_value("alarms.dat", "alarms", alarms).await;
        });

        close_modal();
    };

    view! {
        <div class="search-modal" style=move || if is_open() { "display: flex;" } else { "display: none;" }>
            <div class="search-modal__content" style="max-width: 400px; padding: var(--spacing-xl);">
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
                        // First list user's active clocks
                        <optgroup label="Your Clocks">
                            <For
                                each=move || clock_state.zones.get()
                                key=|z| z.id.clone()
                                children=move |zone| {
                                    view! { <option value=zone.iana_id.clone()>{zone.display_label.clone()}</option> }
                                }
                            />
                        </optgroup>
                        // Provide a default fallback if none
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
