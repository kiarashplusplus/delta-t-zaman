---
artifact_meta:
  produced_by: "os.sketch"
  produced_at: "2025-07-26T12:00:00Z"
  confidence: 0.3
  inputs_used: []
  stale_after: "on_input_change"
  revision: 1
  quality_scores:
    specificity: null
    actionability: null
    non_redundancy: null
    evidence_quality: null
  grade: "hypothesis"
---

# Product Sketch: Delta-T Zaman

> **Grade: HYPOTHESIS** — This sketch has not been validated by Phase 1 research agents.
> All claims marked `[ASSUMED]` are research targets, not facts.

## USP Hypothesis

For remote workers and digital nomads collaborating across 3+ time zones, **Delta-T Zaman** is the only native cross-platform world clock that combines menu-bar presence, a time comparison slider, and zone-aware notifications in a single lightweight app — because no existing solution delivers beautiful, always-available timezone awareness without the bloat of a full calendar app or the limitations of web-only tools.

## Core Features

| # | Feature | Why Required for USP | Simplest Implementation |
|---|---------|---------------------|------------------------|
| 1 | **Multiple clocks display with menu bar / system tray** | The USP promises "always-available timezone awareness" — clocks must be visible at a glance without opening a full window. This is the core delivery surface. | Tauri `SystemTray` API with a context menu listing configured zones and their current times. Main window shows larger, styled clock cards. Use `Intl.DateTimeFormat` for all timezone rendering (DST handled automatically by the OS timezone database). |
| 2 | **Quick timezone search** | Users must be able to configure zones effortlessly — "add Tokyo" not "find Asia/Tokyo in a 400-item dropdown." Frictionless configuration is what separates a polished tool from a utility. | Frontend fuzzy search over the IANA timezone list (~400 entries), indexed by city name, country, and UTC offset. Use a lightweight JS fuzzy library (e.g. `fuse.js`). Persist selections via `@tauri-apps/plugin-store`. |
| 3 | **Time comparison slider** | The USP's differentiator — "if it's 3pm here, what time is it there?" This is the feature that transforms a passive display into an active planning tool. No native competitor offers this. | A draggable HTML range input that offsets `Date.now()` by N hours/minutes. All displayed clocks re-render in real time as the slider moves. Pure frontend logic — no Rust needed. |
| 4 | **Zone-aware notifications** | Completes the value loop: see times → plan across zones → get reminded. Without notifications, users must still set alarms in a separate app, breaking the workflow. | Tauri `@tauri-apps/plugin-notification` for native OS alerts. Alarm scheduling via Rust `tokio::time::sleep` in a background task (avoids JS timer throttling when window is minimized). Persist alarms in the Store plugin. |
| 5 | **Persistent settings** | The app must remember configured zones and alarms across restarts — without this, every launch requires re-setup, which destroys the "just works" promise. | `@tauri-apps/plugin-store` writing a JSON file to the OS user-data directory. Load on startup, save on every mutation. |

## Known Risks

| Category | Risk | Grade | Notes |
|----------|------|-------|-------|
| Market | World clock apps exist in every app store (Dato, Menu World Time, Fig, built-in iOS/Android clocks). The market is crowded with free and paid alternatives. | [KNOWN] | Differentiation must come from the time slider + cross-platform sync + native feel combination. Validate whether users actually switch from existing tools. |
| Market | The addressable market may be small — many users solve this with a quick Google search ("time in Tokyo") or their phone's built-in clock. Willingness to install a dedicated app is unproven. | [ASSUMED] | Phase 1 should validate whether the "quick Google" behavior indicates unmet need or sufficient solution. |
| Technical | JS timers (`setInterval`, `setTimeout`) are throttled or paused when the Tauri WebView is minimized or backgrounded. Clocks will drift or freeze. | [KNOWN] | Mitigated by recalculating from `Date.now()` on focus/resume and using Rust-side scheduling for alarms. Deep-research report confirms this issue and workaround. |
| Technical | Tauri mobile support (iOS/Android) is still beta. Not all desktop plugins work on mobile (no system tray, no global shortcuts, no autostart). Background task execution is constrained by OS power management. | [KNOWN] | MVP should target macOS desktop only. Mobile readiness is a future milestone, not an MVP requirement. |
| Technical | `tauri-plugin-schedule-task` is a community plugin, not first-party. Its long-term maintenance and reliability across OS updates is uncertain. | [ASSUMED] | Evaluate whether a simpler Rust `tokio` background task is sufficient for MVP alarm scheduling, avoiding the community plugin dependency. |
| Legal | macOS distribution requires an Apple Developer certificate ($99/year) for code signing and notarization. Without it, Gatekeeper blocks the app with a scary warning dialog. | [KNOWN] | Required cost for credible macOS distribution. Budget accordingly. |
| Legal | No known IP or regulatory risks for a timezone display app. No user data is collected or transmitted. | [KNOWN] | Low risk. |
| Distribution | No established brand, audience, or distribution channel. Cold-start problem for an indie app in a crowded category. | [ASSUMED] | Potential channels: Product Hunt launch, Hacker News "Show HN", Reddit (r/digitalnomad, r/remotework), Twitter/X indie dev community. Validate which channels reach the target user. |
| Distribution | App store discoverability is poor for utility apps. Search terms like "world clock" return hundreds of results dominated by incumbents. | [ASSUMED] | May need to differentiate on brand/design quality and word-of-mouth rather than app store SEO. Consider starting as a direct-download (DMG) with a landing page. |

## Target User

**Primary persona: The Timezone Juggler.** A remote software engineer, product manager, or freelancer who collaborates daily with teammates or clients in 3–5 time zones. They currently solve the "what time is it there?" problem with a mix of Google searches, their phone's built-in clock app, and mental UTC math — all of which require context-switching away from their work. They want a tool that's always visible (menu bar), instantly consultable, and helps them plan meetings without opening a calendar. They value design quality and will pay for tools that feel native and polished (they already use apps like Raycast, Arc, or Notion).

## Success Criteria

1. **Activation**: If 40%+ of users who download the app configure 3+ time zones within their first session, the "multi-timezone collaboration" use case is validated.
2. **Retention**: If 30%+ of week-1 users are still opening the app daily after 4 weeks, the "always-available" value proposition is confirmed — the app has earned a persistent place in their workflow.
3. **Differentiator validation**: If 20%+ of active users use the time comparison slider at least once per week, the core differentiating feature has proven utility beyond basic clock display.

## Phase 1 Research Targets

The following `[ASSUMED]` items require validation by Phase 1 agents before committing to build:

1. **Market size & willingness to install** (`os.market`) — Validate whether "quick Google search" users represent unmet need or satisfied demand. Quantify the addressable market of people who collaborate across 3+ time zones daily.
2. **Competitive landscape depth** (`os.competition`) — Map existing native world clock apps (Dato, Menu World Time, Fig, There, Clocker) and identify specific feature gaps. Confirm that no competitor offers menu-bar presence + time slider + notifications in one app.
3. **Community plugin reliability** (`os.feasibility`) — Evaluate `tauri-plugin-schedule-task` maintenance status, open issues, and whether a simpler Rust-native approach is preferable for MVP.
4. **Distribution channel viability** (`os.validate`) — Test which channels (Product Hunt, HN, Reddit, Twitter) actually reach remote workers who'd install a world clock app. Define a pre-launch validation experiment.
5. **USP falsifiability** (`os.usp`) — Stress-test whether "native + slider + notifications" is a real differentiator or a feature bundle that users don't actually value as a unit.
6. **Pricing model** (`os.market`) — Determine if this is a free app (growth-first), freemium (slider/notifications gated), or paid upfront ($5–10 indie app). Validate willingness to pay.
