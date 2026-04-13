use leptos::*;
use chrono::{TimeZone, Utc};
use crate::models::TimeZoneEntry;
use crate::state::{GlobalTime, UserPreferencesState};
use crate::utils::{format_time, format_date, format_timezone_offset, get_timezone_offset_minutes};
use crate::daynight::get_day_night_status;

#[component]
pub fn DayNightIndicator(
    #[prop(into)] lat: Signal<f64>,
    #[prop(into)] lng: Signal<f64>,
    #[prop(into)] timestamp: Signal<i64>
) -> impl IntoView {
    let utc_day = create_memo(move |_| timestamp.get() / (24 * 60 * 60 * 1000));
    
    let status = create_memo(move |_| {
        let ms = utc_day.get() * 24 * 60 * 60 * 1000;
        get_day_night_status(lat.get(), lng.get(), Utc.timestamp_millis_opt(ms).unwrap())
    });

    let percent = create_memo(move |_| {
        if let Some(s) = status.get() {
            if let (Some(_sunrise), Some(_sunset)) = (s.sunrise, s.sunset) {
                let t = timestamp.get();
                let start_of_day = Utc.timestamp_millis_opt(t).unwrap().date_naive().and_hms_opt(0,0,0).unwrap().and_utc().timestamp_millis();
                let ms_in_day = 24.0 * 60.0 * 60.0 * 1000.0;
                return ((t - start_of_day) as f64 / ms_in_day) * 100.0;
            }
        }
        0.0
    });

    let bg = create_memo(move |_| {
        if let Some(s) = status.get() {
            if let (Some(sunrise), Some(sunset)) = (s.sunrise, s.sunset) {
                let t = utc_day.get() * 24 * 60 * 60 * 1000;
                let start_of_day = Utc.timestamp_millis_opt(t).unwrap().date_naive().and_hms_opt(0,0,0).unwrap().and_utc().timestamp_millis();
                let ms_in_day = 24.0 * 60.0 * 60.0 * 1000.0;

                let sunrise_percent = ((sunrise.timestamp_millis() - start_of_day) as f64 / ms_in_day) * 100.0;
                let sunset_percent = ((sunset.timestamp_millis() - start_of_day) as f64 / ms_in_day) * 100.0;

                return format!("linear-gradient(to right, #1a1a2e 0%, #1a1a2e {}%, #fdb813 {}%, #fdb813 {}%, #1a1a2e {}%, #1a1a2e 100%)", 
                    sunrise_percent, sunrise_percent, sunset_percent, sunset_percent);
            }
        }
        "none".to_string()
    });

    let is_polar = move || status.get().map(|s| s.polar).unwrap_or(false);
    let is_daylight = move || status.get().map(|s| s.is_daylight).unwrap_or(true);
    
    let class = move || {
        let mut c = "daynight-indicator".to_string();
        if is_polar() {
            c.push_str(" daynight-indicator--polar");
        }
        if is_daylight() {
            c.push_str(" daynight-indicator--day");
        } else {
            c.push_str(" daynight-indicator--night");
        }
        c
    };

    let label = move || {
        let p = is_polar();
        let d = is_daylight();
        match (p, d) {
            (true, true) => "24h daylight",
            (true, false) => "24h darkness",
            (false, true) => "Daytime",
            (false, false) => "Nighttime",
        }
    };

    view! {
        <Show when=move || status.get().is_some()>
            <div class=class role="img" aria-label=label>
                <Show 
                    when=is_polar 
                    fallback=move || view! {
                        <div class="daynight-indicator__bar" style=move || format!("background: {}", bg.get())></div>
                        <div class="daynight-indicator__marker" style=move || format!("left: {}%", percent.get())></div>
                    }
                >
                    <div class="daynight-indicator__text" style="display: block;">{label}</div>
                </Show>
            </div>
        </Show>
    }
}

#[component]
pub fn ClockCard(zone: TimeZoneEntry) -> impl IntoView {
    let time_ctx = use_context::<GlobalTime>().expect("GlobalTime missing");
    let prefs_ctx = use_context::<UserPreferencesState>().expect("UserPreferencesState missing");

    let t = time_ctx.0;
    let iana = zone.iana_id.clone();
    let is_12h = move || prefs_ctx.prefs.get().time_format == "12h";

    let iana1 = iana.clone();
    let time_str = create_memo(move |_| format_time(t.get(), &iana1, is_12h()));
    let iana2 = iana.clone();
    let date_str = create_memo(move |_| format_date(t.get(), &iana2));

    // For badge (Tomorrow / Yesterday)
    let iana3 = iana.clone();
    let badge_info = create_memo(move |_| {
        let ts = t.get();
        // Compare target date and local date. Since we don't have JS `Date` easily without binding, let's use the local offset.
        let local_offset_mins = -js_sys::Date::new_0().get_timezone_offset() as i32;
        let target_offset_mins = get_timezone_offset_minutes(&iana3, ts);
        let diff_mins = target_offset_mins - local_offset_mins;
        
        let diff_str = if diff_mins == 0 {
            "same time".to_string()
        } else {
            let hours = diff_mins as f64 / 60.0;
            if hours > 0.0 {
                format!("+{}h", hours)
            } else {
                format!("{}h", hours)
            }
        };

        // Extremely simplified day difference check
        let local_day = (ts + (local_offset_mins as i64) * 60 * 1000) / (24 * 60 * 60 * 1000);
        let target_day = (ts + (target_offset_mins as i64) * 60 * 1000) / (24 * 60 * 60 * 1000);
        let diff_days = target_day - local_day;

        let badge = if diff_days == 1 {
            Some("Tomorrow".to_string())
        } else if diff_days == -1 {
            Some("Yesterday".to_string())
        } else if diff_days > 1 {
            Some(format!("+{} days", diff_days))
        } else if diff_days < -1 {
            Some(format!("{} days", diff_days))
        } else {
            None
        };

        let offset_str = format_timezone_offset(&iana3, ts);

        (badge, offset_str, diff_str)
    });

    let display_label = zone.display_label.clone();
    let display_label_for_blur = display_label.clone();
    let display_label_for_delete = display_label.clone();
    let meta = crate::utils::get_timezone_metadata(&zone.iana_id);
    let lat = meta.as_ref().map(|m| m.latitude).unwrap_or(0.0);
    let lng = meta.as_ref().map(|m| m.longitude).unwrap_or(0.0);

    let clock_state = use_context::<crate::state::ClockState>().expect("ClockState missing");
    let zone_id_for_delete = zone.id.clone();
    let zone_id_for_rename = zone.id.clone();

    view! {
        <div class="clock-card">
            <div class="clock-card__header">
                <div 
                    class="clock-card__title" 
                    contenteditable="true" 
                    spellcheck="false"
                    on:blur=move |ev| {
                        let new_val = crate::product::normalized_clock_label(
                            &event_target::<web_sys::HtmlElement>(&ev).inner_text(),
                            &display_label_for_blur,
                        );
                        let mut clocks = clock_state.zones.get_untracked();
                        if let Some(c) = clocks.iter_mut().find(|c| c.id == zone_id_for_rename) {
                            c.display_label = new_val.clone();
                            event_target::<web_sys::HtmlElement>(&ev).set_inner_text(&new_val);
                            crate::store::apply_zones(clock_state, clocks.clone());
                            spawn_local(async move {
                                crate::store::save_zones(clocks).await;
                            });
                        }
                    }
                    on:keydown=move |ev| {
                        if ev.key() == "Enter" {
                            ev.prevent_default();
                            let target = event_target::<web_sys::HtmlElement>(&ev);
                            let _ = target.blur();
                        }
                    }
                >
                    {display_label}
                </div>
                <Show when=move || badge_info.get().0.is_some()>
                    <div class="clock-card__badge">{move || badge_info.get().0.clone().unwrap()}</div>
                </Show>
                <button 
                    class="clock-card__delete" 
                    aria-label="Delete clock"
                    on:click=move |_| {
                        if !web_sys::window()
                            .unwrap()
                            .confirm_with_message(&crate::product::confirm_delete_clock_message(&display_label_for_delete))
                            .unwrap_or(false)
                        {
                            return;
                        }
                        let mut clocks = clock_state.zones.get_untracked();
                        clocks.retain(|c| c.id != zone_id_for_delete);
                        for (sort_order, clock) in clocks.iter_mut().enumerate() {
                            clock.sort_order = sort_order;
                        }
                        crate::store::apply_zones(clock_state, clocks.clone());
                        spawn_local(async move {
                            crate::store::save_zones(clocks).await;
                        });
                    }
                >
                    "×"
                </button>
            </div>
            <div class="clock-card__time">{time_str}</div>
            <div class="clock-card__date">{date_str}</div>
            <div class="clock-card__footer">
                <div class="clock-card__offset">{move || badge_info.get().1.clone()}</div>
                <div class="clock-card__relative">{move || badge_info.get().2.clone()}</div>
            </div>
            <DayNightIndicator lat=move || lat lng=move || lng timestamp=t />
        </div>
    }
}

#[component]
pub fn ClockList() -> impl IntoView {
    let clock_state = use_context::<crate::state::ClockState>().expect("ClockState missing");
    let modal_state = use_context::<crate::state::ModalState>().expect("ModalState missing");
    let empty_copy = crate::product::clocks_empty_copy();
    let add_label = empty_copy.action_label.clone().unwrap_or_else(|| "+ Add Timezone".into());

    view! {
        <div class="clock-list" role="list">
            <Show when=move || !clock_state.zones.get().is_empty() fallback=move || view! {
                <div class="clock-list__empty" style="display: grid; gap: 0.85rem; padding: 1.25rem; border: 1px dashed rgba(148, 163, 184, 0.45); border-radius: 1rem; background: rgba(148, 163, 184, 0.08);">
                    <div>
                        <div style="font-size: 1rem; font-weight: 700; margin-bottom: 0.35rem;">{empty_copy.title.clone()}</div>
                        <div style="color: var(--theme-muted); line-height: 1.5;">{empty_copy.body.clone()}</div>
                    </div>
                    <div>
                        <button class="clock-list__add-btn" on:click=move |_| modal_state.show_timezone_search.set(true)>{add_label.clone()}</button>
                    </div>
                </div>
            }>
                <For
                    each=move || clock_state.zones.get()
                    key=|z| z.id.clone()
                    children=move |zone| view! { <ClockCard zone=zone/> }
                />
            </Show>
            <Show when=move || !clock_state.zones.get().is_empty()>
                <button class="clock-list__add-btn" on:click=move |_| modal_state.show_timezone_search.set(true)>"+ Add Timezone"</button>
            </Show>
        </div>
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn clock_label_normalization_trims_without_allowing_blank_names() {
        assert_eq!(
            crate::product::normalized_clock_label("  Paris HQ  ", "Fallback"),
            "Paris HQ"
        );
        assert_eq!(
            crate::product::normalized_clock_label("   ", "Fallback"),
            "Fallback"
        );
    }
}
