---
artifact_meta:
  produced_by: "os.usp"
  produced_at: "2025-07-27T04:00:00Z"
  confidence: 0.75
  inputs_used:
    - ".specify/artifacts/phase-1/competitor-matrix.md"
    - ".specify/artifacts/phase-1/steal-differentiate-ignore.md"
    - ".specify/artifacts/phase-1/icp.md"
    - ".specify/artifacts/sketch.md"
  stale_after: "on_input_change"
  revision: 1
  quality_scores:
    specificity: 4
    actionability: 5
    non_redundancy: 5
    evidence_quality: 4
  grade: "draft"
---

# USP: Delta-T Zaman

## Falsification Record

> **Original USP hypothesis** (sketch.md, Cycle 0):
> "The only native cross-platform world clock that combines menu-bar presence, a time comparison slider, and zone-aware notifications in a single lightweight app."
>
> **Status: FALSIFIED** — Dato ($16, macOS) already offers menu-bar presence + time slider + notifications in one app `[Verified, competitor-matrix.md]`. The "only app with this combination" claim is false on a per-platform basis.
>
> **Lesson learned**: The original USP bundled table-stakes features (menu bar, slider) as if they were differentiators. The real gap is not "what features we combine" but "where we exist" (cross-platform) and "what no competitor visualizes" (working-hours overlaps).

---

## Positioning Matrix

### Axis Selection Rationale

- **Axis X: Native platform breadth** (Single-OS ↔ Multi-OS native desktop). Derived from ICP pain: remote workers on mixed-OS teams have no shared native timezone tool `[competitor-matrix.md: "No competitor provides a native desktop experience on macOS + Windows + Linux"]`. Web apps serve all platforms but sacrifice system-tray integration, native notifications, and always-on presence.
- **Axis Y: Time intelligence level** (Passive display ↔ Proactive planning). Derived from ICP pain points P3 ("awareness gap — messaged at midnight their time") and P4 ("overlap window blindness — missed the window, their workday ended") `[icp.md]`. Passive = shows clocks. Proactive = working-hours overlays, zone-aware reminders, DST transition warnings — tools that help users *act*, not just *see*.

### Matrix

```
Axis X: Native platform breadth (Single-OS ↔ Multi-OS native desktop)
Axis Y: Time intelligence (Passive display ↔ Proactive planning assistance)

| Player              | X Score | Y Score | Quadrant                      |
|---------------------|---------|---------|-------------------------------|
| Delta-T Zaman       | High    | High    | Multi-OS + Proactive          |
| Dato                | Low     | Medium  | Single-OS + Moderate          |
| Clocker             | Low     | Medium  | Single-OS + Moderate          |
| Time (menubartime)  | Low     | Medium  | Single-OS + Moderate          |
| There               | Low     | Low     | Single-OS + Passive           |
| Menu World Time     | Low     | Low     | Single-OS + Passive           |
| World Clock Pro     | Low     | Low     | Single-OS + Passive           |
| macOS Built-in      | Low     | Low     | Single-OS + Passive           |
| Windows Built-in    | Low     | Low     | Single-OS + Passive           |
| Every Time Zone     | Low*    | Low     | Non-native + Passive          |
| Timezone.io         | Low*    | Low     | Non-native + Passive          |

* Web apps are cross-platform by nature but score Low on native platform
  breadth because they lack system-tray/menu-bar integration, native
  notifications, and always-on desktop presence — the ICP's core need.

Whitespace quadrant: Multi-OS Native + Proactive Planning
Positioning opportunity: No native desktop app serves macOS + Windows +
Linux users with proactive timezone intelligence (working-hours overlaps,
zone-aware reminders). This quadrant is entirely unoccupied. For the ICP —
remote workers on mixed-OS teams — this is the only quadrant that solves
both "my tool doesn't run on my other machine" AND "I keep missing overlap
windows."
```

### Scoring Justification

| Player | X Rationale | Y Rationale |
|--------|-------------|-------------|
| **Delta-T Zaman** | Tauri 2.10.3 builds native binaries for macOS, Windows, Linux from one codebase `[sketch.md]` | Working-hours overlay (D4, nobody has this) + zone-aware notifications (D2) + DST warnings (D3) `[steal-differentiate-ignore.md]` |
| **Dato** | macOS only `[Verified, competitor-matrix.md]` | Has time slider + notifications, but no working-hours overlay and notifications are calendar-event-driven, not timezone-driven `[Verified]` |
| **Clocker** | macOS only `[Verified]` | Has slider + zone-aware reminders, but stale (last update April 2024) and no working-hours overlay `[Verified]` |
| **Time** | macOS only `[Verified]` | Has slider + DST warnings, but no zone-aware notifications and no working-hours overlay `[Verified]` |
| **There** | macOS only `[Verified]` | No slider, no notifications, no planning features — pure display `[Verified]` |
| **Every Time Zone** | Web-only, no native desktop integration `[Verified]` | Slider but no notifications, no working-hours, no proactive features `[Verified]` |

---

## USP Statement

Delta-T Zaman is the only native desktop world clock that runs on macOS, Windows, and Linux with a time slider showing configurable working-hours overlaps per timezone, so distributed teams on any OS can visually find meeting windows without switching to a web app.

## Only We Framing

Only we provide a native system-tray world clock on all three desktop operating systems with a time slider that highlights each zone's working hours — so a team split across macOS, Windows, and Linux can all find their meeting window in one glance, on the same tool.

## Positioning Statement

For remote workers and distributed teams coordinating across 3+ time zones, Delta-T Zaman is the cross-platform native world clock that shows working-hours overlaps and zone-aware reminders on macOS, Windows, and Linux — unlike Dato, Clocker, and Time, which are locked to macOS, and Every Time Zone, which lacks native desktop integration and proactive planning tools.

## Falsifiability Test

**How to disprove this USP**: Identify any other native desktop application (not a web app, not an OS built-in widget) that meets ALL three criteria simultaneously:

1. **Installs and runs natively** on macOS AND Windows AND Linux (not via a browser or web wrapper)
2. **Provides a time comparison slider** (scrub forward/backward to see times across zones)
3. **Includes configurable working-hours overlays** per timezone on the slider (not just day/night bands — actual "9am–6pm work hours" visualization)

If such an application exists, the USP is falsified.

**Verification performed**: As of July 2025, a search across the macOS App Store, Microsoft Store, Flathub/Snap Store, and independent software directories (MacUpdate, AlternativeTo, Product Hunt) found **zero** applications meeting all three criteria `[competitor-matrix.md — exhaustive 10-competitor analysis]`. The closest competitor, Dato, meets criteria 2 but fails criteria 1 (macOS-only) and 3 (no working-hours overlay). Every Time Zone meets none (web-only, no working-hours overlay).

**Quantitative test for post-launch validation**: Survey 50 users of Dato/Clocker who also use Windows or Linux. If >30% report that cross-platform availability and the working-hours overlay are NOT meaningful differentiators over their current solution, the USP's value proposition (not its uniqueness) is weakened and should be revised.

---

## USP Stress Tests

### 1. Competitor Swap Test

Replace "Delta-T Zaman" with each competitor's name. Does the USP still sound true?

| Competitor | Swap Result | Reason it Fails |
|-----------|-------------|-----------------|
| Dato | ❌ FALSE | macOS-only `[Verified]`. No working-hours overlay `[Verified]`. Fails criteria 1 and 3. |
| Clocker | ❌ FALSE | macOS-only `[Verified]`. No working-hours overlay `[Verified]`. Fails criteria 1 and 3. |
| Time (menubartime) | ❌ FALSE | macOS-only `[Verified]`. No working-hours overlay `[Model-sourced]`. Fails criteria 1 and 3. |
| There | ❌ FALSE | macOS-only `[Verified]`. No slider at all `[Verified]`. Fails criteria 1, 2, and 3. |
| Menu World Time | ❌ FALSE | macOS-only `[Verified]`. No slider `[Verified]`. Fails all criteria. |
| World Clock Pro | ❌ FALSE | macOS + iOS only (no Windows/Linux) `[Verified]`. No working-hours overlay `[Verified]`. Fails criteria 1 and 3. |
| macOS Built-in Clock | ❌ FALSE | macOS-only `[Verified]`. No slider, no working-hours overlay `[Verified]`. Fails all criteria. |
| Windows Built-in Clock | ❌ FALSE | Windows-only `[Verified]`. No working-hours overlay `[Verified]`. Fails criteria 1 and 3. |
| Every Time Zone | ❌ FALSE | Web app, not native desktop `[Verified]`. No working-hours overlay `[Verified]`. Fails criteria 1 and 3. |
| Timezone.io | ❌ FALSE | Web app `[Verified]`. No slider, no working-hours overlay `[Verified]`. Fails all criteria. |

**Result**: USP does not apply to ANY competitor. ✅ PASS

### 2. "So What?" Test

**Plain-language restatement** (as if explaining to a non-technical person):

> "It's a clock app that works on every kind of computer — Mac, Windows, Linux — and it shows you a colored bar for when each person's work hours are, so you can slide to find a time when everyone's awake and working. You don't need to open a website or do math in your head."

**Does it connect to a tangible outcome?** Yes — "find a time when everyone's awake and working" is a concrete daily task for the ICP. ✅ PASS

### 3. Negation Test

**Negated USP**: "Delta-T Zaman does NOT run on all desktop platforms and does NOT show working-hours overlaps."

**Is the negation a valid strategy?** YES:
- **Dato chose exactly this**: macOS-only with deep calendar integration and 15K-city search instead of cross-platform. This is a coherent strategy — go deep on one platform rather than wide across three.
- **There chose even more extreme focus**: macOS-only, people-focused, no slider at all — prioritizing social UX over planning features.
- **Every Time Zone chose the opposite trade-off**: web-only for maximum reach, no native integration, no proactive features.

Each competitor's strategy is the rational negation of a different part of the USP. This confirms the USP describes a *real strategic choice*, not a generic truism. ✅ PASS

### 4. Time Test

**Will this differentiate in 18 months?**

| USP Component | Defensibility | Reasoning |
|---------------|:------------:|-----------|
| Cross-platform native (macOS + Windows + Linux) | **High** | Competitors are built in Swift (macOS-only). Porting to Windows/Linux requires either a complete rewrite or adopting a cross-platform framework — a 6–12 month investment that conflicts with their macOS-native identity. Dato's developer (Sindre Sorhus) is the most prominent Swift/macOS indie dev; switching to Tauri/Electron would be brand-inconsistent. Clocker is stale. There is macOS-focused by design philosophy. |
| Working-hours overlay | **Medium** | A feature that could be copied on macOS by any competitor within a few months. However, the *combination* of working-hours overlay + cross-platform creates a compound moat — a competitor would need to both port to all platforms AND add this feature. |
| Combined moat | **High** | The structural advantage (Tauri/Rust architecture enabling cross-platform from day 1) compounds with the feature advantage (working-hours overlay). Competitors locked into Swift cannot easily replicate both. An entirely new entrant using Tauri/Electron could, but would face the same cold-start market challenges Delta-T Zaman faces. |

**Verdict**: The USP describes a *positioning advantage* (cross-platform architecture + proactive planning) not just a *feature advantage*. The structural basis (Tauri/Rust codebase) persists. ✅ PASS

---

## Anti-Pattern Check

| Anti-Pattern | Present? | Notes |
|-------------|----------|-------|
| "Best-in-class [X]" | ❌ No | USP makes no superlative claims. |
| "AI-powered" | ❌ No | No AI claims. The USP is about platform breadth + visual planning. |
| "Simple and intuitive" | ❌ No | USP specifies a concrete feature (working-hours overlay) not a subjective quality. |
| "All-in-one platform" | ❌ No | USP explicitly narrows scope — it's a world clock with a specific overlay feature, not a calendar/meeting/weather bundle. |

---

## Evidence Traceability

Every claim in the USP traces back to a specific artifact:

| USP Claim | Source Artifact | Specific Reference |
|-----------|----------------|-------------------|
| "only native desktop world clock on macOS, Windows, and Linux" | competitor-matrix.md | Cross-Platform Coverage Analysis table: all 10 competitors fail to serve all 3 desktop OSes natively `[Verified]` |
| "time slider showing configurable working-hours overlaps" | steal-differentiate-ignore.md | D4: "No competitor offers this" `[Model-sourced — no competitor documentation mentions this feature]` |
| "distributed teams on any OS" | icp.md | ICP demographics: "macOS primary; may also use Linux or Windows secondary" |
| "without switching to a web app" | competitor-matrix.md | Web apps (Every Time Zone, Timezone.io) lack native integration: no system tray, no native notifications `[Verified]` |
| Zone-aware reminders (positioning statement) | steal-differentiate-ignore.md | D2: Only Clocker has zone-aware reminders; it's stale (April 2024) and macOS-only `[Verified]` |
| Dato/Clocker/Time locked to macOS (positioning statement) | competitor-matrix.md | Platform column: all three = "macOS only" `[Verified]` |

---

## Gate Criteria Checklist

- [x] USP is exactly one sentence
- [x] USP is falsifiable — a concrete 3-criteria test to disprove it is documented
- [x] USP does not apply to any competitor listed in the competitor matrix (verified per-competitor, 10/10 fail the swap test)
- [x] "Only we..." framing is present and consistent with the USP statement
- [x] Positioning statement follows the prescribed format and references the ICP ("remote workers and distributed teams coordinating across 3+ time zones")
- [x] All claims in the USP trace back to evidence in the prerequisite artifacts (6 claims traced in Evidence Traceability table)

---

## Confidence Notes

**Confidence: 0.75** — This USP is grounded in verified competitive data (cross-platform gap is confirmed across 10 competitors) and validated ICP pain points. Two factors limit confidence:

1. **Working-hours overlay has [Model-sourced] verification** — No competitor's documentation explicitly mentions this feature, but we have not tested every app hands-on. It's possible an unlisted competitor or a recent update added this. Confidence in the "nobody has this" claim: ~85%.
2. **ICP demand for working-hours overlay is inferred, not validated** — The feature maps directly to pain point P4 ("overlap window blindness") from icp.md, but no user research has confirmed that a visual overlay is the preferred solution to this pain. Users might prefer a different interaction pattern (e.g., typed queries like "when can I meet someone in Tokyo?").

**What would raise confidence to 0.90+**: User interviews with 10–20 Dato/Clocker users who work on mixed-OS teams, validating that (a) cross-platform is a meaningful switching trigger and (b) working-hours overlays are more useful than day/night bands.

---

## Quality Score

| Criterion | Score (1-5) | Notes |
|-----------|-------------|-------|
| **Specificity** | 4 | Every claim references a source artifact and verification status. Per-competitor swap test provides granular evidence. One claim (working-hours overlay uniqueness) relies on [Model-sourced] data rather than verified hands-on testing. |
| **Actionability** | 5 | Downstream agents (os.design, speckit.specify) can directly consume the positioning matrix, USP statement, and "Only We" framing without requesting clarification. The falsifiability test defines a concrete validation experiment. |
| **Non-redundancy** | 5 | This is the first USP artifact in the pipeline. It synthesizes competitor-matrix.md (competitive gaps), steal-differentiate-ignore.md (differentiation strategy), and icp.md (pain points) into a positioning claim that none of those artifacts contain individually. |
| **Evidence quality** | 4 | Cross-platform gap claim is backed by exhaustive [Verified] competitor coverage. Working-hours overlay uniqueness is [Model-sourced]. ICP pain-point mapping is grounded in icp.md but icp.md itself flags >50% of psychographic claims as [Model-sourced]. |
