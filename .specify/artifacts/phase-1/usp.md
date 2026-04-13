---
artifact_meta:
  produced_by: "os.usp"
  produced_at: "2026-04-12T04:45:00Z"
  confidence: 0.85
  inputs_used:
    - ".specify/artifacts/phase-1/competitor-matrix.md"
    - ".specify/artifacts/phase-1/steal-differentiate-ignore.md"
    - ".specify/artifacts/phase-1/icp.md"
  stale_after: "on_input_change"
  revision: 3
  quality_scores:
    specificity: 4
    actionability: 4
    non_redundancy: 5
    evidence_quality: 4
  grade: "draft"
---

# Unique Selling Proposition — Delta-T Zaman: The Temporal Command Center

## Previous USP (Falsified)

### Old USP

> "The only native desktop world clock on macOS + Windows + Linux with a time slider and working-hours overlay."

### Why It No Longer Applies

The old USP positioned Delta-T as a **timezone utility** — a better version of Every Time Zone or Dato. Three structural shifts invalidate it:

1. **Scope expansion.** The product evolved from a desktop clock to a three-layer architecture: iOS mobile app + Cloudflare KV/D1 edge state + Undisk MCP agent governance. The old USP describes only the display layer and ignores the governance and infrastructure layers entirely. [Source — steal-differentiate-ignore.md: 7 differentiation features are all governance/infrastructure, not display]

2. **Platform shift.** The old USP claims "native desktop" as the value. The repositioned product uses a macOS menu bar app as the primary control plane, with mobile (iOS/Android) deferred to Phase 2 per tradeoff.md r3. The menu bar widget — previously classified as Steal (S2) — has been promoted to Core. [Source — tradeoff.md r3: "Ship a macOS menu bar app"; steal-differentiate-ignore.md S2 reclassified to Core]

3. **Category mismatch.** "World clock with time slider" competes in the consumer timezone utility category (Dato $14, World Time Buddy freemium, Every Time Zone free). The repositioned product competes in **developer agent governance** — an unoccupied category with no direct competitor. Claiming a timezone USP surrenders the category-creation opportunity. [Source — competitor-matrix.md: "No existing tool lets a developer set timezone-gated rules for autonomous AI agents from a mobile interface"]

4. **Falsifiability failure.** The old USP was already falsified: World Time Buddy has a mobile time slider; Dato has a macOS clock with overlays; Every Time Zone has the canonical web slider. The "only" claim was fragile because it relied on platform availability (native desktop), which any competitor could replicate.

**Verdict:** The old USP describes a feature (clock + slider) that is now classified as **Steal** (S1, S2 in steal-differentiate-ignore.md). A USP cannot be a Steal feature — by definition, it must be something competitors cannot or do not offer.

---

## Revised USP

> **Delta-T Zaman is the only temporal governance layer that lets a developer set timezone-gated execution rules for autonomous AI agents — controlled from a persistent macOS menu bar clock, enforced at the edge via Cloudflare, and executed through the team's own Undisk MCP workspace.**
>
> *Phase 2 expansion: mobile control plane (iOS/Android), Sleep Fence (circadian kill-switches), and Undo Slider (workspace-level time-travel). [Source — tradeoff.md r3: "Everything else is cut until the core deploy-gate hypothesis has a data-backed answer."]*

---

## Falsifiability Criteria

The USP is valid only as long as all three of the following remain true. If any criterion is disproven, the USP must be revised.

### Criterion 1: No competitor offers timezone-gated governance for AI agents from a persistent desktop interface

**Current status: TRUE** [Source — competitor-matrix.md: comprehensive review of 15 competitors; no existing tool lets a developer set timezone-gated rules for autonomous AI agents from a dedicated desktop or mobile interface.]

**Would be falsified if:** Temporal.io, Inngest, or any orchestration platform ships a dedicated desktop/mobile app with constraint-window semantics (not just cron triggers) applied to MCP-connected agents.

### Criterion 2: No competitor combines temporal undo (workspace-level rollback) with a temporal governance UI

**Current status: TRUE** [Source — competitor-matrix.md: Temporal.io has workflow replay but not workspace undo; Undisk has workspace undo but no mobile UI; Git has per-commit revert but no temporal scrubbing]

**Would be falsified if:** Undisk ships its own mobile app with a visual timeline scrubber and checkpoint restore, or if Temporal.io adds workspace-state rollback (not just workflow replay) with a mobile interface.

### Criterion 3: No competitor offers an automatic circadian pause/resume for agent execution tied to the developer's local timezone

**Current status: TRUE** [Source — competitor-matrix.md: Temporal.io's pause/resume is imperative (requires sending a signal each time); steal-differentiate-ignore.md D4: "No competitor offers an automatic, recurring pause/resume tied to local time"]

**Would be falsified if:** Any agent orchestration platform ships a declarative "sleep schedule" feature that automatically pauses and resumes agent activity based on IANA timezone windows without manual signal sending.

---

## Positioning Matrix

### Axes

- **X-axis: Temporal Intelligence Level** — How deeply the product understands and acts on time.
  - Left (Passive Display): Shows times, converts timezones.
  - Right (Active Governance): Uses time as a constraint/rule engine for system behavior.

- **Y-axis: Agent Infrastructure Depth** — How deeply the product connects to AI agent execution.
  - Bottom (None): No connection to developer infrastructure or agents.
  - Top (Full Orchestration): Deep integration with agent runtimes, MCP, workspaces.

### Plot

```
  Agent Infrastructure Depth
  (Full Orchestration)
  │
  │  Temporal.io ◆         ╔═══════════════════╗
  │                         ║  DELTA-T ZAMAN ★  ║
  │  Inngest ◆              ║ (Temporal Command  ║
  │                         ║     Center)        ║
  │  Trigger.dev ◆          ╚═══════════════════╝
  │
  │  Windmill ◆
  │                               n8n ◆
  │  Modal ◆    E2B ◆
  │
  │  Replit Agent ◆   Retool Workflows ◆
  │
  │───────────────────────────────────────────────
  │
  │  Timezone.io ◇
  │
  │  There ◇
  │
  │  Dato ◇    World Time Buddy ◇
  │
  │  Every Time Zone ◇
  │
  └──────────────────────────────────────────────→
  Passive Display                  Active Governance
               Temporal Intelligence Level

  ◆ = Orchestration / Infrastructure competitor
  ◇ = Timezone utility competitor
  ★ = Delta-T Zaman
```

### Quadrant Analysis

| Quadrant | Description | Occupants |
|---|---|---|
| **Top-Left** (High Agent Depth, Passive Time) | Agent orchestration with basic cron scheduling — time is a trigger, not a governance primitive | Temporal.io, Inngest, Trigger.dev, Windmill |
| **Top-Right** (High Agent Depth, Active Governance) | **Temporal governance layer for agents — time as a constraint engine** | **Delta-T Zaman (sole occupant)** |
| **Bottom-Left** (No Agent Depth, Passive Time) | Consumer timezone utilities — display only, no infrastructure connection | Dato, There, Every Time Zone, World Time Buddy, Timezone.io |
| **Bottom-Right** (No Agent Depth, Active Governance) | Empty — active governance without agent infrastructure has no product rationale | (empty) |
| **Mid-Left** (Moderate Agent Depth, Passive Time) | Compute/runtime platforms — agent execution without temporal intelligence | Modal, E2B, Replit Agent, Retool Workflows, n8n |

**Key insight:** Delta-T is the only product in the top-right quadrant. Orchestration tools have agent depth but treat time passively (cron triggers). Timezone tools understand time visually but have zero agent infrastructure. Delta-T assembles both dimensions. [Source — competitor-matrix.md: feature comparison tables across all three dimensions]

---

## Competitive Evidence

### Evidence Table

Every claim in the USP is grounded in upstream artifact data:

| USP Claim | Supporting Evidence | Source |
|---|---|---|
| "only temporal governance layer" | Zero orchestration competitors (Temporal, Inngest, Trigger.dev, Windmill, n8n, Retool) offer dedicated temporal governance UIs. Linggen (P2P agent phone access, 2026) lacks governance. | competitor-matrix.md: Temporal governance interface row — all ❌ |
| "temporal governance layer" | No competitor applies timezone rules as execution constraints (constraint-window semantics). All use point-in-time cron triggers. | steal-differentiate-ignore.md D1: "Transform scheduling from a trigger into a constraint" |
| "timezone-gated execution rules" | Temporal/Inngest support IANA TZ strings in cron, but this schedules starts, not constrains windows. | competitor-matrix.md: Timezone-gated scheduling row — all 🟡 (cron TZ) vs. Delta-T ✅ (governance windows) |
| "circadian kill-switches" | Temporal's pause/resume requires imperative signals per-invocation. No competitor has declarative recurring sleep schedules. | steal-differentiate-ignore.md D4: "Temporal's signals are imperative; Delta-T's sleep fence is declarative" |
| "workspace-level undo" | Temporal has workflow replay. Git has per-commit revert. Undisk has API-level restore. None combine visual timeline + mobile + workspace checkpoint restore. | competitor-matrix.md: Undo/rollback row; steal-differentiate-ignore.md D3 comparison table |
| "controlled from a macOS menu bar clock" | No competitor provides a persistent desktop clock interface for governance. Menu bar + Cloudflare edge enforcement means no browser needed. Mobile control (Phase 2) builds on same edge API. | competitor-matrix.md: Market Gaps #2 and #3; tradeoff.md r3: macOS menu bar decision |

### Gap Confirmation

The competitor-matrix.md identifies four isolated capabilities that no single product combines:

1. Temporal.io → durable scheduling, no mobile, no MCP governance, no TZ-gating
2. Undisk → file versioning + undo, no temporal rules, no mobile UI
3. Cloudflare → edge state propagation, no agent orchestration
4. World clock apps → timezone awareness, no developer infrastructure

**Delta-T is the assembly.** The USP claims this assembly as the unique value — now delivered via a macOS desktop menu bar app (MVP), with mobile control as Phase 2. [Source — competitor-matrix.md: "The pieces exist in isolation…"; tradeoff.md r3: macOS-first strategy]

---

## Stress Tests

### Test 1: Competitor Swap Test

> "Can any competitor's name replace Delta-T in the USP and it still be true?"

**USP under test:** "[Competitor] is the only temporal governance layer that lets a developer set timezone-gated execution rules for autonomous AI agents — controlled from a persistent macOS menu bar clock, enforced at the edge via Cloudflare, and executed through the team's own Undisk MCP workspace."

| Competitor | Swap Valid? | Reason |
|---|---|---|
| Temporal.io | ❌ No | No mobile app, no clock UI, no workspace undo, no sleep fence. Cron scheduling ≠ governance windows. [competitor-matrix.md] |
| Inngest | ❌ No | No mobile app, no workspace undo, no sleep fence, no clock UI. [competitor-matrix.md] |
| Trigger.dev | ❌ No | No mobile app, no TZ governance, no workspace undo. [competitor-matrix.md] |
| Windmill | ❌ No | No mobile app, no TZ governance, no MCP integration. [competitor-matrix.md] |
| n8n | ❌ No | No mobile app, no workspace undo, no agent governance windows. [competitor-matrix.md] |
| Retool | ❌ No | Mobile app builder ≠ mobile control plane. No agent governance. [competitor-matrix.md] |
| Dato | ❌ No | macOS only, no agent infrastructure, no governance. [competitor-matrix.md] |
| There | ❌ No | macOS/Web only, no developer integrations, no governance. [competitor-matrix.md] |
| Every Time Zone | ❌ No | Web only, display only, no infrastructure connection. [competitor-matrix.md] |
| World Time Buddy | ❌ No | Has mobile app but zero developer infrastructure, zero governance. [competitor-matrix.md] |
| Timezone.io | ❌ No | Web only, Slack integration only, no governance. [competitor-matrix.md] |
| E2B | ❌ No | Sandboxed compute, no temporal governance, no mobile, no TZ awareness. [competitor-matrix.md] |
| Modal | ❌ No | Serverless compute, no temporal governance, no mobile. [competitor-matrix.md] |
| Replit Agent | ❌ No | Browser-based IDE, no temporal governance, no mobile-native control. [competitor-matrix.md] |
| Undisk | ❌ No | Has workspace undo but no mobile UI, no temporal rules engine, no clock interface. [competitor-matrix.md] |

**Result: PASS** — No competitor's name can be substituted. The USP survives the swap test for all 15 competitors.

### Test 2: "So What?" Test

> "Does the USP articulate a clear user benefit?"

**USP dissection:**

| Phrase | Benefit to ICP |
|---|---|
| "timezone-gated execution rules" | → "My agents won't deploy at 2 AM Tokyo time." Addresses ICP Pain #1: "My agents don't know what time it is." [icp.md] |
| "circadian kill-switches" | → "I can sleep knowing agents are paused." Addresses ICP Pain #2: "I can't sleep because my agents might break things." [icp.md] |
| "workspace-level undo" | → "I wake up, see the damage, undo it in 30 seconds from my desktop." Addresses ICP Pain #4: "I have no temporal audit trail for my agents." [icp.md] |
| "controlled from a macOS menu bar clock" | → "It's always visible, always one click away." Addresses developer preference for persistent desktop tools. Mobile control (Phase 2) addresses ICP Pain #5. [icp.md; tradeoff.md r3] |

**Result: PASS** — Every clause maps to a documented ICP pain point. The USP is not feature-listing; it articulates "what this means for you."

### Test 3: Negation Test

> "Is the negation of the USP something a competitor would claim?"

**Negation:** "We are NOT a temporal governance layer. We do NOT let developers set timezone-gated execution rules from a desktop clock interface."

Would any competitor claim this? **No.** No competitor positions itself by *rejecting* temporal governance from mobile. Temporal.io would say "We are the most reliable workflow orchestration platform" — a completely orthogonal claim. Dato would say "We are the best macOS clock" — a different category entirely.

The negation is not a viable competitive position. This is the hallmark of a strong USP: it occupies space no one else wants to defend from the opposite direction.

**Result: PASS** — The negation is not a plausible competitor stance.

### Test 4: Time Test (18-Month Horizon)

> "Will this USP still be valid in 18 months (October 2027)?"

| Threat | Likelihood | Impact | Mitigation |
|---|---|---|---|
| Temporal.io ships a mobile app | Low — Temporal's roadmap is enterprise reliability, not mobile UX. Their UI team ships web improvements, not iOS apps. [competitor-matrix.md] | High — would partially invalidate "only mobile-first" | Delta-T's clock-as-UI paradigm and Undisk integration would remain differentiated even if Temporal ships a basic mobile dashboard |
| Undisk ships temporal rules | Medium — Undisk's 2026 roadmap mentions "governance maturation" for MCP. [competitor-matrix.md: Market Gaps #1] | Medium — would erode the workspace integration moat | Delta-T's value is the mobile UX + edge-state bridge, not just the rules engine. Undisk adding rules would validate the category |
| MCP spec adds native scheduling | Medium — MCP 2026 roadmap mentions governance as a priority area. [competitor-matrix.md] | Low — spec-level scheduling would be a primitive, not a product. Delta-T would implement the spec |
| A new startup targets this exact niche | Low — the niche is narrow (TZ governance × mobile × MCP × Undisk) | Medium — first-mover advantage + deep Undisk integration creates switching cost |

**Result: PASS (with caveats)** — The USP is robust for 18 months. The highest risk is Undisk adding governance primitives, but this would validate Delta-T's category rather than eliminate it. The compound moat (see below) makes replication expensive.

---

## Compound Moat

### Why This Position Is Defensible

The USP is not defended by any single feature — it's defended by the **cost of assembly**. A competitor would need to build or integrate ALL of the following simultaneously:

```
┌─────────────────────────────────────────────┐
│              COMPOUND MOAT                  │
│                                             │
│  ┌─────────┐  ┌──────────┐  ┌───────────┐  │
│  │  iOS    │  │Cloudflare│  │  Undisk   │  │
│  │  Clock  │←→│  KV/D1   │←→│   MCP     │  │
│  │  App    │  │  Edge    │  │ Workspace │  │
│  └─────────┘  └──────────┘  └───────────┘  │
│       ↑              ↑             ↑        │
│  Clock-as-UI    <60s global   25 MCP tools  │
│  paradigm       propagation   deep integ.   │
│                                             │
│  + Timezone governance window semantics     │
│  + Circadian sleep fence (declarative)      │
│  + Temporal undo slider (visual)            │
│  + Stark design language                    │
└─────────────────────────────────────────────┘
```

### Compounding Effects

1. **Data network effect.** Every governance rule a developer creates (sleep fences, TZ windows) becomes embedded in their workflow. Switching means recreating all temporal rules AND migrating Cloudflare KV state AND re-integrating with Undisk — high switching cost. [Model-sourced]

2. **Integration depth over time.** As Undisk adds MCP tools (currently 25), Delta-T's governance surface area grows without Delta-T building new features. The deeper the Undisk integration, the harder to replicate. [Source — competitor-matrix.md: "first-class: reads Undisk audit trail, checkpoints, and version history"]

3. **UX learning curve inversion.** The clock-as-UI paradigm has a learning curve (novel interaction model), but once learned, it becomes the fastest way to manage temporal rules. Competitors shipping traditional dashboards cannot match the interaction speed. [Source — steal-differentiate-ignore.md D7: "the interface IS a clock"]

4. **Edge-state architecture.** Cloudflare KV's 330+ PoP distribution is not replicable by competitors using centralized architectures. Temporal.io or Inngest would need to fundamentally re-architect to match <60s global propagation. [Source — competitor-matrix.md: Edge-state sync row — all ❌ for competitors]

5. **Category creation.** "Temporal governance layer for autonomous agents" is not an existing category. Delta-T defines the category, sets buyer expectations, and owns the vocabulary (sleep fence, governance window, temporal undo). Category creators capture 76% of category economics. [Model-sourced — Play Bigger category design framework]

### Moat Strength Assessment

| Moat Type | Strength (1-5) | Notes |
|---|---|---|
| Switching costs (rule migration) | 3 | Meaningful but not insurmountable — rules are declarative and could be exported |
| Integration depth (Undisk) | 4 | Deep 25-tool MCP integration is expensive to replicate; grows with Undisk's roadmap |
| Architecture (edge state) | 4 | Structural advantage — centralized competitors cannot match without re-architecture |
| Category ownership | 3 | First-mover in "temporal agent governance" but category is unproven; validation risk |
| UX paradigm (clock-as-UI) | 3 | Novel and memorable but unvalidated with users; could be a weakness if unintuitive |
| **Compound** | **4** | The combination is stronger than any individual moat; replication cost is the sum of all five |

---

## Confidence Limiters

The following factors constrain confidence in this USP and could require revision:

### 1. Category Risk (Impact: High)

"Temporal governance for AI agents" is an invented category. If the ICP does not recognize timezone-gated agent governance as a problem worth paying for, the entire USP is addressing a non-market. The 29% trust figure for AI outputs [icp.md, Verified — Stack Overflow 2025] suggests demand exists, but willingness-to-pay for *temporal* governance specifically is unvalidated. [Model-sourced]

### 2. Desktop-First vs Mobile Demand (Impact: Medium)

The USP now emphasizes a macOS menu bar app as the primary control plane. If developers prefer mobile control (e.g., setting rules from their phone while away from their desk), the desktop-first positioning may limit initial adoption. Mobile control is planned for Phase 2. The ICP pain point #5 cites PagerDuty/Datadog mobile as precedent for mobile developer tools. Phase 2 validation will determine mobile priority. [Source — icp.md: "Mobile MCP clients are just emerging"; tradeoff.md r3: macOS desktop decision]

### 3. Undisk Vertical Integration (Impact: LOW — formerly Medium-High)

~~The USP's "workspace-level undo" depends entirely on Undisk's MCP tools. If Undisk changes its API, deprecates tools, or becomes a competitor, the USP's undo claim breaks.~~

**[REVISED r3]:** The Delta-T team IS the Undisk team. Undisk is an internal tool, not a third-party vendor. API stability, roadmap alignment, and co-marketing are controlled by the same team. The "single-vendor dependency" risk is ~0%. Instead, Undisk ownership is the **primary competitive moat** — competitors must build their own MCP-compatible versioned workspace from scratch. [Source — critic-report.md r3: F2 ~0%; moat.md r3: vertical integration moat durability 5/5]

### 4. Clock-as-UI Legibility (Impact: Medium)

"Controlled entirely from a clock interface" is a bold UX bet. If user testing reveals that a clock metaphor is confusing for governance tasks, the USP's "clock interface" claim becomes a weakness. This is the highest UX risk in the product. [Model-sourced — no precedent for clock-as-governance-UI exists]

### 5. Model-Sourced Evidence Gaps

Several supporting claims rely on [Model-sourced] inference rather than verified data:
- Category economics (76% to category creators)
- Switching cost magnitude
- Mobile governance demand
- Clock-as-UI usability

These should be validated through user interviews and prototype testing before the USP is promoted from "draft" to "validated."

---

## Quality Score

| Criterion | Score (1-5) | Rationale |
|---|---|---|
| **Specificity** | 4 | USP names exact capabilities (TZ-gated rules, circadian kill-switches, workspace undo, clock UI, mobile). Not generic. Positioning matrix plots all 15 competitors on two defined axes. |
| **Actionability** | 4 | USP directly informs: product naming ("Temporal Command Center"), marketing copy (each clause = a headline), feature prioritization (3 falsifiability criteria = 3 must-ship features), and competitive positioning (quadrant ownership). |
| **Non-redundancy** | 5 | No overlap with competitor-matrix.md (which catalogs features), steal-differentiate-ignore.md (which classifies them), or icp.md (which profiles users). This artifact synthesizes all three into a single falsifiable claim — a function none of the inputs perform. |
| **Evidence quality** | 4 | All competitor claims cite specific rows/features from competitor-matrix.md. All ICP claims cite specific pain points from icp.md. All differentiation claims cite specific D-codes from steal-differentiate-ignore.md. Confidence limiters are honest about model-sourced gaps. |
| **Overall** | **4.25** | Strong draft USP grounded in comprehensive competitive evidence. Primary risk is category validation (unproven market), not analytical rigor. Recommend promoting to "validated" after ICP interview confirmation of Pain Points #1 and #2. |
