use leptos::*;
use crate::components::clock::ClockList;
use crate::components::alarms::AlarmList;
use crate::components::settings::SettingsPanel;
use crate::components::timezone_search::TimezoneSearchModal;
use crate::components::alarm_form::AlarmFormModal;

#[component]
pub fn App() -> impl IntoView {
    // initialize state
    crate::state::provide_global_time();
    crate::state::provide_clock_state();
    crate::state::provide_user_preferences();
    crate::state::provide_alarm_state();
    crate::state::provide_modal_state();

    let prefs_ctx = use_context::<crate::state::UserPreferencesState>().expect("UserPreferencesState missing");

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
                    <Show when=move || active_tab.get() == "Clocks">
                        <ClockList />
                    </Show>
                    <Show when=move || active_tab.get() == "Alarms">
                        <AlarmList />
                    </Show>
                    <Show when=move || active_tab.get() == "Settings">
                        <SettingsPanel />
                    </Show>
                </div>
            </main>
            <TimezoneSearchModal />
            <AlarmFormModal />
        </div>
    }
}
