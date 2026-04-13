import { invoke } from '@tauri-apps/api/core';
import type { SystemInfo, PlatformCloseBehavior, WindowState } from '../types/preferences';

export async function getSystemInfo(): Promise<SystemInfo> {
  return invoke<SystemInfo>('get_system_info');
}

export async function updateTrayDisplay(zones: any[]): Promise<void> {
  return invoke('update_tray_display', { zones });
}

export async function setAlwaysOnTop(enabled: boolean): Promise<void> {
  return invoke('set_always_on_top', { enabled });
}

export async function setAutostart(enabled: boolean): Promise<void> {
  return invoke('set_autostart', { enabled });
}

export async function hideToTray(): Promise<void> {
  return invoke('hide_to_tray');
}

export async function showFromTray(): Promise<void> {
  return invoke('show_from_tray');
}

export async function scheduleAlarmNotification(config: any): Promise<any> {
  return invoke('schedule_alarm_notification', { config });
}

export async function cancelAlarmNotification(notificationId: string): Promise<void> {
  return invoke('cancel_alarm_notification', { notificationId });
}

export async function getNotificationPermission(): Promise<string> {
  return invoke('get_notification_permission');
}

export async function requestNotificationPermission(): Promise<string> {
  return invoke('request_notification_permission');
}

export async function sendTelemetryEvent(event: any): Promise<void> {
  return invoke('send_telemetry_event', { event });
}

export async function checkForUpdate(): Promise<any> {
  return invoke('check_for_update');
}

export async function installUpdate(): Promise<any> {
  return invoke('install_update');
}

export async function saveWindowState(state: WindowState): Promise<void> {
  return invoke('save_window_state', { state });
}

export async function restoreWindowState(): Promise<WindowState> {
  return invoke<WindowState>('restore_window_state');
}

export async function exportUserData(filePath: string): Promise<any> {
  return invoke('export_user_data', { filePath });
}

export async function importUserData(config: any): Promise<any> {
  return invoke('import_user_data', { config });
}

export async function requestBatteryExemption(): Promise<any> {
  return invoke('request_battery_exemption');
}

export async function toggleCompactMode(enable: boolean): Promise<any> {
  return invoke('toggle_compact_mode', { enable });
}

export async function checkTimezoneDataUpdate(): Promise<any> {
  return invoke('check_timezone_data_update');
}

export async function getPlatformCloseBehavior(): Promise<PlatformCloseBehavior> {
  return invoke<PlatformCloseBehavior>('get_platform_close_behavior');
}

export async function handleDeepLink(url: string): Promise<any> {
  return invoke('handle_deep_link', { url });
}

export async function heartbeatAck(sequence: number): Promise<void> {
  return invoke('heartbeat_ack', { sequence });
}

export async function checkDndStatus(): Promise<any> {
  return invoke('check_dnd_status');
}

export async function requestExactAlarmPermission(): Promise<any> {
  return invoke('request_exact_alarm_permission');
}
