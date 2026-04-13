use leptos::*;
use crate::state::{ClockState, ModalState};
use crate::models::{TimeZoneEntry, TimezoneMetadata};
use crate::utils::TIMEZONES;
use chrono::Utc;

#[derive(Clone, PartialEq)]
struct SearchResult {
    tz: TimezoneMetadata,
    match_name: String,
}

#[component]
pub fn TimezoneSearchModal() -> impl IntoView {
    let modal_state = use_context::<ModalState>().expect("ModalState missing");
    let clock_state = use_context::<ClockState>().expect("ClockState missing");

    let (search_query, set_search_query) = create_signal("".to_string());
    let (selected_index, set_selected_index) = create_signal(0_usize);
    let (warning, set_warning) = create_signal("".to_string());

    let results = create_memo(move |_| {
        let query = search_query.get().to_lowercase();
        if query.is_empty() {
            return vec![];
        }

        let mut matched = Vec::new();
        TIMEZONES.with(|map| {
            for (_, tz) in map.iter() {
                let mut match_name = None;
                
                if let Some(city) = &tz.city {
                    if city.to_lowercase().contains(&query) {
                        match_name = Some(city.clone());
                    }
                }
                
                if match_name.is_none() {
                    for alias in &tz.aliases {
                        if alias.to_lowercase().contains(&query) {
                            match_name = Some(alias.clone());
                            break;
                        }
                    }
                }
                
                if match_name.is_none() && tz.id.to_lowercase().contains(&query) {
                    match_name = Some(tz.id.clone());
                }
                
                if match_name.is_none() {
                    if let Some(country) = &tz.country {
                        if country.to_lowercase().contains(&query) {
                            match_name = tz.city.clone().or_else(|| Some(tz.id.clone()));
                        }
                    }
                }
                
                if let Some(name) = match_name {
                    matched.push(SearchResult {
                        tz: tz.clone(),
                        match_name: name,
                    });
                }
            }
        });

        // Sort results to be deterministic
        matched.sort_by(|a, b| a.match_name.cmp(&b.match_name));
        matched.truncate(50); // limit
        
        if matched.is_empty() {
            set_selected_index.set(0);
        } else {
            // clamp
            let idx = selected_index.get_untracked();
            if idx >= matched.len() {
                set_selected_index.set(matched.len() - 1);
            }
        }
        
        matched
    });

    let is_open = move || modal_state.show_timezone_search.get();
    let search_state = move || crate::product::timezone_search_copy(
        &search_query.get(),
        results.get().len(),
    );

    let close_modal = move || {
        modal_state.show_timezone_search.set(false);
        set_search_query.set("".into());
        set_warning.set("".into());
    };

    let add_selected = move |idx: usize| {
        let res = results.get_untracked();
        if idx < res.len() {
            let result = &res[idx];
            let mut clocks = clock_state.zones.get_untracked();
            
            if clocks.iter().any(|c| c.iana_id == result.tz.id) {
                set_warning.set(format!("{} is already in your clock list.", result.tz.id));
                return;
            }

            if clocks.len() >= 30 {
                set_warning.set("Maximum of 30 timezones reached.".into());
                return;
            }

            clocks.push(TimeZoneEntry {
                id: uuid::Uuid::new_v4().to_string(),
                iana_id: result.tz.id.clone(),
                display_label: result.match_name.clone(),
                sort_order: clocks.len(),
                pinned_to_tray: false,
                created_at: Utc::now().timestamp_millis() as u64,
            });

            crate::store::apply_zones(clock_state, clocks.clone());
            spawn_local(async move {
                crate::store::save_zones(clocks).await;
            });
            close_modal();
        }
    };

    view! {
        <div class="search-modal" style=move || if is_open() { "display: flex;" } else { "display: none;" }>
            <div class="search-modal__content">
                <input 
                    type="text" 
                    class="search-modal__input" 
                    placeholder="Search timezone (e.g., Tokyo, Houston, UTC+9)..."
                    prop:value=search_query
                    on:input=move |ev| {
                        set_search_query.set(event_target_value(&ev));
                        set_warning.set("".into());
                        set_selected_index.set(0);
                    }
                    on:keydown=move |ev| {
                        let key = ev.key();
                        if key == "Escape" {
                            close_modal();
                        } else if key == "ArrowDown" {
                            ev.prevent_default();
                            let len = results.get_untracked().len();
                            let mut idx = selected_index.get_untracked();
                            if len > 0 && idx < len - 1 {
                                idx += 1;
                                set_selected_index.set(idx);
                            }
                        } else if key == "ArrowUp" {
                            ev.prevent_default();
                            let mut idx = selected_index.get_untracked();
                            if idx > 0 {
                                idx -= 1;
                                set_selected_index.set(idx);
                            }
                        } else if key == "Enter" {
                            ev.prevent_default();
                            add_selected(selected_index.get_untracked());
                        }
                    }
                />
                
                <div class="search-modal__warning" style=move || if warning.get().is_empty() { "display: none;" } else { "display: block;" }>
                    {move || warning.get()}
                </div>

                <div style="margin-bottom: 0.85rem; padding: 0.85rem 0.95rem; border-radius: 0.9rem; background: rgba(148, 163, 184, 0.08); border: 1px solid rgba(148, 163, 184, 0.18);">
                    <div style="font-size: 0.96rem; font-weight: 700; margin-bottom: 0.25rem;">
                        {move || search_state().title}
                    </div>
                    <div style="color: var(--theme-muted); line-height: 1.45; font-size: var(--font-size-sm);">
                        {move || search_state().body}
                    </div>
                </div>

                <ul class="search-modal__results">
                    <For
                        each=move || results.get().into_iter().enumerate()
                        key=|(i, r)| format!("{}-{}", r.tz.id, i)
                        children=move |(idx, result)| {
                            let offset_hours = result.tz.utc_offset_minutes as f32 / 60.0;
                            let offset_str = if offset_hours >= 0.0 {
                                format!("+{}", offset_hours)
                            } else {
                                format!("{}", offset_hours)
                            };

                            let is_selected = move || selected_index.get() == idx;
                            let class_name = move || {
                                if is_selected() {
                                    "search-modal__result-item selected"
                                } else {
                                    "search-modal__result-item"
                                }
                            };

                            let match_name = result.match_name.clone();

                            view! {
                                <li 
                                    class=class_name
                                    on:click=move |_| {
                                        set_selected_index.set(idx);
                                        add_selected(idx);
                                    }
                                    on:mouseenter=move |_| {
                                        set_selected_index.set(idx);
                                    }
                                >
                                    <span style="font-weight: var(--font-weight-bold); margin-right: var(--spacing-sm);">
                                        {match_name}
                                    </span>
                                    <span style="color: var(--theme-muted); font-size: var(--font-size-sm);">
                                        {format!("({}) UTC{}", result.tz.country.unwrap_or_else(|| "N/A".into()), offset_str)}
                                    </span>
                                </li>
                            }
                        }
                    />
                </ul>
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
    #[test]
    fn timezone_search_copy_covers_idle_and_results_states() {
        let idle = crate::product::timezone_search_copy("", 0);
        assert!(idle.title.contains("Search by city"));

        let ready = crate::product::timezone_search_copy("paris", 2);
        assert_eq!(ready.title, "2 matches ready");
    }
}
