pub mod commands;
pub mod tray;

use commands::system::*;
use commands::tray::*;
use commands::window::*;
use commands::alarm::*;
use commands::telemetry::*;
use commands::updater::*;
use commands::export::*;
use commands::platform::*;
use commands::tz_data::*;
use commands::deep_link::*;
use commands::heartbeat::*;
use commands::store_api::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_deep_link::init());

    #[cfg(desktop)]
    {
        builder = builder
            .plugin(tauri_plugin_positioner::init())
            .plugin(tauri_plugin_autostart::init(tauri_plugin_autostart::MacosLauncher::LaunchAgent, Some(vec![])))
            .plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {}));
    }

    builder.invoke_handler(tauri::generate_handler![
            get_system_info, check_dnd_status,
            update_tray_display, hide_to_tray, show_from_tray,
            set_always_on_top, set_autostart, save_window_state, restore_window_state, toggle_compact_mode, get_platform_close_behavior,
            schedule_alarm_notification, cancel_alarm_notification, get_notification_permission, request_notification_permission,
            send_telemetry_event,
            check_for_update, install_update,
            export_user_data, import_user_data,
            request_battery_exemption, request_exact_alarm_permission,
            check_timezone_data_update,
            handle_deep_link,
            heartbeat_ack,
            load_store_value, save_store_value
        ])
        .setup(|app| {
            tray::setup_tray(app.handle())?;
            
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                use tauri::Emitter;
                let mut sequence: u64 = 0;
                loop {
                    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                    let timestamp = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_millis() as u64;
                    sequence += 1;
                    
                    #[derive(serde::Serialize, Clone)]
                    struct WebViewHeartbeatEvent {
                        timestamp: u64,
                        sequence: u64,
                    }
                    
                    let _ = app_handle.emit("webview-heartbeat", WebViewHeartbeatEvent { timestamp, sequence });
                }
            });
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
