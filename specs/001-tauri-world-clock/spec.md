# Feature Specification: Cross-Platform World Clock (Delta-T Zaman)

**Feature Branch**: `001-tauri-world-clock`  
**Created**: 2026-04-12  
**Status**: Clarified  
**Input**: User description: "Build a truly perfect cross-platform world clock with Tauri 2.10.3 given Mac, Xcode, Android Studio, Rust, and full toolchain installed"

## Clarifications

### Session 2026-04-12

- Q: Should time zones be displayed as digital text, analog clock faces, or hybrid? → A: Digital text with a visual day/night indicator per zone (most scannable across many zones, renders reliably on all 5 platforms).
- Q: What happens to a one-time alarm after it fires — auto-delete, mark completed, or keep active? → A: One-time alarms are marked "completed" and retained in alarm history; user can clear completed alarms manually.
- Q: Should basic accessibility (keyboard navigation, screen reader labels) be in scope for v1? → A: Yes — basic keyboard navigation and screen reader labels are in scope for v1.
- Q: Is there a maximum number of alarms a user can have active simultaneously? → A: Capped at 50 active alarms.
- Q: FR-001 says "at least 30" timezone entries but plan.md/data-model.md say "max 30" — which is correct? → A: Cap at 30 entries. FR-001 wording corrected from "at least 30" to "up to 30". Prevents unbounded DOM/performance cost.
- Q: Mobile cold start target contradicts between spec (3s) and plan.md (4s) — which is authoritative? → A: Spec's 3-second target is authoritative. Plan.md's 4s was a conservative estimate and should be normalized to 3s.
- Q: What happens if a store file (zones.dat, alarms.dat, preferences.dat) becomes corrupted (invalid JSON)? → A: Graceful fallback — reset the corrupted store to defaults, display a warning to the user, and log the error. Do not crash or lose data from unaffected stores.
- Q: Do alarms persist when their associated timezone is removed from the display list? → A: Yes — alarms reference timezone by IANA ID, not display entry. Removing a timezone from display does not delete its alarms. Orphaned alarms remain functional and editable.
- Q: For recurring alarms, does the frontend or backend compute the next fire time? → A: Frontend computes the next fire time (accounting for timezone, DST, recurrence pattern) and calls schedule_alarm_notification with the computed UTC datetime. Backend only handles OS notification scheduling.
- Q: What observability signals should be collected via Cloudflare? → A: Crash/error reports + anonymous usage metrics (app opens, feature usage: alarms created, zones added). No full performance tracing. Sent to Cloudflare Workers Analytics Engine.
- Q: What is the user consent model for telemetry? → A: Opt-in with a first-launch prompt. Telemetry is disabled by default; user explicitly enables it. Aligns with privacy-first ethos and app store review policies (Apple/Google).
- Q: How should store schema versioning and migration work? → A: Each store file includes a `schema_version` integer. On startup, forward-only migration functions run sequentially (v1→v2→v3…). Migration is fully local — no network required. Failed migration triggers the same corruption-recovery path (reset to defaults + warning).
- Q: Should auto-update move from "deferred" to v1 scope? → A: Yes. Use tauri-plugin-updater with update manifest and binaries hosted on Cloudflare R2 + Workers. Auto-update is the delivery mechanism for schema migrations and bug fixes.
- Q: What Cloudflare architecture for telemetry and updates? → A: Cloudflare Workers + Analytics Engine for telemetry ingestion (serverless, scale-to-zero). Cloudflare R2 + Workers for hosting update manifest and binaries (no origin server needed).
- Q: How should the release pipeline work for 5 platforms? → A: GitHub Actions CI/CD produces signed binaries for all platforms from a single commit. Desktop artifacts are uploaded to Cloudflare R2 with an auto-generated update manifest. Mobile artifacts are submitted to app stores.
- Q: What code signing and notarization is required? → A: macOS requires Apple Developer ID + notarization (GateKeeper). Windows requires Authenticode certificate. Linux is unsigned. iOS/Android use standard app store signing. Ed25519 keys for updater signature verification are CI secrets with public key embedded in tauri.conf.json.
- Q: Should snooze be supported for alarm notifications? → A: Yes. Single snooze of 5 minutes per alarm firing. No re-snooze (prevents infinite snooze loops). After snooze, alarm fires once more as a final alert.
- Q: Should the app remember window position and size? → A: Yes, on desktop. Persisted in UserPreferences. Falls back to centered-on-primary-monitor if saved position is offscreen.
- Q: Should each timezone card show the current date? → A: Yes. Show day-of-week and date. When the date differs from local, show a "+1 day" / "Tomorrow" indicator.
- Q: What UI language(s) should v1 support? → A: English only for UI strings. But the architecture must externalize all strings into locale files (JSON) so translations can be added later without code changes. Time/date formatting already uses Intl API for locale-aware display.
- Q: Should a data export/import feature be included? → A: Yes. JSON export of all timezone entries, alarms, and preferences. Import with version compatibility check. Accessible from Settings screen.
- Q: What content should the Settings and About screens include? → A: Settings: Display, Notifications, Privacy, Updates, Data sections. About: app name, version, build number, licenses, privacy policy link.
- Q: How should bundled timezone data stay fresh between app updates? → A: The auto-update Worker can serve a lightweight timezone-data-only patch (JSON diff) alongside the full binary update. The app checks for timezone data updates on the same schedule as app updates. If new timezone data is available, it is downloaded and merged into the bundled data without requiring a full app update. This is best-effort — stale data is acceptable since DST rule changes are announced months in advance.
- Q: What level of accessibility depth is required beyond basic keyboard/screen reader? → A: Full WCAG 2.1 AA compliance target. High contrast mode (respects OS setting + manual override), `prefers-reduced-motion` support (disable all CSS transitions/animations), minimum 4.5:1 color contrast ratios for all text, focus trap management for dialogs/modals, visible focus indicators on all interactive elements, and font size scaling that respects OS text size settings.
- Q: Should a compact/mini display mode be supported? → A: Yes. A togglable compact mode shows only timezone labels and times in a minimal floating strip (no date, no day/night indicator, no controls). Ideal for always-on-top usage. Toggle via menu, keyboard shortcut (Cmd/Ctrl+M), or tray context menu.
- Q: Should timezone search support fuzzy matching and UTC offset queries? → A: Yes. Fuzzy matching with single-character typo tolerance (Levenshtein distance ≤ 1), plus UTC offset search (e.g., "+5:30" matches "Asia/Kolkata", "GMT-5" matches "America/New_York"). Results ranked: exact prefix > fuzzy prefix > substring > offset match.
- Q: How should platform-specific behaviors be handled across the 5 targets? → A: Each platform has documented behavioral adaptations: macOS (traffic-light buttons close-to-tray by default, native menu bar), Windows (minimize-to-tray via close button, taskbar presence when not tray-only), Linux (Wayland/X11 detection with graceful tray fallback), iOS (background app refresh for alarm scheduling, no tray), Android (foreground service for active alarms, battery optimization request). Platform detection at startup determines which code paths activate.
- Q: Should the app support deep linking via a custom URL scheme? → A: Yes. Register `deltat://` as a custom URL scheme on all platforms. Support `deltat://add?tz=Asia/Tokyo&label=Tokyo+Office` to add a timezone and `deltat://alarm?time=09:00&tz=America/New_York&label=Standup` to create an alarm. If the app is not running, the URL launches it first. Invalid or malformed URLs show a user-friendly error rather than silently failing.
- Q: How should the app behave when the device is in Do Not Disturb / Focus mode? → A: Alarm notifications are subject to OS-level DND/Focus mode rules. The app does NOT attempt to bypass DND. However, when the app is open and an alarm fires while DND is active, the in-app alarm banner still displays (since it's UI within the app window, not an OS notification). The alarm state transitions normally regardless of whether the notification was actually delivered by the OS. On Android, a foreground service notification can be marked as high-priority to optionally bypass DND at the user's discretion.
- Q: What specific error scenarios should be handled beyond the global error boundary? → A: SunCalc calculations with polar coordinates (latitudes >66.5°) where sunrise/sunset may not occur — show "24h day" or "24h night" instead of crashing. SunCalc with invalid/NaN coordinates — fall back to hiding the day/night indicator for that zone. Plugin init failure — degrade gracefully per plugin (e.g., no notifications plugin = alarms fire in-app only; no autostart plugin = hide launch-at-startup toggle). WebView unresponsive — detect via heartbeat, show reload prompt.
- Q: What Android API levels and scheduling APIs should be used for reliable alarm delivery? → A: Target Android API 33+ (Android 13+), min SDK 26 (Android 8). Use `AlarmManager.setExactAndAllowWhileIdle()` for alarm scheduling (survives Doze). Foreground service type: `foregroundServiceType="specialUse"` with user-facing notification. Request `SCHEDULE_EXACT_ALARM` permission (required on API 31+). Battery optimization exemption via `ACTION_REQUEST_IGNORE_BATTERY_OPTIMIZATIONS`. Wake locks are NOT used — rely on exact alarms + foreground service instead.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - View Multiple Time Zones at a Glance (Priority: P1)

A user opens the app and immediately sees the current time displayed as digital text (e.g., "3:45:12 PM") for several world cities/time zones of their choosing. Each zone includes a visual day/night indicator so the user can instantly see whether it is daytime or nighttime in that location. Clocks update in real time (at least once per second on desktop, once per minute on mobile) and correctly reflect any active Daylight Saving Time rules. The user can quickly scan the display to compare times across regions — for example, to schedule an international meeting.

**Why this priority**: This is the core value proposition of a world clock. Without accurate, real-time multi-timezone display, no other feature matters.

**Independent Test**: Can be fully tested by launching the app, verifying clocks show correct times for at least 3 different IANA time zones, and confirming times update continuously without user action.

**Acceptance Scenarios**:

1. **Given** the app is launched for the first time, **When** the main window renders, **Then** a default set of at least 3 time zones (e.g., local time, UTC, and one contrasting zone) is displayed with current, accurate times.
2. **Given** a clock is displayed for a zone currently observing DST, **When** the user compares the displayed time to an authoritative source (e.g., timeanddate.com), **Then** the times match within ±1 second.
3. **Given** the app is running on desktop, **When** 1 second elapses, **Then** all displayed clocks visibly update.
4. **Given** the app is running on a mobile device, **When** 1 minute elapses, **Then** all displayed clocks visibly update.

---

### User Story 2 - Add, Remove, and Reorder Time Zones (Priority: P1)

A user wants to personalize their world clock by adding cities/time zones relevant to their life — colleagues in Tokyo, family in London, a client in São Paulo. They can search for a city or IANA time zone name, add it to their display, remove zones they no longer need, and drag/reorder zones to put the most important ones first. These preferences persist across app restarts.

**Why this priority**: Personalization is essential — a world clock with only fixed zones is nearly useless. This story, combined with P1 Story 1, creates a viable MVP.

**Independent Test**: Can be fully tested by adding 3 new time zones, removing 1, reordering the remaining, closing the app, reopening it, and verifying the configuration persists exactly.

**Acceptance Scenarios**:

1. **Given** the user opens the "Add Time Zone" interface, **When** they type "Tok", **Then** they see results including "Asia/Tokyo" (and any other matching zones) within 300ms.
2. **Given** the user has 5 time zones displayed, **When** they remove one, **Then** it disappears immediately and the layout adjusts without a page reload.
3. **Given** the user reorders zones by dragging "London" above "New York", **When** they restart the app, **Then** "London" still appears above "New York".
4. **Given** the user adds a zone, **When** the app is closed and reopened, **Then** the added zone is still present.

---

### User Story 3 - System Tray / Menu Bar Quick Access (Priority: P2)

On desktop (macOS, Windows, Linux), the user wants quick access to world clock information without opening the full window. The app lives in the system tray (Windows/Linux) or menu bar (macOS). Clicking the tray icon shows a compact dropdown or tooltip displaying current times for pinned zones. The user can toggle the main window from the tray, and the app continues running in the background when the main window is closed.

**Why this priority**: This elevates the app from "an app you open" to "an app that's always there" — the expected behavior for a utility clock app on desktop.

**Independent Test**: Can be fully tested by minimizing the app to tray, verifying the tray icon appears, clicking it to see times, and confirming the main window can be toggled open/closed from the tray.

**Acceptance Scenarios**:

1. **Given** the app is running on macOS, **When** the user closes the main window, **Then** a menu bar icon remains visible and the app does not quit.
2. **Given** the tray icon is visible, **When** the user clicks it, **Then** a compact display showing pinned time zones with current times appears within 200ms.
3. **Given** the tray menu is open, **When** the user selects "Show Window", **Then** the main window opens or comes to the foreground.
4. **Given** the tray menu is open, **When** the user selects "Quit", **Then** the app fully exits.

---

### User Story 4 - Set Alarms and Receive Notifications (Priority: P2)

A user wants to set an alarm for a specific time in a specific time zone — for example, "Alert me when it's 9:00 AM in Tokyo." When the alarm fires, the user receives a native operating system notification with a sound (if enabled) regardless of whether the app window is in the foreground, minimized, or hidden to the system tray.

**Why this priority**: Alarms transform the app from passive display to active assistant, significantly increasing daily utility.

**Independent Test**: Can be fully tested by setting an alarm 1 minute in the future, minimizing the app, and verifying a native OS notification appears at the correct time.

**Acceptance Scenarios**:

1. **Given** the user creates an alarm for "09:00 in Asia/Tokyo", **When** it becomes 09:00 in Asia/Tokyo, **Then** a native notification appears with the alarm title and timezone info.
2. **Given** the app window is minimized (desktop) or backgrounded (mobile), **When** an alarm fires, **Then** the notification still appears.
3. **Given** the user sets a recurring daily alarm, **When** 24 hours pass, **Then** the alarm fires again at the same time in the specified zone.
4. **Given** the user has 3 alarms set, **When** they open the alarms list, **Then** all 3 are shown with their target time, zone, and enabled/disabled status.

---

### User Story 5 - Localized Display and Format Preferences (Priority: P3)

A user in Germany wants to see times in 24-hour format with German day/month names. A user in the US wants 12-hour format with AM/PM. The app detects the system locale by default but allows the user to override the time format (12h/24h), date format, and language of date labels independently.

**Why this priority**: Localization is important for international users but the app is fully functional without it (times are still accurate).

**Independent Test**: Can be fully tested by changing the format preference from 12h to 24h and verifying all displayed clocks switch format immediately.

**Acceptance Scenarios**:

1. **Given** the user's system locale is "de-DE", **When** they launch the app for the first time, **Then** times are displayed in 24-hour format and day names appear in German.
2. **Given** the user manually selects "12-hour" format in settings, **When** they return to the clock view, **Then** all clocks show AM/PM notation.
3. **Given** the user changes the display language to Japanese, **When** they view the clock screen, **Then** day-of-week and month labels appear in Japanese.

---

### User Story 6 - Mobile-Optimized Experience (Priority: P3)

A user on an iPhone or Android device opens the world clock app. The interface adapts to the smaller screen with a vertically scrollable list of clocks. Touch gestures (swipe to delete a zone, long-press to reorder) work naturally. The app respects mobile battery management and does not excessively drain battery when backgrounded.

**Why this priority**: Mobile support extends reach but desktop is the primary platform. Mobile has known platform constraints (no tray icon, limited background execution).

**Independent Test**: Can be fully tested by installing on a physical iOS or Android device, verifying the clock list renders correctly, and confirming swipe-to-delete and reorder gestures work.

**Acceptance Scenarios**:

1. **Given** the user opens the app on a phone in portrait orientation, **When** they have 8 time zones added, **Then** all zones are accessible via scrolling and text is readable without zooming.
2. **Given** the user swipes left on a time zone entry, **When** they confirm deletion, **Then** the zone is removed with a smooth animation.
3. **Given** the app is backgrounded on mobile for 30 minutes, **When** the user returns to the app, **Then** all clocks immediately display the correct current time (no stale display).

---

### Edge Cases

- What happens when the device's system clock is set incorrectly? The app should display times based on the system clock and not attempt to correct it, but may optionally warn the user if a large discrepancy with network time is detected.
- What happens when the user adds the same time zone twice (e.g., "America/New_York" and "US/Eastern" which are aliases)? The app should detect IANA timezone aliases and warn the user, preventing true duplicates.
- What happens during a DST transition while the app is running? Clocks must seamlessly adjust — no restart required, no stale offset cached.
- What happens when the user has zero time zones selected? The app should show the local system time zone by default and prompt the user to add more.
- What happens on Linux desktop environments that do not support a system tray (e.g., newer GNOME without AppIndicator)? The app should function normally without a tray icon and log a warning at startup.
- What happens when the notification permission is denied by the OS? Alarms should still trigger within the app UI (in-app alert/banner) even if native notifications are unavailable, and the user should be guided to enable permissions.
- What happens when the user's device has no internet connection? The app must function fully offline — all timezone data is bundled and no network is required for core functionality.
- What happens when a one-time alarm fires? The alarm transitions to "completed" status and remains in alarm history. The user can view and clear completed alarms from the alarm list.
- What happens when the user tries to create a 51st active alarm? The system should display a clear message that the 50-alarm limit has been reached and suggest completing or deleting existing alarms.
- What happens when a store file (zones.dat, alarms.dat, or preferences.dat) contains invalid JSON or is corrupted? The app resets the affected store to its defaults, displays a one-time warning to the user explaining that settings were reset, and logs the corruption error. Unaffected stores are not touched.
- What happens when the user removes a timezone from the display but has alarms set for that timezone? The alarms persist and continue to function. The alarm list shows the timezone's IANA ID (or city name from bundled data) regardless of whether it is in the user's display list. The user can still edit or delete orphaned alarms.
- For recurring alarms, who computes the next fire time? The frontend computes the next UTC fire time based on the alarm's target time, timezone, DST rules, and recurrence pattern, then calls the backend to schedule the OS notification at that computed time. After each firing, the frontend recomputes and reschedules the next occurrence.
- What happens when the telemetry endpoint (Cloudflare Worker) is unreachable? Telemetry events are silently dropped. No retry queue, no local buffering, no UI indication. Core app functionality is completely unaffected.
- What happens when the auto-update check fails (network unavailable, Cloudflare outage)? The app continues normally with the current version. The user can manually trigger an update check from settings. No error dialog is shown for background update check failures.
- What happens when a schema migration fails (e.g., unexpected data shape in a store file)? The same corruption-recovery path applies: reset the affected store to defaults, display a warning, and log the error. The app launches successfully on the new schema with default data.
- What happens when a store file has no `schema_version` field (pre-migration legacy file)? Treat as schema version 0 and run all migrations from the beginning. This handles the upgrade path from v1.0 (which had no versioning) to later releases.
- What happens when the user snoozes an alarm and then closes the app before the snoozed alarm re-fires? The snoozed alarm's re-fire time is persisted. On next app launch, if the snooze time has passed, the alarm fires immediately. If not, it is rescheduled for the remaining snooze duration.
- What happens when the CI pipeline fails code signing (expired certificate, revoked key)? The pipeline MUST fail the entire release — unsigned binaries MUST NOT be published to R2 or app stores.
- What happens when the user denies Android battery optimization exemption? The app warns that background alarms may be delayed or missed, and proceeds without the exemption. The warning is shown once per session when alarms exist.
- What happens when the user imports a data file from a newer app version? The import MUST reject the file with a clear error message explaining that the user needs to update the app first. Version compatibility is checked via `schema_version`.
- What happens when a Tauri plugin fails to initialize at startup (e.g., notification plugin on a restricted Linux environment)? The app continues launching with degraded functionality: features requiring the failed plugin are disabled with in-app messaging, but all other features work normally.
- What happens when the bundled timezone JSON file is corrupted or missing? The app MUST display an error screen explaining that core data is missing, and suggest reinstalling. This is a non-recoverable state as timezone data is essential for core functionality.
- What happens when the window is restored on a monitor that is no longer connected? The app detects that the saved position is offscreen and falls back to centering on the primary monitor at default size.
- What happens when IANA releases a timezone database update (e.g., a country changes its DST rules) between app updates? The auto-update Worker can serve a timezone-data-only JSON patch. If no patch is available, the app uses bundled (potentially stale) data — times may be off for the affected zone until the next data update. Stale data is acceptable as DST rule changes are announced months in advance.
- What happens when the user enables high contrast mode but the current theme has no high contrast variant? The app falls back to the built-in high contrast token set (solid borders, max contrast, no gradients) regardless of the selected theme. High contrast overrides theme colors.
- What happens when `prefers-reduced-motion` changes while the app is running (e.g., user toggles the OS setting)? The app MUST react to the media query change in real-time — any in-progress animations stop immediately, and subsequent UI updates use instant transitions.
- What happens when the user switches between compact and full mode? The app persists both window positions independently. Switching to compact mode hides the full window and shows the compact strip at its last position (or default). Switching back restores the full window at its last position. Timezone data and state are shared — only the rendering changes.
- What happens when the user types "+5:30" or "GMT-5" in the timezone search? The search engine parses the input as a UTC offset query and returns all timezones matching that offset (e.g., "+5:30" returns Asia/Kolkata, Asia/Colombo). If no matches are found for the offset, the search falls back to substring matching on the literal text.
- What happens when the macOS user clicks the red traffic-light close button? The window hides to the menu bar (equivalent to hide_to_tray) instead of quitting. The app continues running. Cmd+Q fully quits. This matches macOS conventions for utility/accessory apps.
- What happens when the Android app is killed by the OS while a foreground service is running? The foreground service is restarted by Android's service restart mechanism. Active alarms are rescheduled from the persisted alarm store on service restart. Alarms scheduled via the notification plugin's `Schedule.at()` survive process death independently.
- What happens when the app is launched at 200% font scale? All text renders at 2x size using rem-based sizing. The layout adjusts via CSS — timezone cards stack vertically if horizontal space is insufficient. No text is clipped. Scrolling activates if content overflows the viewport.
- What happens when the user clicks a `deltat://add?tz=Invalid/Zone` deep link with an invalid IANA ID? The app shows an error toast: "Unknown timezone: Invalid/Zone" and takes no action. The display list is not modified. Valid fields in the URL are not partially applied.
- What happens when an alarm notification fires while the device is in Do Not Disturb mode? The OS silences or blocks the notification per DND rules. The app's internal state still transitions the alarm to "completed" (one-time) or reschedules (recurring). If the app is open, the in-app banner displays normally. The user may miss the notification — this is expected and consistent with other alarm apps.
- What happens when SunCalc is given coordinates for Tromsø, Norway (69.6°N) in June? There is no sunset (midnight sun). The day/night indicator shows "24h daylight" with a sun icon. The gradient indicator shows continuous daytime. No error is thrown.
- What happens when the WebView becomes unresponsive (e.g., infinite loop in user script, memory pressure)? The Rust backend detects the lack of heartbeat pings (sent every 10 seconds from JS). After 30 seconds of silence, the backend shows a native dialog: "The app is not responding. Reload?" with Reload and Quit options. Reload recreates the WebView with fresh state loaded from stores.
- What happens when a deep link `deltat://alarm?time=09:00&tz=Asia/Tokyo` is received while the app is already running? The app brings itself to the foreground and opens the alarm creation form pre-filled with the provided values. If a modal is already open, it is closed first. The user must confirm to actually create the alarm — the deep link only pre-fills, never auto-creates.
- What happens on Android 12+ when the user revokes the SCHEDULE_EXACT_ALARM permission after alarms are set? Existing exact alarms are cancelled by the OS. The app detects this on next launch (or foreground resume), warns the user, and offers to re-request the permission. If denied, alarms fall back to inexact scheduling with a persistent warning badge on the alarms tab.
- What happens when the app receives a deep link on iOS where custom URL schemes require explicit registration in Info.plist? The `deltat://` scheme is registered in `Info.plist` at build time via Tauri's mobile configuration. If iOS asks "Open in Delta-T Zaman?" the user confirms. If the scheme is somehow not registered (corrupted install), iOS shows "Cannot open URL" — this is non-recoverable and requires reinstall.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST support up to 30 configurable IANA time zones displayed simultaneously.
- **FR-002**: System MUST update displayed times at least once per second on desktop and once per minute on mobile.
- **FR-003**: System MUST automatically handle Daylight Saving Time transitions without user intervention, using the operating system's timezone database.
- **FR-004**: Users MUST be able to search for time zones by city name, country, or IANA identifier with results appearing within 300ms of input.
- **FR-005**: Users MUST be able to add, remove, and reorder displayed time zones, with changes reflected immediately.
- **FR-006**: System MUST persist user preferences (selected zones, order, format settings, alarms) across application restarts.
- **FR-007**: System MUST provide a system tray icon (Windows/Linux) or menu bar icon (macOS) on desktop platforms, with a compact time zone summary on click.
- **FR-008**: System MUST allow the main window to be closed while the app continues running in the system tray (desktop only).
- **FR-009**: Users MUST be able to create, edit, enable/disable, and delete alarms associated with a specific time zone.
- **FR-010**: System MUST deliver alarm notifications via the native operating system notification system.
- **FR-011**: Alarms MUST fire even when the application window is minimized, hidden to tray, or backgrounded (within OS constraints).
- **FR-012**: System MUST support both 12-hour (AM/PM) and 24-hour time formats, defaulting to the user's system locale preference.
- **FR-013**: System MUST display date labels (day names, month names) in the user's chosen locale.
- **FR-014**: System MUST function fully offline — no network connectivity required for any core feature.
- **FR-015**: System MUST detect and prevent adding duplicate time zones (including IANA aliases like "US/Eastern" and "America/New_York").
- **FR-016**: System MUST run on macOS, Windows, and Linux as desktop platforms.
- **FR-017**: System MUST run on iOS and Android as mobile platforms with a touch-optimized interface.
- **FR-018**: System MUST support recurring alarms (daily, weekdays only, weekends only, custom days).
- **FR-019**: System MUST display the UTC offset and timezone abbreviation (e.g., "EST", "JST") alongside each clock.
- **FR-020**: System MUST show the relative time difference from the user's local time zone (e.g., "+9 hours") for each displayed zone.
- **FR-021**: System MUST provide an "always on top" / floating window option on desktop platforms.
- **FR-022**: On mobile, system MUST adapt its layout to portrait and landscape orientations.
- **FR-023**: System MUST launch within 2 seconds on desktop and within 3 seconds on mobile from a cold start.
- **FR-024**: System MUST display times as digital text (not analog clock faces) with a visual day/night indicator for each zone.
- **FR-025**: One-time alarms MUST transition to a "completed" state after firing and be retained in alarm history until the user manually clears them.
- **FR-026**: System MUST support a maximum of 50 active (non-completed) alarms simultaneously and inform the user when the limit is reached.
- **FR-027**: System MUST support basic keyboard navigation for all primary actions (add/remove zones, create/manage alarms, access settings) on desktop platforms.
- **FR-028**: System MUST provide screen reader-compatible labels for all interactive elements and time displays.
- **FR-029**: System MUST detect corrupted store files (invalid JSON) at startup and reset the affected store to defaults, displaying a warning to the user without crashing or affecting other stores.
- **FR-030**: For recurring alarms, the frontend MUST compute the next fire time (accounting for timezone, DST, and recurrence pattern) and schedule the OS notification via the backend. After each alarm fires, the frontend MUST recompute and reschedule the next occurrence.
- **FR-031**: Alarms MUST persist and remain functional when their associated timezone is removed from the display list. The alarm list MUST show the timezone identifier for orphaned alarms.
- **FR-032**: System MUST support opt-in anonymous telemetry, disabled by default. On first launch, the app presents a prompt asking the user to enable telemetry. Collected signals: crash/error reports and anonymous usage metrics (app opens, alarms created, zones added). No personally identifiable information is collected.
- **FR-033**: Telemetry events MUST be sent to a Cloudflare Workers endpoint using the Analytics Engine. Submission is fire-and-forget — failures are silently ignored. Telemetry MUST NOT block any UI operation or degrade offline functionality.
- **FR-034**: System MUST support automatic updates via `tauri-plugin-updater`. Update manifest and binaries are hosted on Cloudflare R2 served through a Cloudflare Worker. The app checks for updates on launch (background, non-blocking) and notifies the user when an update is available.
- **FR-035**: Each store file (zones.dat, alarms.dat, preferences.dat) MUST include a `schema_version` integer field. On startup, the app MUST run forward-only migration functions sequentially (v1→v2→v3…) to transform data to the current schema. Migration is fully local — no network required. A failed migration triggers the corruption-recovery path (FR-029: reset to defaults + warning).
- **FR-036**: The telemetry opt-in preference MUST be persisted in UserPreferences and changeable at any time from settings. Disabling telemetry immediately stops all data collection.
- **FR-037**: Release builds MUST be produced via a CI/CD pipeline (GitHub Actions) that builds, signs, notarizes (macOS), and packages installers for all 5 target platforms. The pipeline MUST upload desktop binaries to Cloudflare R2 and generate the update manifest automatically.
- **FR-038**: Desktop binaries MUST be code-signed: macOS with Apple Developer ID + notarization (GateKeeper), Windows with an Authenticode certificate, Linux unsigned (standard). Mobile builds MUST be signed for their respective app stores (Apple Developer for iOS, Keystore for Android). Ed25519 keys for `tauri-plugin-updater` signature verification MUST be generated in CI and the public key embedded in `tauri.conf.json`.
- **FR-039**: Distribution packages MUST be produced per platform: DMG for macOS, MSI or NSIS installer for Windows, AppImage + .deb for Linux, IPA for iOS (App Store), AAB for Android (Play Store).
- **FR-040**: A privacy policy MUST be hosted at a stable HTTPS URL (e.g., Cloudflare Pages or R2). The policy MUST describe what telemetry data is collected, that it is anonymous and opt-in, and that no PII is stored. The About screen and app store listings MUST link to this URL.
- **FR-041**: Alarm notifications MUST include a "Snooze" action button. Default snooze duration is 5 minutes. Snooze reschedules the notification once — a snoozed alarm does not offer a second snooze. After snooze expires, the notification fires again as a final alert.
- **FR-042**: System MUST persist the main window's position, size, and maximized state across application restarts on desktop platforms. On first launch, the window MUST be centered on the primary monitor at its default size.
- **FR-043**: Each timezone card MUST display the current date (day of week and date) in the target timezone, not just the time. When the date in the target timezone differs from the user's local date, a visual indicator (e.g., "Tomorrow", "+1 day") MUST be shown.
- **FR-044**: On first launch with empty stores, the app MUST display 3 default timezone entries: (1) the user's detected local timezone, (2) UTC, and (3) a geographically contrasting timezone selected based on the user's locale (e.g., Asia/Tokyo for Americas users, America/New_York for Asia/Europe users).
- **FR-045**: App UI strings (button labels, menu items, settings labels, error messages, placeholder text) MUST be externalized into a locale file structure. v1 ships with English (en) as the only supported UI language. The architecture MUST support adding new UI translations without code changes (JSON locale files loaded at startup based on preference).
- **FR-046**: Desktop platforms MUST support keyboard shortcuts: Cmd/Ctrl+N (add timezone), Cmd/Ctrl+K (focus search), Cmd/Ctrl+, (open settings), Cmd/Ctrl+Q (quit), Escape (close modal/dialog). Shortcuts MUST be displayed in menu items and discoverable via tooltips.
- **FR-047**: System MUST provide a data export function (JSON file) containing all timezone entries, alarms, and preferences. System MUST provide a corresponding import function that merges or replaces the current configuration from a previously exported file. Export/import is accessible from the Settings screen.
- **FR-048**: System MUST include a Settings screen with sections: Display (time format, show seconds, theme), Notifications (default sound on/off), Privacy (telemetry toggle), Updates (auto-update toggle, check now button, current version), Data (export/import, reset to defaults), and About (app name, version, build number, licenses, privacy policy link).
- **FR-049**: The auto-update Worker MUST support serving timezone-data-only patches (JSON) independently of full app updates. When a timezone data patch is available, the app downloads and merges it into the bundled timezone data at startup without requiring a full binary update. Timezone data updates follow the same check-on-launch schedule as app updates and are non-blocking.
- **FR-050**: The app MUST respect the OS-level high contrast / increased contrast accessibility setting. When high contrast is active, all UI elements MUST switch to a high-contrast token set with solid borders, no gradients, and minimum 7:1 contrast ratios. A manual high contrast toggle MUST also be available in Settings > Display for users on platforms without a system-level setting.
- **FR-051**: The app MUST respect the `prefers-reduced-motion` media query. When reduced motion is active, ALL CSS transitions, animations, and transform-based effects (clock-card entrance, drag-and-drop preview, day/night gradient transitions) MUST be disabled or replaced with instant state changes. This includes the drag-and-drop reorder animation.
- **FR-052**: All text in the app MUST meet WCAG 2.1 AA color contrast requirements: minimum 4.5:1 for normal text and 3:1 for large text (≥18pt or ≥14pt bold). This MUST be validated against both light and dark theme token sets, and against the high contrast token set.
- **FR-053**: All modal dialogs (add timezone, alarm form, settings, export/import, update prompt) MUST implement a focus trap: focus cycles within the dialog while open, initial focus is set to the first interactive element, and pressing Escape closes the dialog and returns focus to the triggering element. Focus indicators MUST be visible on all interactive elements (using `:focus-visible`).
- **FR-054**: The app MUST respect the OS text size / font scaling setting. All text MUST use relative units (rem/em) so that increasing the OS font scale proportionally increases app text. The layout MUST remain usable and unclipped at up to 200% font scale.
- **FR-055**: System MUST support a compact display mode (toggled via Cmd/Ctrl+M, tray context menu, or Settings). Compact mode renders timezone entries as a minimal single-line strip: `[Label] [Time]` — no date, no day/night indicator, no controls. The compact window is narrower (200px default width), always-on-top by default, and remembers its own position independently from the full window.
- **FR-056**: Timezone search (FR-004) MUST support fuzzy matching with single-character typo tolerance (Levenshtein distance ≤ 1) in addition to exact substring matching. Search MUST also accept UTC offset queries (e.g., "+5:30", "GMT-5", "UTC+9") and return matching timezones. Results MUST be ranked: exact prefix > fuzzy prefix > substring > offset match.
- **FR-057**: On macOS, clicking the window close button (red traffic light) MUST hide the window to the menu bar (equivalent to hide_to_tray) instead of quitting the app, consistent with macOS utility app conventions. Cmd+Q MUST fully quit. On Windows, clicking the X button MUST minimize to system tray if tray is enabled; otherwise, it MUST quit. On Linux, closing MUST follow the same logic as Windows, with Wayland display server detection to determine tray availability.
- **FR-058**: On Android, when the user has one or more active alarms, the app MUST maintain a foreground service with a persistent notification indicating the number of active alarms. This prevents Android from killing the alarm scheduling process. On iOS, the app MUST register for background app refresh to reschedule alarms when the app is suspended.
- **FR-059**: The app MUST register a custom URL scheme `deltat://` on all platforms. The scheme MUST support at minimum: `deltat://add?tz={IANA_ID}&label={optional_label}` to add a timezone to the display list, and `deltat://alarm?time={HH:MM}&tz={IANA_ID}&label={optional_label}` to open the alarm creation form pre-filled with the provided values. If the app is not running, the deep link MUST launch the app first, then process the URL. Invalid or malformed deep links MUST display a user-facing error toast, not crash.
- **FR-060**: When SunCalc computes sunrise/sunset for a timezone whose reference city is at a polar latitude (above 66.5°N or below 66.5°S), and there is no sunrise or sunset on that date (polar day/night), the day/night indicator MUST display "24h daylight" or "24h darkness" with an appropriate icon instead of showing incorrect or NaN times. If SunCalc receives invalid coordinates (NaN, out-of-range), the day/night indicator MUST be hidden for that timezone entry with no error shown to the user.
- **FR-061**: When the device is in Do Not Disturb / Focus mode and an alarm fires, the in-app alarm banner MUST still display if the app is in the foreground (since it is internal UI, not an OS notification). The alarm state MUST transition normally (active → completed/snoozed) regardless of whether the OS delivered the notification. The app MUST NOT attempt to bypass or override the OS DND setting.
- **FR-062**: On Android, the app MUST use `AlarmManager.setExactAndAllowWhileIdle()` for scheduling alarm notifications to ensure delivery during Doze mode. The app MUST request the `SCHEDULE_EXACT_ALARM` permission (required on Android 12+ / API 31+). If the permission is denied, the app MUST warn the user that alarm timing may be imprecise and fall back to `setAndAllowWhileIdle()` (inexact).

### Non-Functional Requirements

- **NFR-001**: On Android, the app MUST request exemption from battery optimization (Doze mode whitelist) when the user creates their first alarm. If the exemption is denied, the app MUST warn that alarms may be delayed when the device is idle. Alarm scheduling MUST use `AlarmManager.setExactAndAllowWhileIdle()` (survives Doze). On Android 12+ (API 31+), the `SCHEDULE_EXACT_ALARM` permission MUST be requested; if denied, fall back to `setAndAllowWhileIdle()` with a warning. The foreground service (FR-058) uses `foregroundServiceType="specialUse"` with a persistent notification.
- **NFR-002**: The app MUST implement a global error boundary that catches unhandled exceptions and plugin initialization failures. On a caught error, the app MUST log the error (and send a crash report if telemetry is enabled), display a user-friendly error message, and attempt to continue operating rather than crashing. Specific failure modes: (a) SunCalc polar coordinate edge cases — display "24h day/night" instead of NaN; (b) SunCalc invalid coordinates — hide day/night indicator for that zone; (c) WebView unresponsive — backend heartbeat detection after 30s silence triggers native reload prompt; (d) Plugin init failure — feature-specific degradation documented per plugin in NFR-004.
- **NFR-003**: All design token color values MUST be validated at build time (via Style Dictionary build script or CI linting) to confirm WCAG 2.1 AA contrast compliance for every foreground/background pair in light, dark, and high-contrast themes.
- **NFR-004**: Platform-specific code paths (macOS close-to-tray, Windows tray minimize, Android foreground service, iOS background refresh, Linux Wayland detection) MUST be gated behind platform detection at startup. Non-applicable platform code MUST NOT be loaded or executed on other platforms.
- **NFR-005**: Each Tauri plugin MUST have a documented degradation behavior if initialization fails at startup or if the plugin becomes unavailable mid-session: (a) notification — alarms fire as in-app banners only, no OS notifications; (b) autostart — hide the "launch at startup" toggle in settings; (c) store — show a fatal error dialog, app cannot function without persistence; (d) positioner — window opens at OS default position; (e) single-instance — skip duplicate detection, allow multiple windows; (f) os — assume "desktop/unknown" platform; (g) updater — disable auto-update silently, no user impact; (h) dialog — use browser-native confirm() / alert() as fallback; (i) deep-link — deep links silently fail, no user impact.

### Key Entities

- **TimeZoneEntry**: Represents a user-selected time zone to display. Attributes include IANA identifier (e.g., "America/New_York"), display label (customizable, e.g., "Mom's House"), sort order position, and pinned-to-tray flag.
- **Alarm**: Represents a scheduled alert. Attributes include target time, associated IANA time zone, label/title, recurrence pattern (once / daily / weekdays / weekends / custom days), enabled/disabled status, lifecycle state (active / snoozed / completed / dismissed), notification sound preference, snooze count (0 or 1, max 1 snooze allowed), and created/completed timestamps. One-time alarms transition to "completed" after firing; recurring alarms remain "active" and reset for the next occurrence. Snoozed alarms re-fire after 5 minutes.
- **UserPreferences**: Represents global application settings. Attributes include time format (12h/24h), locale/language, theme (light/dark/system), high contrast mode (on/off/system), default time zone, always-on-top preference, launch-at-startup preference, telemetry opt-in flag (default: false), auto-update enabled flag (default: true), window state (position x/y, width, height, maximized flag — desktop only), compact mode flag (default: false), compact window state (separate position/size — desktop only).

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can view accurate current time for any selected IANA timezone within 2 seconds of launching the app.
- **SC-002**: 95% of users can add a new time zone to their display in under 10 seconds (search, select, confirm).
- **SC-003**: Displayed times are accurate to within ±1 second of the device's system clock at all times, including during DST transitions.
- **SC-004**: Alarms fire within ±5 seconds of the scheduled time on desktop, and within ±60 seconds on mobile (due to OS scheduling constraints).
- **SC-005**: The app consumes less than 50MB of memory on desktop and less than 30MB on mobile during normal operation with 10 time zones displayed.
- **SC-006**: The app runs on all 5 target platforms (macOS, Windows, Linux, iOS, Android) from a single shared codebase.
- **SC-007**: 90% of first-time users can successfully add a custom time zone and set an alarm without consulting help documentation.
- **SC-008**: The app starts and displays clocks in under 2 seconds on desktop and under 3 seconds on mobile.
- **SC-009**: Battery usage on mobile does not exceed 2% per hour when the app is backgrounded with one active alarm.
- **SC-010**: The app functions identically whether the device is online or completely offline.
- **SC-011**: Telemetry submission adds no more than 50ms to any user-facing operation and never blocks UI rendering.
- **SC-012**: Auto-update check completes within 5 seconds on a typical broadband connection without blocking app startup.
- **SC-013**: Store schema migration completes within 500ms for stores containing the maximum allowed entries (30 zones, 50 alarms).
- **SC-014**: CI/CD pipeline produces signed, installable artifacts for all 5 platforms from a single commit within 30 minutes.
- **SC-015**: Data export produces a valid JSON file that can be imported on any platform running the same or newer app version, restoring all timezone entries, alarms, and preferences.
- **SC-016**: All foreground/background color pairs in the default light, dark, and high-contrast themes meet WCAG 2.1 AA contrast requirements (4.5:1 normal text, 3:1 large text) as validated by automated CI checks.
- **SC-017**: The app UI is fully navigable by keyboard-only (Tab, Shift+Tab, Enter, Escape, Arrow keys) with visible focus indicators on all interactive elements. No keyboard trap exists outside of intentional modal focus traps.
- **SC-018**: In compact mode, the app displays all selected timezones in a window no wider than 250px, with each timezone rendering on a single line, and the window occupying less than 15MB of memory.
- **SC-019**: Timezone search returns relevant results for single-character typos within 50ms for the bundled dataset of ~400 timezones, and correctly resolves UTC offset queries to matching timezones.
- **SC-020**: Timezone data patches (independent of full app updates) are less than 50KB and apply within 200ms without visible UI disruption.
- **SC-021**: Deep links (`deltat://add`, `deltat://alarm`) are processed within 1 second of app launch (cold start) or 200ms (warm, app already running), with the relevant UI state visible to the user.
- **SC-022**: On Android with Doze mode active and battery optimization exemption granted, alarms fire within ±60 seconds of the scheduled time (as per SC-004) without waking the device from deep sleep more than once per alarm.

## Assumptions

- Users have a basic understanding of time zones and are adding zones they personally need (the app does not suggest zones).
- The app name is "Delta-T Zaman" (as suggested by the repository name), meaning "time difference" — suitable for a world clock.
- The system clock on the user's device is reasonably accurate; the app does not implement NTP synchronization.
- Desktop platforms (macOS, Windows, Linux) are the primary targets; mobile (iOS, Android) is supported but considered secondary with known limitations (no system tray, limited background execution).
- The app does not require user authentication. Core features are purely local — all timezone display, alarm management, and preferences work offline. Optional cloud services (telemetry and auto-update) use Cloudflare infrastructure and require network access but never block core functionality.
- A curated list of approximately 400+ IANA time zones with associated city names will be bundled for offline search (no network lookup needed).
- The app uses the system's native WebView for rendering; no Chromium is bundled (consistent with Tauri's architecture, resulting in small binary size).
- Auto-update is included in v1 scope using `tauri-plugin-updater`. Update manifest and binaries are hosted on Cloudflare R2 served via a Cloudflare Worker. Desktop users receive automatic update checks on launch; mobile updates flow through app store mechanisms.
- The app will support light and dark themes, defaulting to the system preference.
- Sound/vibration for alarm notifications follows OS-level notification settings; the app does not bundle custom alarm sounds in v1.
- v1 ships with English (en) as the only UI language. The locale file architecture supports adding translations for UI chrome (button labels, menus, etc.) in future versions without code changes. Time/date formatting already uses the Intl API for locale-aware display.
- Code signing certificates (Apple Developer ID, Windows Authenticode) and Ed25519 update signing keys are managed as CI secrets. Key rotation procedures are a post-v1 operational concern.
- A privacy policy will be authored and hosted at a stable HTTPS URL (Cloudflare Pages) before any public release or app store submission.
- Snooze is limited to a single 5-minute snooze per alarm firing. This keeps alarm UX simple and avoids infinite snooze chains.
- IANA timezone database updates occur 3-4 times per year. The app's bundled data may be up to 3 months stale; the timezone data patch mechanism reduces this to days. Stale data is acceptable for most users since DST changes are announced well in advance.
- High contrast mode uses a separate design token set validated at build time. The three token sets (light, dark, high-contrast) are independent — high contrast is not derived from the current theme.
- On Android, a foreground service notification is a reasonable trade-off for reliable alarm delivery. Users who find the persistent notification unacceptable can disable alarms.
- Fuzzy search with Levenshtein distance ≤ 1 is sufficient for typo tolerance in timezone names. More aggressive fuzzy matching (distance ≥ 2) would produce too many false positives for a 400-item dataset.
- The `deltat://` custom URL scheme is registered at build time in platform-specific configuration files (Info.plist on iOS/macOS, AndroidManifest.xml on Android, registry on Windows, .desktop file on Linux). Deep links are a convenience feature — the app is fully functional without them.
- The app does not attempt to override or bypass OS Do Not Disturb / Focus mode settings. Missed notifications during DND are expected behavior consistent with system-level alarm apps. Users who need guaranteed alarm delivery should configure DND exceptions at the OS level.
- Android target: API 33 (Android 13) as target SDK, API 26 (Android 8.0 Oreo) as minimum SDK. This covers ~95% of active Android devices and provides access to notification channels, exact alarm APIs, and foreground service types.
- WebView heartbeat monitoring (10s interval, 30s timeout) is a best-effort crash detection mechanism. It cannot detect all WebView failure modes but covers the most common ones (infinite loops, memory pressure OOM). False positives (slow JS execution) are mitigated by the generous 30s timeout.
