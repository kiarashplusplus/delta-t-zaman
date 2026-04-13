# Tasks: Cross-Platform World Clock (Delta-T Zaman)

**Input**: Design documents from `/specs/001-tauri-world-clock/`
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅, contracts/ipc-commands.md ✅, quickstart.md ✅

**Tests**: Not explicitly requested — test infrastructure is included in setup but individual test tasks are omitted. Add tests following research.md R9 (Vitest + Playwright + cargo test) and the file layout in quickstart.md.

**Organization**: Tasks are grouped by the 6 user stories from spec.md (US1–US6) to enable independent implementation and testing.

## Format: `[ID] [P?] [Story?] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[US#]**: Which user story this task belongs to (US1–US6)
- All paths are relative to repository root

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Scaffold the Tauri project, install all dependencies, configure build tooling

- [ ] T001 Initialize Tauri 2.10.3 project with vanilla-ts template using `npm create tauri-app` per quickstart.md — generates src/, src-tauri/, package.json, vite.config.ts
- [ ] T002 Install frontend dependencies in package.json: `suncalc`, `style-dictionary@4` (dev), `@tauri-apps/plugin-notification`, `@tauri-apps/plugin-store`, `@tauri-apps/plugin-autostart`, `@tauri-apps/plugin-os`, `@tauri-apps/plugin-dialog`, `@tauri-apps/plugin-updater`, `@tauri-apps/plugin-deep-link`, `@tauri-apps/plugin-positioner`
- [ ] T003 [P] Add Rust plugin dependencies to src-tauri/Cargo.toml: tauri-plugin-notification, tauri-plugin-store, tauri-plugin-autostart, tauri-plugin-single-instance, tauri-plugin-positioner, tauri-plugin-os, tauri-plugin-updater, tauri-plugin-dialog, tauri-plugin-deep-link, reqwest (telemetry HTTP), serde/serde_json, uuid
- [ ] T004 [P] Configure Vite build in vite.config.ts with TypeScript compilation, asset handling, and Tauri integration per quickstart.md
- [ ] T005 [P] Configure TypeScript in tsconfig.json with strict mode, DOM lib, ES2022 target, path aliases (@/ → src/) per quickstart.md
- [ ] T006 [P] Configure tauri.conf.json with app metadata (productName: "Delta-T Zaman", identifier: "com.deltat.zaman"), window settings (420×680 default, 320×480 min), CSP policy, and updater public key placeholder per quickstart.md
- [ ] T007 [P] Configure Vitest in vitest.config.ts and Playwright in playwright.config.ts for test infrastructure per research.md R9 — tests/ directory with unit/, dom/, e2e/ subdirectories
- [ ] T008 Initialize mobile targets: `cargo tauri ios init` and `cargo tauri android init` per quickstart.md — verify gen/ios/ and gen/android/ directories are created with valid project files and initial build does not error

**Checkpoint**: Project scaffolding complete — all dependencies installed, build tools configured, `cargo tauri dev` runs a blank window

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T009 [P] Create base design tokens in tokens/base/color.json (primitive palette: gray-50..900, accent, danger, success, warning), tokens/base/spacing.json (4px scale: xs through 2xl), tokens/base/typography.json (font families, sizes, weights, line-heights in rem), tokens/base/shadow.json (elevation sm/md/lg), tokens/base/timing.json (animation durations fast/normal/slow) per research.md R2
- [ ] T010 [P] Create semantic theme tokens in tokens/semantic/light.json and tokens/semantic/dark.json mapping base primitives to semantic roles (surface, on-surface, primary, on-primary, border, muted, accent, danger, success) per research.md R2
- [ ] T011 [P] Create component-level tokens in tokens/component/clock-card.json and tokens/component/alarm-card.json (background, border, text, badge colors) per research.md R2
- [ ] T012 [P] Create high-contrast token overrides in tokens/high-contrast/overrides.json with solid borders, no gradients, minimum 7:1 contrast ratios per research.md R18 and FR-050
- [ ] T013 Create Style Dictionary config in style-dictionary.config.json — transform tokens into src/styles/tokens.css (CSS custom properties for light + dark via data-theme attribute) and src/styles/high-contrast.css per research.md R2; add `build:tokens` script to package.json
- [ ] T014 [P] Create TypeScript type definitions in src/types/timezone.ts — TimeZoneEntry interface (id, iana_id, display_label, sort_order, pinned_to_tray, created_at), TimezoneMetadata interface (city, country, country_code, utc_offset_minutes, latitude, longitude, aliases[]) matching data-model.md
- [ ] T015 [P] Create TypeScript type definitions in src/types/alarm.ts — Alarm interface (id, title, target_time, timezone_id, recurrence, custom_days, enabled, state, snooze_count, sound_enabled, notification_id, created_at, completed_at), AlarmState enum (active/snoozed/completed/dismissed), Recurrence enum (once/daily/weekdays/weekends/custom) matching data-model.md and contracts/ipc-commands.md
- [ ] T016 [P] Create TypeScript type definitions in src/types/preferences.ts — UserPreferences interface (time_format, locale, theme, high_contrast, default_timezone, always_on_top, autostart, telemetry_opt_in, auto_update, window_state, compact_mode, compact_window_state), SystemInfo, PlatformCloseBehavior, WindowState, all IPC request/response types from contracts/ipc-commands.md
- [ ] T017 [P] Create bundled timezone metadata in src/data/timezones.json — ~400 IANA zones with id, city, country, country_code, utc_offset_minutes, latitude, longitude, aliases[] per data-model.md and research.md R3; manually include UTC entry (omitted by Chrome WebViews)
- [ ] T018 [P] Create UI string locale file in src/locales/en.json — externalize all button labels, menu items, settings labels, error messages, placeholder text, ARIA labels, toast messages per FR-045
- [ ] T019 [P] Create typed pub/sub event bus in src/state/event-bus.ts — subscribe(event, callback), publish(event, data), unsubscribe(id) with TypeScript generics for type-safe events, returns unsubscribe function per research.md R1
- [ ] T020 [P] Create persistence service in src/services/persistence.ts — wraps @tauri-apps/plugin-store for zones.dat, alarms.dat, preferences.dat; schema_version field in each store; migration runner for version upgrades per FR-035 (forward-only v1→v2→v3…, failed migration triggers corruption-recovery per FR-029); corrupted store recovery (backup + reset to defaults per FR-029); debounced writes per research.md R7
- [ ] T021 [P] Create IPC service in src/services/ipc.ts — wraps all @tauri-apps/api/core invoke() calls with typed request/response interfaces; one function per command (25 total from contracts/ipc-commands.md); centralized error handling
- [ ] T022 [P] Create platform detection service in src/services/platform.ts — uses @tauri-apps/plugin-os to detect platform (macos/windows/linux/ios/android), expose is_mobile, is_desktop, supports_tray flags; conditionally load platform-specific code paths per research.md R19 and NFR-004
- [ ] T023 [P] Create accessibility utilities in src/services/accessibility.ts — trapFocus(container) for modal dialogs per FR-053, matchMedia listeners for prefers-reduced-motion (FR-051) and prefers-contrast (FR-050), data-attribute toggles, .sr-only class helper, ARIA live region announcer per research.md R18
- [ ] T024 [P] Create keyboard shortcuts service in src/services/keyboard-shortcuts.ts — register global shortcuts: Cmd/Ctrl+N (add timezone), Cmd/Ctrl+K (search), Cmd/Ctrl+, (settings), Cmd/Ctrl+M (compact mode), Cmd/Ctrl+Q (quit), Escape (close modal/search), document key event delegation per FR-046 and FR-027
- [ ] T025 Create root HTML structure in src/index.html — [data-theme] and [data-reduced-motion] attributes on html, meta viewport, link to styles/main.css, tab navigation container (Clocks/Alarms/Settings tabs), main content area, toast container, import main.ts via script module per plan.md
- [ ] T026 [P] Create base CSS in src/styles/main.css — @import tokens.css, BEM component class structure, .sr-only utility, :focus-visible indicators (2px solid, 2px offset), responsive CSS grid layout, @container queries for clock list, scrollbar styling, transition defaults (respects reduced-motion), print styles disabled per research.md R2/R18
- [ ] T027 [P] Implement Rust plugin registration in src-tauri/src/lib.rs — register all 9 Tauri plugins (notification, store, autostart, single-instance, positioner, os, updater, dialog, deep-link) with NFR-005 degradation on init failure; generate_handler![] for all 25 IPC commands per contracts/ipc-commands.md
- [ ] T028 [P] Implement Rust desktop entry point in src-tauri/src/main.rs — cfg windows_subsystem attribute to suppress console window on Windows, call lib::run(); verify `cargo build` compiles without errors
- [ ] T029 [P] Implement Rust command module exports in src-tauri/src/commands/mod.rs — pub mod for system, tray, alarm, window, telemetry, updater, export, platform, tz_data, deep_link, heartbeat; verify `cargo check` confirms all 11 submodules compile and all public command functions are reachable from lib.rs
- [ ] T030 [P] Implement get_system_info command in src-tauri/src/commands/system.rs — return platform, arch, locale, timezone (IANA), app_version, is_mobile, supports_tray, os_version per contracts/ipc-commands.md
- [ ] T031 [P] Create Tauri capability declarations in src-tauri/capabilities/main.json — all required permissions: core:default, notification (default + send + request + is-permission-granted), store:default, os:default, autostart:default, single-instance:default, positioner:default, updater:default, dialog (default + open + save), deep-link:default per contracts/ipc-commands.md capability requirements
- [ ] T032 [P] Implement global error boundary in src/main.ts — window.onerror + window.onunhandledrejection handlers, plugin init failure detection with per-plugin NFR-005 degradation (notification → in-app only, autostart → hide toggle, store → fatal dialog, etc.), crash telemetry hook, user-friendly error toast per NFR-002
- [ ] T033 Build and run `npm run build:tokens` to generate src/styles/tokens.css from Style Dictionary — verify CSS custom properties output for all themes, validate WCAG contrast pairs per NFR-003

**Checkpoint**: Foundation ready — type system, design tokens, event bus, persistence, IPC, platform detection, accessibility, and Rust backend skeleton are all in place. User story implementation can now begin.

---

## Phase 3: User Story 1 — View Multiple Time Zones at a Glance (Priority: P1) 🎯 MVP

**Goal**: Display accurate, real-time clocks for multiple IANA timezones with day/night indicators and auto-DST handling (FR-001, FR-002, FR-003, FR-014, FR-019, FR-020, FR-024, FR-043, FR-044).

**Independent Test**: Launch the app → verify 3 default timezone clocks display correct times → times update every second on desktop → day/night indicators reflect actual solar position → UTC offset and abbreviation shown → "Tomorrow" badge appears when a zone is on a different date.

### Implementation for User Story 1

- [ ] T034 [P] [US1] Create time ticker state in src/state/time.ts — publishes current UTC timestamp at 1-second intervals (desktop) via event bus using requestAnimationFrame + Date.now(), coalesces to single tick for all subscribers, pauses when document.hidden (Page Visibility API) per research.md R1 and FR-002
- [ ] T035 [P] [US1] Create clocks state in src/state/clocks.ts — manages TimeZoneEntry[] array, load/save to zones.dat via persistence service, exposes add/remove/reorder/update functions, publishes zone-change events via event bus, enforces max 30 zones (FR-001)
- [ ] T036 [P] [US1] Create preferences state in src/state/preferences.ts — manages UserPreferences singleton, loads from preferences.dat via persistence service, merges defaults from get_system_info on first launch (FR-044), publishes preference-change events via event bus
- [ ] T037 [P] [US1] Implement day/night calculation service in src/services/daynight.ts — wraps SunCalc: compute sunrise/sunset for lat/lng + date, handle polar edge cases (|lat| > 66.5°: return {polar: true, isDaylight: boolean} for "24h daylight"/"24h darkness"), handle NaN/invalid coords (return null → hide indicator), cache results for 60 seconds per research.md R5 and FR-060
- [ ] T038 [US1] Implement day/night indicator component in src/components/daynight-indicator.ts — create(container) renders solar-based gradient bar (sunrise position, sunset position, current time marker) or "24h daylight/darkness" text for polar zones, accepts lat/lng/timestamp, BEM classes (daynight-indicator, --day, --night, --polar), ARIA label describing light conditions per FR-024/FR-060
- [ ] T039 [US1] Implement clock card component in src/components/clock-card.ts — create(container, zoneEntry, metadata) renders: digital time in 12h/24h format per preference using Intl.DateTimeFormat, date with abbreviated day-of-week (FR-043), UTC offset with abbreviation (e.g., "UTC-5 EST") per FR-019, relative offset from local timezone (e.g., "+3h" or "same time") per FR-020, day/night indicator (T038), "Tomorrow"/"+1 day" badge when zone date differs from local; controller.update(timestamp) refreshes time via textContent only (≤1ms per tick); subscribes to time ticker (T034) per research.md R1/R3
- [ ] T040 [US1] Implement clock list component in src/components/clock-list.ts — create(container) renders scrollable list of clock-card instances, subscribes to clocks state (T035) for add/remove/reorder DOM updates, empty state shows local timezone card + "Add a timezone" prompt per FR-001, ARIA role="list" per FR-028
- [ ] T041 [US1] Implement first-launch default zones in src/main.ts — on empty zones.dat, populate 3 defaults: (1) user's detected local timezone from get_system_info, (2) UTC (manually constructed entry), (3) geographically contrasting zone based on locale (e.g., en-US → Europe/London) per FR-044
- [ ] T042 [US1] Wire app initialization in src/main.ts — load stores via persistence service, call get_system_info, initialize platform service, apply saved theme to [data-theme], create clock-list and mount to Clocks tab, start time ticker, restore window state (save_window_state/restore_window_state) per plan.md

**Checkpoint**: User Story 1 complete — app launches showing 3 default timezone clocks with real-time 1/sec updates, day/night indicators, UTC offsets, and proper DST handling. Core MVP is functional.

---

## Phase 4: User Story 2 — Add, Remove, and Reorder Time Zones (Priority: P1) 🎯 MVP

**Goal**: Users can search for timezones with fuzzy matching, add/remove them, drag to reorder, rename labels, and all changes persist across restarts (FR-004, FR-005, FR-006, FR-015, FR-056).

**Independent Test**: Search "Toky" (fuzzy) → see Tokyo result → add it → search "+5:30" (offset) → add India → try adding Tokyo again → see duplicate warning → drag Tokyo above India → close and reopen app → verify order persists.

### Implementation for User Story 2

- [ ] T043 [P] [US2] Implement timezone search service in src/services/timezone-search.ts — load src/data/timezones.json, hybrid search: exact substring match + fuzzy match (Levenshtein distance ≤ 1, ~50 LOC custom implementation), UTC offset query parsing ("+5:30", "GMT-5", "UTC+9"), results ranked: exact prefix > fuzzy prefix > substring > offset match, alias-aware deduplication, returns top 20 results in <50ms per FR-004/FR-056 and research.md R4
- [ ] T044 [US2] Implement timezone search component in src/components/timezone-search.ts — modal overlay with search input (debounce 150ms), results dropdown with keyboard navigation (ArrowUp/Down/Enter/Escape), add-on-select, duplicate detection warning toast (FR-015: checks iana_id + known aliases), max 30 zone limit check with message (FR-001), ARIA live region announcing result count, focus trap per FR-053, opens via Cmd/Ctrl+N or Cmd/Ctrl+K or "+" button
- [ ] T045 [US2] Implement drag-and-drop reorder in src/components/clock-list.ts — HTML5 Drag API (draggable attribute, dragstart/dragover/drop events) with CSS transform animation for visual placeholder, touch fallback (touchstart/touchmove/touchend with pointer position tracking), keyboard accessibility (Space to grab, Arrow keys to move, Space/Enter to drop, Escape to cancel, ARIA live announcements "Grabbed item 2 of 5", "Moved to position 3") per research.md R8 (~150 LOC)
- [ ] T046 [US2] Implement zone removal in src/components/clock-card.ts — delete button (trash icon) on hover/focus, confirmation for zones with associated alarms (warn: "2 alarms reference this timezone"), emits zone-remove event to clocks state, sort_order re-indexing after removal
- [ ] T047 [US2] Implement display label editing in src/components/clock-card.ts — inline contenteditable field for display_label, max 100 characters, persisted on blur/Enter, Escape reverts edit, empty label falls back to city name per data-model.md
- [ ] T048 [US2] Wire persistence for zone CRUD in src/state/clocks.ts — auto-save to zones.dat on every add/remove/reorder/label-edit via persistence service, debounced writes (300ms) to avoid rapid-fire saves during drag reorder

**Checkpoint**: User Stories 1 AND 2 complete — full timezone management with fuzzy search, add, remove, reorder (drag + keyboard), label editing, and cross-restart persistence. Viable MVP ready for user testing.

---

## Phase 5: User Story 3 — System Tray / Menu Bar Quick Access (Priority: P2)

**Goal**: Desktop users get a persistent system tray icon showing pinned zone times, with window toggle, compact mode, always-on-top, autostart, and platform-correct close behavior (FR-007, FR-008, FR-021, FR-055, FR-057).

**Independent Test**: Pin 2 zones to tray → minimize to tray → verify tray tooltip shows times → right-click tray icon → see zone times + menu items → click "Show Window" → toggle compact mode via Cmd/Ctrl+M → verify narrow single-line view → close window → verify hides to tray on macOS, minimizes on Windows.

### Implementation for User Story 3

- [ ] T049 [P] [US3] Implement Rust tray setup and event handling in src-tauri/src/tray.rs — create SystemTray with icon, build context menu dynamically (pinned zone times + separator + "Show Window" + "Toggle Compact Mode" + "Quit"), handle tray icon click (toggle main window), handle menu item clicks, emit tray-action events to frontend per contracts/ipc-commands.md
- [ ] T050 [P] [US3] Implement update_tray_display command in src-tauri/src/commands/tray.rs — accept Vec<TrayZone> (zone_id, label, formatted_time, is_pinned), rebuild tray tooltip text and context menu items, called by frontend every 60 seconds per contracts/ipc-commands.md
- [ ] T051 [P] [US3] Implement hide_to_tray and show_from_tray commands in src-tauri/src/commands/tray.rs — hide: set window visible(false); show: set visible(true) + focus + unminimize; Linux graceful fallback: if tray unavailable, minimize instead of hide per contracts/ipc-commands.md and FR-057
- [ ] T052 [P] [US3] Implement get_platform_close_behavior command in src-tauri/src/commands/window.rs — return platform-specific close action: macOS → hide_to_tray (always), Windows → hide_to_tray if tray_enabled else quit, Linux → detect Wayland/X11 via env vars + check tray support → hide_to_tray or quit; mobile → suspend per contracts/ipc-commands.md and FR-057
- [ ] T053 [P] [US3] Implement set_always_on_top command in src-tauri/src/commands/window.rs — set window.set_always_on_top(enabled), persist preference; verify window stays above all other windows when enabled, toggling off returns to normal z-order, preference persists across restart per contracts/ipc-commands.md and FR-021
- [ ] T054 [P] [US3] Implement save_window_state and restore_window_state commands in src-tauri/src/commands/window.rs — save: read window position/size/maximized → write to preferences.dat; restore: read from preferences.dat → apply to window, detect offscreen position (disconnected monitor) → reset to centered per contracts/ipc-commands.md and FR-042
- [ ] T055 [P] [US3] Implement set_autostart command in src-tauri/src/commands/window.rs — enable/disable launch at login using tauri-plugin-autostart, persist preference; verify app appears in macOS Login Items / Windows Task Manager Startup / Linux autostart directory when enabled, and is removed when disabled per contracts/ipc-commands.md
- [ ] T056 [US3] Implement toggle_compact_mode command in src-tauri/src/commands/window.rs — when enabling: save full window state, create/show compact window (200px default width, always-on-top by default, independent position persistence), hide full window; when disabling: save compact window state, show full window, hide compact window; update compact_mode in preferences.dat per contracts/ipc-commands.md and FR-055
- [ ] T057 [US3] Implement compact view component in src/components/compact-view.ts — create(container) renders minimal single-line strip per timezone: `[Label] [Time]`, no date/day-night/controls, narrow layout (200px default), borderless window appearance, subscribes to clocks state + time ticker per FR-055
- [ ] T058 [US3] Wire tray integration in src/main.ts — call update_tray_display every 60 seconds with pinned zones formatted data, listen for tray-action events (show_window → show_from_tray, quit → app exit, zone_clicked → scroll to zone, toggle_compact → toggle_compact_mode), apply platform close behavior on window close per FR-007/FR-008, create tray-preview data formatter in src/components/tray-preview.ts

**Checkpoint**: User Story 3 complete — system tray integration, compact mode, always-on-top, window state persistence, autostart, and platform-correct close behavior all functional on desktop.

---

## Phase 6: User Story 4 — Set Alarms and Receive Notifications (Priority: P2)

**Goal**: Users can create/manage alarms tied to any IANA timezone, receive native OS notifications with Snooze/Dismiss actions, alarms recur on schedule, orphaned alarms survive timezone removal, and everything persists independently (FR-009, FR-010, FR-011, FR-018, FR-025, FR-026, FR-030, FR-031, FR-041, FR-061, FR-062).

**Independent Test**: Create alarm for 1 min from now in Asia/Tokyo timezone → minimize app → receive native notification → tap Snooze → receive re-fire 5 min later → create daily recurring alarm → verify it fires next day → remove the Tokyo timezone from display → verify alarm still works.

### Implementation for User Story 4

- [ ] T059 [P] [US4] Create alarms state in src/state/alarms.ts — manages Alarm[] array, load/save to alarms.dat via persistence service, alarm state transitions: active → completed (one-time after firing), active → snoozed (snooze pressed) → completed (after re-fire), active → dismissed (dismiss pressed), enable/disable toggle, enforce 50-alarm active limit (FR-026), retain completed/dismissed for 30 days (FR-025), publish alarm-change events via event bus
- [ ] T060 [P] [US4] Implement alarm scheduler service in src/services/alarm-scheduler.ts — compute next fire time for each alarm: resolve target_time (HH:mm) in alarm's timezone using Intl.DateTimeFormat, handle DST transitions (skip invalid times, resolve ambiguous times to first occurrence per spec edge case), recurring logic (daily: tomorrow same time, weekdays: next Mon-Fri, weekends: next Sat-Sun, custom: next matching day), call schedule_alarm_notification IPC with computed UTC datetime, handle snooze reschedule (current time + 5 min), on app startup reschedule all active alarms per FR-030/FR-041
- [ ] T061 [P] [US4] Implement schedule_alarm_notification and cancel_alarm_notification commands in src-tauri/src/commands/alarm.rs — schedule: use tauri-plugin-notification Schedule.at(datetime) with title/body/alarm_id, add action buttons "Snooze" and "Dismiss", store notification_id; cancel: remove pending notification by id; emit alarm-fired event on delivery, emit alarm-snoozed event on Snooze action per contracts/ipc-commands.md
- [ ] T062 [P] [US4] Implement get_notification_permission and request_notification_permission commands in src-tauri/src/commands/alarm.rs — check: return current permission state (granted/denied/default); request: trigger OS permission dialog, return result per contracts/ipc-commands.md
- [ ] T063 [US4] Implement alarm form component in src/components/alarm-form.ts — create/edit modal form: time picker input (HH:mm, 24h internal format displayed in user's 12h/24h preference), timezone selector dropdown (all ~400 IANA zones from bundled data, not limited to display list), title input (max 200 chars), recurrence radio group (once/daily/weekdays/weekends/custom), custom day checkboxes (Mon–Sun, visible only when custom selected), sound toggle, form validation (required: time + timezone), modal with focus trap (FR-053), ARIA labels per FR-028
- [ ] T064 [US4] Implement alarm card component in src/components/alarm-card.ts — create(container, alarm) renders: alarm title, target time in alarm's timezone (formatted per user preference), "in [timezone city]" label, recurrence pattern text, enabled/disabled toggle switch, state badge (🟢 active / 🟡 snoozed / ⚪ completed / ❌ dismissed), edit button → opens alarm form, delete button with confirmation
- [ ] T065 [US4] Implement alarm list component in src/components/alarm-list.ts — create(container) renders alarm cards grouped by state (active first, then snoozed, then completed/dismissed), empty state shows "No alarms yet" + create button, on first alarm creation: check notification permission → request if needed (FR-009), "+" button opens alarm form, mount to Alarms tab, ARIA role="list"
- [ ] T066 [US4] Wire alarm events in src/main.ts — listen for alarm-fired events: update alarm state (completed if one-time, reschedule if recurring via alarm-scheduler), show in-app alarm banner if app is foreground (FR-061, banner displays regardless of OS DND); listen for alarm-snoozed events: update alarm state to snoozed, schedule re-fire via alarm-scheduler (current time + 5 min); max 1 snooze per firing per data-model.md
- [ ] T067 [US4] Implement orphaned alarm handling in src/state/alarms.ts — when a timezone is removed from the display list, alarms referencing that timezone_id remain fully functional, alarm card shows timezone city name from bundled timezones.json data (not from display list), alarm scheduling continues normally per FR-031
- [ ] T068 [US4] Implement alarm persistence edge cases in src/state/alarms.ts — on app restart: load alarms.dat, for each active/snoozed alarm check if fire time has passed → if yes and one-time: mark completed; if yes and recurring: compute next occurrence and reschedule; if snoozed and snooze time passed: fire immediately; persist snooze re-fire time to survive crashes per spec.md edge cases

**Checkpoint**: User Story 4 complete — alarm creation/editing/deletion, native OS notifications with Snooze/Dismiss, recurring alarms (daily/weekdays/weekends/custom), orphaned alarm resilience, and full lifecycle management working across all platforms.

---

## Phase 7: User Story 5 — Localized Display and Format Preferences (Priority: P3)

**Goal**: Users can switch between 12h/24h time format, toggle themes (light/dark/system/high-contrast), configure all app settings from a dedicated panel, and the UI respects OS accessibility preferences in real-time (FR-012, FR-013, FR-048, FR-050, FR-051, FR-054).

**Independent Test**: Open Settings → change time format from 12h to 24h → verify all clocks switch instantly → change theme to dark → verify UI updates → enable high contrast → verify token set switches and contrast ratios increase → increase OS font size to 200% → verify layout remains usable.

### Implementation for User Story 5

- [ ] T069 [P] [US5] Implement settings component in src/components/settings.ts — create(container) renders sectioned form: **Display** (time format 12h/24h radio, show-seconds toggle, theme selector light/dark/system, high contrast toggle on/off/system, compact mode toggle desktop-only), **Notifications** (default sound on/off), **Privacy** (telemetry opt-in toggle with explanation text, default: off), **Updates** (auto-update toggle, "Check Now" button, current version label), **Data** (Export button, Import button, "Reset to Defaults" with confirmation), all changes apply immediately and persist to preferences.dat per FR-048
- [ ] T070 [P] [US5] Implement about component in src/components/about.ts — create(container) renders: app name "Delta-T Zaman", version string from tauri.conf.json, build info, open-source licenses list (SunCalc, Style Dictionary, Tauri plugins), privacy policy link that opens in default browser; verify version matches tauri.conf.json and all license entries are present per FR-048
- [ ] T071 [US5] Implement theme switching in src/main.ts — on theme preference change: set [data-theme] attribute on document.documentElement to "light", "dark", or auto-detect via matchMedia('(prefers-color-scheme: dark)') for "system" mode; on high_contrast change: toggle high-contrast.css stylesheet; listen for OS prefers-color-scheme changes in real-time and re-apply if mode is "system" per research.md R2/R18
- [ ] T072 [US5] Implement format preference application — on time_format or locale change in preferences state, publish preference-change event, all clock-card and alarm-card instances re-render with updated Intl.DateTimeFormat options (hour12: true/false), settings tab and alarm form time picker reflect current format, persist immediately per FR-012/FR-013
- [ ] T073 [US5] Implement reduced motion support in src/main.ts — listen for prefers-reduced-motion media query changes, set [data-reduced-motion="true"] on document.documentElement; verify all CSS transition-duration values resolve to 0s when attribute is set, and toggle responds to OS prefers-reduced-motion changes in real-time per FR-051
- [ ] T074 [US5] Implement font scaling support in src/styles/main.css — verify all text sizes use rem/em units (no px for text), all layout containers use CSS grid/flexbox that adapts to content size, test at 200% browser zoom that no text is clipped and all controls remain accessible per FR-054

**Checkpoint**: User Story 5 complete — all display preferences (format, locale, theme, high contrast, font scaling, reduced motion) working with immediate UI response and cross-restart persistence.

---

## Phase 8: User Story 6 — Mobile-Optimized Experience (Priority: P3)

**Goal**: iOS and Android users get a touch-optimized, battery-friendly experience with swipe-to-delete, responsive layout, reduced tick rate, and reliable background alarms with OS integration (FR-016, FR-017, FR-022, FR-058, FR-062, NFR-001).

**Independent Test**: Install on a physical iOS/Android device → verify clock list renders correctly in portrait and landscape → swipe-left-to-delete a timezone → long-press-to-reorder → background the app for 30 minutes with an active alarm → return and verify clocks show correct time → verify alarm fired on schedule.

### Implementation for User Story 6

- [ ] T075 [P] [US6] Implement responsive mobile CSS in src/styles/main.css — portrait/landscape media query breakpoints, vertically scrollable clock list (overscroll-behavior: contain), touch-friendly tap targets (min 44×44px per WCAG 2.5.5), increased spacing between interactive elements, hide desktop-only controls (tray pin toggle, compact mode) on mobile, full-width clock cards per FR-022
- [ ] T076 [US6] Implement touch gestures in src/components/clock-list.ts — swipe-left-to-delete: touchstart/touchmove/touchend tracking with velocity threshold, reveal delete button on partial swipe, full swipe triggers removal with undo toast (3s); long-press-to-reorder: 500ms hold activates drag mode with haptic feedback (if available), smooth CSS transform animation respecting prefers-reduced-motion per FR-017
- [ ] T077 [US6] Implement mobile tick rate optimization in src/state/time.ts — detect is_mobile from platform service, switch to 1-minute update interval on mobile (saves battery), switch to 1-second when app is in foreground and user is actively viewing clocks tab, pause entirely when app is backgrounded (Page Visibility API) per FR-002 and research.md R11
- [ ] T078 [US6] Implement request_battery_exemption and request_exact_alarm_permission commands in src-tauri/src/commands/platform.rs — battery: Android only, trigger REQUEST_IGNORE_BATTERY_OPTIMIZATIONS intent on first alarm creation, return granted/already_exempt status, warn user on denial; exact alarm: Android 12+ only (API 31+), open system settings for SCHEDULE_EXACT_ALARM permission, fallback to setAndAllowWhileIdle() with "timing may be imprecise" warning on denial; both return {granted: true} on non-applicable platforms per contracts/ipc-commands.md, FR-062, NFR-001
- [ ] T079 [US6] Implement Android foreground service notification in src-tauri/src/commands/platform.rs — when active alarms exist: create persistent notification showing "N active alarms", foregroundServiceType="specialUse", auto-update count on alarm add/remove/complete, auto-dismiss when last alarm completes/disables per FR-058

**Checkpoint**: User Story 6 complete — mobile app is touch-optimized, battery-friendly, and delivers reliable alarm notifications on both iOS and Android with proper OS permission handling.

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: Features that span multiple user stories, infrastructure services, CI/CD, and final quality validation

- [ ] T080 [P] Implement export_user_data and import_user_data commands in src-tauri/src/commands/export.rs — export: serialize zones.dat + alarms.dat + preferences.dat into JSON envelope {export_version: "1.0.0", app_version, exported_at, timezone_entries[], alarms[], preferences{}}, exclude window_state and compact_window_state; import: validate export_version compatibility, run schema migration if needed, merge mode (add missing entries by UUID, keep existing preferences) or replace mode (overwrite all stores), trigger full UI refresh after import per contracts/ipc-commands.md, FR-047, and data-model.md export schema
- [ ] T081 [P] Implement export/import UI in src/components/export-import.ts — export button: invoke export_user_data → open OS save dialog (tauri-plugin-dialog) with default filename "deltat-backup-{date}.json" → write file; import button: open OS file picker (tauri-plugin-dialog, filter: .json) → read file → show merge/replace mode selector → invoke import_user_data → show success/error toast with counts per FR-047
- [ ] T082 [P] Implement send_telemetry_event command in src-tauri/src/commands/telemetry.rs — check telemetry_opt_in in preferences.dat, if false return immediately; HTTP POST to Cloudflare Worker endpoint with event {type, timestamp, app_version, platform, session_id (random per launch)}, 5s timeout via reqwest, fire-and-forget (silent on network failure, never blocks UI >50ms per SC-011), supported event types: app_open, zone_added, zone_removed, alarm_created, alarm_fired, crash_report per contracts/ipc-commands.md and FR-032/FR-033/FR-036
- [ ] T083 [P] Implement check_for_update and install_update commands in src-tauri/src/commands/updater.rs — check: use tauri-plugin-updater to fetch manifest from Cloudflare Worker URL, compare versions, emit update-available event to frontend if newer version exists, complete within 5s (SC-012), do not block app startup; install: download update binary, verify Ed25519 signature, prompt user via non-intrusive banner (not modal), apply on next restart per contracts/ipc-commands.md and FR-034
- [ ] T084 [P] Implement check_timezone_data_update command in src-tauri/src/commands/tz_data.rs — fetch timezone patch manifest from Cloudflare Worker, if patch_version > current timezone_data_version: download patch (<50KB per SC-020), apply in-memory to override bundled timezones.json entries, update timezone_data_version in preferences.dat, apply within 200ms (SC-020); non-blocking, called on app launch after UI renders; silent on network failure per contracts/ipc-commands.md and FR-049
- [ ] T085 [P] Implement handle_deep_link command in src-tauri/src/commands/deep_link.rs — parse deltat:// URLs, validate tz parameter against bundled timezones.json, validate time parameter matches HH:MM 24h format, URL-decode and sanitize label (max 50 chars, strip HTML), return structured DeepLinkResult {action: add_timezone|create_alarm|unknown, timezone?, label?, alarm_time?, valid, error_message?}, silently ignore unknown query parameters per contracts/ipc-commands.md and FR-059
- [ ] T086 [P] Implement deep link frontend integration in src/services/deep-link.ts — listen for deep-link-received events from Rust, call handle_deep_link command to parse/validate, on add_timezone action: add zone to display list (check duplicates), on create_alarm action: open alarm form pre-filled with time/timezone/label, show error toast for invalid URLs, process within 200ms warm / 1s cold start per FR-059 and SC-021; wire listener in src/main.ts
- [ ] T087 [P] Implement heartbeat monitoring in src/services/heartbeat.ts (frontend) + src-tauri/src/commands/heartbeat.rs (backend) — Rust: emit webview-heartbeat event every 10s with monotonic sequence counter + timestamp, track last ack sequence, if 3 beats missed (30s silence) show native Reload/Quit dialog; Frontend: listen for webview-heartbeat events, immediately invoke heartbeat_ack with received sequence number; wire in src/main.ts per contracts/ipc-commands.md and NFR-002c
- [ ] T088 [P] Implement check_dnd_status command in src-tauri/src/commands/system.rs — mobile: query platform DND/Focus mode API, return {dnd_active, platform_mode_name}; desktop: always return {dnd_active: false} (not reliably detectable); informational only — app never bypasses DND per contracts/ipc-commands.md and FR-061
- [ ] T089 [P] Create Cloudflare Worker for telemetry ingestion in infra/workers/telemetry/ — validate incoming event schema (type, timestamp, app_version, platform), write to Cloudflare Analytics Engine, CORS headers for app origin, ~50 lines per research.md R12
- [ ] T090 [P] Create Cloudflare Worker for update manifest and timezone data patches in infra/workers/updater/ — serve JSON update manifest from R2 bucket (version, download URLs per platform, Ed25519 signature), serve timezone patch JSON (patch_version, affected zones, changes), ~100 lines per research.md R13/R17
- [ ] T091 Create CI pipeline in .github/workflows/ci.yml — on push/PR: cargo clippy (Rust lint), cargo test (Rust unit tests), npm run lint (TS lint), tsc --noEmit (type check), npm run build:tokens (token build), vitest run (frontend unit tests), cargo tauri build --debug (build verification), matrix: ubuntu-latest + macos-latest + windows-latest per research.md R14
- [ ] T092 Create release pipeline in .github/workflows/release.yml — on tag push (v*): matrix build for macOS (universal binary, Apple Developer ID signing + notarization), Windows (x64 + arm64, Authenticode signing), Linux (x64 AppImage + .deb), iOS (Xcode archive, App Store submission), Android (signed AAB, Play Store submission); upload desktop artifacts to Cloudflare R2; generate update manifest JSON; Ed25519 sign update bundles per research.md R14/R15 and FR-037/FR-038/FR-039
- [ ] T093 [P] Create privacy policy document in infra/privacy-policy.md — cover all required categories: optional anonymous telemetry (event types listed), local-only data storage, no cloud sync, no user accounts, no third-party analytics, opt-in default (off); verify document is complete and Cloudflare Pages deployment returns 200 per FR-040
- [ ] T094 [P] Implement first-launch telemetry opt-in prompt in src/main.ts — on first launch (no telemetry_opt_in key in preferences.dat), display a modal dialog explaining anonymous telemetry (event types, no PII, disabled by default) with "Enable" and "No Thanks" buttons; persist choice to preferences.dat telemetry_opt_in field; dialog MUST be dismissible and default to off if closed without action per FR-032 and Constitution Amendment A1
- [ ] T095 [P] Add cold start performance benchmarking in tests/perf/cold-start.ts — measure app launch to first-render time on desktop (<2s threshold per FR-023) and mobile (<3s threshold per FR-023/SC-008); add npm script `perf:cold-start`; fail CI if thresholds exceeded; document measurement methodology in test file comments
- [ ] T096 [P] Add WCAG contrast ratio validation script in scripts/validate-contrast.ts — parse generated tokens.css, extract all foreground/background color pairs from light, dark, and high-contrast token sets, compute contrast ratios, fail if any pair is below 4.5:1 for normal text or 3:1 for large text per FR-052 and SC-016; integrate into `npm run build:tokens` as post-build check; output failing pairs with actual vs required ratios
- [ ] T097 [P] Add end-to-end keyboard navigation validation in tests/e2e/keyboard-nav.spec.ts — Playwright test that verifies all primary actions (add zone, remove zone, reorder zones, create alarm, manage alarm, open settings, quit) are completable via keyboard-only (Tab, Shift+Tab, Enter, Escape, Arrow keys) with visible focus indicators on all interactive elements per FR-027 and SC-017
- [ ] T098 Run quickstart.md validation — follow quickstart.md setup steps end-to-end on a clean checkout, verify `cargo tauri dev` launches successfully, verify all 6 user stories function correctly, verify CI pipeline passes
- [ ] T099 [P] Add time accuracy validation in tests/unit/tick-accuracy.test.ts — unit test that verifies displayed times are accurate to within ±1 second of the device system clock across timezone conversions and DST transitions; test boundary cases: DST spring-forward, DST fall-back, UTC offset change, leap second handling per SC-003
- [ ] T100 [P] Add alarm timing validation in tests/integration/alarm-timing.test.ts — integration test that schedules alarms at known future times and verifies they fire within ±5 seconds on desktop; document mobile ±60s constraint per SC-004/SC-022; test edge cases: alarm during DST transition, alarm at midnight boundary, multiple simultaneous alarms
- [ ] T101 [P] Add memory profiling benchmark in tests/perf/memory-profile.ts — measure RSS memory usage with 10 timezone cards displayed on desktop (threshold <50MB per SC-005); add npm script `perf:memory`; log per-component memory breakdown; fail CI if threshold exceeded
- [ ] T102 [P] Add store migration benchmark in tests/perf/migration-benchmark.ts — seed stores with maximum allowed entries (30 zones, 50 alarms) and measure schema migration duration (threshold <500ms per SC-013); test migration from each prior schema version; fail if threshold exceeded
- [ ] T103 [P] Add export/import round-trip validation in tests/integration/export-import-roundtrip.test.ts — export all data (zones, alarms, preferences) to JSON, import on a fresh store, verify byte-level equivalence of all entities; test cross-version import (older export → newer app) per SC-015
- [ ] T104 [P] Add compact mode constraint validation in tests/e2e/compact-mode.spec.ts — Playwright test that enables compact mode and verifies: window width ≤250px, each timezone renders on a single line, memory usage <15MB; test with 1, 5, and 10 timezones per SC-018
- [ ] T105 [P] Add timezone search performance benchmark in tests/perf/search-performance.ts — measure search latency for the bundled ~400 timezone dataset (threshold <50ms per SC-019); test single-character typo tolerance; test UTC offset queries (e.g., "UTC+5" resolves to matching zones); fail CI if threshold exceeded

**Checkpoint**: All features implemented, infrastructure services deployed, CI/CD operational, privacy policy published, performance benchmarks passing, WCAG contrast validated, keyboard nav verified, all buildable success criteria validated. Application ready for v1 release.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion — **BLOCKS all user stories**
- **User Stories (Phases 3–8)**: All depend on Foundational phase completion
  - US1 and US2 (both P1) should be completed first for MVP
  - US3–US6 can proceed in priority order or in parallel
- **Polish (Phase 9)**: Can start after US1+US2 for cross-cutting features; CI/CD can be developed in parallel with any phase

### User Story Dependencies

- **US1 (P1)**: Can start after Foundational — no dependencies on other stories
- **US2 (P1)**: Can start after Foundational — extends clock-list.ts from US1 (DnD, search modal) but can be developed concurrently with US1 if care is taken on shared files
- **US3 (P2)**: Can start after Foundational — independent Rust commands, but frontend wiring uses clocks state from US1
- **US4 (P2)**: Can start after Foundational — fully independent data model and UI (separate Alarms tab)
- **US5 (P3)**: Can start after Foundational — independent settings panel, but preferences affect rendering in US1/US4
- **US6 (P3)**: Can start after Foundational — extends US1/US2 components with touch support, adds mobile-specific Rust commands

### Within Each User Story

- State modules before services
- Services before components
- Components before integration wiring
- Core functionality before edge case handling
- Story-specific wiring in main.ts is always the last task

### Parallel Opportunities

- **Phase 2 Tokens**: T009, T010, T011, T012 — all different directories, fully parallel
- **Phase 2 Types**: T014, T015, T016 — all different files, fully parallel
- **Phase 2 Services**: T019, T020, T021, T022, T023, T024 — all different files, fully parallel
- **Phase 2 Rust**: T027, T028, T029, T030, T031 — all different files, fully parallel
- **US1 State**: T034, T035, T036, T037 — all different files, fully parallel
- **US3 Rust Commands**: T049, T050, T051, T052, T053, T054, T055 — all different command files, fully parallel
- **US4 Core**: T059, T060, T061, T062 — state + scheduler + Rust commands, fully parallel
- **Phase 9**: T080–T090, T093–T105 — all independent features, fully parallel

---

## Implementation Strategy

### MVP First (User Stories 1 + 2)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (**CRITICAL** — blocks all stories)
3. Complete Phase 3: User Story 1 — View clocks
4. Complete Phase 4: User Story 2 — Add/remove/reorder
5. **STOP and VALIDATE**: Test end-to-end — add zones, remove zones, reorder, restart, verify persistence
6. Deploy/demo if ready — this is the MVP

### Incremental Delivery

1. Setup + Foundational → Foundation ready
2. US1 + US2 → Core clock app (**MVP!**)
3. US3 → System tray, compact mode (desktop upgrade)
4. US4 → Alarms + notifications (major feature addition)
5. US5 → Settings + localization (customization)
6. US6 → Mobile optimization (platform parity)
7. Polish → Infrastructure, CI/CD, telemetry, deep links

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: US1 → US2 (P1 stories, MVP path)
   - Developer B: US4 state + scheduler + Rust alarm commands (can start backend while A does UI)
   - Developer C: US3 Rust tray/window commands (backend work, no frontend overlap)
3. After MVP:
   - Developer A: US5 + US6 (preferences + mobile)
   - Developer B: US4 UI components (alarm form, alarm list)
   - Developer C: Phase 9 Polish (CI/CD, telemetry, deep links, export/import)

---

## Summary

| Phase | Tasks | Parallel | Scope |
|-------|-------|----------|-------|
| 1. Setup | 8 | 5 | Project scaffolding |
| 2. Foundational | 25 | 19 | Core infrastructure |
| 3. US1 (P1) 🎯 | 9 | 4 | View clocks |
| 4. US2 (P1) 🎯 | 6 | 1 | Add/remove/reorder |
| 5. US3 (P2) | 10 | 7 | System tray |
| 6. US4 (P2) | 10 | 4 | Alarms & notifications |
| 7. US5 (P3) | 6 | 2 | Settings & preferences |
| 8. US6 (P3) | 5 | 1 | Mobile optimization |
| 9. Polish | 26 | 23 | Cross-cutting & infra |
| **Total** | **105** | **71** | |

- **MVP scope**: Phases 1–4 (48 tasks) — Setup + Foundation + US1 + US2
- **Full scope**: All 105 tasks across 9 phases
- **Parallel ratio**: 71/105 tasks (67.6%) can be parallelized within their phase

---

## Notes

- [P] tasks = different files, no dependencies on other tasks in the same parallel batch
- [US#] label maps task to specific user story for traceability
- Each user story is independently completable and testable
- Commit after each task or logical group
- Stop at any checkpoint to validate the story independently
- All file paths are relative to repository root
- Test tasks are omitted per template rules — add tests following research.md R9 strategy and quickstart.md file layout
