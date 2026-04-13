---
artifact_meta:
  produced_by: "os.feasibility"
  produced_at: "2025-07-27T06:00:00Z"
  confidence: 0.80
  inputs_used:
    - ".specify/artifacts/phase-1/usp.md"
    - ".specify/artifacts/phase-1/market-map.md"
    - "deep-research-report.md"
  stale_after: "on_input_change"
  revision: 1
  quality_scores:
    specificity: 4
    actionability: 5
    non_redundancy: 5
    evidence_quality: 4
  grade: "draft"
---

# Feasibility Analysis: Delta-T Zaman

## Technical Constraints

### TC-1: Cross-Platform Native via Tauri 2.10.3

**Severity: medium**

Tauri 2.10.3 can produce native desktop binaries for macOS, Windows, and Linux from a single codebase. The architecture uses system WebViews (WebKit on macOS, WebView2 on Windows, WebKitGTK on Linux) plus a Rust backend. This is a proven, stable approach for desktop — Tauri's desktop support is mature (stable since 2.0 in 2024) `[Verified — v2.tauri.app/concept/architecture]`.

**Platform-specific gotchas:**

| Platform | WebView Engine | Gotcha | Severity |
|----------|---------------|--------|----------|
| **macOS** | WebKit (system) | Always available on macOS 10.13+. No runtime to install. Rendering is consistent. | low |
| **Windows** | WebView2 (Edge) | Pre-installed on Windows 11. On Windows 10, requires WebView2 Runtime installation via bootstrapper. Tauri supports `downloadBootstrapper` (1–2 MB, needs internet) or `offlineInstaller` (~127 MB) or `embedBootstrapper` modes. A known build bug exists with `embedBootstrapper` on some setups (tauri-apps/tauri#13572). | medium |
| **Linux** | WebKitGTK | Requires `libwebkitgtk-4.1` package. Behavior varies across distros and WebKitGTK versions — Ubuntu 20.04 ships WebKitGTK 2.28 (Safari 14 equivalent), while Ubuntu 22.04 ships 2.36 (Safari 16 equivalent). CSS/JS feature availability differs. `[Verified — v2.tauri.app/reference/webview-versions]` | medium |

**GNOME system tray risk:** GNOME desktop environment removed native system tray (StatusNotifierItem) support in GNOME 3.26. Tray icons require the user to install the "AppIndicator and KStatusNotifierItem Support" GNOME Shell extension. This extension is actively maintained (supports GNOME 45–50 as of 2025) but broke temporarily during GNOME 48 transition `[Verified — extensions.gnome.org/extension/615, Arch Linux Forums]`. KDE Plasma and XFCE support tray natively. **Impact**: ~30-40% of desktop Linux users run GNOME; tray icon may not appear without user action.

**Mitigation**: Document GNOME tray extension requirement in Linux installation guide. Consider a fallback window mode (non-tray) for Linux users without tray support. This is a documentation issue, not a blocking technical problem.

**Verdict**: Tauri is technically capable of delivering on the cross-platform USP claim. The WebView fragmentation across Linux distros is the highest-risk area but manageable with CI testing across WebKitGTK versions.

---

### TC-2: Working-Hours Overlay with Time Slider

**Severity: low**

The working-hours overlay is a **standard frontend rendering problem**, not a complex systems challenge. Implementation breakdown:

1. **Time slider**: An HTML `<input type="range">` or custom draggable element mapping 0–1440 (minutes in a day) to a visual timeline. Libraries like D3.js, or plain SVG/Canvas, handle this trivially. Every Time Zone (web app) already implements a similar slider in pure HTML/CSS `[Verified — everytimezone.com]`.

2. **Working-hours overlay**: For each timezone, render a colored band from `workStart` to `workEnd` (configurable per zone, default 09:00–17:00 local). The overlap calculation is: for N timezones, find the intersection of all working-hour intervals after converting to a common reference (UTC). This is O(N) linear arithmetic — trivial.

3. **Timezone arithmetic**: The JS `Intl.DateTimeFormat` API handles all timezone conversion, including DST transitions, automatically using the OS's IANA timezone database `[Verified — deep-research-report.md, MDN Web Docs]`. No custom timezone math is needed. The `Temporal` API (Stage 3 TC39 proposal, available in some runtimes) would further simplify this but is not required.

4. **DST edge cases**: When the slider crosses a DST transition boundary, working hours shift by 1 hour. `Intl` handles this transparently — the developer renders each timezone's local time for the slider's UTC offset, and DST is baked in.

**Estimated implementation effort**: 2–3 days for a competent frontend developer. The overlay is a visual rendering task, not an algorithmic challenge.

**Verdict**: No technical risk. This is the easiest part of the build.

---

### TC-3: Bundle Size Target (<50MB)

**Severity: low**

The <50MB target is **trivially achievable** with Tauri. Typical Tauri 2 desktop app bundle sizes `[Verified — web search, v2.tauri.app/concept/size]`:

| App Complexity | Bundle Size |
|---------------|-------------|
| Minimal (Hello World) | 0.7–3 MB |
| Starter w/ React/Vue | 2.5–4 MB |
| Medium complexity (plugins, assets) | 4–10 MB |
| Electron equivalent | 80–200 MB |

A world clock app with a React/Solid frontend, system tray, notifications, and store plugins would realistically produce a **3–8 MB** installer on macOS and Linux. Windows with `downloadBootstrapper` adds 1–2 MB; with `offlineInstaller` for WebView2, up to ~135 MB (but this is an edge case for offline Windows deployment).

**Optimization levers** (from Tauri docs) `[Verified — v2.tauri.app/concept/size]`:
- Rust: `lto = true`, `opt-level = "s"`, `strip = true` in `Cargo.toml`
- Tauri 2.4+: `removeUnusedCommands` option to strip unused IPC commands
- Frontend: Tree-shaking, minification, avoiding heavy libraries

**Verdict**: The 50MB target has a 5–10× safety margin. Bundle size is a non-issue.

---

### TC-4: Memory Target (<30MB RAM)

**Severity: medium**

Tauri apps use significantly less RAM than Electron:

| Framework | Typical Idle RAM |
|-----------|-----------------|
| Tauri (simple app) | 30–50 MB |
| Tauri (complex app) | 40–80 MB |
| Electron (simple app) | 150–300 MB |

`[Verified — rustify.rs, openreplay.com, raftlabs.medium.com benchmarks]`

The <30MB target from sketch.md is **aggressive but potentially achievable** for a minimal world clock. The base Tauri runtime + system WebView overhead is approximately 20–30 MB. Adding React/Solid framework overhead pushes idle memory to 30–50 MB typically.

**Mitigations to hit 30MB**:
- Use a lightweight framework (Solid.js or vanilla JS) instead of React
- Minimize DOM nodes (world clock UI is inherently simple)
- Avoid large dependency trees
- Consider Svelte (compiles away framework runtime)

**Verdict**: 30MB is tight but plausible with a lightweight frontend. Recommend targeting <50MB as the public claim and optimizing toward 30MB internally. Revise sketch.md target to <50MB to avoid false precision.

---

### TC-5: JS Timer Throttling When Backgrounded

**Severity: medium**

**Confirmed risk**: `setInterval` timers are paused by WebView engines after ~5–6 minutes when the window is minimized or hidden `[Verified — deep-research-report.md, tauri-apps/tauri discussions]`. This means a minimized world clock would show stale times when the user brings it back.

**Mitigations** (all proven, from deep-research-report.md):

1. **Recalculate on focus**: On `visibilitychange` or `focus` event, recalculate all displayed times from `Date.now()`. Cost: ~0 effort. Eliminates stale display instantly.
2. **Rust-side timer**: Use `tauri::async_runtime::spawn` with `tokio::time::interval` for any time-critical background work (notification scheduling). Rust threads are not throttled by WebView power management.
3. **System tray tooltip update**: Update tray tooltip from Rust side on a longer interval (every 60 seconds) independent of WebView state.

**Verdict**: Manageable with known mitigations. Not a blocker. The recalculate-on-focus pattern is standard practice for all WebView-based clock apps.

---

### TC-6: System Tray / Menu-Bar Presence Across All 3 Platforms

**Severity: medium**

Tauri's `SystemTray` API is stable on all three desktop platforms `[Verified — deep-research-report.md, v2.tauri.app docs]`:

| Platform | Tray Support | Notes |
|----------|-------------|-------|
| macOS | ✅ Native menu bar | Works out of the box. Icon appears in menu bar. Supports tooltips, menus, click events. |
| Windows | ✅ System tray | Works out of the box. Icon in notification area. |
| Linux | ⚠️ Depends on DE | KDE Plasma: ✅. XFCE: ✅. GNOME: Requires AppIndicator extension (see TC-1). |

Additional plugins that enhance tray UX `[Verified — deep-research-report.md]`:
- `@tauri-apps/plugin-positioner`: Position dropdown window relative to tray icon ✅
- `@tauri-apps/plugin-single-instance`: Prevent duplicate instances ✅
- `@tauri-apps/plugin-autostart`: Launch on login ✅

**Verdict**: Tray works on macOS and Windows without caveats. Linux has the GNOME tray extension dependency (documented in TC-1). This does not block MVP but should be documented clearly.

---

### TC-7: Solo Developer Maintaining 3 Platforms

**Severity: medium**

The `tauri-action` GitHub Action (1.5K stars, 222 forks, actively maintained) automates building for all 3 platforms in a single CI workflow `[Verified — github.com/tauri-apps/tauri-action]`. Each platform runs on its native runner (macOS runner, Windows runner, Ubuntu runner).

**CI/CD reality for a solo dev**:
- A single `release.yml` workflow with a matrix strategy (`os: [macos-latest, windows-latest, ubuntu-latest]`) produces all 3 platform binaries on push/tag.
- Cross-compilation from a single OS is **not supported** — Tauri requires native runners per platform because it links against system WebView libraries.
- Build time: ~5–15 minutes per platform on GitHub Actions runners `[Model-sourced]`.

**What breaks between platforms** (from deep-research-report.md and community reports):
1. File paths: `/` vs `\` — mitigated by Rust's `std::path::PathBuf` and Tauri's app data dir APIs
2. Window chrome: macOS has a traffic light (close/minimize/maximize), Windows has its own, Linux varies by DE
3. WebView CSS rendering: Minor differences in font rendering and CSS feature support across WebKit vs WebView2 vs WebKitGTK
4. Code signing: Different per-platform (see Infrastructure Costs)
5. Tray behavior: Different click semantics (left-click vs right-click context menu conventions)

**Verdict**: A solo developer can maintain 3 platforms with Tauri + GitHub Actions. The `tauri-action` does the heavy lifting. Platform-specific bugs will arise but are manageable with a release cadence of every 2–4 weeks.

---

### TC-8: Zone-Aware Notifications

**Severity: low**

Tauri's `@tauri-apps/plugin-notification` is first-party, stable, and supports all desktop platforms `[Verified — deep-research-report.md]`. The notification logic is straightforward:

1. User configures: "Notify me when it's 9:00 AM in Tokyo"
2. Rust backend: Convert target time to system local time using `chrono` + `chrono-tz` crates
3. Schedule: Use `tokio::time::sleep_until` or a simple Rust timer to trigger at the computed instant
4. Fire: Call `app.notification().builder().title("...").body("...").show()`

The `tauri-plugin-schedule-task` community plugin (see TC-9) is **not required** for this. Simple `tokio` timers in the Rust backend are sufficient for MVP because the app runs continuously in the tray.

**Verdict**: No technical risk. Standard Rust async programming.

---

### TC-9: Community Plugin (tauri-plugin-schedule-task) Reliability

**Severity: medium**

`tauri-plugin-schedule-task` by charlesschaefer `[Verified — github.com/charlesschaefer/tauri-plugin-schedule-task]`:
- **Stars**: ~5 (very low adoption) `[Verified — GitHub Discussions #14531]`
- **Purpose**: Uses OS-level task schedulers (Windows Task Scheduler, macOS BackgroundTasks, Android WorkManager)
- **Maintenance**: Active GitHub repo with recent commits
- **Risk**: Low adoption means edge cases are underexplored. A breaking change or abandonment would require migration.

**Recommendation**: **Do NOT use this plugin for MVP.** Instead:
- Use `tokio` timers in Rust for notification scheduling while the app is running (it runs in the tray, so it's always alive)
- If the app is force-quit, notifications don't fire — this is acceptable for MVP
- Revisit OS-level scheduling post-MVP if users request "fire notifications even when app isn't running"

**Verdict**: The plugin is a nice-to-have, not a dependency. MVP can ship without it by keeping the app running in the tray.

---

### TC-10: Offline-First Operation (Rule 4.1)

**Severity: low**

**Delta-T Zaman is inherently offline-first.** Per Rule 4.1, offline constraints are explicitly addressed:

| Capability | Online Required? | Explanation |
|-----------|-----------------|-------------|
| Display current time in multiple zones | ❌ No | System clock + JS `Intl` API use OS-bundled IANA timezone data |
| Time slider with working-hours overlay | ❌ No | Pure frontend rendering, no external data |
| Zone-aware notifications | ❌ No | Computed from system clock, fired via OS notification system |
| Persist user preferences | ❌ No | `@tauri-apps/plugin-store` writes to local filesystem |
| System tray presence | ❌ No | Native OS integration |
| Auto-update | ✅ Yes | Requires internet to check for and download updates |
| Initial timezone search (city lookup) | ❌ No | IANA timezone IDs are a static list (~400 entries) bundled with the app |

Tauri packages all web assets locally into the binary. The app loads from `tauri://localhost` (or equivalent), not from a remote server. CSP policy (`default-src 'self'`) ensures no external network calls `[Verified — deep-research-report.md]`.

**The only network dependency is auto-update**, which is optional and fails gracefully if offline. The core product — time display, slider, overlays, notifications — works entirely offline from first launch.

**Verdict**: Full compliance with Rule 4.1. Offline-first is the default architecture, not a feature to be added.

---

## Licensing Risks

### LR-1: Tauri Framework License

**Severity: low**

Tauri is dual-licensed under **MIT OR Apache-2.0** `[Verified — v2.tauri.app/concept/architecture]`. Both are permissive licenses that allow:
- Commercial use ✅
- Modification ✅
- Distribution ✅
- Private use ✅
- No copyleft obligation ✅

**No viral/copyleft risk.** The Tauri SBOM (Software Bill of Materials) is published and auditable. Tauri 2 underwent an external security audit `[Verified — deep-research-report.md]`.

---

### LR-2: WebView Runtime Licenses

**Severity: low**

| Runtime | Platform | License | Commercial Use |
|---------|----------|---------|---------------|
| **WebKit** | macOS, Linux | LGPL 2.1 (WebKitGTK), Apple's WebKit license | ✅ LGPL allows dynamic linking without copyleft obligations. Apps that *use* WebKit (not modify it) are not derivative works per LGPL §5. Tauri uses system-installed WebKit — does not bundle or modify it. `[Verified — webkit.org/licensing-webkit]` |
| **WebView2** | Windows | Microsoft proprietary (ms-edge-webview2 license) | ✅ Free to redistribute. Apps use the Evergreen Runtime installed on the user's system. No per-unit or royalty fees. Restrictions: cannot reverse-engineer, cannot redistribute WebView2 as a standalone product. `[Verified — learn.microsoft.com/microsoft-edge/webview2/concepts/distribution]` |

**Key clarification on WebKit LGPL**: The LGPL applies to *WebKit the library*, not to apps that run inside WebKit. A Tauri app is a "work that uses the Library" (LGPL §5), not a "work based on the Library." The app's own code is not subject to LGPL. This is the same legal basis Electron apps use (Chromium has BSD license, but principle is identical for LGPL — using, not modifying) `[Verified — webkit.org/licensing-webkit, LGPL 2.1 text]`.

---

### LR-3: First-Party Plugin Licenses

**Severity: low**

All Tauri first-party plugins (`@tauri-apps/plugin-*`) are licensed under MIT OR Apache-2.0, consistent with the core framework `[Verified — github.com/tauri-apps/plugins-workspace]`. Relevant plugins:

| Plugin | License | Risk |
|--------|---------|------|
| plugin-notification | MIT/Apache-2.0 | None |
| plugin-store | MIT/Apache-2.0 | None |
| plugin-single-instance | MIT/Apache-2.0 | None |
| plugin-autostart | MIT/Apache-2.0 | None |
| plugin-positioner | MIT/Apache-2.0 | None |
| plugin-global-shortcut | MIT/Apache-2.0 | None |
| plugin-updater | MIT/Apache-2.0 | None |

---

### LR-4: Frontend Framework and Dependency Licenses

**Severity: low**

Recommended frontend frameworks and their licenses:

| Dependency | License | Risk |
|-----------|---------|------|
| React | MIT | None |
| Solid.js | MIT | None |
| Svelte | MIT | None |
| Vite (build tool) | MIT | None |
| chrono (Rust crate) | MIT/Apache-2.0 | None |
| chrono-tz (Rust crate) | MIT/Apache-2.0 | None |
| tokio (Rust async runtime) | MIT | None |

**No copyleft, GPL, or AGPL dependencies identified** in the planned dependency tree. All core dependencies are MIT or Apache-2.0 `[Model-sourced — based on crates.io and npm registry metadata]`.

**Recommendation**: Run `cargo deny check licenses` and `npx license-checker` before release to verify no transitive dependency introduces copyleft obligations.

---

### LR-5: Distribution Platform Terms

**Severity: low**

| Channel | Terms | Cost |
|---------|-------|------|
| Direct download (website) | No restrictions | Free |
| Mac App Store (optional) | Apple's Developer Program Agreement; 30% revenue share (15% for small businesses <$1M) | $99/yr program fee `[Verified]` |
| Microsoft Store (optional) | Microsoft Store Policies; 15% revenue share (down from 30% for apps, as of 2021) | $19 one-time registration `[Model-sourced]` |
| Flathub/Snap Store (Linux) | Open distribution | Free |

**MVP recommendation**: Distribute via direct download (website) only. App Store distribution is optional and can be added post-launch. This avoids the 15–30% revenue share on a $7.99 product.

---

## Infrastructure Costs

### IC-1: Code Signing Certificates

**Severity: high**

Code signing is **required** for professional distribution on macOS and strongly recommended on Windows.

| Item | Cost | Frequency | Source |
|------|------|-----------|--------|
| **Apple Developer Program** | $99/year | Annual | `[Verified — developer.apple.com/programs/enroll]` |
| **Windows OV Code Signing Certificate** | $129–$226/year | Annual | `[Verified — codesigncert.com, ssl2buy.com, forum.juce.com]` |
| **Linux code signing** | $0 | N/A | Not required for AppImage/deb/RPM distribution |

**macOS signing details**: Without an Apple Developer ID certificate, macOS Gatekeeper will block the app with a "cannot be opened because the developer cannot be verified" warning. Users *can* bypass this (right-click → Open), but it destroys trust and conversion. Notarization (submitting to Apple for malware scan) also requires the $99/yr program. **This is non-negotiable for macOS distribution** `[Verified — deep-research-report.md]`.

**Windows signing details**: Without a code signing certificate, Windows SmartScreen shows "Windows protected your PC" on first run. Users *can* click "More info → Run anyway," but this significantly reduces install completion rates. An OV (Organization Validation) certificate removes the warning after building reputation. An EV (Extended Validation) certificate removes it immediately but costs $300–$500/year `[Model-sourced]`.

**Cheapest viable path**: FastSSL OV certificate at $129/year `[Verified — forum.juce.com]` or Certum Open Source Code Signing (for open-source projects, ~$27/year) `[Model-sourced]`.

**Total signing cost: $228–$325/year** (Apple $99 + Windows OV $129–$226).

---

### IC-2: CI/CD Build Costs (GitHub Actions)

**Severity: low**

GitHub Actions pricing for the free tier (2025) `[Verified — docs.github.com/billing/reference/actions-runner-pricing, github.com/pricing]`:

| Runner OS | Price/Minute | Multiplier vs Free Minutes |
|-----------|-------------|---------------------------|
| Linux | $0.006 | 1× |
| Windows | $0.010 | 2× |
| macOS (M1) | $0.062 | 10× |

**Free tier**: 2,000 minutes/month on the free plan. Public repos get unlimited minutes.

**Estimated usage per release** (build all 3 platforms):
- Linux build: ~5 min = 5 free minutes
- Windows build: ~8 min = 16 free minutes (2× multiplier)
- macOS build: ~10 min = 100 free minutes (10× multiplier)
- **Total per release: ~121 free minutes consumed**

At 2 releases/month: **~242 free minutes/month** — well within the 2,000 free-tier limit. Even at 4 releases/month (484 minutes), the free tier suffices.

**If the repo is public**: Unlimited free minutes, so **$0/year for CI/CD** `[Verified]`.

**If the repo is private**: 2,000 free minutes/month handles ~16 releases/month. Overage at posted rates would cost ~$7.50/release. Unlikely to be needed.

**Verdict**: CI/CD costs are effectively **$0** for an indie project with semi-monthly releases.

---

### IC-3: Hosting and Domain

**Severity: low**

| Item | Cost | Notes |
|------|------|-------|
| Domain name (.com) | $10–$15/year | Via Namecheap, Cloudflare Registrar, NameSilo `[Verified — namecheap.com, namesilo.com]` |
| Landing page hosting | $0/year | GitHub Pages (free for public repos) or Cloudflare Pages (free tier) `[Model-sourced]` |
| Update server (for auto-updater) | $0/year | Can serve update JSON from GitHub Releases (free) or a static S3 bucket ($0.023/GB) `[Model-sourced]` |

**Total hosting: ~$12/year**

---

### IC-4: Total Year 1 Cost Estimate

**Severity: medium**

| Category | Low Estimate | High Estimate |
|----------|-------------|---------------|
| Apple Developer Program | $99 | $99 |
| Windows OV Code Signing | $129 | $226 |
| Domain (.com) | $10 | $15 |
| Landing page hosting | $0 | $0 |
| CI/CD (GitHub Actions) | $0 | $0 |
| Auto-update hosting | $0 | $5 |
| **Total Year 1** | **$238** | **$345** |

**Context**: Against the revenue forecast of $5K–$50K Year 1 at $7.99/unit `[market-map.md]`, the infrastructure cost is **0.5–7% of revenue** at the low end of revenue projections. Even at $5K revenue, this is affordable for a bootstrapped indie project.

**10× scale cost** (10× users, 10× downloads): Infrastructure costs remain essentially flat because there is no server-side compute, no database, no API hosting. The only scaling cost would be bandwidth for the auto-updater, which is negligible when served from GitHub Releases (free, unlimited for public repos). **Year 1 at 10× scale: same $238–$345** `[Model-sourced]`.

**Cold-start scenario**: No server to cold-start. The app is a desktop binary. Landing page is static. There is no backend.

**Burst-traffic scenario**: Not applicable — there is no server to overload. If a Product Hunt launch drives 10K downloads in a day, the bottleneck is GitHub Releases CDN (which handles it, being GitHub's infrastructure) or the landing page host (GitHub Pages/Cloudflare Pages both handle traffic spikes) `[Model-sourced]`.

---

## Data Availability

### DA-1: IANA Timezone Database (tzdata)

**Severity: low**

The IANA Time Zone Database is the authoritative source for timezone definitions worldwide. It is:
- **Free and open**: Published by IANA at iana.org/time-zones, managed per BCP 175 `[Verified — iana.org/time-zones]`
- **Regularly updated**: Multiple releases per year (2025a, 2025b, 2025c released in 2025; 2026a already available) `[Verified — iana.org/time-zones, lists.iana.org/tz-announce]`
- **Pre-bundled with every OS**: macOS, Windows, and Linux all ship their own copy of tzdata. The JS `Intl` API uses the OS's copy. No need to bundle or update independently.
- **Coverage**: ~400 timezone identifiers covering every inhabited region on Earth

**DST data freshness**: When a country changes its DST rules (e.g., a country abolishes DST), the IANA database is updated within weeks. OS vendors (Apple, Microsoft, Linux distros) then push updates. The Tauri app inherits this automatically — no app update needed for timezone data.

**Risk**: If a user's OS has stale tzdata (e.g., unpatched Windows 10), DST calculations for recently-changed regions could be wrong. This is extremely rare and not specific to Delta-T Zaman — every app using `Intl` faces this. **No mitigation needed beyond standard OS update recommendations.**

---

### DA-2: City/Location Search Data

**Severity: low**

Users will search for timezones by city name (e.g., "Tokyo" → "Asia/Tokyo"). Options:

1. **IANA timezone list**: ~400 canonical zone names (e.g., "America/New_York"). These are city-based and cover major cities. Can be bundled as a static JSON file (<50KB) `[Model-sourced]`.
2. **Extended city mapping**: For "search by any city" (like Dato's 15K cities), use a precompiled mapping from city name → IANA zone. The `moment-timezone` npm package includes such data. Or use the GeoNames database (Creative Commons Attribution 4.0, free for commercial use) `[Model-sourced]`.

**MVP recommendation**: Start with the ~400 IANA canonical zones. This covers all major cities. Add extended city search post-MVP if users request it.

**No privacy/GDPR/CCPA implications**: The app does not collect, transmit, or process any user data. Timezone preferences are stored locally via `plugin-store`. No analytics, no accounts, no telemetry. **Full GDPR/CCPA compliance by design** (there is no data to regulate).

---

### DA-3: Localization / i18n Data

**Severity: low**

The JS `Intl` API provides locale-aware date/time formatting (day names, month names, number formatting) for all major locales, powered by ICU data bundled with the OS/WebView `[Verified — MDN Web Docs]`. No external i18n data files needed for date/time display.

For UI string translation (menu items, labels), the `tauri-plugin-i18n` community plugin or a JS i18n library (e.g., `i18next`) can be used. This is a post-MVP concern — English-only is sufficient for launch targeting the remote-worker ICP.

---

## Blockers

All items rated **high** or **critical**:

| # | Blocker | Severity | Category | Proposed Mitigation |
|---|---------|----------|----------|-------------------|
| B-1 | **macOS code signing required** — Without Apple Developer ID ($99/yr), Gatekeeper blocks the app. Users see a scary warning. Conversion rate drops significantly. | **high** | Infrastructure | Purchase Apple Developer Program membership before first macOS release. Budget $99/yr. Non-negotiable for credible macOS distribution. |
| B-2 | **Windows code signing recommended** — Without OV cert ($129–$226/yr), SmartScreen warns users. Less severe than macOS (users can bypass more easily), but hurts trust for a paid app ($7.99). | **high** | Infrastructure | Purchase OV code signing certificate. Budget $129–$226/yr. Can defer to post-launch if budget is tight, but should be in place before charging users. |
| B-3 | **GNOME tray icon dependency** — System tray on GNOME requires AppIndicator extension installed by user. Without it, tray icon is invisible. Affects ~30-40% of desktop Linux users. | **medium** | Technical | Document requirement clearly in Linux install guide. Implement fallback: if tray creation fails, launch in windowed mode instead. Consider docking to panel as alternative UX. |
| B-4 | **WebView2 Runtime on Windows 10** — Not pre-installed on all Windows 10 machines. Requires bootstrapper download or offline installer bundling. Build bug exists with `embedBootstrapper` mode. | **medium** | Technical | Use `downloadBootstrapper` mode (default, works reliably). Document that Windows 10 users need internet during first install. Windows 11 users are unaffected (WebView2 pre-installed). |
| B-5 | **WebKitGTK version fragmentation on Linux** — Different distros ship different WebKitGTK versions (Safari 12–16 equivalent). CSS/JS feature availability varies. | **medium** | Technical | Test on Ubuntu 20.04 (oldest supported LTS) and Ubuntu 24.04 (current LTS). Use CSS features available in Safari 14+ (WebKitGTK 2.28+). Set minimum distro requirements in docs. |
| B-6 | **JS timer throttling when backgrounded** — WebView pauses `setInterval` after ~5–6 minutes when minimized. | **medium** | Technical | Recalculate all times from `Date.now()` on `visibilitychange`/`focus` event. Use Rust-side `tokio` timers for notification scheduling. Both mitigations are well-documented and trivial to implement. |
| B-7 | **RAM target <30MB is aggressive** — Typical Tauri idle is 30–50MB. Achieving <30MB requires very lightweight frontend. | **medium** | Technical | Use Solid.js or Svelte instead of React. Revise public claim to "<50MB RAM" (still dramatically better than Electron's 150–300MB). Target 30MB internally but do not promise it externally. |

**No critical blockers identified.** All high-severity items (B-1, B-2) have straightforward mitigations (purchasing certificates). The total mitigation cost is $228–$325/year, which is viable for an indie app with $5K–$50K revenue potential.

---

## Verification Needed

Citation verification summary:

| Tag | Count | Percentage |
|-----|-------|-----------|
| `[Verified]` | 34 | ~74% |
| `[Model-sourced]` | 12 | ~26% |

Since >50% of citations are `[Verified]`, no verification-needed section is required per gate criteria. However, the following `[Model-sourced]` claims should be confirmed before build:

1. **Electron app install size 80–200MB** — Widely reported but not independently measured for this analysis
2. **Tauri build time ~5–15 minutes per platform on GitHub Actions** — Should be benchmarked with a skeleton Tauri project
3. **Microsoft Store registration fee $19 one-time** — Verify on current Microsoft Partner Center pricing
4. **EV code signing certificate $300–$500/year** — Verify with Sectigo/DigiCert current pricing
5. **Certum Open Source Code Signing ~$27/year** — Verify eligibility and current pricing
6. **Frontend dependency licenses (React, Solid, etc.)** — Run `npx license-checker` on actual dependency tree before release

---

## Quality Score

| Criterion | Score (1-5) | Notes |
|-----------|-------------|-------|
| **Specificity** | 4 | Every constraint cites a source (URL, artifact, or search result). Bundle sizes, RAM benchmarks, CI/CD pricing all reference specific data. Two areas lack primary benchmarks: Tauri build time on GH Actions, and actual RAM usage of a prototype. |
| **Actionability** | 5 | Every blocker has a concrete mitigation with cost estimate. Downstream agents (os.design, speckit.specify) can proceed immediately — the only prerequisite actions are "buy Apple cert" and "buy Windows cert." No ambiguous recommendations. |
| **Non-redundancy** | 5 | This artifact adds: platform-specific technical analysis (GNOME tray, WebView2, WebKitGTK versions), cost estimates, licensing audit, and memory/bundle benchmarks — none of which exist in usp.md or market-map.md. |
| **Evidence quality** | 4 | 74% of citations are `[Verified]` via web search during this run. Pricing data for Apple ($99/yr), Windows OV certs ($129–$226/yr), and GitHub Actions rates are confirmed current as of July 2025. Remaining `[Model-sourced]` items are well-known facts unlikely to be wrong, but flagged for manual verification. |
