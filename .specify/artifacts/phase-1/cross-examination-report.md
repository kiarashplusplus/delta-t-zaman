---
artifact_meta:
  produced_by: "os.cross-examine"
  produced_at: "2026-04-12T06:30:00Z"
  confidence: 0.82
  inputs_used:
    - ".specify/artifacts/sketch.md"
    - ".specify/artifacts/phase-1/market-map.md"
    - ".specify/artifacts/phase-1/icp.md"
    - ".specify/artifacts/phase-1/why-now.md"
    - ".specify/artifacts/phase-1/competitor-matrix.md"
    - ".specify/artifacts/phase-1/ux-teardown.md"
    - ".specify/artifacts/phase-1/steal-differentiate-ignore.md"
    - ".specify/artifacts/phase-1/usp.md"
    - ".specify/artifacts/phase-1/feasibility.md"
    - ".specify/artifacts/phase-1/validate.md"
    - ".specify/artifacts/phase-1/critic-report.md"
    - ".specify/artifacts/phase-1/tradeoff.md"
    - ".specify/artifacts/phase-1/moat.md"
  stale_after: "on_input_change"
  revision: 1
  grade: "draft"
---

# Cross-Examination Report — Delta-T Zaman Corpus

## Summary

| Severity | Count |
|----------|-------|
| **Critical** | 2 |
| **High** | 5 |
| **Medium** | 6 |
| **Low** | 3 |
| **TOTAL** | 16 |

**Overall corpus consistency score: 0.52** (below implementation-ready threshold of 0.80)

The corpus has two systemic breaks:
1. **Platform contradiction** — usp.md, feasibility.md, and validate.md say "mobile-first iOS" while tradeoff.md r3 decided "macOS desktop menu bar"
2. **Undisk ownership gap** — usp.md, feasibility.md, and validate.md treat Undisk as a hostile third-party vendor, while critic.md r3, tradeoff.md r3, and moat.md r3 correctly treat it as an internal tool

Both were introduced by the loop-undo replay process: later artifacts were corrected but earlier artifacts were not cascaded.

---

## Findings

### CRITICAL Findings

#### CX-1: Platform Strategy Contradiction — "mobile-first iOS" vs "macOS desktop menu bar"

| Attribute | Detail |
|-----------|--------|
| **Severity** | CRITICAL |
| **Type** | Broken decision chain |
| **Artifact A** | usp.md r2 (line 46) |
| **Artifact B** | tradeoff.md r3 (line 32) |

**usp.md r2 states:**
> "Delta-T Zaman is the only **mobile-first** temporal governance layer that lets a developer set timezone-gated execution rules, circadian kill-switches, and workspace-level undo for autonomous AI agents — **controlled entirely from a clock interface on their phone.**"

**tradeoff.md r3 states:**
> "Ship a **macOS menu bar app** (Tauri desktop — stable, no App Store risk) that validates exactly one thing: will a developer create a timezone-gated deploy rule for their Undisk workspace? Everything else — **iOS, Android**, Sleep Fence, Undo Slider, clock-as-primary-UI, billing, push notifications, multi-workspace — **is cut** until that question has a data-backed answer."

**Impact:** The USP sentence — the single most important artifact in the corpus — describes a product that the tradeoff explicitly decided NOT to build at MVP. Every downstream artifact that references "mobile-first" or "from their phone" is stale.

**Affected artifacts:** usp.md (lines 34, 46, 145, 150, 161, 171, 203, 204, 212, 296), feasibility.md (line 25), validate.md (lines 141, 358, 363, 445, 472, 489, 514, 599, 621)

**Remediation:** Rewrite the USP sentence to reflect macOS desktop as the MVP platform. The "mobile-first" claim becomes a Phase 2 aspiration, not an MVP truth. The USP's falsifiable core — temporal governance for agents via a clock interface — remains valid on desktop.

---

#### CX-2: Undisk Ownership — "third-party vendor risk" vs "internal tool"

| Attribute | Detail |
|-----------|--------|
| **Severity** | CRITICAL |
| **Type** | Stale reference (post-undo inconsistency) |
| **Artifact A** | feasibility.md r2 (lines 329, 351) |
| **Artifact B** | critic-report.md r3 (F2 section), moat.md r3 |

**feasibility.md r2 states:**
> "R1: **Undisk single-vendor dependency** — Undisk changes API, raises prices, or becomes a competitor" (line 329)
> "A core USP pillar (workspace-level undo) depends entirely on a **single third-party service** with no alternative." (line 351)

**critic-report.md r3 states (corrected):**
> F2 Undisk dependency: ~0% probability. The team IS the Undisk builder.

**moat.md r3 states (corrected):**
> Vertical integration moat — competitors must build their own MCP-compatible versioned workspace from scratch. Undisk ownership is the primary moat.

**Impact:** Three r2 artifacts (usp.md, feasibility.md, validate.md) still frame Undisk as an existential risk, while three r3 artifacts (critic, tradeoff, moat) correctly frame it as the primary competitive advantage. This will cause confusion during implementation — developers may waste effort building Undisk abstraction layers or evaluating alternatives that don't exist.

**Affected artifacts:**
- feasibility.md: lines 169, 329, 351, 367
- usp.md: lines 298-300
- validate.md: lines 76, 565, 597

**Remediation:** Remove all "Undisk dependency risk" framing. Replace with "Undisk vertical integration advantage." Remove "Contact Undisk team" mitigations. Remove "Undisk partnership confirmed" pre-build checklist items.

---

### HIGH Findings

#### CX-3: MVP Scope — "3 journeys" vs "Deploy Gate only"

| Attribute | Detail |
|-----------|--------|
| **Severity** | HIGH |
| **Type** | Scope mismatch |
| **Artifact A** | feasibility.md r2 (lines 129, 143) |
| **Artifact B** | tradeoff.md r3 (line 32) |

**feasibility.md r2 describes three journeys as in-scope:**
- Journey 1: Geopolitical Deploy (line 111)
- Journey 2: Agentic Sleep Fence (line 129)
- Journey 3: Undo Slider (line 143)

**tradeoff.md r3 cuts two of three:**
> "Everything else — iOS, Android, **Sleep Fence, Undo Slider**, clock-as-primary-UI... is cut"

**Impact:** feasibility.md's technical architecture, API mapping, cost estimates, and timeline are all scoped for 3 journeys. The tradeoff cut to 1 journey. The feasibility cost model ($50/mo at 1K users) includes infrastructure for Sleep Fence (Cloudflare Cron Triggers) and Undo Slider (Undisk polling) that won't exist at MVP.

**Affected artifacts:** feasibility.md (entire Journey 2 and 3 sections), validate.md (line 229: landing page describes all 3 journeys, line 443: "Build Journey 2 + 3" as next step)

**Remediation:** Mark Journey 2 and 3 sections in feasibility.md as "Phase 2 — deferred per tradeoff.md r3." Update validate.md landing page to only describe Deploy Gate.

---

#### CX-4: Feasibility Verdict Stale — "YELLOW" may now be GREEN

| Attribute | Detail |
|-----------|--------|
| **Severity** | HIGH |
| **Type** | Stale assessment |
| **Artifact A** | feasibility.md r2 (line 25) |
| **Artifact B** | tradeoff.md r3, critic-report.md r3 |

**feasibility.md states:**
> "Verdict: **YELLOW** — proceed with mitigations for three medium-high risks: **Undisk single-vendor dependency**, unvalidated clock-as-governance UX, and **iOS background execution constraints**."

Two of the three YELLOW drivers are now eliminated:
1. Undisk dependency → eliminated (internal tool)
2. iOS background execution → eliminated (building macOS desktop, not iOS)

Only "clock-as-governance UX" risk remains. The verdict should likely be upgraded to GREEN (or YELLOW with only 1 risk).

**Remediation:** Revise the feasibility verdict section. Eliminate R1 (Undisk) and R4 (iOS background). Re-score overall risk.

---

#### CX-5: Validate.md Pre-Build Checklist — Contains Satisfied/Moot Items

| Attribute | Detail |
|-----------|--------|
| **Severity** | HIGH |
| **Type** | Stale reference |
| **Artifact A** | validate.md r2 (lines 597, 599) |
| **Artifact B** | tradeoff.md r3, critic-report.md r3 |

**validate.md checklist includes:**
> "Undisk partnership confirmed — Direct communication with Undisk team confirming: (a) API stability commitment... (b) no plans to build competing mobile governance UI, (c) willingness to co-market" (line 597)

This is moot — the team IS the Undisk team.

> "Mobile-first preference validated — >=53% of interviewees prefer controlling agent governance from their phone" (line 599)

This gate is no longer relevant for MVP since the tradeoff chose macOS desktop. Mobile preference becomes a Phase 2 validation gate.

**Remediation:** Mark "Undisk partnership confirmed" as PRE-SATISFIED. Move "Mobile-first preference validated" to Phase 2 gates.

---

#### CX-6: Timeline Inconsistency — "10-week MVP" vs "6-8 week build"

| Attribute | Detail |
|-----------|--------|
| **Severity** | HIGH |
| **Type** | Contradiction |
| **Artifact A** | feasibility.md r2 (line 25) |
| **Artifact B** | tradeoff.md r3 (build timeline) |

**feasibility.md says:** "10-week MVP build" (scoped for 3 journeys, iOS + macOS)
**tradeoff.md says:** "6-8 week build" (scoped for 1 journey, macOS only, ~15-20 hrs/week)

The 6-8 week figure is more current and reflects the reduced scope. The 10-week figure is from the pre-tradeoff era.

**Remediation:** Update feasibility.md timeline to reflect tradeoff scope cuts. Note that 10-week was for 3-journey scope; 6-8 weeks is for Deploy Gate only.

---

#### CX-7: ICP Pain Points Reference Mobile — But MVP Is Desktop

| Attribute | Detail |
|-----------|--------|
| **Severity** | HIGH |
| **Type** | Scope mismatch |
| **Artifact A** | icp.md r2 |
| **Artifact B** | tradeoff.md r3 |

icp.md pain point #5: "There's no mobile control plane for my dev infrastructure." This pain point directly motivated the mobile-first USP. With macOS desktop as the MVP, this pain point is no longer the primary driver.

**Remediation:** icp.md does not need rewriting (the pain point is real), but usp.md and validate.md should stop citing ICP Pain #5 as the primary validation target. The MVP validates Pain #1 (timezone governance) and Pain #2 (agent safety overnight), not Pain #5 (mobile control).

---

### MEDIUM Findings

#### CX-8: Pricing References Inconsistent

| Attribute | Detail |
|-----------|--------|
| **Severity** | MEDIUM |
| **Type** | Minor inconsistency |

validate.md references $19/mo pricing. tradeoff.md says pricing is CUT from MVP (feature F22, scored 4.25 = conditional). Need alignment on whether MVP charges or is free-to-use.

**Remediation:** Clarify: MVP is likely free (macOS menu bar app, no billing). $19/mo is Phase 2 pricing validation.

---

#### CX-9: Competitor Matrix References Mobile Gaps — Now Less Relevant

| Attribute | Detail |
|-----------|--------|
| **Severity** | MEDIUM |
| **Type** | Emphasis mismatch |

competitor-matrix.md identifies "no mobile control interface" as a key market gap. With macOS desktop MVP, this gap is a future opportunity, not a current differentiator.

**Remediation:** No text changes needed in competitor-matrix.md, but any artifact citing this gap as current MVP differentiation should be revised.

---

#### CX-10: steal-differentiate-ignore.md S2 "Menu Bar Widget" — Now Core, Not Steal

| Attribute | Detail |
|-----------|--------|
| **Severity** | MEDIUM |
| **Type** | Reclassification needed |

S2 in steal-differentiate-ignore.md classified "persistent desktop menu bar widget" as a Steal feature. tradeoff.md r3 made it THE core product form factor.

**Remediation:** Note in revision that S2 was promoted from Steal to Core per tradeoff.md r3.

---

#### CX-11: feasibility.md Cost Model Over-Scoped

| Attribute | Detail |
|-----------|--------|
| **Severity** | MEDIUM |
| **Type** | Stale data |

Cost model includes Cloudflare Cron Triggers (for Sleep Fence) and D1 writes (for Undo Slider polling). MVP only needs KV reads/writes for Deploy Gate.

**Remediation:** Add a note that cost model reflects full 3-journey scope. MVP costs will be lower.

---

#### CX-12: feasibility.md Architecture Diagram Over-Scoped

| Attribute | Detail |
|-----------|--------|
| **Severity** | MEDIUM |
| **Type** | Stale diagram |

The ASCII architecture diagram (line 185-186) includes "Sleep Fence" and "Undo Slider" boxes in the iOS app section. MVP architecture is macOS menu bar -> Cloudflare Worker -> Undisk (Deploy Gate only).

**Remediation:** Add note that diagram reflects full vision. MVP subset is F1+F2+F4+F5+F6+F7+F9 per tradeoff.md.

---

#### CX-13: validate.md Landing Page Describes All 3 Journeys

| Attribute | Detail |
|-----------|--------|
| **Severity** | MEDIUM |
| **Type** | Scope mismatch |

validate.md landing page wireframe (line 229) describes "3 feature blocks mapping to Journeys 1-3: Deploy Gate, Sleep Fence, Undo Slider." The landing page should only describe what is being built.

**Remediation:** Revise landing page section to focus on Deploy Gate. Mention Sleep Fence and Undo Slider as "coming soon" if at all.

---

### LOW Findings

#### CX-14: "Tauri 2.10.x (mobile verified)" in feasibility.md

| Attribute | Detail |
|-----------|--------|
| **Severity** | LOW |
| **Type** | Emphasis mismatch |

feasibility.md emphasizes Tauri mobile verification. With macOS desktop MVP, Tauri desktop stability is more relevant.

**Remediation:** Minor — no text change needed, but speckit.specify should reference Tauri desktop, not mobile.

---

#### CX-15: sketch.md Predates All Pivots

| Attribute | Detail |
|-----------|--------|
| **Severity** | LOW |
| **Type** | Stale but harmless |

sketch.md (confidence 0.30) describes the original world clock concept before the temporal governance pivot. It is a historical artifact.

**Remediation:** No action needed. sketch.md is hypothesis-grade by design.

---

#### CX-16: Market Map "Why Now" Timing Claims

| Attribute | Detail |
|-----------|--------|
| **Severity** | LOW |
| **Type** | Minor staleness |

why-now.md references "MCP specification December 2024" as recent. This is still accurate but the MCP ecosystem has evolved further. No correction needed for MVP scope.

**Remediation:** No action needed for MVP.

---

## Decision Chain Audit

### 1. Platform Decision

| Artifact | Says | Status |
|----------|------|--------|
| sketch.md | Cross-platform (Tauri) | Warning — Vague, predates pivot |
| icp.md | "iPhone primary... macOS for development" | Warning — Describes ICP, not product decision |
| usp.md r2 | **"mobile-first... from their phone"** | STALE — contradicts tradeoff |
| feasibility.md r2 | iOS + macOS, Tauri mobile | STALE — scope reduced to macOS |
| validate.md r2 | Mobile-first validation gates | STALE — MVP is desktop |
| critic-report.md r3 | HG3: "mobile preference validation" | Warning — Should be Phase 2 gate |
| **tradeoff.md r3** | **macOS desktop menu bar** | AUTHORITATIVE |
| moat.md r3 | Platform-agnostic (moat is Undisk ownership) | Consistent |

### 2. Undisk Relationship

| Artifact | Says | Status |
|----------|------|--------|
| usp.md r2 | "Undisk Dependency (Impact: Medium-High)" | STALE |
| feasibility.md r2 | "single-vendor dependency... third-party service" | STALE |
| validate.md r2 | "Undisk partnership confirmed" checklist | STALE (moot) |
| **critic-report.md r3** | **F2: ~0% probability. Internal tool.** | AUTHORITATIVE |
| **tradeoff.md r3** | **Undisk as internal capability** | AUTHORITATIVE |
| **moat.md r3** | **Vertical integration moat** | AUTHORITATIVE |

### 3. MVP Scope

| Artifact | Says | Status |
|----------|------|--------|
| feasibility.md r2 | 3 journeys (Deploy Gate + Sleep Fence + Undo Slider) | STALE |
| validate.md r2 | Landing page describes all 3 journeys | STALE |
| **tradeoff.md r3** | **Deploy Gate only. Sleep Fence + Undo Slider CUT.** | AUTHORITATIVE |
| critic-report.md r3 | References all 3 journeys in contradiction table | Warning — Partially stale |

### 4. Target User

| Artifact | Says | Status |
|----------|------|--------|
| icp.md r2 | Solo developer, 2-5 person team, AI-native | Consistent across all |
| market-map.md r2 | Agent-heavy developers | Consistent |
| tradeoff.md r3 | Solo developer MVP | Consistent |

### 5. Pricing

| Artifact | Says | Status |
|----------|------|--------|
| validate.md r2 | $19/mo | Warning — Pricing CUT from MVP per tradeoff |
| feasibility.md r2 | $19/mo, break-even at 85 customers | Warning — Phase 2 pricing |
| tradeoff.md r3 | Pricing validation is CONDITIONAL (F22) | AUTHORITATIVE |

### 6. Timeline

| Artifact | Says | Status |
|----------|------|--------|
| feasibility.md r2 | 10-week MVP | STALE (pre-scope-cut) |
| **tradeoff.md r3** | **6-8 weeks at 15-20 hrs/week** | AUTHORITATIVE |

---

## Recommended Revision Order

Artifacts need revision in dependency order. The r3 artifacts (critic, tradeoff, moat) are clean — they were replayed with correct context. The r2 artifacts need updating:

| Priority | Artifact | Findings to Fix | Key Changes |
|----------|----------|----------------|-------------|
| 1 | **usp.md** | CX-1, CX-2, CX-7 | Rewrite USP sentence for macOS desktop. Remove Undisk dependency risk. Reframe mobile as Phase 2. |
| 2 | **feasibility.md** | CX-2, CX-3, CX-4, CX-6, CX-11, CX-12 | Remove Undisk vendor risk. Mark Journey 2+3 as deferred. Update verdict to GREEN/YELLOW-1-risk. Fix timeline to 6-8 weeks. |
| 3 | **validate.md** | CX-1, CX-2, CX-5, CX-13 | Remove "Undisk partnership" checklist item. Move mobile preference to Phase 2. Revise landing page to Deploy Gate only. Update go/no-go criteria. |
| 4 | **steal-differentiate-ignore.md** | CX-10 | Note S2 promotion from Steal to Core. |
| 5 | **icp.md** | CX-7 (minor) | No text changes needed — pain points are real. Add note that MVP validates Pain #1/#2, not #5. |

**Estimated revision effort:** ~2-3 hours of focused editing across 3 primary artifacts.

---

## Quality Score

| Dimension | Score (1-5) | Notes |
|-----------|-------------|-------|
| **Specificity** | 5 | Every finding quotes exact line numbers and text from both contradicting artifacts |
| **Actionability** | 5 | Each finding has a concrete remediation. Revision order is dependency-aware. |
| **Non-redundancy** | 4 | CX-1 and CX-7 overlap slightly (both about mobile to desktop). Kept separate because one is about the USP sentence and the other about ICP pain mapping. |
| **Evidence quality** | 5 | All findings cite specific artifact text with line numbers. No model-sourced claims. |
