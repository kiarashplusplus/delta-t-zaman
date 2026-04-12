# IPC Command Contracts: Delta-T Zaman

**Branch**: `001-tauri-world-clock` | **Date**: 2026-04-12

These contracts define the Tauri IPC boundary between the vanilla TypeScript frontend and the Rust backend. All commands are invoked via `@tauri-apps/api/core` `invoke()` and registered in `src-tauri/src/lib.rs` via `generate_handler![]`.

---

## Command Index

| Command | Direction | Purpose | Platform |
|---------|-----------|---------|----------|
| `get_system_info` | JS → Rust | Detect platform, locale, timezone | All |
| `update_tray_display` | JS → Rust | Refresh system tray tooltip/menu | Desktop |
| `set_always_on_top` | JS → Rust | Toggle floating window | Desktop |
| `set_autostart` | JS → Rust | Toggle launch-at-login | Desktop |
| `hide_to_tray` | JS → Rust | Minimize to system tray | Desktop |
| `show_from_tray` | JS → Rust | Restore window from tray | Desktop |
| `schedule_alarm_notification` | JS → Rust | Schedule a native notification | All |
| `cancel_alarm_notification` | JS → Rust | Cancel a scheduled notification | All |
| `get_notification_permission` | JS → Rust | Check notification permission | All |
| `request_notification_permission` | JS → Rust | Request notification permission | All |
| `send_telemetry_event` | JS → Rust | Send anonymous telemetry event | All |
| `check_for_update` | JS → Rust | Check for app updates | Desktop |
| `install_update` | JS → Rust | Download and install update | Desktop |
| `save_window_state` | JS → Rust | Persist window position/size | Desktop |
| `restore_window_state` | JS → Rust | Restore saved window position/size | Desktop |
| `export_user_data` | JS → Rust | Export all stores to JSON file | All |
| `import_user_data` | JS → Rust | Import stores from JSON file | All |
| `request_battery_exemption` | JS → Rust | Request Android Doze whitelist | Android |
| `toggle_compact_mode` | JS → Rust | Switch between full and compact display | Desktop |
| `check_timezone_data_update` | JS → Rust | Check for timezone data patches | All |
| `get_platform_close_behavior` | JS → Rust | Get platform-specific close behavior | All |
| `handle_deep_link` | JS → Rust | Parse and validate incoming deep link URL | All |
| `heartbeat_ack` | JS → Rust | Acknowledge WebView heartbeat ping | All |
| `check_dnd_status` | JS → Rust | Check if device is in Do Not Disturb mode | All |
| `request_exact_alarm_permission` | JS → Rust | Request SCHEDULE_EXACT_ALARM (Android 12+) | Android |

---

## Command Specifications

### `get_system_info`

Returns platform information for initializing app defaults.

**Request**: No parameters.

**Response**:
```typescript
interface SystemInfo {
  platform: 'macos' | 'windows' | 'linux' | 'ios' | 'android';
  locale: string;          // BCP 47 tag, e.g., "en-US"
  timezone: string;        // IANA timezone ID, e.g., "America/New_York"
  is_mobile: boolean;
  supports_tray: boolean;  // false on mobile, false on some Linux DEs
}
```

**Errors**: None expected — all fields are available on all platforms.

**Rust signature**:
```rust
#[tauri::command]
fn get_system_info() -> SystemInfo { ... }
```

---

### `update_tray_display`

Updates the system tray tooltip and context menu with current timezone data.

**Request**:
```typescript
interface TrayZone {
  label: string;      // e.g., "New York"
  time: string;       // e.g., "3:45 PM"
  is_daytime: boolean;
}

// Parameter: zones: TrayZone[]
```

**Response**: `void` (no return value).

**Errors**:
- `TrayUnavailable`: System tray not supported on this platform.

**Behavior**:
- Updates tray tooltip to show all pinned zones
- Rebuilds tray context menu with zone items + "Show Window" + "Quit"
- Called by frontend every 60 seconds (or on zone change)

**Rust signature**:
```rust
#[tauri::command]
fn update_tray_display(app: AppHandle, zones: Vec<TrayZone>) -> Result<(), String> { ... }
```

---

### `set_always_on_top`

Toggles the main window's always-on-top (floating) mode.

**Request**: `enabled: boolean`

**Response**: `void`

**Errors**:
- `WindowNotFound`: Main window label not found (should not happen).

**Rust signature**:
```rust
#[tauri::command]
fn set_always_on_top(window: tauri::WebviewWindow, enabled: bool) -> Result<(), String> { ... }
```

---

### `set_autostart`

Toggles whether the app launches at system login.

**Request**: `enabled: boolean`

**Response**: `void`

**Errors**:
- `PlatformUnsupported`: Called on mobile (silently ignored).
- `PermissionDenied`: OS denied autostart registration.

**Rust signature**:
```rust
#[tauri::command]
fn set_autostart(app: AppHandle, enabled: bool) -> Result<(), String> { ... }
```

---

### `hide_to_tray`

Hides the main window while keeping the app running in the system tray.

**Request**: No parameters.

**Response**: `void`

**Behavior**:
- Hides the main window (not closed/destroyed)
- Tray icon remains visible with context menu
- On macOS: app remains in menu bar
- On Linux without tray: window is minimized instead (graceful fallback)

**Rust signature**:
```rust
#[tauri::command]
fn hide_to_tray(window: tauri::WebviewWindow) -> Result<(), String> { ... }
```

---

### `show_from_tray`

Restores and focuses the main window from the system tray.

**Request**: No parameters.

**Response**: `void`

**Behavior**:
- Shows the hidden window
- Brings to foreground and focuses
- If window was never hidden, this is a no-op

**Rust signature**:
```rust
#[tauri::command]
fn show_from_tray(window: tauri::WebviewWindow) -> Result<(), String> { ... }
```

---

### `schedule_alarm_notification`

Schedules a native OS notification for an alarm. Uses `@tauri-apps/plugin-notification` internally.

**Request**:
```typescript
interface AlarmNotificationConfig {
  alarm_id: string;           // Alarm UUID for tracking
  title: string;              // Notification title
  body: string;               // Notification body text
  schedule_at: string;        // ISO 8601 datetime (UTC) for one-time
  sound_enabled: boolean;
  action_type: 'one_time' | 'recurring';
}
```

**Response**:
```typescript
interface ScheduleResult {
  notification_id: string;    // OS notification ID for cancellation
}
```

**Errors**:
- `PermissionDenied`: Notification permission not granted.
- `ScheduleFailed`: OS rejected the schedule request.
- `AlarmLimitReached`: 50 active alarm limit exceeded.

**Rust signature**:
```rust
#[tauri::command]
async fn schedule_alarm_notification(
    app: AppHandle,
    config: AlarmNotificationConfig,
) -> Result<ScheduleResult, String> { ... }
```

---

### `cancel_alarm_notification`

Cancels a previously scheduled notification.

**Request**: `notification_id: string`

**Response**: `void`

**Errors**:
- `NotFound`: Notification ID not recognized (already fired or invalid).

**Rust signature**:
```rust
#[tauri::command]
async fn cancel_alarm_notification(
    app: AppHandle,
    notification_id: String,
) -> Result<(), String> { ... }
```

---

### `get_notification_permission`

Checks the current notification permission state.

**Request**: No parameters.

**Response**:
```typescript
type PermissionState = 'granted' | 'denied' | 'prompt' | 'prompt-with-rationale';
```

**Rust signature**:
```rust
#[tauri::command]
async fn get_notification_permission(app: AppHandle) -> String { ... }
```

---

### `request_notification_permission`

Requests notification permission from the user. Shows OS permission dialog if state is `prompt`.

**Request**: No parameters.

**Response**: `PermissionState` (updated state after request).

**Behavior**:
- If `granted`: returns immediately
- If `prompt`: shows OS dialog, returns result
- If `denied`: returns `denied` (user must enable in OS settings)
- If `prompt-with-rationale` (Android): caller should show explanation first

**Rust signature**:
```rust
#[tauri::command]
async fn request_notification_permission(app: AppHandle) -> String { ... }
```

---

### `send_telemetry_event`

Sends an anonymous telemetry event to the Cloudflare Workers endpoint. Fire-and-forget — the frontend does not wait for a response (FR-033).

**Request**:
```typescript
interface TelemetryEvent {
  event_type: 'app_open' | 'zone_added' | 'zone_removed' | 'alarm_created' | 'alarm_fired' | 'crash_report';
  metadata?: Record<string, string>;  // e.g., { platform: "macos", app_version: "1.0.0" }
}
```

**Response**: `void` (fire-and-forget, no return value).

**Errors**: None — all failures are silently ignored. Never surfaces errors to the user.

**Behavior**:
- Checks `telemetry_opt_in` preference before sending. If false, returns immediately.
- Sends HTTP POST to Cloudflare Worker endpoint. Uses `reqwest` with a 5-second timeout.
- On network failure or non-200 response: silently discards. No retry, no local queue.
- Must not add more than 50ms to any user-facing operation (SC-011).

**Rust signature**:
```rust
#[tauri::command]
async fn send_telemetry_event(
    app: AppHandle,
    event: TelemetryEvent,
) -> () { ... }
```

---

### `check_for_update`

Checks the Cloudflare-hosted update manifest for a newer app version (FR-034).

**Request**: No parameters.

**Response**:
```typescript
interface UpdateCheckResult {
  update_available: boolean;
  version?: string;          // e.g., "1.2.0"
  release_notes?: string;    // Markdown release notes
  download_url?: string;     // Cloudflare R2 URL
}
```

**Errors**:
- `NetworkError`: Unable to reach update endpoint (silently handled — not shown to user on background checks).
- `PlatformUnsupported`: Called on mobile (mobile updates go through app stores).

**Behavior**:
- Called on app launch (background, non-blocking) and on-demand from settings.
- Checks `auto_update_enabled` preference. If false and not manual, returns `{ update_available: false }`.
- Completes within 5 seconds (SC-012).

**Rust signature**:
```rust
#[tauri::command]
async fn check_for_update(app: AppHandle) -> Result<UpdateCheckResult, String> { ... }
```

---

### `install_update`

Downloads and installs a pending update via `tauri-plugin-updater`.

**Request**: No parameters (uses the update detected by `check_for_update`).

**Response**:
```typescript
interface InstallResult {
  status: 'downloading' | 'installing' | 'restart_required';
  progress?: number;  // 0-100 download percentage
}
```

**Errors**:
- `NoUpdatePending`: No update available to install.
- `DownloadFailed`: Network error during binary download.
- `InstallFailed`: OS-level installation error.

**Behavior**:
- Downloads update binary from Cloudflare R2.
- Applies update via Tauri's built-in updater mechanism.
- Prompts user to restart the app when installation completes.

**Rust signature**:
```rust
#[tauri::command]
async fn install_update(app: AppHandle) -> Result<InstallResult, String> { ... }
```

---

### `save_window_state`

Persists the current window position, size, and maximized state to preferences (FR-042).

**Request**:
```typescript
interface WindowState {
  x: number;
  y: number;
  width: number;
  height: number;
  maximized: boolean;
}
```

**Response**: `void`

**Behavior**:
- Called on window move, resize, and before close (debounced to avoid excessive writes).
- Desktop only — silently ignored on mobile.

**Rust signature**:
```rust
#[tauri::command]
fn save_window_state(app: AppHandle, state: WindowState) -> Result<(), String> { ... }
```

---

### `restore_window_state`

Reads saved window state and applies it on app startup (FR-042).

**Request**: No parameters.

**Response**:
```typescript
interface WindowState {
  x: number | null;       // null = center on primary monitor
  y: number | null;
  width: number;
  height: number;
  maximized: boolean;
}
```

**Behavior**:
- Called once at startup before the window is shown.
- Validates saved position is on a connected monitor. If offscreen, returns null for x/y (triggers centering).
- Desktop only — returns defaults on mobile.

**Rust signature**:
```rust
#[tauri::command]
fn restore_window_state(app: AppHandle) -> WindowState { ... }
```

---

### `export_user_data`

Exports all user data (zones, alarms, preferences) to a JSON file at the user-selected path (FR-047).

**Request**: `file_path: string` (user-selected save location from OS file dialog)

**Response**:
```typescript
interface ExportResult {
  success: boolean;
  file_path: string;
  bytes_written: number;
}
```

**Errors**:
- `WriteError`: Could not write to the specified path.
- `SerializationError`: Failed to serialize store data.

**Behavior**:
- Reads all three stores (zones.dat, alarms.dat, preferences.dat).
- Excludes window state fields from export (device-specific).
- Wraps in export envelope with `export_version`, `app_version`, `exported_at`.

**Rust signature**:
```rust
#[tauri::command]
async fn export_user_data(app: AppHandle, file_path: String) -> Result<ExportResult, String> { ... }
```

---

### `import_user_data`

Imports user data from a previously exported JSON file (FR-047).

**Request**:
```typescript
interface ImportConfig {
  file_path: string;      // User-selected file from OS file dialog
  mode: 'merge' | 'replace';
}
```

**Response**:
```typescript
interface ImportResult {
  success: boolean;
  zones_imported: number;
  alarms_imported: number;
  preferences_updated: boolean;
}
```

**Errors**:
- `ReadError`: Could not read the specified file.
- `ParseError`: Invalid JSON or not a Delta-T Zaman export.
- `VersionError`: Export version is newer than current app (user must update).
- `ValidationError`: Imported data fails entity validation rules.

**Behavior**:
- Validates `export_version` compatibility.
- Runs schema migration on imported data if needed.
- In `merge` mode: adds zones/alarms not already present (by UUID), keeps existing preferences.
- In `replace` mode: overwrites all stores with imported data.
- Triggers full UI refresh after import.

**Rust signature**:
```rust
#[tauri::command]
async fn import_user_data(app: AppHandle, config: ImportConfig) -> Result<ImportResult, String> { ... }
```

---

### `request_battery_exemption`

Requests Android battery optimization exemption (Doze whitelist) so alarms fire reliably in background (NFR-001).

**Request**: No parameters.

**Response**:
```typescript
interface BatteryExemptionResult {
  granted: boolean;
  already_exempt: boolean;
}
```

**Errors**: None — always returns a result.

**Behavior**:
- Android only — returns `{ granted: true, already_exempt: true }` on other platforms.
- Called when the user creates their first alarm.
- If not exempt, triggers Android's `REQUEST_IGNORE_BATTERY_OPTIMIZATIONS` intent.
- If user denies, the app warns that alarms may be delayed when the device is idle.

**Rust signature**:
```rust
#[tauri::command]
async fn request_battery_exemption(app: AppHandle) -> BatteryExemptionResult { ... }
```

---

### `toggle_compact_mode`

Toggles between full and compact display mode (FR-055). Manages window visibility, size, and position.

**Request**:
```typescript
interface ToggleCompactRequest {
  enable: boolean;  // true = switch to compact, false = switch to full
}
```

**Response**:
```typescript
interface ToggleCompactResult {
  compact_mode: boolean;
}
```

**Errors**: None — always returns a result.

**Behavior**:
- Desktop only — returns current mode unchanged on mobile.
- When enabling compact: saves full window state, hides full window, shows compact window at saved compact position (or default). Compact window is always-on-top by default.
- When disabling compact: saves compact window state, hides compact window, shows full window at saved position.
- Updates `compact_mode` in preferences.dat.

**Rust signature**:
```rust
#[tauri::command]
async fn toggle_compact_mode(app: AppHandle, enable: bool) -> ToggleCompactResult { ... }
```

---

### `check_timezone_data_update`

Checks the Cloudflare Worker for timezone data patches and applies if available (FR-049).

**Request**: No parameters.

**Response**:
```typescript
interface TzDataUpdateResult {
  updated: boolean;
  patch_version: string | null;  // Version applied, or null if no update
  zones_affected: number;
}
```

**Errors**:
- `NetworkError`: Cannot reach Cloudflare Worker (silently ignored by caller).
- `PatchError`: Patch cannot be applied (incompatible base version).

**Behavior**:
- Fetches timezone patch manifest from the same Cloudflare Worker as update checks.
- If `patch_version` > current `timezone_data_version` in preferences, downloads and applies the patch in-memory.
- Updates `timezone_data_version` in preferences.dat on successful apply.
- Non-blocking — called on app launch after UI is rendered.
- If fetch fails or no update is available, returns `{ updated: false, patch_version: null, zones_affected: 0 }`.

**Rust signature**:
```rust
#[tauri::command]
async fn check_timezone_data_update(app: AppHandle) -> Result<TzDataUpdateResult, String> { ... }
```

---

### `get_platform_close_behavior`

Returns the platform-specific window close behavior configuration (FR-057).

**Request**: No parameters.

**Response**:
```typescript
interface PlatformCloseBehavior {
  platform: 'macos' | 'windows' | 'linux' | 'ios' | 'android';
  close_action: 'hide_to_tray' | 'quit' | 'suspend';
  tray_available: boolean;
  quit_shortcut: string | null;  // e.g., "Cmd+Q" on macOS
}
```

**Errors**: None — always returns a result.

**Behavior**:
- macOS: `close_action = 'hide_to_tray'`, `quit_shortcut = 'Cmd+Q'`
- Windows: `close_action = tray_enabled ? 'hide_to_tray' : 'quit'`
- Linux: Detects Wayland vs X11 and tray support. `close_action = tray_available ? 'hide_to_tray' : 'quit'`
- Mobile: `close_action = 'suspend'`, `tray_available = false`

**Rust signature**:
```rust
#[tauri::command]
fn get_platform_close_behavior(app: AppHandle) -> PlatformCloseBehavior { ... }
```

---

### `handle_deep_link`

Parses and validates an incoming deep link URL. Returns structured data for the frontend to act on (FR-059).

**Request**:
```typescript
interface HandleDeepLinkRequest {
  url: string;  // e.g., "deltat://add?tz=Asia/Tokyo&label=Tokyo+Office"
}
```

**Response**:
```typescript
interface DeepLinkResult {
  action: 'add_timezone' | 'create_alarm' | 'unknown';
  timezone?: string;       // IANA ID
  label?: string;          // Display label
  alarm_time?: string;     // HH:MM format (for create_alarm)
  valid: boolean;
  error_message?: string;  // Human-readable error if valid=false
}
```

**Errors**:
| Code | Condition |
|------|-----------|
| `invalid_url` | URL is malformed or not a `deltat://` scheme |
| `unknown_action` | URL path is not a recognized action |

**Validation rules**:
- `tz` parameter MUST be a valid IANA timezone ID present in the bundled dataset
- `time` parameter MUST match `HH:MM` 24-hour format
- `label` is URL-decoded, max 50 characters, HTML-stripped
- Unknown query parameters are silently ignored

**Rust signature**:
```rust
#[tauri::command]
fn handle_deep_link(url: String) -> Result<DeepLinkResult, String> { ... }
```

---

### `check_dnd_status`

Checks whether the device is currently in Do Not Disturb / Focus mode (FR-061). Desktop platforms always return `false` (DND detection is not reliably available).

**Request**: No parameters.

**Response**:
```typescript
interface DndStatus {
  dnd_active: boolean;
  platform_mode_name?: string;  // e.g., "Focus", "Do Not Disturb", "Quiet Hours"
}
```

**Errors**: None — always returns a result. Returns `{ dnd_active: false }` on platforms where detection is unavailable.

**Rust signature**:
```rust
#[tauri::command]
fn check_dnd_status() -> DndStatus { ... }
```

---

### `request_exact_alarm_permission`

Requests the `SCHEDULE_EXACT_ALARM` permission on Android 12+ (API 31+). On other platforms or Android < 12, returns `granted: true` immediately (FR-062).

**Request**: No parameters.

**Response**:
```typescript
interface ExactAlarmPermission {
  granted: boolean;
  should_show_rationale: boolean;  // true if user previously denied and should see explanation
}
```

**Errors**:
| Code | Condition |
|------|-----------|
| `permission_denied` | User denied the permission request |

**Behavior**:
- Android 12+: Opens system settings for exact alarm permission
- Android < 12: Returns `{ granted: true, should_show_rationale: false }` (no permission needed)
- Non-Android: Returns `{ granted: true, should_show_rationale: false }`

**Rust signature**:
```rust
#[tauri::command]
fn request_exact_alarm_permission() -> Result<ExactAlarmPermission, String> { ... }
```

---

### `heartbeat_ack`

Acknowledges a WebView heartbeat ping from the Rust backend. Called by the frontend in response to every `webview-heartbeat` event (NFR-002c).

**Request**: `sequence: number` (the sequence number from the received `webview-heartbeat` event)

**Response**: `void`

**Errors**: None — always succeeds.

**Behavior**:
- Resets the backend's missed-heartbeat counter for the WebView.
- If the backend receives no ack for 30 seconds (3 missed beats at 10-second intervals), it shows a native dialog with Reload/Quit options.
- Lightweight — no I/O, just updates an in-memory counter.

**Rust signature**:
```rust
#[tauri::command]
fn heartbeat_ack(sequence: u64) { ... }
```

---

## Event Contracts (Rust → JS)

Events emitted from Rust to the frontend via `app.emit()`:

### `alarm-fired`

Emitted when an alarm notification is delivered by the OS.

```typescript
interface AlarmFiredEvent {
  alarm_id: string;
  fired_at: string;  // ISO 8601 UTC
}
```

**Frontend listener**:
```typescript
import { listen } from '@tauri-apps/api/event';
const unlisten = await listen<AlarmFiredEvent>('alarm-fired', (event) => {
  // Update alarm state to 'completed' if one-time
});
```

### `tray-action`

Emitted when user interacts with the system tray.

```typescript
interface TrayActionEvent {
  action: 'show_window' | 'quit' | 'zone_clicked' | 'toggle_compact';
  zone_id?: string;  // Only for 'zone_clicked'
}
```

### `update-available`

Emitted when a background update check finds a new version.

```typescript
interface UpdateAvailableEvent {
  version: string;         // e.g., "1.2.0"
  release_notes?: string;  // Markdown release notes
}
```

**Frontend listener**:
```typescript
import { listen } from '@tauri-apps/api/event';
const unlisten = await listen<UpdateAvailableEvent>('update-available', (event) => {
  // Show non-intrusive update notification banner
});
```

### `alarm-snoozed`

Emitted when a user presses the Snooze action button on an alarm notification.

```typescript
interface AlarmSnoozedEvent {
  alarm_id: string;
  snoozed_at: string;       // ISO 8601 UTC
  snooze_until: string;     // ISO 8601 UTC (snoozed_at + 5 minutes)
}
```

**Frontend listener**:
```typescript
import { listen } from '@tauri-apps/api/event';
const unlisten = await listen<AlarmSnoozedEvent>('alarm-snoozed', (event) => {
  // Update alarm state to 'snoozed', schedule re-fire notification
});
```

### `deep-link-received`

Emitted when the app receives a deep link URL, either at launch or while running (FR-059).

```typescript
interface DeepLinkReceivedEvent {
  url: string;  // Raw URL, e.g., "deltat://add?tz=Asia/Tokyo"
}
```

**Frontend listener**:
```typescript
import { listen } from '@tauri-apps/api/event';
const unlisten = await listen<DeepLinkReceivedEvent>('deep-link-received', (event) => {
  // Call handle_deep_link command to parse and validate, then act on result
});
```

### `webview-heartbeat`

Emitted by the Rust backend every 10 seconds. The frontend MUST respond with a heartbeat acknowledgment. If the backend receives no acknowledgment for 30 seconds, it treats the WebView as unresponsive (NFR-002c).

```typescript
interface WebViewHeartbeatEvent {
  timestamp: number;  // Unix epoch ms
  sequence: number;   // Monotonically increasing
}
```

**Frontend listener**:
```typescript
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
const unlisten = await listen<WebViewHeartbeatEvent>('webview-heartbeat', (event) => {
  invoke('heartbeat_ack', { sequence: event.payload.sequence });
});
```

---

## Capability Requirements

`src-tauri/capabilities/main.json`:
```json
{
  "identifier": "main-capability",
  "description": "Main window capabilities",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "notification:default",
    "notification:allow-send-notification",
    "notification:allow-request-permission",
    "notification:allow-is-permission-granted",
    "store:default",
    "os:default",
    "autostart:default",
    "single-instance:default",
    "positioner:default",
    "updater:default",
    "dialog:default",
    "dialog:allow-open",
    "dialog:allow-save",
    "deep-link:default"
  ]
}
```
