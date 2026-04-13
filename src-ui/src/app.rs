use leptos::*;
use crate::components::clock::ClockList;
use crate::components::alarms::AlarmList;
use crate::components::settings::SettingsPanel;
use crate::components::timezone_search::TimezoneSearchModal;
use crate::components::alarm_form::AlarmFormModal;

fn alarm_badge_text(attention: &crate::models::AlarmAttention) -> Option<String> {
    attention
        .has_attention()
        .then(|| attention.attention_count().to_string())
}

#[component]
pub fn App() -> impl IntoView {
    // initialize state
    crate::state::provide_global_time();
    crate::state::provide_clock_state();
    crate::state::provide_user_preferences();
    crate::state::provide_alarm_state();
    crate::state::provide_modal_state();

    let clock_state = use_context::<crate::state::ClockState>().expect("ClockState missing");
    let prefs_ctx = use_context::<crate::state::UserPreferencesState>().expect("UserPreferencesState missing");
    let alarm_state = use_context::<crate::state::AlarmState>().expect("AlarmState missing");
    let alarm_attention_ctx = use_context::<crate::state::AlarmAttentionState>().expect("AlarmAttentionState missing");

    let (hydrated, set_hydrated) = create_signal(false);
    create_effect(move |_| {
        if !hydrated.get() {
            spawn_local(async move {
                crate::client::hydrate_app_state(
                    clock_state,
                    prefs_ctx,
                    alarm_state,
                    alarm_attention_ctx,
                )
                .await;
                set_hydrated.set(true);
            });
        }
    });

    create_effect(move |_| {
        let prefs = prefs_ctx.prefs.get();
        let document = web_sys::window().unwrap().document().unwrap();
        let doc_el = document.document_element().unwrap();
        
        let is_dark = prefs.theme == "dark" || (prefs.theme == "system" && web_sys::window().unwrap().match_media("(prefers-color-scheme: dark)").unwrap().unwrap().matches());
        let is_hc = prefs.high_contrast == "on" || (prefs.high_contrast == "system" && web_sys::window().unwrap().match_media("(prefers-contrast: more)").unwrap().unwrap().matches());

        if is_hc {
            let _ = doc_el.set_attribute("data-theme", "high-contrast");
        } else if is_dark {
            let _ = doc_el.set_attribute("data-theme", "dark");
        } else {
            let _ = doc_el.set_attribute("data-theme", "light");
        }
    });

    let (active_tab, set_active_tab) = create_signal("Clocks".to_string());

    view! {
        <div id="app">
            <nav class="tabs-container">
                <button 
                    class=move || if active_tab.get() == "Clocks" { "tab active" } else { "tab" }
                    on:click=move |_| set_active_tab.set("Clocks".into())
                >
                    "Clocks"
                </button>
                <button 
                    class=move || if active_tab.get() == "Alarms" { "tab active" } else { "tab" }
                    on:click=move |_| set_active_tab.set("Alarms".into())
                >
                    "Alarms"
                    <Show when=move || alarm_badge_text(&alarm_attention_ctx.attention.get()).is_some()>
                        <span style="margin-left: 0.45rem; padding: 0.12rem 0.45rem; border-radius: 999px; background: rgba(239, 68, 68, 0.14); color: #b91c1c; font-size: 0.78rem; font-weight: 700;">
                            {move || alarm_badge_text(&alarm_attention_ctx.attention.get()).unwrap_or_default()}
                        </span>
                    </Show>
                </button>
                <button 
                    class=move || if active_tab.get() == "Settings" { "tab active" } else { "tab" }
                    on:click=move |_| set_active_tab.set("Settings".into())
                >
                    "Settings"
                </button>
            </nav>
            <main class="content-area">
                <div class="panel">
                    <Show
                        when=move || hydrated.get()
                        fallback=move || view! {
                            {
                                let copy = crate::product::app_boot_copy();
                                view! {
                            <div style="padding: 2.25rem 1.5rem; text-align: center;">
                                <div style="font-size: 1.1rem; font-weight: 700; margin-bottom: 0.5rem;">
                                    {copy.title}
                                </div>
                                <div style="color: var(--theme-muted); line-height: 1.5;">
                                    {copy.body}
                                </div>
                            </div>
                                }
                            }
                        }
                    >
                        {move || {
                            let summary = crate::workflow::session_summary(
                                clock_state.zones.get().len(),
                                alarm_state.alarms.get().len(),
                                &alarm_attention_ctx.attention.get(),
                            );
                            view! {
                                <div style="margin-bottom: 1rem; padding: 0.95rem 1rem; border-radius: 1rem; background: rgba(148, 163, 184, 0.08); border: 1px solid rgba(148, 163, 184, 0.16);">
                                    <div style="font-weight: 700; margin-bottom: 0.2rem;">{summary.title}</div>
                                    <div style="color: var(--theme-muted); line-height: 1.45;">{summary.body}</div>
                                </div>
                            }
                        }}
                        <Show when=move || active_tab.get() == "Clocks">
                            <ClockList />
                        </Show>
                        <Show when=move || active_tab.get() == "Alarms">
                            <AlarmList />
                        </Show>
                        <Show when=move || active_tab.get() == "Settings">
                            <SettingsPanel />
                        </Show>
                    </Show>
                </div>
            </main>
            <TimezoneSearchModal />
            <AlarmFormModal />
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recovered_alarm_attention_produces_tab_badge_text() {
        let attention = crate::models::AlarmAttention {
            triggered_alarm_ids: vec!["a1".into()],
            snoozed_alarm_ids: vec!["a2".into(), "a3".into()],
        };

        assert_eq!(alarm_badge_text(&attention).as_deref(), Some("3"));
        assert_eq!(
            alarm_badge_text(&crate::models::AlarmAttention::default()),
            None
        );
    }

    #[test]
    fn boot_message_only_exists_while_hydrating() {
        let copy = crate::product::app_boot_copy();
        assert_eq!(copy.body, "Syncing clocks, alarms, and preferences...");
    }

    #[test]
    fn product_boot_copy_matches_boot_message() {
        let copy = crate::product::app_boot_copy();
        assert_eq!(copy.title, "Preparing your workspace");
        assert_eq!(copy.body, "Syncing clocks, alarms, and preferences...");
    }

    #[test]
    fn session_summary_copy_promotes_attention_state() {
        let summary = crate::workflow::session_summary(
            2,
            3,
            &crate::models::AlarmAttention {
                triggered_alarm_ids: vec!["a1".into()],
                snoozed_alarm_ids: vec![],
            },
        );

        assert!(summary.title.contains("1 alarm needs attention"));
        assert!(summary.body.contains("Alarms tab"));
    }
}
