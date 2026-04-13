use leptos::*;
use crate::state::{AlarmState, ClockState, UserPreferencesState};

fn persist_preferences(prefs: crate::models::UserPreferences) {
    spawn_local(async move {
        crate::client::persist_preferences(prefs).await;
    });
}

#[component]
pub fn SettingsPanel() -> impl IntoView {
    let prefs_ctx = use_context::<UserPreferencesState>().expect("UserPreferencesState missing");
    let clock_state = use_context::<ClockState>().expect("ClockState missing");
    let alarm_state = use_context::<AlarmState>().expect("AlarmState missing");
    let alarm_attention_state =
        use_context::<crate::state::AlarmAttentionState>().expect("AlarmAttentionState missing");
    let (data_feedback, set_data_feedback) = create_signal(None::<crate::product::BannerCopy>);
    let (is_processing_data, set_is_processing_data) = create_signal(false);
    let (system_info, set_system_info) = create_signal(None::<crate::models::SystemInfo>);
    let (system_info_error, set_system_info_error) = create_signal(None::<String>);
    let (system_info_loaded, set_system_info_loaded) = create_signal(false);

    create_effect(move |_| {
        if !system_info_loaded.get() {
            spawn_local(async move {
                match crate::client::load_system_info().await {
                    Ok(info) => {
                        set_system_info.set(Some(info));
                        set_system_info_error.set(None);
                    }
                    Err(error) => {
                        set_system_info_error
                            .set(Some(crate::client::format_js_error(error)));
                    }
                }
                set_system_info_loaded.set(true);
            });
        }
    });

    view! {
        <div class="settings-panel">
            <div class="settings-section">
                <h3>"Display & General"</h3>

                <label>
                    <span>"Theme"</span>
                    <select
                        on:change=move |ev| {
                            let mut prefs = prefs_ctx.prefs.get_untracked();
                            prefs.theme = event_target_value(&ev);
                            prefs_ctx.prefs.set(prefs.clone());
                            persist_preferences(prefs);
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
                            let mut prefs = prefs_ctx.prefs.get_untracked();
                            prefs.high_contrast = event_target_value(&ev);
                            prefs_ctx.prefs.set(prefs.clone());
                            persist_preferences(prefs);
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
                            let mut prefs = prefs_ctx.prefs.get_untracked();
                            prefs.time_format = event_target_value(&ev);
                            prefs_ctx.prefs.set(prefs.clone());
                            persist_preferences(prefs);
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
                            let prefs_ctx = prefs_ctx;
                            let set_data_feedback = set_data_feedback;
                            spawn_local(async move {
                                if let Err(error) =
                                    crate::client::set_always_on_top_preference(prefs_ctx, is_checked).await
                                {
                                    set_data_feedback.set(Some(crate::workflow::error_notice(
                                        "Always on Top",
                                        &crate::client::format_js_error(error),
                                    )));
                                }
                            });
                        }
                    />
                </label>

                <label>
                    <span>"Launch on Startup"</span>
                    <crate::components::ui::Toggle
                        checked=Signal::derive(move || prefs_ctx.prefs.get().autostart)
                        on_change=move |is_checked| {
                            let prefs_ctx = prefs_ctx;
                            let set_data_feedback = set_data_feedback;
                            spawn_local(async move {
                                if let Err(error) =
                                    crate::client::set_autostart_preference(prefs_ctx, is_checked).await
                                {
                                    set_data_feedback.set(Some(crate::workflow::error_notice(
                                        "Launch on Startup",
                                        &crate::client::format_js_error(error),
                                    )));
                                }
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
                            let mut prefs = prefs_ctx.prefs.get_untracked();
                            prefs.telemetry_opt_in = Some(is_checked);
                            prefs_ctx.prefs.set(prefs.clone());
                            persist_preferences(prefs);
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
                            let mut prefs = prefs_ctx.prefs.get_untracked();
                            prefs.auto_update = is_checked;
                            prefs_ctx.prefs.set(prefs.clone());
                            persist_preferences(prefs);
                        }
                    />
                </label>
            </div>

            <div class="settings-section">
                <h3>"Data Management"</h3>
                <Show when=move || data_feedback.get().is_some()>
                    {move || {
                        data_feedback.get().map(|feedback| {
                            view! {
                                <div
                                    style=move || format!(
                                        "margin-top: 0.75rem; margin-bottom: 0.75rem; padding: 0.75rem 0.9rem; border-radius: 0.9rem; font-size: var(--font-size-sm); line-height: 1.45; {}",
                                        crate::product::banner_style(&feedback.tone)
                                    )
                                >
                                    <div style="font-weight: 700; margin-bottom: 0.2rem;">{feedback.title}</div>
                                    <div>{feedback.body}</div>
                                </div>
                            }
                        })
                    }}
                </Show>

                <div style="display: flex; gap: 1rem; margin-top: 0.5rem;">
                    <button
                        id="btn-export-data"
                        disabled=move || is_processing_data.get()
                        on:click=move |_| {
                            let set_data_feedback = set_data_feedback;
                            let set_is_processing_data = set_is_processing_data;

                            set_is_processing_data.set(true);
                            set_data_feedback.set(None);

                            spawn_local(async move {
                                let feedback = match crate::client::export_with_picker(
                                    "delta-t-zaman-export.json".to_string(),
                                )
                                .await
                                {
                                    Ok(crate::client::PickerFlow::Completed(result)) => {
                                        crate::workflow::export_notice(&result)
                                    }
                                    Ok(crate::client::PickerFlow::Cancelled { stage }) => {
                                        crate::workflow::cancelled_notice("Export", stage)
                                    }
                                    Err(error) => crate::workflow::error_notice(
                                        "Export",
                                        &crate::client::format_js_error(error),
                                    ),
                                };

                                set_data_feedback.set(Some(feedback));
                                set_is_processing_data.set(false);
                            });
                        }
                    >
                        {move || if is_processing_data.get() { "Working..." } else { "Export Data" }}
                    </button>
                    <button
                        id="btn-import-data"
                        disabled=move || is_processing_data.get()
                        on:click=move |_| {
                            let prefs_ctx = prefs_ctx;
                            let clock_state = clock_state;
                            let alarm_state = alarm_state;
                            let alarm_attention_state = alarm_attention_state;
                            let set_data_feedback = set_data_feedback;
                            let set_is_processing_data = set_is_processing_data;

                            set_is_processing_data.set(true);
                            set_data_feedback.set(None);

                            spawn_local(async move {
                                let feedback = match crate::client::import_with_picker_and_refresh(
                                    clock_state,
                                    prefs_ctx,
                                    alarm_state,
                                    alarm_attention_state,
                                )
                                .await
                                {
                                    Ok(crate::client::PickerFlow::Completed(result)) => {
                                        crate::workflow::import_notice(&result)
                                    }
                                    Ok(crate::client::PickerFlow::Cancelled { stage }) => {
                                        crate::workflow::cancelled_notice("Import", stage)
                                    }
                                    Err(error) => crate::workflow::error_notice(
                                        "Import",
                                        &crate::client::format_js_error(error),
                                    ),
                                };

                                set_data_feedback.set(Some(feedback));
                                set_is_processing_data.set(false);
                            });
                        }
                    >
                        {move || if is_processing_data.get() { "Working..." } else { "Import Data" }}
                    </button>
                </div>
            </div>

            <div class="settings-section">
                <h3>"System & Platform"</h3>
                <Show
                    when=move || system_info.get().is_some()
                    fallback=move || {
                        if let Some(error) = system_info_error.get() {
                            view! {
                                <div style="margin-top: 0.75rem; padding: 0.85rem 0.95rem; border-radius: 0.9rem; background: rgba(239, 68, 68, 0.08); color: #991b1b; line-height: 1.45;">
                                    {format!("System status unavailable: {}", error)}
                                </div>
                            }
                        } else {
                            view! {
                                <div style="margin-top: 0.75rem; padding: 0.85rem 0.95rem; border-radius: 0.9rem; background: rgba(59, 130, 246, 0.08); color: #1d4ed8; line-height: 1.45;">
                                    "Loading system status..."
                                </div>
                            }
                        }
                    }
                >
                    {move || {
                        system_info.get().map(|info| {
                            let rows = crate::product::system_info_rows(&info);
                            view! {
                                <div style="display: grid; gap: 0.65rem; margin-top: 0.75rem;">
                                    <For
                                        each=move || rows.clone().into_iter()
                                        key=|(label, _)| label.clone()
                                        children=move |(label, value)| {
                                            view! {
                                                <div style="display: flex; justify-content: space-between; gap: 1rem; padding: 0.75rem 0.9rem; border-radius: 0.9rem; background: rgba(148, 163, 184, 0.08); border: 1px solid rgba(148, 163, 184, 0.14);">
                                                    <span style="font-weight: 700;">{label}</span>
                                                    <span style="color: var(--theme-muted); text-align: right;">{value}</span>
                                                </div>
                                            }
                                        }
                                    />
                                </div>
                            }
                        })
                    }}
                </Show>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn import_feedback_becomes_warning_when_conflicts_exist() {
        let feedback = crate::workflow::import_notice(&crate::models::ImportResult {
            success: true,
            zones_imported: 1,
            alarms_imported: 2,
            preferences_updated: false,
            conflicts_detected: 2,
            warnings: vec!["Skipped 2 conflicting alarm entries during merge.".into()],
        });

        assert_eq!(feedback.tone, crate::product::BannerTone::Warning);
        assert!(feedback.body.contains("Skipped 2 conflicting alarm entries"));
    }

    #[test]
    fn export_feedback_mentions_written_bytes() {
        let feedback = crate::workflow::export_notice(&crate::models::ExportResult {
            success: true,
            file_path: "/tmp/export.json".into(),
            bytes_written: 512,
        });

        assert_eq!(feedback.tone, crate::product::BannerTone::Success);
        assert!(feedback.body.contains("512 bytes"));
    }

    #[test]
    fn feedback_styles_cover_each_message_kind() {
        assert!(crate::product::banner_style(&crate::product::BannerTone::Info).contains("color"));
        assert!(crate::product::banner_style(&crate::product::BannerTone::Success).contains("color"));
        assert!(crate::product::banner_style(&crate::product::BannerTone::Warning).contains("color"));
        assert!(crate::product::banner_style(&crate::product::BannerTone::Error).contains("color"));
    }
}
