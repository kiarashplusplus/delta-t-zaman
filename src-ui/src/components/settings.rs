use leptos::*;
use crate::state::UserPreferencesState;

#[component]
pub fn SettingsPanel() -> impl IntoView {
    let prefs_ctx = use_context::<UserPreferencesState>().expect("UserPreferencesState missing");

    view! {
        <div class="settings-panel">
            <div class="settings-section">
                <h3>"Display & General"</h3>
                
                <label>
                    <span>"Theme"</span>
                    <select 
                        on:change=move |ev| {
                            let mut prefs = prefs_ctx.prefs.get();
                            prefs.theme = event_target_value(&ev);
                            prefs_ctx.prefs.set(prefs);
                        }
                    >
                        <option value="system" selected=move || prefs_ctx.prefs.get().theme == "system">"System"</option>
                        <option value="light" selected=move || prefs_ctx.prefs.get().theme == "light">"Light"</option>
                        <option value="dark" selected=move || prefs_ctx.prefs.get().theme == "dark">"Dark"</option>
                    </select>
                </label>
                
                <label>
                    <span>"High Contrast"</span>
                    <select
                        on:change=move |ev| {
                            let mut prefs = prefs_ctx.prefs.get();
                            prefs.high_contrast = event_target_value(&ev);
                            prefs_ctx.prefs.set(prefs.clone());
                            spawn_local(async move {
                                crate::ipc::set_store_value("preferences.dat", "preferences", prefs).await;
                            });
                        }
                    >
                        <option value="system" selected=move || prefs_ctx.prefs.get().high_contrast == "system">"System"</option>
                        <option value="on" selected=move || prefs_ctx.prefs.get().high_contrast == "on">"On"</option>
                        <option value="off" selected=move || prefs_ctx.prefs.get().high_contrast == "off">"Off"</option>
                    </select>
                </label>

                <label>
                    <span>"Time Format"</span>
                    <select
                        on:change=move |ev| {
                            let mut prefs = prefs_ctx.prefs.get();
                            prefs.time_format = event_target_value(&ev);
                            prefs_ctx.prefs.set(prefs.clone());
                            spawn_local(async move {
                                crate::ipc::set_store_value("preferences.dat", "preferences", prefs).await;
                            });
                        }
                    >
                        <option value="12h" selected=move || prefs_ctx.prefs.get().time_format == "12h">"12-hour"</option>
                        <option value="24h" selected=move || prefs_ctx.prefs.get().time_format == "24h">"24-hour"</option>
                    </select>
                </label>

                <label>
                    <span>"Always on Top"</span>
                    <crate::components::ui::Toggle 
                        checked=Signal::derive(move || prefs_ctx.prefs.get().always_on_top)
                        on_change=move |is_checked| {
                            let mut prefs = prefs_ctx.prefs.get();
                            prefs.always_on_top = is_checked;
                            prefs_ctx.prefs.set(prefs.clone());
                            spawn_local(async move {
                                crate::ipc::set_store_value("preferences.dat", "preferences", prefs).await;
                            });
                        }
                    />
                </label>

                <label>
                    <span>"Launch on Startup"</span>
                    <crate::components::ui::Toggle 
                        checked=Signal::derive(move || prefs_ctx.prefs.get().autostart)
                        on_change=move |is_checked| {
                            let mut prefs = prefs_ctx.prefs.get();
                            prefs.autostart = is_checked;
                            prefs_ctx.prefs.set(prefs.clone());
                            spawn_local(async move {
                                crate::ipc::set_store_value("preferences.dat", "preferences", prefs).await;
                            });
                        }
                    />
                </label>
            </div>

            <div class="settings-section">
                <h3>"Privacy"</h3>
                <label>
                    <span>"Allow crash reports & analytics"</span>
                    <crate::components::ui::Toggle 
                        checked=Signal::derive(move || prefs_ctx.prefs.get().telemetry_opt_in.unwrap_or(false))
                        on_change=move |is_checked| {
                            let mut prefs = prefs_ctx.prefs.get();
                            prefs.telemetry_opt_in = Some(is_checked);
                            prefs_ctx.prefs.set(prefs.clone());
                            spawn_local(async move {
                                crate::ipc::set_store_value("preferences.dat", "preferences", prefs).await;
                            });
                        }
                    />
                </label>
            </div>

            <div class="settings-section">
                <h3>"Updates"</h3>
                <label>
                    <span>"Check for updates automatically"</span>
                    <crate::components::ui::Toggle 
                        checked=Signal::derive(move || prefs_ctx.prefs.get().auto_update)
                        on_change=move |is_checked| {
                            let mut prefs = prefs_ctx.prefs.get();
                            prefs.auto_update = is_checked;
                            prefs_ctx.prefs.set(prefs.clone());
                            spawn_local(async move {
                                crate::ipc::set_store_value("preferences.dat", "preferences", prefs).await;
                            });
                        }
                    />
                </label>
            </div>

            <div class="settings-section">
                <h3>"Data Management"</h3>
                <div style="display: flex; gap: 1rem; margin-top: 0.5rem;">
                    <button id="btn-export-data">"Export Data"</button>
                    <button id="btn-import-data">"Import Data"</button>
                </div>
            </div>
        </div>
    }
}