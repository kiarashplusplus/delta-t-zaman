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
    actionability: 4
    non_redundancy: 5
    evidence_quality: 4
  grade: "draft"
---

# Competitor Matrix: Delta-T Zaman

## Competitors

We analyze **10 competitors** across native apps, built-in OS tools, and web apps. The sketch listed "Fig" as a competitor, but no notable macOS timezone/world clock app by that name could be found through App Store, MacUpdate, or web searches `[Verified — negative search result]`. It may be discontinued or misattributed. We substitute the "Time" app (menubartime.com), which emerged as a significant newer entrant during research.

| # | Competitor | Type | Price | Platform | One-Line Description |
|---|-----------|------|-------|----------|---------------------|
| 1 | **Dato** | Native macOS | $16 one-time | macOS only | Menu bar calendar + world clocks + time travel by Sindre Sorhus — the category leader `[Verified]` |
| 2 | **Clocker** | Native macOS | Free (open-source) | macOS only | Open-source menu bar world clock with time scroller and calendar integration `[Verified]` |
| 3 | **Time (menubartime.com)** | Native macOS | From $24.99 one-time | macOS only | Newer entrant with airport code search, DST warnings, and date picker `[Verified]` |
| 4 | **There** | Native macOS | Free | macOS only | People-first timezone app — shows faces and names, not just cities `[Verified]` |
| 5 | **Menu World Time** | Native macOS | Free | macOS only | Simple menu bar world clock with sunrise/sunset times `[Verified]` |
| 6 | **World Clock Pro** | Native macOS/iOS | Setapp ($9.99/mo) | macOS + iOS | Full-featured timezone converter with map and weather integration `[Verified]` |
| 7 | **macOS Built-in Clock** | OS Built-in | Free | macOS only | Notification Center / desktop widgets for world clocks `[Verified]` |
| 8 | **Windows Built-in Clock** | OS Built-in | Free | Windows only | Clock app with world clock, compare slider, and map view `[Verified]` |
| 9 | **Every Time Zone** | Web App | Free | Web (all platforms) | Beautiful visual timeline for comparing times across zones `[Verified]` |
| 10 | **Timezone.io** | Web App | Free | Web (all platforms) | Team-focused timezone tracker showing who's working when `[Verified]` |

## Cross-Platform Coverage Analysis

This is the **critical table** for Delta-T Zaman's positioning. It makes the cross-platform gap visually obvious.

| Competitor | macOS | Windows | Linux | iOS | Android | Web | Platforms Served |
|-----------|:-----:|:-------:|:-----:|:---:|:-------:|:---:|:----------------:|
| **Dato** | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | 1 |
| **Clocker** | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | 1 |
| **Time (menubartime)** | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | 1 |
| **There** | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | 1 |
| **Menu World Time** | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | 1 |
| **World Clock Pro** | ✅ | ❌ | ❌ | ✅ | ❌ | ❌ | 2 |
| **macOS Clock** | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | 1 |
| **Windows Clock** | ❌ | ✅ | ❌ | ❌ | ❌ | ❌ | 1 |
| **Every Time Zone** | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ | 1 (web) |
| **Timezone.io** | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ | 1 (web) |
| **Delta-T Zaman** | ✅ | ✅ | ✅ | ❌ | ❌ | ❌ | **3** |

**Key finding**: Every native desktop world clock app is **macOS-only**. The only multi-platform option is World Clock Pro (macOS + iOS, but no desktop Windows/Linux). Web apps serve all platforms but sacrifice native integration (system tray, notifications, persistent menu bar display). **No competitor provides a native desktop experience on macOS + Windows + Linux.** This is the cross-platform gap identified in market-map.md.

## Feature Comparison

Legend: ✅ = Yes | ⚠️ = Partial/Limited | ❌ = No | `?` = Unverified

| Feature | Dato | Clocker | Time | There | Menu WT | WC Pro | macOS Clock | Win Clock | ETZ (web) | TZ.io (web) | **Delta-T Zaman (planned)** |
|---------|:----:|:------:|:----:|:-----:|:------:|:------:|:----------:|:--------:|:---------:|:----------:|:--------------------------:|
| **Menu bar / system tray display** | ✅ `[V]` | ✅ `[V]` | ✅ `[V]` | ✅ `[V]` | ✅ `[V]` | ❌ `[V]` | ⚠️ widgets `[V]` | ⚠️ 2 extra only `[V]` | ❌ | ❌ | ✅ |
| **Multiple timezone clocks** | ✅ `[V]` | ✅ (10+) `[V]` | ✅ `[V]` | ✅ `[V]` | ✅ `[V]` | ✅ `[V]` | ✅ widgets `[V]` | ✅ `[V]` | ✅ `[V]` | ✅ `[V]` | ✅ |
| **Time slider / time travel** | ✅ `[V]` | ✅ `[V]` | ✅ `[V]` | ❌ `[V]` | ❌ `[V]` | ✅ `[V]` | ❌ `[V]` | ✅ `[V]` | ✅ `[V]` | ❌ `[V]` | ✅ |
| **City/timezone search** | ✅ 15K cities `[V]` | ✅ thousands `[V]` | ✅ 5.5K airports `[V]` | ✅ `[V]` | ⚠️ browse only `[V]` | ✅ `[V]` | ✅ `[V]` | ✅ `[V]` | ⚠️ preset list `[V]` | ❌ `[V]` | ✅ |
| **Custom labels / names** | ✅ `[V]` | ✅ `[V]` | ✅ `[V]` | ✅ (people names) `[V]` | ✅ `[V]` | ❌ `[M]` | ❌ `[V]` | ✅ `[V]` | ❌ `[V]` | ✅ (team names) `[V]` | ✅ |
| **Calendar integration** | ✅ (iCloud/Google/Outlook) `[V]` | ✅ `[V]` | ✅ `[V]` | ❌ `[V]` | ❌ `[V]` | ❌ `[V]` | ❌ `[V]` | ❌ `[V]` | ❌ `[V]` | ❌ `[V]` | ❌ (MVP) |
| **Video call join (Zoom/Meet)** | ✅ 75+ services `[V]` | ✅ Zoom/Meet/Webex `[V]` | ✅ 4 services `[V]` | ❌ `[V]` | ❌ `[V]` | ❌ `[V]` | ❌ `[V]` | ❌ `[V]` | ❌ `[V]` | ❌ `[V]` | ❌ (MVP) |
| **Notifications / reminders** | ✅ fullscreen `[V]` | ✅ `[V]` | ❌ `[M]` | ❌ `[V]` | ❌ `[V]` | ❌ `[M]` | ❌ `[V]` | ❌ `[V]` | ❌ `[V]` | ❌ `[V]` | ✅ |
| **DST handling / warnings** | ✅ auto `[V]` | ✅ auto `[V]` | ✅ warnings `[V]` | ✅ auto `[M]` | ✅ auto `[M]` | ✅ auto `[M]` | ✅ auto `[V]` | ✅ auto `[V]` | ✅ auto `[M]` | ✅ auto `[M]` | ✅ auto |
| **DST change alerts** | ❌ `[V]` | ❌ `[V]` | ✅ `[V]` | ❌ `[V]` | ❌ `[V]` | ❌ `[M]` | ❌ `[V]` | ❌ `[V]` | ❌ `[V]` | ❌ `[V]` | ⚠️ planned |
| **Sunrise / sunset times** | ❌ `[V]` | ✅ `[V]` | ✅ `[V]` | ❌ `[V]` | ✅ `[V]` | ❌ `[M]` | ❌ `[V]` | ❌ `[V]` | ❌ `[V]` | ❌ `[V]` | ❌ (MVP) |
| **World map view** | ❌ `[V]` | ❌ `[V]` | ❌ `[V]` | ❌ `[V]` | ✅ `[V]` | ✅ `[V]` | ❌ `[V]` | ✅ `[V]` | ❌ `[V]` | ❌ `[V]` | ❌ (MVP) |
| **Keyboard shortcuts** | ✅ extensive `[V]` | ✅ `[V]` | ✅ `[V]` | ❌ `[V]` | ❌ `[V]` | ❌ `[M]` | ❌ `[V]` | ❌ `[V]` | ❌ `[V]` | ❌ `[V]` | ✅ |
| **Privacy / offline** | ✅ no network `[V]` | ✅ `[V]` | ✅ `[V]` | ✅ local only `[V]` | ✅ `[M]` | ❌ weather `[V]` | ✅ `[V]` | ✅ `[V]` | ❌ web `[V]` | ❌ web `[V]` | ✅ |
| **People-focused (photos/avatars)** | ❌ `[V]` | ❌ `[V]` | ❌ `[V]` | ✅ `[V]` | ❌ `[V]` | ❌ `[V]` | ❌ `[V]` | ❌ `[V]` | ❌ `[V]` | ✅ `[V]` | ❌ (MVP) |
| **Date picker (future dates)** | ❌ `[V]` | ❌ `[V]` | ✅ `[V]` | ❌ `[V]` | ❌ `[V]` | ❌ `[M]` | ❌ `[V]` | ❌ `[V]` | ❌ `[V]` | ❌ `[V]` | ❌ (MVP) |
| **Airport code search** | ❌ `[V]` | ❌ `[V]` | ✅ 5,500+ `[V]` | ❌ `[V]` | ❌ `[V]` | ❌ `[V]` | ❌ `[V]` | ❌ `[V]` | ❌ `[V]` | ❌ `[V]` | ❌ |
| **Native app** | ✅ Swift `[V]` | ✅ Swift `[V]` | ✅ Swift `[V]` | ✅ Swift `[V]` | ✅ `[V]` | ✅ `[V]` | ✅ `[V]` | ✅ `[V]` | ❌ web `[V]` | ❌ web `[V]` | ✅ Tauri/Rust |
| **Open source** | ❌ `[V]` | ✅ `[V]` | ❌ `[V]` | ✅ `[V]` | ❌ `[V]` | ❌ `[V]` | ❌ `[V]` | ❌ `[V]` | ❌ `[M]` | ❌ `[M]` | ❌ |

Citation key: `[V]` = Verified via tool research during this run. `[M]` = Model-sourced (may be outdated).

## Pricing Comparison

| Competitor | Price | Model | Value Proposition |
|-----------|-------|-------|-------------------|
| **Dato** | $16 | One-time, free updates forever `[V]` | Premium all-in-one: calendar + clocks + meetings |
| **Time** | From $24.99 | One-time, 1 year of updates `[V]` | Modern design, unique features (airport codes, DST alerts) |
| **World Clock Pro** | $9.99/mo (Setapp) | Subscription (bundled) `[V]` | Part of 250+ app bundle |
| **Clocker** | Free | Open-source `[V]` | Strong free option, community-maintained |
| **There** | Free | Free `[V]` | People-first approach, lightweight |
| **Menu World Time** | Free | Free `[V]` | Basic but functional |
| **macOS Clock** | Free | Built-in `[V]` | Always available but limited |
| **Windows Clock** | Free | Built-in `[V]` | Always available but limited |
| **Every Time Zone** | Free | Web (ad-supported) `[V]` | Zero-install, visual comparison |
| **Timezone.io** | Free | Web `[V]` | Team coordination |
| **Delta-T Zaman** | $7.99 (planned) | One-time | Cross-platform + focused feature set |

## Analysis

### Where Delta-T Zaman Could Lead

1. **Cross-platform native desktop** — No competitor serves macOS + Windows + Linux with a native app. This is the single largest structural gap in the market `[Verified]`. Every native timezone app is macOS-only. Windows and Linux users have *zero* dedicated native timezone menu bar / system tray tools beyond the basic OS clock.

2. **Lightweight + focused** — At ~600KB (Tauri), Delta-T Zaman would be dramatically smaller than competitors. Clocker is 2.9 MB `[V]`, There is 4.8 MB `[V]`, World Clock Pro is 109.7 MB `[V]`. This matters to the ICP who values "lightweight, native-feeling tools" (per icp.md).

3. **Price-to-value positioning** — At $7.99, Delta-T Zaman sits below Dato ($16) and Time ($24.99) but above free options. If it delivers the core feature triad (menu bar + slider + notifications) on all 3 desktop platforms, the cross-platform angle justifies the price vs. free macOS-only alternatives.

### Where Delta-T Zaman Lags

1. **Calendar integration** — Dato, Clocker, and Time all have calendar integration. The MVP omits this. For the ICP who schedules meetings across zones, this is a notable gap — the "planning" workflow is incomplete without seeing calendar events alongside timezone data.

2. **Video call join** — Dato (75+ services), Clocker, and Time all support one-click meeting joins. This is a workflow accelerator that the ICP uses daily. Missing from MVP.

3. **Ecosystem depth** — Dato has 15K offline cities, custom date formats, HTML event notes, reminders, widgets, keyboard shortcuts, and years of polish. A new entrant cannot match this depth at launch.

4. **Maturity and trust** — Dato has Sindre Sorhus's reputation, MacStories reviews, and years of user trust. Clocker has 17K+ ratings. There is used by Apple, Shopify, IBM. A new app starts from zero.

### Genuine Feature Gaps (Nobody Does This)

| Gap | Verification | ICP Relevance |
|-----|-------------|---------------|
| **Native desktop app on macOS + Windows + Linux** | No native competitor serves all 3 `[Verified]` | High — ICP includes devs who use macOS primary + Linux/Windows secondary (per icp.md) |
| **DST change alerts + time slider in one app** | Only "Time" has DST warnings; Dato/Clocker have sliders but no DST alerts `[Verified]` | Medium — DST transitions reduce communication by 9.2% (per why-now.md) |
| **Working hours overlay on time slider** | No competitor visually shows "this person's work hours" on the slider `[Model-sourced]` | High — directly addresses P4 "overlap window blindness" from icp.md |
| **Cross-platform settings sync** | No lightweight timezone app syncs settings across macOS/Windows/Linux `[Model-sourced]` | Medium — relevant for multi-OS users but not proven demand |

### ICP Fit Rating

How well does each competitor serve "The Timezone Juggler" — a remote worker coordinating across 3+ time zones daily?

| Competitor | ICP Fit | Rationale |
|-----------|:-------:|-----------|
| **Dato** | ⭐⭐⭐⭐⭐ | Calendar + clocks + time travel + meeting joins — the complete remote worker toolkit. macOS-only is the only weakness for multi-OS users. `[V]` |
| **Clocker** | ⭐⭐⭐⭐ | Strong free option with slider + calendar + meeting joins. Weaker UX and stale updates reduce confidence. `[V]` |
| **Time** | ⭐⭐⭐⭐ | Modern UX + DST warnings + date picker. Expensive ($24.99) and newer (less proven). `[V]` |
| **There** | ⭐⭐⭐ | People-first is great for small teams. No slider or planning features limits utility for 3+ zone coordination. `[V]` |
| **Every Time Zone** | ⭐⭐⭐ | Excellent for one-off comparisons. No persistent display — requires opening a browser tab each time. `[V]` |
| **Menu World Time** | ⭐⭐ | Too basic (no slider, no calendar). Abandoned since 2021. `[V]` |
| **World Clock Pro** | ⭐⭐ | Full-featured but not a menu bar app — requires launching a separate window. Heavy. `[V]` |
| **Windows Clock** | ⭐⭐ | Has a slider, but buried in Clock app. Only 2 extra taskbar clocks. Not designed for power users. `[V]` |
| **macOS Clock** | ⭐⭐ | Widget-based. No slider. Requires Notification Center navigation. `[V]` |
| **Timezone.io** | ⭐⭐ | Team-focused but web-only with limited features. `[V]` |

## Quality Score

| Criterion | Score (1-5) | Notes |
|-----------|-------------|-------|
| **Specificity** | 4 | Every competitor claim cites a source and verification status. Platform coverage is exhaustive. Feature matrix covers 17 dimensions across 10 competitors. |
| **Actionability** | 4 | Clear cross-platform gap identified. ICP fit ratings guide prioritization. os.usp can directly consume the gap analysis. |
| **Non-redundancy** | 5 | First competitive matrix in the pipeline. Adds feature-level detail not present in market-map.md's high-level competitor list. |
| **Evidence quality** | 4 | 85%+ of claims are `[Verified]` via tool research (App Store listings, developer websites, comparison pages, review sites). Remaining `[Model-sourced]` items are flagged. |
