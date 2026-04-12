# Research: Cross-Platform World Clock (Delta-T Zaman)

**Branch**: `001-tauri-world-clock` | **Date**: 2026-04-12 | **Spec**: [spec.md](./spec.md)

## R1: Frontend Framework Selection

**Decision**: Vanilla TypeScript (no framework) + Vite

**Rationale**:
- **Zero framework overhead**: 0 KB runtime — no framework code shipped at all. Direct DOM manipulation via `document.createElement`, `querySelector`, `textContent`.
- **Type safety**: TypeScript provides compile-time type checking for IPC payloads, state shapes, and component interfaces — catches bugs before runtime across 5 platforms.
- **Maximum control**: Every DOM update is explicit and surgical. For a 1/sec clock tick, `element.textContent = newTime` is the absolute minimum work — no diffing, no reactivity system, no reconciliation.
- **No abstraction leaks**: No framework-specific gotchas with Tauri's WebView lifecycle, IPC bridge, or platform differences. The WebView is just a browser — vanilla TS compiles to standard JS that works identically everywhere.
- **Tauri ecosystem**: Official `vanilla-ts` template in `npm create tauri-app`. Simplest typed starting point.
- **Long-term stability**: No framework version migrations. TypeScript + DOM APIs are the platform — backward compatibility is guaranteed.
- **Minimal build tooling**: Vite handles TS compilation, bundling, and HMR. No framework-specific plugins, preprocessors, or compilers needed.

**Architecture pattern**: Component-as-module. Each UI component is a TS module exporting a `create(container: HTMLElement)` function that returns a typed controller object with `update()` and `destroy()` methods. State managed via a simple typed pub/sub event bus.

**Alternatives considered**:
- **Svelte 5**: Excellent compiled output (15 KB) and elegant reactivity, but adds build complexity (Svelte compiler, preprocessor config) and creates framework lock-in for a small utility app.
- **React**: 42 KB runtime, virtual DOM overhead unnecessary for simple text updates.
- **SolidJS**: 8 KB but niche ecosystem, no official Tauri template.
- **Vue**: 34 KB runtime, no significant advantage for this use case.
- **Vanilla JS (untyped)**: Same architecture but loses type safety for IPC contracts and state — too risky across 5 platforms.

## R2: Design Token & Styling Approach

**Decision**: Style Dictionary + generated CSS custom properties

**Rationale**:
- **Single source of truth**: Design tokens defined once in JSON/JSONC (`tokens/`), transformed into platform-specific outputs — CSS custom properties for the WebView, and potentially Swift/Kotlin constants for native layers.
- **Cross-platform consistency**: Style Dictionary is built by Amazon for exactly this problem — maintaining visual consistency across heterogeneous platforms. Tokens define colors, spacing, typography, shadows, and timing values that compile to `--dt-color-bg`, `--dt-spacing-md`, etc.
- **Theming via token sets**: Light/dark themes are separate token files that Style Dictionary merges at build time. Runtime theme switching flips a `[data-theme]` attribute on `<html>` which activates the corresponding CSS custom property block.
- **Zero runtime cost**: Style Dictionary runs at build time only. Output is a plain `.css` file with custom properties — no runtime JS, no CSS-in-JS, no PostCSS.
- **BEM for selectors**: Component styles still use BEM naming (`clock-card`, `clock-card__time`, `clock-card--night`), but all values reference tokens (`var(--dt-color-surface)`) instead of hardcoded hex/px values.
- **Modern CSS features**: `@container` queries, `gap`, `grid`/`flexbox`, `:focus-visible` — all supported in target WebViews, all referencing token values.
- **Screen reader utilities**: `.sr-only` utility class defined in base CSS.

**Token structure**:
```
tokens/
├── base/
│   ├── color.json        # Primitive palette (gray-50..900, accent, danger, etc.)
│   ├── spacing.json      # 4px scale (xs, sm, md, lg, xl, 2xl)
│   ├── typography.json   # Font families, sizes, weights, line-heights
│   ├── shadow.json       # Elevation levels (sm, md, lg)
│   └── timing.json       # Animation durations (fast, normal, slow)
├── semantic/
│   ├── light.json        # Light theme mappings (surface, on-surface, etc.)
│   └── dark.json         # Dark theme mappings
└── component/
    ├── clock-card.json   # Component-specific tokens
    └── alarm-card.json
```

**Build output**: `src/styles/tokens.css` — generated CSS custom properties imported by `styles.css`.

**Alternatives considered**:
- **Plain CSS custom properties (hand-written)**: Works but becomes unmaintainable at 50+ tokens. No single source of truth, no build-time validation, no multi-platform output.
- **Tailwind CSS 4.x**: Powerful utility framework but adds build tooling (PostCSS, JIT), learning curve, and 10 KB overhead. Overkill for a 15-component utility app.
- **CSS Modules**: Scoped by default but generates hashed class names that complicate debugging.
- **Open Props**: Pre-built tokens but no customization pipeline and adds a dependency.

## R3: Timezone Data Strategy

**Decision**: Bundle static JSON metadata (~250 KB gzipped) + `Intl.DateTimeFormat` for rendering

**Rationale**:
- `Intl.supportedValuesOf('timeZone')` works on all target WebViews (Safari 15.4+, Chrome 99+, WebKitGTK 615+) but returns only IANA IDs — no city labels, countries, or coordinates
- Bundled JSON provides: IANA ID, canonical city name, country, UTC offset hint, latitude/longitude (for day/night), and alias mappings
- ~400 canonical IANA zones + ~195 aliases cover all major world cities
- 250 KB is negligible (<0.5% of total app size)
- Enables fully offline search — no network dependency

**Key finding**: Chrome-based engines (WebView2, Android WebView) omit `'UTC'` from `supportedValuesOf('timeZone')`. Must always add `'UTC'` manually as a fallback.

**DST handling**: `Intl.DateTimeFormat` handles DST transitions automatically and correctly on all target WebViews. Tested with US spring-forward, EU fall-back, and Australia spring-forward transitions. No known bugs in target browser versions.

**Alias deduplication**: Build a static lookup map from alias → canonical ID (e.g., `US/Eastern` → `America/New_York`). Detect duplicates at add-time by resolving aliases before comparison.

**Alternatives considered**:
- **Runtime-only (no bundle)**: Saves 250 KB but loses city names, coordinates, and offline search capability. Unacceptable for FR-014 (offline-first).
- **Smaller bundle (IDs only, ~30 KB)**: Loses city labels and coordinates. Users search by city name (FR-004), so labels are essential.

## R4: Timezone Search Implementation

**Decision**: Hybrid substring + fuzzy matching with UTC offset parsing

**Rationale**:
- 400 zones is a small dataset — substring search completes in 1-3ms
- Matches on city name, country, and IANA ID simultaneously
- Results sorted by: exact prefix match > fuzzy prefix > word-start match > substring match > offset match
- Fuzzy matching uses Levenshtein distance ≤ 1 for single-character typo tolerance (e.g., "Tokio" → "Tokyo")
- UTC offset parsing detects patterns like "+5:30", "GMT-5", "UTC+9" and matches against `utc_offset_minutes` in bundled data
- No external fuzzy library needed — Levenshtein distance ≤ 1 can be computed with a simple two-row matrix in ~50 LOC

**Alternatives considered**:
- **Fuse.js** (12 KB): Full fuzzy matching with configurable thresholds. More powerful but overkill — adds 12 KB to bundle and distance ≤ 1 is sufficient for timezone names.
- **Custom trie**: Fast prefix matching but substring-only, misses mid-word matches. Doesn't support fuzzy or offset queries.

## R5: Day/Night Indicator Calculation

**Decision**: SunCalc library (~3 KB) with bundled latitude/longitude coordinates

**Rationale**:
- Uses NREL Solar Position Algorithm — 95%+ accuracy
- Calculates actual sunrise/sunset times for each timezone's reference city
- Handles polar regions correctly (midnight sun / polar night)
- Only ~3 KB gzipped, coordinates already included in timezone bundle (R3)
- Returns continuous solar altitude — can render gradient day/night indicator, not just binary

**Alternatives considered**:
- **Simple 6 AM–6 PM rule**: Zero dependency but inaccurate at high latitudes and during summer/winter solstices. Confusing UX when indicator says "night" but it's daylight in Reykjavik.
- **Server-side API**: Violates offline-first requirement (FR-014).

## R6: Alarm Scheduling Architecture

**Decision**: Hybrid approach — `@tauri-apps/plugin-notification` (v2.3.3) for alarm notifications + Rust async tasks for desktop tray updates

**Rationale**:
- The official notification plugin has **built-in scheduling** (`Schedule.at()`, `Schedule.interval()`, `Schedule.every()`) — no need for community schedule-task plugin
- Official plugin = maintained by Tauri team, tested on all 5 platforms, documented, stable API
- Supports notification action buttons ("Snooze", "Dismiss"), custom sounds, vibration (Android)
- Permission flow built-in: `isPermissionGranted()` → `requestPermission()` → `sendNotification()`
- Rust `tokio::spawn` for background tray tooltip updates on desktop (not throttled like JS timers)

**Notification scheduling API**:
```typescript
import { sendNotification, Schedule } from '@tauri-apps/plugin-notification';
// One-time alarm
sendNotification({
  title: 'Alarm: Morning standup',
  body: '09:00 AM in Asia/Tokyo',
  schedule: Schedule.at(new Date('2026-04-13T00:00:00Z'))
});
// Recurring daily
sendNotification({
  title: 'Daily alarm',
  schedule: Schedule.interval({ hour: 9, minute: 0 }) // daily at 9:00
});
```

**Alternatives considered**:
- **tauri-plugin-schedule-task** (community, v0.1.0): Runs arbitrary Rust code on schedule. More powerful but: (a) iOS support untested by maintainer, (b) no npm package (GitHub-only install), (c) API is Rust-only — JS can't define task logic. Unnecessary since we only need notifications, not arbitrary code execution.
- **tokio-cron-scheduler** (Rust): Full cron expressions but desktop-only. Mobile would need separate implementation.
- **OS-level schedulers** (launchd, systemd, Task Scheduler): Maximum reliability but requires 3 platform-specific implementations and elevated privileges.

## R7: Persistence Strategy

**Decision**: `@tauri-apps/plugin-store` (JSON key-value store)

**Rationale**:
- Official Tauri plugin, supported on all 5 platforms
- Simple API: `store.set(key, value)` / `store.get(key)` / `store.save()`
- Persists to a platform-appropriate location automatically
- Three stores: `zones.dat` (timezone entries), `alarms.dat` (alarm list), `preferences.dat` (settings)
- JSON serialization — human-readable, debuggable

**Alternatives considered**:
- **SQLite via plugin-sql**: Relational queries unnecessary for key-value settings data. Adds complexity.
- **Raw filesystem**: More code for serialization, path resolution, and error handling.

## R8: Drag-and-Drop Implementation

**Decision**: Custom vanilla drag-and-drop with HTML5 Drag API + keyboard fallback

**Rationale**:
- **Zero dependencies**: No DnD library needed for reordering a simple list of 1–30 items.
- **HTML5 Drag API**: Native `draggable="true"`, `dragstart`, `dragover`, `drop` events. Works on all desktop WebViews. Touch fallback via `touchstart`/`touchmove`/`touchend` for mobile.
- **Keyboard accessibility**: Custom implementation — focus item, press Space to grab, arrow keys to move position, Space/Enter to drop, Escape to cancel. ARIA live region announces position changes.
- **Implementation size**: ~150 lines of JS for the full drag controller (pointer + keyboard + touch + ARIA).
- **Animations**: CSS `transition: transform 0.2s` on reordered items for smooth visual feedback.

**Alternatives considered**:
- **SortableJS** (8 KB): Mature library with touch support, but adds a dependency for a simple list reorder. Also has accessibility gaps.
- **svelte-dnd-action**: Svelte-specific, not applicable to vanilla TS.
- **@dnd-kit/core**: React-specific, not applicable to vanilla TS.

## R9: Testing Strategy

**Decision**: Vitest (unit/integration) + Playwright (E2E) + cargo test (Rust)

**Rationale**:
- **Vitest**: Native Vite integration, fast HMR-aware test runner. Tests pure TS modules directly — no framework adapters needed.
- **Playwright**: Cross-browser E2E testing, supports WebView testing via Tauri's WebDriver integration
- **cargo test**: Rust command handler tests with Tauri mock runtime

**Test layers**:
1. **Unit** (Vitest): Timezone search, day/night calculation, alarm state transitions, formatting logic, DOM component render functions
2. **DOM** (Vitest + jsdom/happy-dom): Component create/update/destroy lifecycle, event handler wiring, ARIA attribute correctness
3. **Integration** (Vitest): Store plugin interaction, IPC command round-trips
4. **E2E** (Playwright): Full user flows — add timezone, create alarm, tray interaction

## R10: Tauri Plugin Matrix

| Plugin | Version | Purpose | Platforms |
|--------|---------|---------|-----------|
| `@tauri-apps/plugin-notification` | 2.3.3 | Alarm notifications + scheduling | All 5 |
| `@tauri-apps/plugin-store` | 2.x | Preference persistence | All 5 |
| `@tauri-apps/plugin-autostart` | 2.x | Launch at login | Desktop only |
| `@tauri-apps/plugin-single-instance` | 2.x | Prevent duplicate instances | Desktop only |
| `@tauri-apps/plugin-positioner` | 2.x | Position tray dropdown | Desktop only |
| `@tauri-apps/plugin-updater` | 2.x | Auto-update via Cloudflare R2 | Desktop only |
| `@tauri-apps/plugin-os` | 2.x | Detect platform, locale | All 5 |
| `@tauri-apps/plugin-dialog` | 2.x | File save/open dialogs for export/import | All 5 |
| `@tauri-apps/plugin-deep-link` | 2.x | Custom `deltat://` URL scheme registration | All 5 |

## R11: Cross-Platform Considerations

| Concern | macOS | Windows | Linux | iOS | Android |
|---------|-------|---------|-------|-----|---------|
| WebView engine | WKWebView | WebView2 (Edge) | WebKitGTK | WKWebView | Chrome WebView |
| System tray | Menu bar icon ✓ | System tray ✓ | AppIndicator (DE-dependent) | N/A | N/A |
| Notifications | UNUserNotification | Windows Toast | D-Bus | UNUserNotification | NotificationCompat |
| Autostart | Login Items | Registry/Startup | XDG autostart | N/A | N/A |
| Background | Unrestricted | Unrestricted | Unrestricted | BackgroundTasks (limited) | Doze mode constraints |
| Code signing | Apple Developer ID | OV/EV cert (optional) | N/A | Apple Developer (required) | Keystore (required) |
| Close behavior | Hide to menu bar (FR-057) | Tray or quit (FR-057) | Tray or quit (FR-057) | Suspend | Suspend |
| Accessibility | System high contrast, VoiceOver | Windows High Contrast, Narrator | GTK high contrast, Orca | VoiceOver, Dynamic Type | TalkBack, Font scale |
| Clock update rate | 1/sec | 1/sec | 1/sec | 1/min | 1/min |

## Production Bundle Size Estimate

```
Framework runtime:          0 KB  (vanilla TS — no framework)
Style Dictionary output:    2 KB  gzipped (generated CSS custom properties)
App CSS:                    4 KB  gzipped (component styles referencing tokens)
DnD:                        0 KB  (custom, included in app code)
SunCalc:                    3 KB  gzipped
Timezone JSON:            250 KB  gzipped
App code (TS→JS):          12 KB  gzipped (estimate — no framework overhead)
─────────────────────────────────
Frontend total:           ~271 KB gzipped

Tauri binary (Rust):     ~5-10 MB (platform dependent, includes updater plugin)
WebView:                    0 MB  (system-provided)
─────────────────────────────────
Total app size:          ~6-12 MB (vs Electron ~150 MB)
```

## R12: Telemetry Architecture

**Decision**: Cloudflare Workers + Analytics Engine for anonymous telemetry ingestion

**Rationale**:
- **Serverless scale-to-zero**: No minimum cost — Workers only execute when telemetry events arrive. Analytics Engine provides built-in aggregation with SQL query support and 90-day retention.
- **Fire-and-forget**: The Rust backend sends an HTTP POST with a 5-second timeout. On failure, the event is silently dropped — no retry queue, no local buffering. This guarantees telemetry never degrades app performance (SC-011: ≤50ms impact).
- **Privacy-first**: No PII collected. Events contain only: event type, platform, app version, and anonymous feature counts (zones added, alarms created). No user IDs, no device fingerprinting, no IP retention.
- **Opt-in model**: Telemetry is disabled by default. First-launch prompt asks user to enable. Preference stored in `preferences.dat` (`telemetry_opt_in`) and toggleable from settings at any time (FR-032/036).
- **Minimal infrastructure**: One Cloudflare Worker (~50 lines) validates the event schema and writes to Analytics Engine. No database, no auth, no API keys needed from the client.

**Signal types collected**:
- `app_open` — app launch count (anonymous adoption metric)
- `zone_added` / `zone_removed` — feature usage
- `alarm_created` / `alarm_fired` — alarm feature engagement
- `crash_report` — unhandled errors with stack trace (no user context)

**Alternatives considered**:
- **Sentry/Datadog**: Full-featured but heavyweight dependency, SDK adds bundle size, requires API keys embedded in the app binary. Overkill for a utility app.
- **PostHog**: Good privacy story but self-hosted adds infrastructure complexity. Cloud version has per-event pricing that's unpredictable.
- **No telemetry**: Defers all observability — can't diagnose crashes or measure feature adoption. Acceptable for hobby projects but not for a multi-platform app with auto-update.

## R13: Auto-Update Strategy

**Decision**: `tauri-plugin-updater` with Cloudflare R2 (storage) + Workers (manifest serving)

**Rationale**:
- **Tauri-native**: `tauri-plugin-updater` handles platform-specific update mechanics (download, verify signature, swap binary, prompt restart). No custom update logic needed.
- **Cloudflare R2**: S3-compatible object storage with zero egress fees. Stores update binaries (one per platform, ~6-12 MB each). Total cost negligible for a utility app's update traffic.
- **Cloudflare Worker as manifest proxy**: A ~30-line Worker serves the update manifest JSON from R2, allowing version logic (e.g., staged rollouts, platform filtering) without a traditional server.
- **Signature verification**: Tauri updater validates Ed25519 signatures on downloaded binaries. Private key stays in CI; public key embedded in `tauri.conf.json`.
- **Non-blocking**: Update check runs on launch in a background task. If the check fails (network down, Cloudflare outage), the app continues normally. User can also trigger a manual check from settings.
- **Schema migration delivery**: Auto-update is the mechanism that delivers new schema versions to users. Combined with forward-only local migration (FR-035), this ensures data format upgrades happen seamlessly.

**Update flow**:
1. App launches → background `check_for_update` call to Worker endpoint
2. Worker reads manifest from R2, returns latest version info
3. If newer version available → emit `update-available` event to frontend
4. Frontend shows non-intrusive banner: "Update v1.2.0 available"
5. User clicks update → `install_update` downloads binary from R2, verifies signature
6. Tauri applies update → prompts restart

**Mobile note**: Mobile platforms (iOS/Android) use their respective app store update mechanisms. `tauri-plugin-updater` and the Cloudflare infrastructure apply to desktop only (macOS, Windows, Linux).

**Alternatives considered**:
- **GitHub Releases**: Free and well-integrated, but requires a GitHub API call per update check and rate-limits anonymous requests (60/hr). Cloudflare R2 has no rate limits and zero egress cost.
- **S3 + CloudFront**: Works but egress costs are non-trivial for binary downloads at scale. R2's zero-egress model is strictly better.
- **Custom update server**: Maximum flexibility but unnecessary infrastructure. A 30-line Worker achieves the same result with zero maintenance.

## R14: CI/CD & Release Pipeline

**Decision**: GitHub Actions with platform matrix + Cloudflare R2 upload

**Rationale**:
- **GitHub Actions**: Free for public repos, generous minutes for private. Native macOS, Windows, and Linux runners available. `tauri-apps/tauri-action` handles the full build → sign → bundle workflow.
- **Matrix strategy**: A single `release.yml` workflow builds all 5 platform targets in parallel: `macos-latest` (DMG + universal binary), `windows-latest` (MSI/NSIS), `ubuntu-latest` (AppImage + .deb), `macos-latest` (iOS), `ubuntu-latest` (Android AAB).
- **Cloudflare R2 upload**: After successful signing, desktop artifacts are uploaded to R2 via `wrangler r2 object put`. The update manifest JSON is regenerated and uploaded alongside, pointing to the new binaries.
- **Mobile pipeline**: iOS builds are submitted to App Store Connect via `xcrun altool`; Android AABs are uploaded to Google Play via the Play Developer API. Both require store-specific credentials as GitHub secrets.
- **Quality gates**: CI runs on every PR (`ci.yml`): lint (clippy + eslint), type-check (tsc), unit tests (vitest + cargo test), build verification. Release only triggers on tagged commits (`v*`).

**Workflow structure**:
```
.github/workflows/
├── ci.yml          # PR checks: lint, type-check, test, build verify
└── release.yml     # Tagged releases: build, sign, upload, publish
```

**Alternatives considered**:
- **GitLab CI**: Equivalent capability but GitHub Actions has better Tauri ecosystem support (official `tauri-action`).
- **CircleCI**: macOS runners are expensive. GitHub Actions includes macOS runners in the free tier.
- **Manual releases**: Error-prone, doesn't scale to 5 platforms, no reproducibility guarantee.

## R15: Code Signing Strategy

**Decision**: Platform-native signing with keys managed as CI secrets

**Rationale**:
- **macOS**: Apple Developer ID certificate + `codesign` + `notarytool` for GateKeeper notarization. Certificate (.p12) stored as a base64-encoded GitHub secret. `tauri-action` handles the signing and notarization steps automatically.
- **Windows**: Authenticode certificate (OV or EV). OV certificate stored as a GitHub secret; EV requires USB token and is deferred to post-v1. `signtool.exe` called by `tauri-action`.
- **Linux**: No code signing required. AppImage and .deb packages are unsigned (standard practice for Linux desktop apps).
- **iOS**: Apple Developer Program certificate + provisioning profile. Managed in GitHub secrets, applied by `xcodebuild`.
- **Android**: Keystore file for signing AAB. Stored as a base64-encoded GitHub secret.
- **Ed25519 (updater)**: `tauri-plugin-updater` uses Ed25519 signatures to verify update integrity. Key pair generated once via `tauri signer generate`. Private key stored as CI secret (`TAURI_SIGNING_PRIVATE_KEY`); public key embedded in `tauri.conf.json`.

**Key rotation**: For platform-native certificates, follow each platform's standard renewal/rotation process. For Ed25519 updater keys:
1. **Rotation trigger**: Key compromise, personnel change, or annual rotation policy.
2. **Transitional update**: The current (old) key signs a special update that includes the new public key in `tauri.conf.json`. Users who install this update will trust the new key for subsequent updates.
3. **Rollout**: Ship the transitional update, wait for ≥90% adoption (measured via update telemetry), then rotate the CI secret to the new private key.
4. **Emergency rotation**: If the private key is compromised, immediately rotate the CI secret, ship a transitional update, and disable the old key's update endpoint.
5. **CI secret management**: Private keys stored as GitHub repository secrets (not environment secrets). Access limited to the release workflow. Secret scanning enabled in the repository to prevent accidental commits.
6. **Certificate monitoring**: Calendar reminders set 30 days before Apple/Windows certificate expiry. The CI/CD pipeline (R14) validates certificate expiry as a pre-build step.

**Alternatives considered**:
- **Azure SignTool**: Better for EV certificates but adds Azure dependency.
- **Apple Transporter**: Alternative to `notarytool` but less CI-friendly.

## R16: Data Export/Import Architecture

**Decision**: JSON file with version envelope, merge/replace modes

**Rationale**:
- **JSON format**: Consistent with the app's existing store format. Human-readable, debuggable, no additional serialization dependency.
- **Version envelope**: `export_version` integer ensures forward compatibility. If the export was created by a newer app version, import is rejected with a clear upgrade prompt.
- **Schema migration on import**: Imported data may be from an older schema version. The same forward-only migration pipeline (FR-035) runs on imported data before merging/replacing.
- **Merge vs. replace**: Merge adds missing zones/alarms (by UUID match) without overwriting existing data — safe for partial restores. Replace overwrites all stores — clean slate from a backup.
- **Window state excluded**: `window_x/y/width/height/maximized` are device-specific and excluded from export. Prevents a backup from a 4K monitor creating a tiny window on a laptop.
- **File dialog**: Uses the OS native file save/open dialog for path selection. No predefined export location.

**Alternatives considered**:
- **Cloud sync**: Requires server infrastructure, auth, conflict resolution. Violates offline-first and simplicity principles. Deferred to post-v1.
- **Binary format (MessagePack/CBOR)**: Smaller files but not human-readable. JSON is fine for the data volumes involved (< 100 KB typically).

## R17: Timezone Data Freshness

**Decision**: Lightweight JSON patches served alongside update manifest from Cloudflare Worker

**Rationale**:
- IANA timezone database updates 3-4 times per year. DST rule changes (e.g., a country abolishing DST) can make bundled data stale between app releases.
- Full app update is overkill for a 10-50 KB timezone data change. A lightweight patch mechanism allows timezone data to update independently.
- Patches are served from the same Cloudflare Worker that hosts the update manifest — zero additional infrastructure.
- Patches are cumulative (each patch contains all changes since the bundled `base_version`) — the client never needs to apply multiple patches sequentially.
- Patches are applied in-memory at startup — the bundled `timezones.json` is never modified on disk (ensures app integrity and allows rollback).
- If the patch fetch fails (offline, Cloudflare outage), the app uses bundled data. Stale data is acceptable since DST rule changes are announced months in advance.

**Alternatives considered**:
- **Full timezone JSON replacement**: Simpler but wasteful — downloads ~250 KB instead of a 10-50 KB diff.
- **npm package (moment-timezone data)**: External dependency, heavier than needed, licensing concerns.
- **OS timezone data**: Not available cross-platform via Tauri APIs. Platform behavior varies.

## R18: Accessibility Architecture

**Decision**: Three-tier token system (light/dark/high-contrast) with runtime media query listeners

**Rationale**:
- **High contrast token set**: A third complete set of design tokens alongside light and dark. Not derived from either — independently designed with solid borders, no gradients, minimum 7:1 contrast ratios. Loaded via a `data-theme="high-contrast"` attribute on `<html>`.
- **`prefers-reduced-motion` listener**: A `matchMedia('(prefers-reduced-motion: reduce)')` listener disables all CSS `transition` and `animation` properties globally by toggling a `data-reduced-motion` attribute. Components check this before starting animations.
- **`prefers-contrast` listener**: Detects OS high contrast setting. Maps to the high contrast token set. Manual override in Settings > Display takes priority.
- **Focus trap utility**: A shared `trapFocus(containerEl)` utility that cycles Tab focus within a container. Used by all modals and dialogs. Returns focus to the trigger element on close.
- **Font scaling**: All text sizes use `rem` units referencing a root `font-size` that respects the OS text size setting. Layout uses CSS grid/flex that adapts to content overflow.
- **Build-time contrast validation**: A CI step (npm script) parses the generated token CSS and validates every foreground/background pair against WCAG AA thresholds (4.5:1 normal, 3:1 large). Build fails on violation.

**Alternatives considered**:
- **CSS-only approach (no JS listeners)**: `@media (prefers-reduced-motion)` works for CSS but JS animations (e.g., drag-and-drop) still need runtime detection.
- **Third-party a11y library**: Overkill for the scope. A focused `trapFocus()` utility plus media query listeners is ~100 lines of code.

## R19: Platform-Specific Behavior Matrix

**Decision**: Runtime platform detection with conditional code paths

**Rationale**:
- `tauri-plugin-os` provides platform detection at startup. Platform-specific modules are conditionally imported.
- **macOS**: Red traffic-light button triggers `hide_to_tray` instead of quit (standard for utility apps). `Cmd+Q` quits. Native menu bar with Edit menu for text field support. `NSApp.setActivationPolicy(.accessory)` for menu bar behavior.
- **Windows**: Close button behavior depends on `tray_enabled` preference. If tray enabled, close minimizes to tray. If not, close quits. Taskbar icon always visible.
- **Linux**: `XDG_SESSION_TYPE` check at startup to detect Wayland vs X11. Tray support depends on desktop environment (AppIndicator extension). If tray unavailable, close quits and a warning is logged.
- **iOS**: Background App Refresh registered for alarm rescheduling. App goes through `active → inactive → background → suspended` lifecycle. Alarms scheduled via local notifications persist through suspension.
- **Android**: Foreground service with persistent notification for active alarms (prevents Doze from killing scheduling). `REQUEST_IGNORE_BATTERY_OPTIMIZATIONS` requested on first alarm creation (NFR-001).

**Alternatives considered**:
- **Compile-time platform gating**: Rust `#[cfg(target_os)]` works for backend but frontend needs runtime detection since it's a single WebView bundle.
- **Ignoring platform conventions**: Leads to bad UX (e.g., macOS users expect close-to-tray for utility apps).

---

## R20: Deep Linking Architecture

**Decision**: Use `tauri-plugin-deep-link` for `deltat://` custom URL scheme registration

**Rationale**:
- Tauri 2 provides `tauri-plugin-deep-link` for cross-platform URL scheme registration.
- **macOS/iOS**: URL scheme registered in Info.plist. macOS uses `NSAppleEventManager`; iOS uses `UIApplication.open(_:options:completionHandler:)`.
- **Windows**: URL scheme registered in Windows Registry (`HKCU\Software\Classes\deltat`). Tauri handles this during install via WiX/NSIS.
- **Android**: Intent filter registered in AndroidManifest.xml (`<data android:scheme="deltat" />`).
- **Linux**: `.desktop` file includes `MimeType=x-scheme-handler/deltat`. Registration via `xdg-mime`.
- URL parsing and validation happen in the Rust backend (`handle_deep_link` command). The frontend receives pre-validated structured data.
- Security: Only `add` and `alarm` actions are supported. No arbitrary code execution, no data export via URL. All parameters are sanitized.

**Supported URL formats**:
- `deltat://add?tz=Asia/Tokyo&label=Tokyo+Office` — add timezone
- `deltat://alarm?time=09:00&tz=America/New_York&label=Standup` — pre-fill alarm creation form

**Alternatives considered**:
- **Universal links (HTTPS-based)**: Requires a web domain and server-side association file. Overkill for a local-first app. Also complicates the offline-first promise (FR-014).
- **No deep linking**: Functional but misses opportunity for sharing timezone setups and integration with other apps.
- **Custom protocol handler without plugin**: Possible via raw platform APIs but duplicates work the plugin already handles.

---

## R21: Error Resilience & Recovery Architecture

**Decision**: Layered error handling — global boundary + domain-specific recovery + heartbeat monitoring

**Rationale**:
- **Layer 1 — Global error boundary (NFR-002)**: Catches unhandled exceptions. Logs, reports (if telemetry on), shows user-friendly message. Prevents white-screen-of-death.
- **Layer 2 — Domain-specific recovery**:
  - SunCalc: Wraps every `SunCalc.getTimes()` call in try-catch. Checks return values for NaN. Polar latitude detection at >66.5° triggers special "24h day/night" display path.
  - Plugin degradation (NFR-005): Each plugin init is wrapped individually. Failure triggers feature flag disabling rather than app crash.
  - Network requests: All Cloudflare calls (telemetry, updates, tz patches) use 10-second timeout. Failures are silently swallowed (fire-and-forget for telemetry, cached for updates).
- **Layer 3 — WebView heartbeat (new)**:
  - Rust backend emits `webview-heartbeat` event every 10 seconds.
  - Frontend responds with `heartbeat_ack` command.
  - If backend receives no ack for 30 seconds (3 missed beats), it shows a native dialog with Reload/Quit options.
  - Reload destroys and recreates the WebView. App state is intact in stores.

**Alternatives considered**:
- **Process supervisor (restart entire app)**: Too heavy. WebView crash ≠ Rust crash. Stores survive WebView reload.
- **No heartbeat**: WebView hangs are silent failures. Users see a frozen window with no way to recover.
- **Shorter heartbeat interval**: Creates noise. 10s is long enough to avoid false positives from slow renders.

---

## R22: Android Alarm Scheduling Deep Dive

**Decision**: Use `AlarmManager.setExactAndAllowWhileIdle()` + foreground service + Doze exemption

**Rationale**:
- **API selection**: `setExactAndAllowWhileIdle()` is the only Android API that guarantees alarm delivery during Doze mode with reasonable precision (±1 minute). Standard `setExact()` is deferred during Doze.
- **SCHEDULE_EXACT_ALARM permission**: Required on Android 12+ (API 31). Without it, even "exact" alarms use the inexact path. The app detects API level at runtime and requests this permission before scheduling the first alarm.
- **Foreground service**: `foregroundServiceType="specialUse"` keeps the process alive. The notification shows "N active alarms". Tapping the notification opens the app.
- **Battery exemption**: `ACTION_REQUEST_IGNORE_BATTERY_OPTIMIZATIONS` is requested on first alarm creation. This exempts the app from some Doze restrictions. Google Play allows this for alarm apps.
- **No wake locks**: Wake locks drain battery excessively and are unnecessary when using exact alarm APIs. The AlarmManager → BroadcastReceiver → Notification path doesn't need to hold the CPU awake.
- **Target SDK**: 33 (Android 13). Min SDK: 26 (Android 8.0). This covers notification channels (API 26+), exact alarm permission (API 31+), and foreground service types (API 29+).

**Alternatives considered**:
- **WorkManager**: Designed for deferrable work, not exact-time alarms. Cannot guarantee ±60 second precision.
- **FCM (Firebase Cloud Messaging)**: Requires Google Play Services and a server. Violates offline-first requirement (FR-014).
- **Wake locks**: `PowerManager.PARTIAL_WAKE_LOCK` keeps CPU awake but drains battery. Exact alarms are more efficient.
- **JobScheduler**: Cannot set exact times. Designed for batched background work.
