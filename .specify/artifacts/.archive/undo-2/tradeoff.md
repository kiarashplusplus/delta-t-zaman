---
artifact_meta:
  produced_by: "os.tradeoff"
  produced_at: "2026-07-20T01:30:00Z"
  confidence: 0.75
  inputs_used:
    - ".specify/artifacts/phase-1/critic-report.md"
    - ".specify/artifacts/phase-1/feasibility.md"
    - ".specify/artifacts/phase-1/usp.md"
    - ".specify/artifacts/phase-1/steal-differentiate-ignore.md"
    - ".specify/artifacts/phase-1/validate.md"
  stale_after: "on_input_change"
  revision: 2
  quality_scores:
    specificity: 5
    actionability: 5
    non_redundancy: 5
    evidence_quality: 4
  grade: "draft"
---

# Tradeoff Analysis — Delta-T Zaman: The Temporal Command Center

## Executive Summary

**IN:** Journey 1 (Deploy Gate) on macOS desktop only, using Cloudflare KV + Worker middleware with Undisk MCP integration behind an abstraction layer — delivered as a functional CLI + menu bar widget in 8 weeks, with the clock UI as an optional overlay validated by prototype testing. **OUT:** iOS app, Android app, Journey 2 (Sleep Fence), Journey 3 (Undo Slider), billing, push notifications, analytics, team features, multi-workspace, and the "mobile-first" USP framing. **WHY:** The critic identified solo founder scope as a 35% kill probability; the intersection of Tauri mobile beta + Cloudflare middleware + Undisk MCP + novel clock UI is multiplicative complexity that a solo developer cannot ship in 10 weeks. We cut to a provably shippable core that validates the temporal governance thesis without betting on three unvalidated variables simultaneously (mobile platform, clock UI, agent demand).

---

## Feature Triage

### Full Feature Classification

| # | Feature | Build in MVP | Build in v1.1 | Build Later | Never Build | Justification |
|---|---------|:---:|:---:|:---:|:---:|---|
| 1 | **Journey 1: Deploy Gate** (timezone-gated Undisk deployment) | ✅ | | | | Core thesis validation. Simplest journey — 6 Undisk API calls mapped. feasibility.md confirms clean API surface. This is the minimum provable value. |
| 2 | **Journey 2: Sleep Fence** (circadian kill-switch) | | ✅ | | | Critic F3 mitigation: "Ruthlessly cut to ONE journey." Sleep Fence needs Cloudflare Cron Triggers + timezone-aware scheduling — additional complexity. Defer to v1.1 where it becomes the second validation signal. Critic notes this journey is the most Undisk-independent (only needs Cron + set_policy). |
| 3 | **Journey 3: Undo Slider** (temporal scrub + revert) | | | ✅ | | Highest complexity: requires `list_changes`, `list_versions`, `get_diff`, `restore_version`, `workspace_checkpoint` — 5 Undisk tools with a visual timeline UI. Novel UX with no precedent. Defer until demand is validated and UX paradigm is proven. |
| 4 | **macOS desktop app** (Tauri menu bar widget) | ✅ | | | | Tauri desktop is stable (not beta). Menu bar persistence matches ICP "persistent background + burst interaction" pattern [steal-differentiate-ignore.md S2]. Zero App Store risk. Direct download distribution. |
| 5 | **iOS app** (Tauri mobile) | | ✅ | | | Tauri mobile is "beta/experimental" [feasibility.md, deep-research-report.md]. Critic F3: "Ship macOS desktop FIRST... Defer iOS to Phase 2." TestFlight distribution adds Apple Developer gatekeeping. Background execution constraints (30s BGTaskScheduler) add complexity without enforcement value (enforcement is server-side). |
| 6 | **Android app** (Tauri mobile) | | | ✅ | | Already cut in feasibility.md MVP scope. Android market share among ICP is secondary to iOS [icp.md: "iPhone + MacBook Pro"]. Wait for Tauri mobile stabilization + iOS validation. |
| 7 | **Windows desktop** | | | ✅ | | ICP primary persona uses macOS [icp.md]. Windows expands TAM but adds platform-specific testing burden. Defer until macOS proves the concept. |
| 8 | **Linux desktop** | | | ✅ | | Same logic as Windows. Tauri supports Linux but adds CI/packaging complexity. |
| 9 | **Cloudflare KV integration** (edge state for rule reads) | ✅ | | | | Core infrastructure. KV delivers 1-5ms rule reads globally. Required for Deploy Gate rule evaluation. Low risk, low cost (free tier covers MVP). [feasibility.md: Risk level LOW] |
| 10 | **Cloudflare D1 integration** (structured rule history) | | ✅ | | | Rule history is valuable but not required for MVP. MVP can store current rules in KV only. D1 adds SQL query capability for temporal queries — useful but not minimal. Deploy Gate only needs "is this action allowed right now?" not "show me all rule changes over time." |
| 11 | **Cloudflare Worker middleware** (REST API) | ✅ | | | | The enforcement brain. Worker handles rule CRUD, evaluates allow/deny, and calls Undisk `set_policy`. This IS the product's server-side logic. Non-negotiable. |
| 12 | **Undisk MCP integration** (`set_policy`, `audit_trail`) | ✅ | | | | Deploy Gate requires `set_policy` to lock/unlock workspace writes. `audit_trail` provides tamper-evident logging. These two tools are the minimum Undisk surface area for Journey 1. Abstract behind a workspace interface per critic F2 mitigation. |
| 13 | **Multi-timezone display** (world clock base) | ✅ | | | | The "trojan horse" free utility [steal-differentiate-ignore.md S1, validate.md acquisition strategy]. Visual timezone display is the acquisition hook. Ship as a minimal menu bar clock showing configured zones. |
| 14 | **Clock-as-governance UI** (tap timezone to set rules) | 🟡 Prototype only | ✅ | | | Critic F4: 25% kill probability, "no precedent for clock-as-governance-UI exists." MVP ships with a conventional rule-creation form alongside the clock display. Clock interaction is tested via Figma prototype. If prototype test passes (≥80% per critic's raised threshold), promote to v1.1 as the primary interaction. |
| 15 | **Working hours overlay** (from original sketch) | | ✅ | | | Visual enhancement. Not required for governance validation. Adds design complexity. |
| 16 | **Team scheduling** | | | | ❌ | steal-differentiate-ignore.md I8: "Team features are a v2/v3 concern." ICP is solo developer. Zero validation that teams want this. |
| 17 | **Custom MCP server** (middleware layer) | | | | ❌ | feasibility.md confirms: "The agent connects directly to Undisk's MCP server... The Cloudflare Worker is NOT an MCP server — it's a REST API." No custom MCP server needed. Worker is standard HTTP middleware. |
| 18 | **Push notifications** (rule trigger alerts) | | ✅ | | | Requires APNs/FCM integration. macOS supports local notifications via Tauri plugin natively — ship local notifications in MVP for rule-trigger confirmations. Remote push is v1.1 when iOS ships. |
| 19 | **Webhook integrations** | | | ✅ | | Nice-to-have for power users. Not required for core governance thesis validation. Adds API surface to maintain. |
| 20 | **API for external tools** | | | ✅ | | Same as webhooks. The Worker already has internal REST endpoints; exposing them publicly requires auth, rate limiting, documentation. Defer. |
| 21 | **Analytics dashboard** | | | ✅ | | Premature. MVP needs basic telemetry (rule creation count, enforcement count) via Cloudflare Analytics or Plausible. A custom dashboard is engineering vanity at this stage. |
| 22 | **Billing/subscription system** | | ✅ | | | MVP is free alpha. Billing gates revenue but doesn't validate the thesis. Stripe integration is straightforward but still 1-2 weeks of work (pricing page, checkout flow, webhook handling, entitlements). Ship in v1.1 alongside iOS. |
| 23 | **Visual timezone slider** (Every Time Zone-style) | | ✅ | | | steal-differentiate-ignore.md S1: "Steal." Great for UX but adds frontend complexity. MVP menu bar widget shows current times statically. Slider interaction is a v1.1 enhancement. |
| 24 | **OpenTelemetry / structured observability** | | | ✅ | | steal-differentiate-ignore.md S5: "Steal." Valuable but engineering-heavy. MVP uses structured Worker logs + Undisk `audit_trail`. OTel integration is a scale concern. |
| 25 | **Free tier with governance limits** | ✅ | | | | steal-differentiate-ignore.md S6: "Steal." MVP IS the free tier — all alpha users get full access. Natural free tier = 3 timezone rules, 1 workspace. Monetization boundaries defined but not enforced in MVP. |
| 26 | **Step-level retry / durability** | | ✅ | | | steal-differentiate-ignore.md S3: "Steal." Worker → Undisk calls should retry on transient failures. Basic retry logic (3 attempts, exponential backoff) in MVP. Full durable execution (Inngest-style) is v1.1. |
| 27 | **Edge-state sync for governance** | ✅ | | | | steal-differentiate-ignore.md D5: "Differentiate." KV provides this natively. Rule changes propagate globally <60s. This is inherent in the Cloudflare architecture, not extra work. |
| 28 | **Multi-workspace support** | | | ✅ | | Critic C7: "Single-workspace MVP undermines 'temporal command center' positioning." Acknowledged, but multi-workspace multiplies testing surface. MVP governs one Undisk workspace. |
| 29 | **Execution-based pricing model** | | ✅ | | | steal-differentiate-ignore.md S4: "Steal." Pricing model is defined but not implemented until billing ships in v1.1. |

### Triage Summary

| Category | Count | Items |
|----------|-------|-------|
| **Build in MVP** | 9 | Journey 1, macOS desktop, KV, Worker, Undisk integration (set_policy + audit_trail), multi-TZ display, clock UI prototype, free tier, edge-state sync |
| **Build in v1.1** | 10 | Journey 2, iOS app, D1, clock-as-governance UI, working hours overlay, push notifications, billing, visual slider, step-level retry, pricing model |
| **Build Later** | 7 | Journey 3, Android, Windows, Linux, webhooks, API, analytics dashboard, OTel, multi-workspace |
| **Never Build** | 2 | Team scheduling (for now), custom MCP server |

---

## The One Journey Decision

### Decision: **Journey 1 — Deploy Gate**

### Justification

**1. Strongest demand signal (projected, not validated)**

validate.md's interview script (Questions 4-6) centers on "agents doing something unexpected when unsupervised" and "controlling when agents can act." The Deploy Gate scenario — "my agent deployed at 2AM in the wrong timezone" — is the most concrete, incident-driven pain point. It maps directly to real developer behavior: agents writing to production workspaces at inopportune times. Sleep Fence (Journey 2) is a generalized version of the same thesis; Undo Slider (Journey 3) is a recovery mechanism that only matters if the governance failed. **Deploy Gate is the atomic unit of temporal governance.** If developers won't gate a deploy by timezone, they won't adopt sleep fences or undo sliders.

Critic-report.md notes that icp.md Pain #1 ("My agents don't know what time it is") and Pain #2 ("I can't sleep because my agents might break things") are both served by Deploy Gate. One journey, two pain points validated.

**2. Technically simplest**

feasibility.md maps Journey 1 to 6 steps with only 2 Undisk MCP tools (`set_policy` for lock/unlock, `audit_trail` for logging). Compare:
- Journey 2 (Sleep Fence): Adds Cloudflare Cron Triggers, timezone-aware scheduling, recurring state transitions (pause at 23:00, resume at 07:00), handoff notes via `workspace_collaborate`.
- Journey 3 (Undo Slider): Requires 5 Undisk tools (`list_changes`, `list_versions`, `get_diff`, `restore_version`, `workspace_checkpoint`) plus a novel timeline visualization UI.

Journey 1's total Undisk API surface is the smallest. The Worker middleware is the simplest (rule check → allow/deny → `set_policy`). No Cron Triggers, no timeline visualization, no checkpoint management.

**3. Best validates the core thesis**

The core thesis is: "Developers will adopt timezone-based constraints for AI agent actions." Deploy Gate tests this directly: does a developer create a rule like "block Undisk writes in production workspace outside 09:00-18:00 JST"? If yes → the temporal governance category exists. If no → Sleep Fence and Undo Slider are also dead.

Critic F1 (45% kill probability — invented demand) is the existential risk. Deploy Gate is the fastest path to a binary answer: either developers create timezone deploy rules within 7 days of installation, or they don't. validate.md's activation metric ("time-to-first-rule ≤ 5 minutes") is designed to measure this.

**4. What we sacrifice**

- Sleep Fence is the most emotionally compelling journey (icp.md Pain #2: "I can't sleep because my agents might break things"). It is the USP sentence's "circadian kill-switches." Deferring it weakens the emotional pitch.
- Undo Slider is the most technically differentiated feature (no competitor combines visual timeline + workspace checkpoint restore). Deferring it reduces competitive moat.

**Verdict:** These sacrifices are acceptable because neither journey can be validated if Deploy Gate fails. Deploy Gate is the necessary precondition.

---

## Platform Triage

### Decision: **macOS desktop only (Tauri desktop, menu bar widget)**

### Justification

| Option | Dev Cost (weeks) | Validation Value | Risk | Verdict |
|--------|:---:|:---:|:---:|:---:|
| macOS only (Tauri desktop) | 1-2 weeks | HIGH — proves governance concept | LOW — Tauri desktop is stable | ✅ **Selected** |
| iOS only (Tauri mobile) | 3-4 weeks | HIGH — validates mobile-first USP | HIGH — Tauri mobile is beta | ❌ Too risky for solo dev |
| macOS + iOS | 5-6 weeks | HIGHEST — validates both platforms | HIGH — doubles testing surface | ❌ Exceeds scope budget |
| Desktop only (macOS + Win + Linux) | 3-4 weeks | MEDIUM — wider TAM | MEDIUM — cross-platform testing | ❌ Premature breadth |

**The critic's recommendation is explicit** (F3 mitigation): "Ship macOS desktop FIRST (Tauri desktop is stable) with a basic web dashboard, not a novel clock UI. Validate the governance concept before investing in the UX innovation. Defer iOS to Phase 2 after desktop governance is proven."

**What this means for the USP:**

The USP says "controlled entirely from a clock interface on their phone." A macOS-only MVP falsifies this claim. This is deliberate. Per critic C2: "Either the USP must be softened to 'mobile-capable' or the MVP must actually deliver a shippable mobile experience."

**We soften the USP for MVP:** "Delta-T Zaman lets developers set timezone-gated execution rules for autonomous AI agents — starting with a macOS menu bar clock that governs Undisk workspaces." The mobile-first USP is the aspiration; the desktop MVP is the validation vehicle.

**Cost savings:** Eliminating iOS from MVP saves:
- 2-3 weeks of Tauri mobile debugging (WKWebView quirks, CSP issues, background execution)
- TestFlight provisioning and code signing
- Apple Developer review risk
- APNs push notification integration
- Physical device testing matrix

These weeks are reallocated to Worker + Undisk integration quality and validation infrastructure (landing page, prototype testing).

**ICP compatibility:** icp.md describes the primary persona as using "MacBook Pro (M-series)" as their primary workstation. A macOS menu bar clock IS on their primary device. They interact with it during coding sessions. The "from the phone" scenario is strongest for "check agent status while away from desk" — which is a v1.1 use case, not the MVP validation case.

---

## Dependency Triage

| Dependency | MVP Status | Justification |
|---|---|---|
| **Undisk MCP** (`set_policy`, `audit_trail`) | ✅ **Required — but abstracted** | Deploy Gate cannot function without `set_policy` to lock/unlock workspace writes. `audit_trail` provides the governance log. **Critical mitigation (critic F2):** ALL Undisk calls go through a `WorkspaceProvider` interface. If Undisk fails, a mock provider can substitute for testing. If Undisk pivots, the interface accepts an alternative implementation. Day-1 architecture decision, not optional. |
| **Cloudflare KV** | ✅ **Required** | Rule storage and fast reads. KV is the source of truth for "is this action allowed right now?" Free tier covers MVP. No alternative needed — KV is commodity infrastructure. |
| **Cloudflare Worker** | ✅ **Required** | The policy enforcement point. REST API for rule CRUD + Undisk MCP client. This is the product's backend. |
| **Cloudflare D1** | ❌ **Deferred to v1.1** | Structured rule history (SQL queries over time) is valuable but not minimal. MVP stores current rules in KV as JSON. Rule history = KV versioning (or simple KV audit log). D1 becomes essential when Journey 3 (Undo Slider) ships and temporal queries are needed. |
| **Cloudflare Cron Triggers** | ❌ **Deferred to v1.1** | Only needed for Journey 2 (Sleep Fence) — automatic recurring pause/resume. Journey 1 uses on-demand rule evaluation (agent checks Worker before acting), not time-triggered transitions. |
| **Custom MCP server** | ❌ **Never needed** | Per feasibility.md: the Worker is standard HTTP middleware, not an MCP server. Agents connect directly to Undisk. Delta-T modifies Undisk policy; agents encounter `PERMISSION_DENIED` transparently. No MCP protocol extensions required. |
| **Apple Developer account** | ❌ **Deferred to v1.1** | No iOS build in MVP. macOS app distributed as direct download (.dmg or Homebrew cask). $99/yr saved until iOS is ready. |
| **Stripe / billing** | ❌ **Deferred to v1.1** | MVP is free alpha. Zero revenue until governance thesis is validated. |

### Undisk Abstraction Layer (Non-Negotiable)

The critic rates Undisk dependency at 30% kill probability. The abstraction must be a day-1 architectural decision:

```typescript
interface WorkspaceProvider {
  setPolicy(workspace: string, rules: PolicyRule[]): Promise<void>;
  getAuditTrail(workspace: string, since: string): Promise<AuditEntry[]>;
  // Future: restoreVersion, listChanges, checkpoint
}

class UndiskProvider implements WorkspaceProvider { /* real calls */ }
class MockProvider implements WorkspaceProvider { /* local-only for testing */ }
```

**If Undisk confirms partnership → use UndiskProvider. If Undisk is non-responsive → MVP still functions with MockProvider for demos and testing, while scouting alternatives.** Per critic soft gate #4: "If Undisk declines or is non-responsive, this is a YELLOW flag — proceed but abstract all Undisk calls behind a workspace interface and actively scout alternatives."

---

## Scope Budget

### Total: 8 weeks (reduced from 10)

The 2-week reduction comes from cutting iOS (saving ~3 weeks) and reallocating 1 week to validation infrastructure. The critic notes that "industry benchmarks suggest solo developers underestimate timelines by 2-3x for novel architectures" — but by cutting iOS and Journey 2+3, we reduce to a well-understood architecture: Tauri desktop (stable) + Cloudflare Worker (commodity) + Undisk API client (HTTP calls). This is NOT a novel architecture. It's a standard middleware + desktop client.

| Week | Focus | Deliverables | Hard Gate |
|------|-------|-------------|-----------|
| **Week 1** | **Scaffold + Validate Tauri Desktop** | Tauri project init (React + Rust). Menu bar icon with timezone display. `fetch()` to external endpoint confirmed. Basic clock showing 4 configured timezones. | Tauri desktop renders clock + makes HTTP call → PASS/FAIL. If FAIL: fallback to Electron (1-week pivot). |
| **Week 2** | **Cloudflare Worker + KV** | Worker deployed with REST endpoints: `POST /rules`, `GET /rules`, `GET /rules/check?workspace=X&action=deploy`. KV schema for rule storage. Simple API key auth. | Worker responds to rule CRUD and check requests → PASS/FAIL. |
| **Week 3** | **Undisk Integration** | `WorkspaceProvider` interface. `UndiskProvider` implementation: `set_policy` for lock/unlock, `audit_trail` for governance log. `MockProvider` for testing. Worker calls Undisk on rule activation/deactivation. | Worker successfully calls `set_policy` on a real Undisk workspace → PASS/FAIL. If FAIL: continue with MockProvider, escalate Undisk contact. |
| **Week 4** | **Deploy Gate End-to-End** | Full Journey 1 flow: User creates timezone rule in desktop app → Worker stores in KV → Agent attempts Undisk write → Worker evaluates rule → `set_policy` blocks/allows → User sees result in app. Local notifications on rule trigger. | End-to-end Journey 1 works with a real Undisk workspace + real agent → PASS/FAIL. **Week 4 checkpoint: if E2E doesn't work, re-scope.** |
| **Week 5** | **Rule Creation UI** | Form-based rule creation: select timezone, set time window, select action (block writes / allow writes), select workspace. No clock-as-UI — conventional form inputs. Rule list view showing active rules and their status. | User can create, view, edit, and delete rules through the UI. |
| **Week 6** | **Polish + Clock Display Enhancement** | Multi-timezone clock display with rule status indicators (🔒 locked, 🔓 unlocked). Rule enforcement status visible at a glance. Error handling for Undisk API failures. Retry logic (3 attempts, exponential backoff). | Clock display clearly shows governance state per timezone. |
| **Week 7** | **Dogfooding + Bug Fixes** | Internal use with a real Undisk workspace and coding agent (Cursor / Claude Code). Fix top 10 bugs. Performance optimization (Worker response time <100ms p95). Edge cases: timezone transitions, DST handling, network failures. | Founder uses Delta-T daily for 1 week without critical failures. |
| **Week 8** | **Alpha Release + Validation Infrastructure** | macOS .dmg or Homebrew distribution. Landing page live (Astro on Cloudflare Pages). Waitlist form. Basic telemetry (rule creation events, enforcement events). README and setup guide. | Alpha available for download. Landing page collecting signups. Telemetry confirms events are logged. |

### The "Skateboard" Version (Weeks 1-4)

If everything goes wrong after week 4, what's the minimum shippable thing?

**Skateboard = CLI + Worker. No desktop app.**

```bash
# Set a rule
delta-t rule create --tz "Asia/Tokyo" --window "09:00-18:00" --action block-writes --workspace production

# Check rule status
delta-t rule status --workspace production

# List rules
delta-t rule list
```

The Cloudflare Worker + KV + Undisk `set_policy` integration is the value core. The desktop UI is a convenience layer. If Tauri causes delays, a CLI that calls the same Worker REST API delivers identical governance value with zero UI risk. The CLI takes 2-3 days to build (Rust CLI with `clap` + `reqwest`).

**This is the ultimate fallback:** If week 4 FAILS the hard gate, ship the CLI + Worker as an open-source tool, collect feedback, and decide whether to invest in UI.

---

## Kill List

Every item below is deliberately excluded from MVP. For each, we acknowledge what's lost.

### 1. iOS App

**What's lost:** The "mobile-first" USP. The ability to set rules from bed/commute. The "controlled from their phone" positioning. The most emotionally resonant demo ("tap timezone on phone → agent blocked").

**Why it's worth it:** Tauri mobile is beta. TestFlight adds 1-2 weeks of Apple gatekeeping. iOS background execution constraints don't affect enforcement (server-side) but add debugging complexity. Critic F3 explicitly recommends this cut. **The governance thesis does not require mobile to be validated.** A developer who won't set timezone rules from their Mac won't set them from their phone either. If desktop governance succeeds, mobile is a distribution expansion. If it fails, mobile wouldn't have saved it.

### 2. Android App

**What's lost:** Access to ICP members who use Android (minority per icp.md but non-zero).

**Why it's worth it:** Already cut in feasibility.md. Tauri Android has the same beta risks as iOS. ICP primary persona uses iPhone. Zero market signal to prioritize Android over iOS, let alone over macOS.

### 3. Journey 2: Sleep Fence

**What's lost:** The "circadian kill-switch" — the most emotionally compelling feature. The USP's second pillar. The strongest "peace of mind" narrative. The feature that directly addresses icp.md Pain #2 ("I can't sleep because my agents might break things").

**Why it's worth it:** Sleep Fence requires Cron Triggers (new infrastructure), recurring state transitions (new complexity), and timezone-aware scheduling (new edge cases: DST, timezone changes). Each adds scope. Journey 1's on-demand rule evaluation is fundamentally simpler than Journey 2's time-triggered transitions. **If Deploy Gate validates temporal governance demand, Sleep Fence is a 2-week v1.1 addition (Cron Trigger + existing `set_policy`).** The work is deferred, not destroyed. Critic F2 mitigation actually recommends Sleep Fence as the most Undisk-independent journey — making it an ideal v1.1 feature when Undisk stability is better understood.

### 4. Journey 3: Undo Slider

**What's lost:** The "temporal undo" — the most technically differentiated feature. Visual workspace time-travel. The feature that no competitor can replicate without Undisk's immutable versioning. The USP's third pillar.

**Why it's worth it:** Undo Slider requires 5 Undisk tools, a timeline visualization UI, checkpoint management, diff preview rendering, and a novel "scrub through time" interaction. This is more complex than Journeys 1 and 2 combined. It also has the highest UI risk — a timeline scrubber on mobile is unproven; on desktop it's possible but still novel. **Undo Slider is a Phase 2+ feature that depends on Journey 1 proving temporal governance demand AND Journey 2 proving time-triggered automation demand.** Building it before either is validated is premature.

### 5. Clock-as-Governance UI (Interactive Clock for Rule Setting)

**What's lost:** The product's most distinctive visual identity. The "interface IS a clock" differentiator. The marketing hook ("control agents from a clock on your phone"). Everything that makes Delta-T visually memorable vs. yet another dashboard.

**Why it's worth it:** Critic F4 assigns 25% kill probability: "No precedent exists for using a clock as a governance control interface." Forcing structured data entry (timezone, time window, action, scope) through a clock metaphor may require more taps than a conventional form. **MVP ships with a conventional form for rule creation AND a clock for timezone display.** The clock shows time + rule status (locked/unlocked); the form handles rule CRUD. This separates the validated (clock as display — per Dato/Every Time Zone) from the unvalidated (clock as input). The Figma prototype test (critic hard gate #2: ≥80% task completion) determines whether clock-as-input ships in v1.1.

### 6. D1 (Structured History Database)

**What's lost:** SQL queries over rule history. Temporal range queries. The data foundation for Journey 3's timeline.

**Why it's worth it:** KV handles current rule state. MVP doesn't need "show me all rule changes between Tuesday and Thursday." It needs "is this deploy allowed right now?" D1's value emerges when temporal queries matter (Journey 3) or when structured analytics are needed. Adding D1 means maintaining a SQL schema, migration scripts, and dual-write consistency between KV and D1. Unnecessary complexity for a boolean allow/deny system.

### 7. Billing / Subscriptions

**What's lost:** Revenue from day 1. Willingness-to-pay validation in production (vs. interview soft commits).

**Why it's worth it:** validate.md projects 0 paying customers in Month 1 even with billing. The free alpha IS the validation strategy. Billing before validation is premature optimization. Stripe integration takes 1-2 weeks and introduces payment failure handling, webhook processing, entitlement management, pricing page design — all engineering time better spent on governance quality. **Billing ships in v1.1 alongside iOS, when there's a user base to convert.**

### 8. Push Notifications (Remote)

**What's lost:** Real-time alerts when a rule triggers ("Your TYO deploy gate unlocked at 09:00"). The "it just works in the background" experience.

**Why it's worth it:** macOS supports local notifications via Tauri's notification plugin. MVP uses local notifications triggered by the app when it detects rule state changes (polling Worker every 30s while app is running). Remote push requires APNs setup, certificate management, and server-side push infrastructure. Since enforcement is server-side and the macOS app is a control plane (not enforcement plane), local notifications with periodic sync are sufficient for MVP.

### 9. Analytics Dashboard

**What's lost:** Visual metrics on rule enforcement, agent behavior patterns, governance coverage.

**Why it's worth it:** MVP needs telemetry (events logged to Worker + KV), not a dashboard. Console logs + Cloudflare Analytics + Undisk `audit_trail` provide sufficient observability for an 8-week alpha with <100 users. A custom dashboard is engineering vanity before product-market fit.

### 10. Multi-Workspace Support

**What's lost:** Governing more than one Undisk workspace. Per critic C7: "A developer with a 'production' and 'staging' workspace can only protect one at launch."

**Why it's worth it:** Multi-workspace multiplies the rule evaluation logic (which workspace does this agent action target?), the UI complexity (workspace selector, per-workspace rule lists), and the testing surface. The ICP likely has 1-2 critical workspaces; governing the primary one validates the thesis. Multi-workspace is a natural v1.1 expansion.

### 11. Webhook Integrations / External API

**What's lost:** Third-party tool integration (Slack alerts, PagerDuty escalation, CI/CD pipeline triggers).

**Why it's worth it:** Zero evidence any ICP user wants Delta-T to call Slack. The product is a governance tool, not an integration platform. Adding webhook support means designing an event schema, building delivery infrastructure, handling retries, and documenting an API. This is a classic "build it when users ask for it" feature.

### 12. Team Scheduling / Collaboration

**What's lost:** Multi-user governance. Shared rules. Team-wide sleep fences.

**Why it's worth it:** steal-differentiate-ignore.md I8: "Team features are a v2/v3 concern if individual adoption proves the concept." The ICP is a solo developer. Zero evidence teams want this. Building team features before individual adoption is validated is building for an imaginary customer.

---

## Tradeoff Consequences

Each cut has a cost. Acknowledging these costs prevents future amnesia about what was deliberately sacrificed.

| Cut | What We Lose | Risk if Wrong | Recovery Plan |
|-----|-------------|---------------|---------------|
| **iOS app** | Mobile-first USP. "From your phone" marketing. Largest emotional differentiator vs. competitors. | If mobile IS the critical differentiator (>60% of interview subjects insist on phone control), desktop-only MVP fails to attract the ICP. | validate.md interview Question 9 tests mobile preference. If ≥60% prefer phone → fast-track iOS to v1.1 (weeks 9-12). Tauri mobile work done in MVP scaffold is reusable. |
| **Sleep Fence** | "Peace of mind" narrative. Most emotionally resonant feature. Second USP pillar. | If "agents running overnight" is the #1 pain point (not deploy governance), MVP misses the primary demand signal. | Interview Questions 3-4 and 7 test this directly. If sleep anxiety > deploy anxiety → re-scope v1.1 to ship Sleep Fence before iOS. Cron Trigger + existing `set_policy` = 2-week build. |
| **Undo Slider** | Technical differentiation. "No competitor can do this" claim. Third USP pillar. The Undisk moat. | If "undo agent damage" is the primary need, MVP doesn't address it. | Undo Slider depends on Undisk tools that are already documented and mapped. It's a UI/UX project, not an infrastructure project. Can be fast-tracked to v1.1 if demand signals point here. |
| **Clock-as-UI** | Visual identity. Marketing memorability. "The interface IS a clock" positioning. | If conventional form-based governance fails to excite users, the product is perceived as "just another config tool." | Figma prototype testing occurs in parallel with MVP build. If clock tests well (≥80% completion), it's promoted to primary UI in v1.1. Clock display remains in MVP — only clock-as-input is deferred. |
| **D1 database** | Structured query capability. Rule history analytics. Foundation for Journey 3. | If users want to understand "what rules were active last Tuesday at 3pm," KV can't answer that. | Adding D1 to an existing Worker takes 1 day (schema creation + dual-write from existing KV writes). This is a trivial expansion whenever needed. |
| **Billing** | Revenue. Real willingness-to-pay validation. The difference between "I'd pay $19" and actually paying $19. | If users love the free alpha but never convert, we learn this 3 months later instead of immediately. | Stripe integration is a well-understood 1-2 week project. validate.md's fake-door checkout test provides pricing signal before billing ships. |

### Cumulative Risk Assessment

The biggest cumulative risk of these cuts is **identity dilution.** The USP as written — "mobile-first temporal governance layer... controlled entirely from a clock interface on their phone" — is falsified by a macOS-only, form-based, deploy-gate-only MVP. The MVP is "a macOS menu bar tool that timezone-gates Undisk writes."

This is deliberate. The critic's core finding is that zero external validation has occurred. The MVP's job is NOT to be the final product — it's to answer one question: **"Do developers create timezone rules for their AI agents?"** If yes, mobile + clock UI + sleep fence + undo slider are all expansions of a proven thesis. If no, none of those features would have saved the product.

---

## Quality Score

| Criterion | Score (1-5) | Rationale |
|---|---|---|
| **Specificity** | 5 | 29 features triaged with individual justifications. Week-by-week scope budget with hard gates. Skateboard fallback defined. Every kill list item includes recovery plan. Dependency decisions cite exact Undisk tools and Cloudflare services. |
| **Actionability** | 5 | A solo developer can read this artifact and know exactly what to build in week 1, what to ignore, and when to pivot. The scope budget is executable as-is. Hard gates at weeks 1, 4, and 8 provide clear checkpoints. The skateboard version is a concrete fallback with specific deliverables. |
| **Non-redundancy** | 5 | No overlap with feasibility.md (which scopes ALL journeys), validate.md (which defines validation criteria), or critic-report.md (which identifies problems). This artifact uniquely answers: "Given the constraints identified by all other artifacts, what exactly should be built?" |
| **Evidence quality** | 4 | All tradeoff decisions cite specific critic findings (F1-F5, C1-C7), feasibility.md risk assessments (R1-R10), and steal-differentiate-ignore.md classifications (S1-S6, D1-D7, I1-I9). Score not 5 because the "one journey" decision and platform triage are ultimately judgment calls informed by evidence, not determined by it. The correct tradeoffs depend on assumptions (e.g., "desktop governance demand predicts mobile governance demand") that haven't been validated. |
| **Overall** | **4.75** | Ruthlessly scoped artifact that transforms a 10-week multi-platform project into an 8-week focused validation vehicle. Primary contribution: converting the critic's five fatal flaws into specific build/skip decisions with recovery plans for each cut. |
