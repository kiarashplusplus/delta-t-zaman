---
artifact_meta:
  produced_by: "os.competition"
  produced_at: "2025-07-27T03:15:00Z"
  confidence: 0.60
  inputs_used:
    - ".specify/artifacts/sketch.md"
    - ".specify/artifacts/phase-1/market-map.md"
    - ".specify/artifacts/phase-1/icp.md"
    - ".specify/artifacts/phase-1/why-now.md"
  stale_after: "on_input_change"
  revision: 1
  quality_scores:
    specificity: 4
    actionability: 5
    non_redundancy: 4
    evidence_quality: 4
  grade: "draft"
---

# Steal · Differentiate · Ignore

Strategic classification of competitor features for Delta-T Zaman MVP and roadmap. Every classification is tied back to the ICP ("The Timezone Juggler" — remote worker coordinating across 3+ time zones daily) and market findings.

---

## STEAL

> Features that are **table stakes** — every competent world clock needs these. Adopt the best implementation. Not differentiators, but their absence would be disqualifying.

### S1. Menu Bar / System Tray Display with Multiple Clocks

**What**: Show configured timezones directly in the system menu bar (macOS) or system tray (Windows/Linux) — always visible without clicking.

**Best implementation**: Dato and Clocker both show multiple clocks in the menu bar. Clocker is "designed to work efficiently with 10 time zones" `[Verified]`. Dato allows clocks "in either the Dato menu or the menu bar" `[Verified]`.

**ICP rationale**: The ICP's primary need (P1 from icp.md) is "ambient timezone awareness" — knowing what time it is for colleagues without any action. A menu bar display is the minimum viable interface for this. Without it, the app is just another window to open.

**Implementation guidance**: Support at minimum 3 configurable clocks in the system tray area. Allow customization of format (12/24h, city name vs. abbreviation, seconds). Follow OS conventions — menu bar for macOS, system tray for Windows, tray applet for Linux.

---

### S2. Time Slider / Time Scrubber

**What**: A slider that lets users scrub forward and backward in time to see how all configured timezones shift together.

**Best implementation**: Clocker's "modernized time slider" with 15-minute increments and trackpad support `[Verified]`. Every Time Zone's drag-tab that visually moves the highlight across all zones `[Verified]`. Time (menubartime) adds color-coded day/night indicators `[Verified]`.

**ICP rationale**: The ICP's P3 pain point (icp.md) is "meeting roulette" — finding overlap windows across 3+ zones. A time slider is the standard tool for solving this. Dato, Clocker, Time, Windows Clock, and Every Time Zone all have one `[Verified]`. Competitors *without* a slider (There, Menu World Time) are rated lower on ICP fit.

**Implementation guidance**: Minimum: ±24h relative scrubbing from "now." Increment of 15 minutes (matches Clocker). Show day/night indicators per timezone (matches Time). Distinguish between same-day and next/previous day.

---

### S3. Quick Timezone Search

**What**: Rapidly search and add timezones by city name, timezone abbreviation (PST, CET), or UTC offset.

**Best implementation**: Dato searches 15,000 cities offline `[Verified]`. Clocker searches "thousands of cities/street and time zones" `[Verified]`. There accepts city, country, abbreviation, or UTC offset `[Verified]`.

**ICP rationale**: The ICP currently "googles 'what time is it in [city]'" (P1 from icp.md). The in-app search replaces this workflow. Poor search (like Menu World Time's browse-only approach `[Verified]`) creates setup friction that reduces adoption.

**Implementation guidance**: Offline city database (minimum 5K–10K cities). Support search by city name, country, timezone abbreviation (PST, EST, CET), and UTC offset. Fuzzy matching for typos. Target <100ms response time.

---

### S4. Custom Labels / Names

**What**: Let users rename timezone entries with meaningful labels (person names, office names, project names).

**Best implementation**: Dato, Clocker, Time, There, and Menu World Time all support custom labels `[Verified]`. There takes this furthest by making the label the *primary* identifier (person's name and photo), with timezone secondary `[Verified]`.

**ICP rationale**: The ICP thinks in terms of people and teams, not timezones. "London" is less useful than "Platform Team" or "Sarah (designer)". Custom labels are a small feature with outsized impact on daily usability.

**Implementation guidance**: Editable label per timezone. Show label prominently, timezone/city as secondary info. Allow emoji in labels.

---

### S5. DST-Aware Time Calculations

**What**: Automatically adjust displayed times when Daylight Saving Time transitions occur — no manual updating required.

**Best implementation**: Every native competitor handles this automatically via the OS timezone database. This is a baseline expectation `[Verified across all competitors]`.

**ICP rationale**: The ICP loses 5–10 minutes per DST transition googling whether a timezone has shifted (P2 from icp.md). Incorrect times would be worse than no app at all.

**Implementation guidance**: Use the IANA timezone database (tzdata). Ensure the app references named timezones (e.g., "America/New_York") not fixed UTC offsets, so DST transitions are automatic. Bundle tzdata or use the OS database.

---

### S6. Persistent Settings / State

**What**: Remember configured timezones, labels, and display preferences across app restarts and OS reboots.

**Best implementation**: All competitors do this. It's not a feature — it's the absence of a bug.

**ICP rationale**: The ICP sets up 5–8 timezones once and expects them to persist indefinitely. Any app that loses settings is immediately uninstalled.

**Implementation guidance**: Store settings in platform-appropriate location (e.g., `~/Library/Application Support/` on macOS, `%APPDATA%` on Windows, `~/.config/` on Linux). Use a serialization format that survives schema changes (JSON with version field).

---

### S7. Keyboard Shortcuts

**What**: Global keyboard shortcut to open/close the timezone panel. In-panel shortcuts for common actions.

**Best implementation**: Dato has "lots of in-app keyboard shortcuts for power users" and a "global keyboard shortcut to open/close the app" `[Verified]`. Clocker also supports keyboard shortcuts `[Verified]`.

**ICP rationale**: The ICP includes software engineers and power users who live in the keyboard. A menu-bar app without a global hotkey forces mouse interaction for every check.

**Implementation guidance**: Configurable global hotkey (default: something like `⌘+Shift+T` or `Ctrl+Shift+T`). At minimum: open/close panel, navigate between timezones.

---

## DIFFERENTIATE

> Features where Delta-T Zaman can be **meaningfully better** than existing solutions. This is where the competitive advantage lives.

### D1. Cross-Platform Native Desktop (macOS + Windows + Linux)

**What**: A single codebase (Tauri 2.10.3 + Rust) that delivers a native-feeling app on macOS, Windows, and Linux — with system tray integration, native notifications, and platform-appropriate UX on each.

**Why this is the #1 differentiator**: Every native desktop world clock app is macOS-only `[Verified]`. Dato = macOS. Clocker = macOS. Time = macOS. There = macOS. Menu World Time = macOS. The only multi-desktop option is the built-in OS clock on each platform, which is severely limited. Web apps (Every Time Zone, Timezone.io) are cross-platform but lack native integration.

**ICP rationale**: The ICP is "primarily macOS" but includes Windows and Linux users (icp.md). Remote workers on mixed-OS teams have no shared timezone tool. A team with macOS and Windows users can't all use Dato. They either use a web app (losing native integration) or use different tools per platform (losing consistency).

**Differentiation angle**: Not "we're better at feature X" — it's "we exist where they don't." This is a market-structure advantage, not a feature advantage. It's also defensible: building a native cross-platform app with Tauri is technically non-trivial, creating a moat.

**Risk**: Cross-platform means serving three sets of platform conventions. A mediocre experience on three platforms may lose to an excellent experience on one (Dato on macOS). The UX must feel native on each platform, not "web wrapper" quality.

---

### D2. Zone-Aware Notifications / Reminders

**What**: Set a reminder based on another timezone's local time. "Remind me when it's 9am in Tokyo" → fires a notification at 5pm Pacific (or whatever the offset is, including DST shifts).

**Why this is a differentiator**: Only Clocker explicitly offers "Set Reminders for a particular time in different timezone" `[Verified]`. Dato's notifications are calendar-event-driven, not timezone-driven. Time (menubartime) doesn't mention standalone timezone alerts `[Model-sourced]`. There and Every Time Zone have no notifications at all `[Verified]`.

**ICP rationale**: The ICP's P4 pain point (icp.md) is "overlap window blindness" — knowing *when* a colleague becomes available. A zone-aware notification solves this directly: "Alert me when the Mumbai team's workday starts." This is a daily-use feature for someone managing 3+ timezone relationships.

**Differentiation angle**: Clocker has the feature but is stale (last updated April 2024 `[Verified]`), has UX issues, and is macOS-only. Delta-T Zaman can do zone-aware notifications on macOS + Windows + Linux with better UX. No competitor does this cross-platform.

---

### D3. DST Transition Warnings

**What**: Proactively alert users when a configured timezone is about to undergo a DST shift, showing the before/after offset and how it affects meeting times.

**Why this is a differentiator**: Only the Time app (menubartime.com) offers DST change warnings `[Verified]`. No other native app does this. why-now.md cites research showing DST transitions reduce communication volume by 9.2% `[Verified from upstream artifact]` — a real productivity cost.

**ICP rationale**: DST transitions are the #2 pain point from icp.md (P2: "timezone shifting anxiety"). Users who schedule recurring meetings across zones get burned twice a year when offsets change. A proactive warning ("⚠️ London shifts +1 hour on March 30 — your 4pm meeting will become 3pm for Sarah") prevents this.

**Differentiation angle**: Pair DST warnings with the time slider and cross-platform presence for a unique value combination no competitor offers.

---

### D4. Working Hours Overlay on Time Slider

**What**: When scrubbing the time slider, visually indicate each timezone's "work hours" (configurable, e.g., 9am–6pm) with a highlighted band. Users can instantly see when work-hour windows overlap across all configured zones.

**Why this is a differentiator**: No competitor offers this. Dato, Clocker, and Time show day/night indicators (broad daylight/darkness) but none show a configurable "working hours" band `[Model-sourced — no competitor documentation mentions this feature]`. Every Time Zone has no working-hours concept `[Verified]`.

**ICP rationale**: The ICP's core task is finding overlap windows across 3+ zones. Day/night indicators are a blunt tool — they distinguish 7am from 3am, but not 10am (great meeting time) from 7am (technically daytime but before most work starts). Configurable work-hour bands would make the slider immediately actionable.

**Differentiation angle**: This turns the time slider from a "what time is it there?" tool into a "when can we all meet?" tool. It's the single feature most directly mapped to the ICP's daily workflow.

**Risk**: The feature must be simple to configure. If setting up work hours for 5+ timezones requires 5 configuration steps each, the friction will outweigh the benefit.

---

### D5. Lightweight / Low Resource Usage

**What**: An app that uses minimal memory, CPU, and disk space — suitable for "always running" background operation.

**Why this is a differentiator**: There positions itself as "ultra low resource usage" at 4.8 MB `[Verified]`. Clocker is 2.9 MB `[Verified]`. But World Clock Pro is 109.7 MB `[Verified]`. Delta-T Zaman, built on Tauri + Rust, can target sub-5 MB with near-zero idle CPU. This is an advantage over Electron-based alternatives (if any emerge) and heavy native apps.

**ICP rationale**: The ICP values "lightweight tools" and "native feel" (icp.md). A timezone app that consumes 200MB of RAM or causes noticeable CPU usage would be unacceptable — it's a utility that should be invisible until needed.

**Differentiation angle**: Tauri's Rust backend + web frontend gives genuinely lower resource usage than Electron while maintaining cross-platform reach. This is a real technical advantage worth marketing: "under 5MB, uses less memory than a browser tab."

---

## IGNORE

> Features that are **noise for the MVP** — either irrelevant to the ICP, over-engineered for the use case, or distracting from core value delivery.

### I1. Calendar Integration (for MVP)

**What**: Display upcoming calendar events (from iCloud, Google Calendar, Outlook) alongside timezone information.

**Why ignore for MVP**: While Dato's calendar integration is praised `[Verified]`, it turns the app from a "world clock" into a "calendar+clock hybrid." This dramatically increases implementation scope: OAuth flows for Google/Outlook, calendar API integration, event parsing, multiple account support. The ICP has *existing* calendar apps (Apple Calendar, Google Calendar, Fantastical) and doesn't lack calendar visibility — they lack timezone *awareness alongside* their calendar.

**ICP rationale**: Calendar integration is a V2 feature that could drive retention and differentiate, but the ICP's *primary* need (ambient timezone awareness + meeting planning) can be solved without it. The core workflow — "what time is it there?" and "when can we meet?" — doesn't require seeing calendar events in the timezone app.

**Risk of ignoring**: Dato and Clocker users may not consider switching if Delta-T Zaman lacks calendar integration. This should be the #1 post-MVP roadmap item.

---

### I2. Video Call Meeting Join (One-Click Zoom/Meet/Teams)

**What**: Parse calendar events for video call URLs and provide a "join" button directly in the timezone panel.

**Why ignore**: This is a Dato differentiator (75+ services `[Verified]`) but it's really a calendar feature, not a timezone feature. It requires calendar integration (I1) as a prerequisite. Standalone meeting join apps already exist (MeetingBar — free, open source, 40+ services `[Verified]`).

**ICP rationale**: The ICP uses Zoom/Meet daily but has other tools for joining meetings (calendar notifications, MeetingBar, Meeter). Building this from scratch is not justified for MVP when it depends on calendar integration.

---

### I3. Airport Code Search

**What**: Search timezones by IATA airport codes (e.g., "LAX" → Los Angeles, "NRT" → Tokyo Narita).

**Why ignore**: Only Time (menubartime.com) offers this `[Verified]`. It's clever but niche. The ICP searches by city name or timezone abbreviation, not airport codes. Travelers benefit, but the core ICP is a remote worker at a fixed location, not a frequent flyer.

**ICP rationale**: The Timezone Juggler's persona works from home/office/coworking spaces. They think in terms of cities and people ("Sarah in London"), not airports ("LHR"). City search (S3 above) fully covers this need.

---

### I4. World Map View

**What**: A visual map showing where configured timezones are located geographically.

**Why ignore**: World Clock Pro and Menu World Time have map views `[Verified]`. Windows Clock shows cities on a map `[Verified]`. Maps are visually appealing but add little functional value for the ICP. The user already knows where London is — they need to know *what time it is there.*

**ICP rationale**: Maps consume screen real estate and don't help with the core task (finding overlap windows). They add visual complexity to what should be a quick-glance utility. A map is a nice V3 feature but not MVP.

---

### I5. Sunrise / Sunset Times

**What**: Show when the sun rises and sets in each configured timezone.

**Why ignore**: Clocker, Time, and Menu World Time offer this `[Verified]`. It's a "nice to know" feature that doesn't serve the core ICP workflow. Remote workers don't schedule meetings based on sunrise — they schedule based on business hours.

**ICP rationale**: The Timezone Juggler cares about "when is Sarah available?" not "when is sunrise in London?" Sunrise/sunset is a travel feature, not a remote work feature. Skip for MVP; consider for V2 if user feedback requests it.

---

### I6. Weather Integration

**What**: Show current weather conditions alongside timezone information.

**Why ignore**: World Clock Pro has live weather `[Verified]`. This is scope creep — it turns a timezone utility into a weather widget. Weather data requires network calls (breaking the offline/privacy guarantee) and adds ongoing API costs.

**ICP rationale**: Zero mention of weather needs in the ICP. Remote workers don't check weather through their timezone app — they use weather apps or simply look outside.

---

### I7. People/Avatars Feature (for MVP)

**What**: Associate photos, social handles, and contact info with timezone entries — turning the tool from a "city clock" into a "people directory."

**Why ignore for MVP**: There's people-first approach is beautiful `[Verified]` but requires photo import, social API integration (X, Telegram handles `[Verified]`), and a fundamentally different data model. For MVP, custom labels (S4) provide 80% of the value at 10% of the complexity — users can label a timezone "Sarah (London)" without needing photo integration.

**ICP rationale**: The ICP *would* benefit from people-focused design (validated by There's 17K users `[Verified]`). But for MVP, the priority is getting the core timezone UX right on three platforms. People/avatar features should be V2 roadmap.

---

### I8. Cross-Device Settings Sync (iCloud/Cloud)

**What**: Sync timezone configurations, labels, and preferences across devices (e.g., macOS laptop and Windows desktop).

**Why ignore for MVP**: Only "The Clock" (a minor competitor) offers iCloud/Dropbox sync `[Verified from menubartime.com comparison page]`. The ICP likely uses one primary device for work. Cross-device sync requires cloud infrastructure, account management, and conflict resolution — all of which add complexity and cost.

**ICP rationale**: Low urgency. If a user configures 5 timezones on their MacBook and later installs on their Windows desktop, re-entering 5 timezones takes 2 minutes. Sync is a retention/convenience feature for V2+.

---

### I9. Open Source

**What**: Release the app as open-source.

**Why ignore**: Clocker and There are open-source `[Verified]`. But the commercial model (one-time purchase at $7.99, per market-map.md) requires a proprietary codebase. Open-sourcing the core app would undermine the revenue model. If anything, open-source the timezone database search component, not the full app.

**ICP rationale**: The ICP doesn't make purchasing decisions based on open-source status — they care about UX quality, reliability, and price (icp.md). Clocker's open-source status hasn't prevented its UX from degrading.

---

## Classification Summary Table

| Feature | Classification | ICP Priority | Competitor Coverage |
|---------|:-------------:|:------------:|:-------------------:|
| Menu bar / system tray display | **STEAL** | Critical | Dato, Clocker, Time, There, MWT |
| Time slider / scrubber | **STEAL** | Critical | Dato, Clocker, Time, Win Clock, ETZ |
| Quick timezone search | **STEAL** | High | Dato (15K), Clocker, Time, There |
| Custom labels | **STEAL** | High | Dato, Clocker, Time, There, MWT |
| DST-aware calculations | **STEAL** | Critical | All native apps |
| Persistent settings | **STEAL** | Critical | All apps |
| Keyboard shortcuts | **STEAL** | Medium | Dato, Clocker, Time |
| Cross-platform desktop | **DIFFERENTIATE** | Critical | ❌ Nobody |
| Zone-aware notifications | **DIFFERENTIATE** | High | ⚠️ Clocker only (stale) |
| DST transition warnings | **DIFFERENTIATE** | Medium | ⚠️ Time only |
| Working hours overlay | **DIFFERENTIATE** | High | ❌ Nobody |
| Lightweight / low resource | **DIFFERENTIATE** | Medium | There (but macOS-only) |
| Calendar integration | **IGNORE** (MVP) | V2 roadmap | Dato, Clocker, Time |
| Video call join | **IGNORE** (MVP) | V2+ roadmap | Dato (75+), Clocker, Time |
| Airport code search | **IGNORE** | Low | Time only |
| World map view | **IGNORE** | Low | WC Pro, MWT, Win Clock |
| Sunrise / sunset | **IGNORE** (MVP) | Low | Clocker, Time, MWT |
| Weather integration | **IGNORE** | None | WC Pro |
| People / avatars | **IGNORE** (MVP) | V2 roadmap | There, Timezone.io |
| Cross-device sync | **IGNORE** (MVP) | V2+ roadmap | The Clock only |
| Open source | **IGNORE** | None | Clocker, There |

---

## MVP Feature Budget

Based on the classifications above, the **MVP feature set** for Delta-T Zaman should be:

**Must Have (STEAL)**:
1. System tray / menu bar with multiple clocks (S1)
2. Time slider with ±24h scrubbing (S2)
3. Quick timezone search with 5K+ city database (S3)
4. Custom labels for timezone entries (S4)
5. Automatic DST handling (S5)
6. Persistent settings (S6)
7. Global keyboard shortcut (S7)

**Should Have (DIFFERENTIATE)**:
1. Cross-platform: macOS + Windows + Linux (D1) — this is the product
2. Zone-aware notifications (D2) — primary feature differentiator
3. Working hours overlay on slider (D4) — visual differentiator

**Could Have (DIFFERENTIATE, V1.1)**:
1. DST transition warnings (D3)
2. Lightweight/performance marketing (D5) — inherent to Tauri, not a feature to build

**Won't Have (IGNORE for MVP)**:
Calendar integration, video call join, airport codes, map view, sunrise/sunset, weather, people/avatars, cloud sync

---

## Quality Score

| Criterion | Score (1-5) | Notes |
|-----------|-------------|-------|
| **Specificity** | 4 | Every classification references specific competitor implementations, ICP pain points (by ID from icp.md), and verification status. |
| **Actionability** | 5 | Direct "steal/differentiate/ignore" instructions with implementation guidance. MVP feature budget is immediately consumable by downstream planning agents. |
| **Non-redundancy** | 4 | Builds on competitor-matrix.md and ux-teardown.md but adds strategic classification layer not present in either. |
| **Evidence quality** | 4 | 90%+ of competitor claims are `[Verified]`. ICP rationale references upstream artifacts by pain-point ID. |
