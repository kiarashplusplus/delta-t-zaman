# Data Model: Cross-Platform World Clock (Delta-T Zaman)

**Branch**: `001-tauri-world-clock` | **Date**: 2026-04-12 | **Spec**: [spec.md](./spec.md)

## Entities

### TimeZoneEntry

Represents a user-selected timezone displayed on the main clock screen.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `id` | string (UUID v4) | PK, unique, immutable | Unique identifier for this entry |
| `iana_id` | string | Required, valid IANA timezone ID | Canonical IANA identifier (e.g., `"America/New_York"`) |
| `display_label` | string | Required, max 100 chars | User-customizable label (defaults to city name, e.g., `"Mom's House"`) |
| `sort_order` | integer | >= 0 | Position in the display list (0 = top) |
| `pinned_to_tray` | boolean | Default: `false` | Whether this zone appears in the system tray compact view |
| `created_at` | string (ISO 8601) | Immutable, auto-set | Timestamp when entry was added |

**Validation rules**:
- `iana_id` must resolve to a canonical timezone via the bundled alias map. Aliases (e.g., `US/Eastern`) are resolved to canonical form (e.g., `America/New_York`) before storage.
- No two entries may share the same resolved `iana_id` (duplicate detection — FR-015).
- `sort_order` values are contiguous (0, 1, 2, ...). Reordering re-indexes all entries.
- Maximum 30 entries (FR-001).

**State transitions**: None — entries are created or deleted, no intermediate states.

---

### Alarm

Represents a scheduled alert tied to a specific timezone.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `id` | string (UUID v4) | PK, unique, immutable | Unique identifier |
| `title` | string | Required, max 200 chars | User-provided alarm label |
| `target_time` | string (`HH:mm`) | Required, 24h format | Time of day the alarm fires (in the target timezone) |
| `timezone_id` | string | Required, valid IANA timezone ID | The timezone in which `target_time` is evaluated |
| `recurrence` | enum | Required | One of: `once`, `daily`, `weekdays`, `weekends`, `custom` |
| `custom_days` | integer[] | Required if recurrence=`custom` | Days of week (0=Sun, 1=Mon, ..., 6=Sat) |
| `enabled` | boolean | Default: `true` | Whether the alarm is active and will fire |
| `state` | enum | Required | One of: `active`, `snoozed`, `completed`, `dismissed` |
| `snooze_count` | integer | Default: `0` | Number of times snoozed this firing (max 1) |
| `sound_enabled` | boolean | Default: `true` | Whether notification includes sound |
| `created_at` | string (ISO 8601) | Immutable, auto-set | When the alarm was created |
| `completed_at` | string (ISO 8601) \| null | Auto-set on completion | When a one-time alarm fired |
| `notification_id` | string \| null | Internal | OS notification schedule ID for cancellation |

**Validation rules**:
- `target_time` must be valid 24h time (`00:00`–`23:59`).
- `timezone_id` must be a valid canonical IANA ID.
- `custom_days` must be non-empty when `recurrence` is `custom`, with values in `[0, 6]`.
- Maximum 50 active (non-completed) alarms simultaneously (FR-026).
- `notification_id` is managed internally — not user-facing.

**State transitions**:

```
                  ┌──────────┐
     create ────► │  active   │ ◄──── enable
                  └─────┬────┘
                        │
         ┌──────────────┼───────────────┐
         │              │               │
         ▼              ▼               ▼
  ┌──────────┐   ┌──────────┐    ┌──────────┐
  │  snoozed │   │ dismissed│    │ disabled │
  │(5 min)   │   │ (manual) │    │(toggle)  │
  └─────┬────┘   └──────────┘    └─────┬────┘
        │                              │
        │ re-fires                     │ enable
        ▼                              ▼
  ┌──────────┐                   ┌──────────┐
  │completed │                   │  active   │
  │(one-time)│                   └──────────┘
  └──────────┘
```

- **active → snoozed**: User presses Snooze on notification. Sets `snooze_count = 1`. Re-fires after 5 minutes.
- **snoozed → completed**: Snoozed alarm re-fires (one-time). Sets `completed_at`, retains in history. No second snooze allowed.
- **active → completed**: One-time alarm fires and user does not snooze. Sets `completed_at`, retains in history.
- **active → dismissed**: User manually dismisses an alarm (any recurrence type).
- **active ↔ disabled**: User toggles `enabled` flag. Disabled alarms do not fire.
- **Recurring active**: After firing (or snooze + re-fire), alarm remains `active`, resets `snooze_count = 0`, and resets for next occurrence.
- **completed/dismissed → (delete)**: User manually clears from history. Permanent deletion.

---

### UserPreferences

Represents global application settings. Singleton — one instance per app installation.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `time_format` | enum | Required | `12h` or `24h`. Default: detected from system locale |
| `locale` | string (BCP 47) | Required | Display locale (e.g., `"en-US"`, `"de-DE"`, `"ja-JP"`) |
| `theme` | enum | Required | `light`, `dark`, or `system`. Default: `system` |
| `default_timezone` | string | Required | IANA ID of user's local timezone (auto-detected) |
| `always_on_top` | boolean | Default: `false` | Desktop only — floating window mode |
| `launch_at_startup` | boolean | Default: `false` | Desktop only — auto-launch on login |
| `tray_enabled` | boolean | Default: `true` | Desktop only — show system tray icon |
| `show_seconds` | boolean | Default: `true` (desktop), `false` (mobile) | Show seconds in clock display |
| `telemetry_opt_in` | boolean | Default: `false` | Whether anonymous telemetry is enabled (FR-032/036) |
| `auto_update_enabled` | boolean | Default: `true` | Whether automatic update checks are enabled (FR-034) |
| `window_x` | integer \| null | Default: `null` | Desktop only — last window X position (null = center) |
| `window_y` | integer \| null | Default: `null` | Desktop only — last window Y position (null = center) |
| `window_width` | integer | Default: `400` | Desktop only — last window width in pixels |
| `window_height` | integer | Default: `600` | Desktop only — last window height in pixels |
| `window_maximized` | boolean | Default: `false` | Desktop only — whether window was maximized |
| `ui_language` | string | Default: `"en"` | UI string language code (v1: only "en" supported) |
| `high_contrast` | enum | Default: `system` | `on`, `off`, or `system`. Overrides theme colors when active (FR-050) |
| `compact_mode` | boolean | Default: `false` | Whether compact display mode is active (FR-055) |
| `compact_window_x` | integer \| null | Default: `null` | Desktop only — compact window X position (null = default) |
| `compact_window_y` | integer \| null | Default: `null` | Desktop only — compact window Y position (null = default) |
| `compact_window_width` | integer | Default: `200` | Desktop only — compact window width in pixels |
| `compact_window_height` | integer | Default: `400` | Desktop only — compact window height in pixels |
| `timezone_data_version` | string \| null | Default: `null` | Version of the latest applied timezone data patch (null = bundled only) |

**Validation rules**:
- `locale` must be a valid BCP 47 language tag.
- `default_timezone` must be a valid canonical IANA ID.
- Platform-specific fields (`always_on_top`, `launch_at_startup`, `tray_enabled`, `window_*`, `compact_window_*`) are ignored on mobile.
- `window_x`/`window_y` set to `null` means center on primary monitor (first-launch default).
- On window restore, if saved position is offscreen (monitor disconnected), reset to `null` (re-center).
- `high_contrast` value `system` defers to OS accessibility setting; `on`/`off` override it.
- `compact_window_*` fields are independent from `window_*` — each mode has its own saved position/size.

**State transitions**: None — singleton with direct field updates.

---

## Relationships

```
UserPreferences (1)
    │
    ├── TimeZoneEntry (0..30)    # User's selected timezones
    │       │
    │       └── Alarm (0..50)    # Alarms reference a timezone
    │            via timezone_id = TimeZoneEntry.iana_id
    │            (NOT a foreign key — alarms persist even if timezone removed)
    │
    └── (global settings apply to all views)
```

**Key relationship rules**:
- Alarms reference a timezone by `iana_id`, not by `TimeZoneEntry.id`. This means an alarm can reference a timezone that is not currently in the user's display list. This is intentional — removing a timezone from the display should not delete associated alarms.
- A timezone entry can have zero or more alarms. An alarm belongs to exactly one timezone.
- UserPreferences is a singleton — there is exactly one instance.

---

## Persistence Schema

Three separate store files via `@tauri-apps/plugin-store`:

### `zones.dat`
```json
{
  "schema_version": 1,
  "zones": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "iana_id": "America/New_York",
      "display_label": "New York",
      "sort_order": 0,
      "pinned_to_tray": true,
      "created_at": "2026-04-12T08:00:00Z"
    }
  ]
}
```

### `alarms.dat`
```json
{
  "schema_version": 1,
  "alarms": [
    {
      "id": "660e8400-e29b-41d4-a716-446655440001",
      "title": "Morning standup",
      "target_time": "09:00",
      "timezone_id": "Asia/Tokyo",
      "recurrence": "weekdays",
      "custom_days": [],
      "enabled": true,
      "state": "active",
      "snooze_count": 0,
      "sound_enabled": true,
      "created_at": "2026-04-12T08:30:00Z",
      "completed_at": null,
      "notification_id": "notif-abc123"
    }
  ]
}
```

### `preferences.dat`
```json
{
  "schema_version": 1,
  "time_format": "12h",
  "locale": "en-US",
  "theme": "system",
  "default_timezone": "America/New_York",
  "always_on_top": false,
  "launch_at_startup": false,
  "tray_enabled": true,
  "show_seconds": true,
  "telemetry_opt_in": false,
  "auto_update_enabled": true,
  "window_x": null,
  "window_y": null,
  "window_width": 400,
  "window_height": 600,
  "window_maximized": false,
  "ui_language": "en",
  "high_contrast": "system",
  "compact_mode": false,
  "compact_window_x": null,
  "compact_window_y": null,
  "compact_window_width": 200,
  "compact_window_height": 400,
  "timezone_data_version": null
}
```

**Schema versioning rules** (FR-035):
- Every store file includes a top-level `schema_version` integer (starts at `1`).
- On startup, the app reads `schema_version` from each store. If missing, it is treated as version `0` (legacy pre-versioning file).
- Forward-only migration functions run sequentially: `migrate_v0_to_v1()`, `migrate_v1_to_v2()`, etc. Each function transforms the JSON in-place and increments `schema_version`.
- After all migrations complete, the store is written back to disk with the current schema version.
- If migration fails (unexpected data shape, parse error), the store is reset to defaults with a user-visible warning (FR-029 corruption-recovery path).
- Migration runs locally — no network required.

---

Static file `src/data/timezones.json` — read-only, bundled at build time.

```json
[
  {
    "id": "America/New_York",
    "city": "New York",
    "country": "United States",
    "country_code": "US",
    "utc_offset_minutes": -300,
    "latitude": 40.7128,
    "longitude": -74.0060,
    "aliases": ["US/Eastern", "EST5EDT"]
  }
]
```

| Field | Type | Description |
|-------|------|-------------|
| `id` | string | Canonical IANA timezone ID |
| `city` | string | Primary city name (English) |
| `country` | string | Country name (English) |
| `country_code` | string | ISO 3166-1 alpha-2 country code |
| `utc_offset_minutes` | integer | Standard UTC offset in minutes (not DST-adjusted) |
| `latitude` | float | Reference city latitude (for SunCalc) |
| `longitude` | float | Reference city longitude (for SunCalc) |
| `aliases` | string[] | Legacy IANA aliases that map to this canonical ID |

### Timezone Data Patch Schema (FR-049)

Served by the Cloudflare Worker at the same endpoint as update checks. A patch contains only the timezone entries that have changed since the app's bundled version.

```json
{
  "patch_version": "2026b",
  "base_version": "2026a",
  "updated": [
    {
      "id": "America/New_York",
      "utc_offset_minutes": -300,
      "aliases": ["US/Eastern", "EST5EDT"]
    }
  ],
  "added": [],
  "removed": []
}
```

| Field | Type | Description |
|-------|------|-------------|
| `patch_version` | string | IANA TZDB version this patch brings data to (e.g., "2026b") |
| `base_version` | string | Minimum bundled version this patch can apply to |
| `updated` | object[] | Timezone entries with changed fields (partial — only changed fields) |
| `added` | object[] | New timezone entries (full objects matching bundled schema) |
| `removed` | string[] | IANA IDs that are no longer valid (rare) |

**Patch rules**:
- Patches are cumulative — each patch contains all changes since `base_version`.
- If the app's `timezone_data_version` (from preferences.dat) is already ≥ `patch_version`, the patch is skipped.
- Patches are applied in-memory on startup — the bundled `timezones.json` is never modified on disk.
- If a patch cannot be applied (parse error, incompatible base version), it is silently skipped and the bundled data is used.

---

## Export/Import Schema (FR-047)

The data export function produces a single JSON file containing all user data. Used for backup/restore and cross-device transfer.

### `deltat-backup-{date}.json`
```json
{
  "export_version": 1,
  "app_version": "1.0.0",
  "exported_at": "2026-04-12T15:00:00Z",
  "zones": { /* full zones.dat content */ },
  "alarms": { /* full alarms.dat content */ },
  "preferences": { /* full preferences.dat content minus window state */ }
}
```

**Import rules**:
- `export_version` must be ≤ current app's export version. If higher, reject with "please update the app" message.
- `app_version` is informational only (not used for compatibility checks).
- Window state fields (`window_x`, `window_y`, `window_width`, `window_height`, `window_maximized`, `compact_window_x`, `compact_window_y`, `compact_window_width`, `compact_window_height`) are excluded from export — they are device-specific.
- On import, the user chooses **merge** (add zones/alarms not already present, keep existing preferences) or **replace** (overwrite all stores with imported data).
- Schema versions in imported data are migrated forward if needed (same path as startup migration).
