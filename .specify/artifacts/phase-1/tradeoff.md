---
artifact_meta:
  produced_by: "os.tradeoff"
  produced_at: "2026-07-20T01:30:00Z"
  confidence: 0.74
  inputs_used:
    - ".specify/artifacts/phase-1/critic-report.md"
    - ".specify/artifacts/phase-1/feasibility.md"
    - ".specify/artifacts/phase-1/validate.md"
    - ".specify/artifacts/phase-1/icp.md"
    - ".specify/artifacts/phase-1/usp.md"
    - ".specify/artifacts/phase-1/steal-differentiate-ignore.md"
  stale_after: "on_input_change"
  revision: 3
  quality_scores:
    specificity: 5
    actionability: 5
    non_redundancy: 5
    evidence_quality: 4
  grade: "draft"
  replay_context: "Undisk MCP owned by same team. Internal dependency. Solo founder runs both products. r3 reframe: Undisk risk eliminated, demand risk dominant."
---

# Tradeoff Matrix — Delta-T Zaman: The Temporal Command Center (r3)

## Executive Summary

**The founder's time is the product. Not code. Not architecture. Time.**

A solo founder running a production service (Undisk MCP) has roughly 25-30 productive hours/week for Delta-T after Undisk maintenance, support, and infrastructure. That's ~250-300 hours for the entire MVP. Every feature that enters scope consumes hours from that fixed budget. The tradeoff question is not "what should we build?" — it's "what can we afford to learn with 300 hours?"

**Verdict:** Ship a macOS menu bar app (Tauri desktop — stable, no App Store risk) that validates exactly one thing: will a developer create a timezone-gated deploy rule for their Undisk workspace? Everything else — iOS, Android, Sleep Fence, Undo Slider, clock-as-primary-UI, billing, push notifications, multi-workspace — is cut until that question has a data-backed answer.

**What's new in r3:** Undisk ownership eliminates vendor dependency risk (was YELLOW reason #1 in feasibility.md). This is genuinely transformative for technical execution but changes nothing about demand validation. The scarcest resource shifted from "will Undisk cooperate?" to "does anyone want this?" — and the tradeoff matrix must reflect that shift.

---

## Decision Matrix

### Scoring Methodology

Each candidate feature is scored on four dimensions per the constitutional framework:

| Dimension | Weight | Definition |
|-----------|--------|------------|
| **Impact** (I) | ×2 | Movement on the primary success metric (Governance Rules Enforced/Week — validate.md North Star) |
| **Strategic Alignment** (SA) | ×1.5 | Direct support of the USP: "mobile-first temporal governance layer... from a clock interface" |
| **Effort** (E) | +additive | Engineering days for solo founder. 1=<1d, 2=1-3d, 3=3-7d, 4=1-3wk, 5=>3wk |
| **Risk** (R) | +additive | Probability of failing to deliver value. Technical risk + market uncertainty + dependency risk |

**Formula:** `Priority = (Impact × 2 + SA × 1.5) / (Effort + Risk)`

**Cut line:** Priority ≥ 1.0 = build. Priority < 1.0 = cut. Adjusted upward if total effort exceeds ~300 hours.

### Full Scoring Matrix

| # | Feature | Impact | SA | Effort | Risk | Priority | Verdict |
|---|---------|--------|-----|--------|------|----------|---------|
| F1 | Journey 1: Deploy Gate (timezone-gated Undisk writes) | 5 | 5 | 3 | 2 | **3.50** | ✅ BUILD |
| F2 | Cloudflare Worker middleware (REST API + rule evaluation) | 5 | 4 | 3 | 1 | **3.75** | ✅ BUILD |
| F3 | Cloudflare KV (edge rule storage) | 4 | 3 | 2 | 1 | **2.83** | ✅ BUILD |
| F4 | Undisk `set_policy` integration (workspace lock/unlock) | 5 | 5 | 2 | 1 | **5.83** | ✅ BUILD |
| F5 | macOS menu bar app (Tauri desktop) | 4 | 3 | 3 | 1 | **2.63** | ✅ BUILD |
| F6 | Multi-timezone display (clock face, 3-5 zones) | 3 | 4 | 2 | 1 | **4.00** | ✅ BUILD |
| F7 | Conventional rule-creation form (set timezone + time window + action) | 4 | 3 | 2 | 1 | **3.83** | ✅ BUILD |
| F8 | Undisk `audit_trail` integration (enforcement logging) | 3 | 3 | 2 | 1 | **3.50** | ✅ BUILD |
| F9 | Local macOS notifications (rule triggered/enforced) | 3 | 2 | 2 | 1 | **3.00** | ✅ BUILD |
| F10 | Basic telemetry (rule count, enforcement count via CF Analytics) | 2 | 1 | 1 | 1 | **2.75** | ✅ BUILD |
| F11 | Clock-as-governance UI (tap timezone block → rule drawer) | 3 | 5 | 4 | 4 | **1.69** | 🟡 CONDITIONAL |
| F12 | Journey 2: Sleep Fence (circadian kill-switch via Cron Triggers) | 4 | 4 | 4 | 3 | **1.71** | 🟡 CONDITIONAL |
| F13 | iOS app via Tauri mobile | 3 | 5 | 5 | 4 | **1.50** | 🟡 CONDITIONAL |
| F14 | Cloudflare D1 (structured rule history with SQL) | 2 | 2 | 2 | 1 | **2.33** | 🟡 DEFER to v1.1 |
| F15 | Visual timezone slider (Every Time Zone-style scrubber) | 2 | 3 | 3 | 1 | **1.63** | 🟡 DEFER to v1.1 |
| F16 | Working hours overlay | 2 | 3 | 2 | 1 | **2.17** | 🟡 DEFER to v1.1 |
| F17 | Billing/subscription (Stripe integration) | 1 | 1 | 3 | 2 | **0.70** | ❌ CUT |
| F18 | Journey 3: Undo Slider (temporal scrub + workspace revert) | 3 | 4 | 5 | 4 | **1.33** | ❌ CUT from MVP |
| F19 | Push notifications (APNs/FCM remote) | 2 | 3 | 3 | 3 | **1.08** | ❌ CUT |
| F20 | Android app | 2 | 3 | 5 | 4 | **0.94** | ❌ CUT |
| F21 | Multi-workspace support | 2 | 2 | 4 | 3 | **0.93** | ❌ CUT |
| F22 | Windows desktop | 2 | 1 | 3 | 2 | **0.90** | ❌ CUT |
| F23 | Linux desktop | 2 | 1 | 3 | 2 | **0.90** | ❌ CUT |
| F24 | Open-source clock UI component | 1 | 2 | 3 | 2 | **0.80** | ❌ CUT |
| F25 | Team/org support | 1 | 1 | 5 | 4 | **0.39** | ❌ CUT |
| F26 | Webhook integrations | 1 | 1 | 3 | 2 | **0.70** | ❌ CUT |
| F27 | Public API for external tools | 1 | 1 | 4 | 3 | **0.50** | ❌ CUT |
| F28 | Analytics dashboard | 1 | 1 | 4 | 2 | **0.58** | ❌ CUT |
| F29 | Custom MCP server | 1 | 1 | 5 | 3 | **0.44** | ❌ NEVER |
| F30 | OpenTelemetry / structured observability | 1 | 1 | 4 | 2 | **0.58** | ❌ CUT |
| F31 | Agent-level rate limiting (per-agent throttling) | 2 | 2 | 4 | 3 | **0.93** | ❌ CUT |
| F32 | Real-time event stream (SSE/WebSocket for live timeline) | 2 | 3 | 4 | 3 | **1.07** | ❌ CUT |
| F33 | Execution-based pricing model | 1 | 1 | 3 | 3 | **0.58** | ❌ CUT |
| F34 | Free timezone utility as trojan horse (standalone app positioning) | 2 | 2 | 3 | 3 | **0.92** | ❌ CUT |

### Scoring Calibration Notes

- **F4 scores highest (5.83):** `set_policy` is a single API call to an API the team owns. Effort=2 because implementation is <2 days. Risk=1 because the API is internal. This is the atomic kernel of the entire product.
- **F11 (Clock-as-UI) scores 1.69 — conditional:** SA=5 because it IS the USP, but Risk=4 because critic-report.md F3 assigns 25% kill probability and "no precedent for clock-as-governance-UI exists." The conventional form (F7, Priority=3.83) delivers the same governance value with lower risk.
- **F13 (iOS app) scores 1.50 — conditional:** SA=5 because USP says "from their phone," but Effort=5 (Tauri mobile is beta, TestFlight adds Apple gatekeeping, 30s BGTaskScheduler constraint) and Risk=4 (critic-report.md F5: 20% kill, F7: 15% kill for App Store rejection).
- **F17 (Billing) scores 0.70 — below cut line:** Impact=1 because billing doesn't validate the governance hypothesis. It validates willingness-to-pay, but validate.md's interview Q11-12 already captures that signal pre-build. Building billing before confirming demand is sunk-cost-in-advance.
- **F34 (Trojan horse) scores 0.92 — below cut line:** Critic-report.md F5 notes "the App Store has hundreds of free world clocks" and "if the free tier attracts timezone-curious users (not agent-governance-curious users), the funnel fills with people who will never convert." Positioning as a clock utility dilutes the governance signal. **Lead with governance, not clocks.**

---

## Feature Priority Stack

Ordered by priority score. Features above the line are in MVP scope.

### Tier 1: Non-Negotiable Core (Priority ≥ 2.5)

| Rank | Feature | Priority | Effort (days) | Cited Justification |
|------|---------|----------|---------------|---------------------|
| 1 | **F4: Undisk `set_policy` integration** | 5.83 | 2-3 | feasibility.md: "set_policy is the correct mechanism for multi-hour governance windows." Team owns the API. Zero external dependency. This single call IS the product. |
| 2 | **F6: Multi-timezone display** | 4.00 | 2-3 | icp.md Pain #3: "98% of remote workers cite timezone differences as a major hurdle." validate.md: visual timezone display is the acquisition hook. But served as context for governance — not standalone utility. |
| 3 | **F7: Conventional rule form** | 3.83 | 2-3 | Builds the governance interaction without the unvalidated clock metaphor. A dropdown (timezone) + time picker (start/end) + action selector (block writes/read-only) delivers identical rule-creation capability with near-zero UX risk. |
| 4 | **F2: Cloudflare Worker middleware** | 3.75 | 5-7 | feasibility.md: Worker is "the enforcement brain." Handles rule CRUD, evaluates allow/deny, proxies to Undisk. Three functions: REST API, rule evaluation, policy proxy. Non-negotiable infrastructure. |
| 5 | **F1: Journey 1 (Deploy Gate)** | 3.50 | Composite | The integration of F2+F3+F4+F7. feasibility.md maps it to 6 steps with 2 Undisk tools. Simplest journey. Validates the atomic thesis: "will a developer gate a deploy by timezone?" |
| 6 | **F8: Undisk `audit_trail` integration** | 3.50 | 1-2 | icp.md Pain #4: "no temporal audit trail for my agents." Audit trail is a single API call. Zero incremental risk (team owns the API). Provides rule-enforcement evidence. |
| 7 | **F9: Local macOS notifications** | 3.00 | 1-2 | Tauri's notification plugin supports macOS natively. When a rule triggers (blocks or allows), the user sees "Deploy blocked — TYO workspace locked until 09:00 JST." Completes the feedback loop. |
| 8 | **F3: Cloudflare KV** | 2.83 | 2-3 | feasibility.md: "KV delivers 1-5ms rule reads globally... Risk level: LOW." Rules are set-and-forget; eventual consistency is acceptable. Free tier covers MVP. |
| 9 | **F10: Basic telemetry** | 2.75 | 0.5-1 | Cloudflare Analytics (free) or Plausible ($9/mo). Track: rules created, rules triggered, unique users with active rules. Minimum data to inform v1.1 decisions. |
| 10 | **F5: macOS menu bar app** | 2.63 | 5-7 | Tauri desktop is production-stable (not beta like mobile). Menu bar persistence matches icp.md "burst interaction" pattern. Zero App Store risk — direct download. Critic-report.md F2 mitigation: "Ship macOS desktop first... validate governance concept." |

**Total estimated effort:** 21-31 engineering days (~5-8 weeks at 4 productive days/week)

### Tier 2: Conditional — Build Only If Hard Gates Pass (Priority 1.0-2.49)

| Rank | Feature | Priority | Gate Required | Build If... |
|------|---------|----------|--------------|-------------|
| 11 | **F14: D1 rule history** | 2.33 | MVP ships | ...MVP validates and rule history queries are needed for v1.1 timeline features |
| 12 | **F16: Working hours overlay** | 2.17 | Prototype test | ...clock display proves engaging in prototype testing |
| 13 | **F12: Journey 2 (Sleep Fence)** | 1.71 | HG1 demand + MVP metrics | ...≥50 users have active Deploy Gate rules AND interview Q7 shows "sleep mode" enthusiasm |
| 14 | **F11: Clock-as-governance UI** | 1.69 | HG2 prototype ≥75% | ...Figma prototype achieves ≥75% task completion. If <50%, pivot to form-only. |
| 15 | **F15: Visual timezone slider** | 1.63 | Clock UI validated | ...clock-as-UI passes and visual scrubbing adds value beyond static display |
| 16 | **F13: iOS app** | 1.50 | HG3 mobile ≥60% + HG4 Tauri build | ...≥60% of interviewees accept mobile control AND Tauri iOS builds successfully on physical device in <2 weeks |

### Tier 3: Below Cut Line — Do NOT Build in MVP (Priority < 1.0)

Everything below 1.0 is explicitly excluded. See **Do NOT Build** section.

---

## Do NOT Build

This is the most important section of the artifact. Every item has a cited reason for exclusion.

### DNB-1: iOS App (F13, Priority 1.50 — conditional, not MVP)

**What it is:** Native iOS app via Tauri mobile, distributed through TestFlight, enabling governance control from the developer's phone.

**Why it seems important:** The USP literally says "controlled entirely from a clock interface on their phone." Mobile-first is the stated differentiator. icp.md lists "no mobile control plane for dev infrastructure" as Pain #5.

**Why it must die (for now):**
- **Critic-report.md F2:** "Solo founder scope across two products" — 30% kill. Adding iOS (beta Tauri mobile + Apple provisioning + TestFlight + BGTaskScheduler constraints) to a founder already maintaining Undisk is scope suicide.
- **Critic-report.md F5:** Tauri mobile is "beta/experimental." 20% kill probability specifically for device build failures.
- **Critic-report.md F7:** Apple IAP rejection risk at 15% kill. WebView-based app + web billing = App Store lottery.
- **Critic-report.md C2:** "'Mobile-first' USP vs. MVP that ships beta iOS... A TestFlight beta of 1/3 journeys on a 'beta/experimental' framework is not 'mobile-first.'"
- **Feasibility.md:** "iOS background execution is constrained (max ~30s per wake)... enforcement lives on Cloudflare, not the phone. The phone is a control plane, not an enforcement point." The phone adds nothing to enforcement quality.
- **Validate.md HG3:** Mobile preference is UNVALIDATED. If <40% of interviewees accept mobile, the entire iOS investment is wasted.

**What happens if we're wrong:** If interviews reveal strong mobile demand (HG3 passes ≥60%), we've delayed by ~8 weeks but preserved the option. macOS proves the governance concept; iOS becomes a known-demand build, not a speculative one. This is preferable to building iOS first and discovering nobody uses it from their phone.

**Tie-breaking rationale:** The single factor that tips this: enforcement is server-side. The phone is a remote control, not the engine. A macOS menu bar app is an equally valid remote control with zero platform risk.

### DNB-2: Journey 2 — Sleep Fence (F12, Priority 1.71 — conditional)

**What it is:** Automatic circadian kill-switch that pauses all Undisk workspace writes during the developer's sleep hours (e.g., 11PM-7AM CDT) via Cloudflare Cron Triggers.

**Why it seems important:** icp.md Pain #2: "I can't sleep because my agents might break things." 46% of developers actively distrust AI accuracy (Stack Overflow 2025). The Sleep Fence directly addresses nighttime anxiety.

**Why it must die (for now):**
- **Effort inflation:** Requires Cloudflare Cron Triggers (timezone-aware cron syntax), recurring state transitions (pause at 23:00, resume at 07:00), `workspace_collaborate` → `handoff_note` integration. feasibility.md maps it to 5 steps vs. Journey 1's 6 simpler steps. The "recurring" aspect (fire every night, not once) adds testing complexity.
- **Critic-report.md F6:** "Cloudflare Cron Triggers have documented jitter — they fire 'approximately' at the scheduled time." KV eventual consistency creates a 0-60 second window. "If the developer sets a fence at 11 PM and an agent writes at 10:59:58 PM, the write succeeds." Trust-violating edge cases undermine the core promise ("I can sleep").
- **Strategic sequencing:** Deploy Gate (Journey 1) validates the *concept* of temporal governance. If developers won't gate a single deploy action, they won't adopt nightly sleep schedules. Sleep Fence is a superset — it succeeds only if Deploy Gate succeeds first.
- **Critic-report.md F1:** 40% kill probability on demand. Building Journey 2 before confirming Journey 1 demand multiplies wasted effort.

**What happens if we're wrong:** If Sleep Fence is the killer feature (not Deploy Gate), MVP feedback will reveal it: users will request "I want this to run automatically every night" after using Deploy Gate manually. That signal justifies v1.1 investment. Without the signal, we're guessing.

### DNB-3: Journey 3 — Undo Slider (F18, Priority 1.33)

**What it is:** Visual timeline scrubber on the phone showing Undisk workspace history. User scrubs to a point in time, sees what changed, taps "revert."

**Why it seems important:** icp.md Pain #4: "no temporal audit trail for agents." The Undo Slider is the product's most visually dramatic feature — it photographs well, demos well, differentiates clearly.

**Why it must die (for now):**
- **Highest technical complexity:** Requires 5 Undisk tools (`list_changes`, `list_versions`, `get_diff`, `restore_version`, `workspace_checkpoint`). feasibility.md explicitly defers it: "MVP: 🟡 Basic (list + single restore)... Full visual timeline scrubber with diff preview: Phase 2."
- **Novel UX with zero precedent:** A "temporal scrub" interaction for workspace state has never been built. Design iteration is unpredictable. Critic-report.md F3: clock-as-UI already has 25% kill — adding a second novel interaction paradigm (timeline scrubber) compounds UX risk.
- **Depends on Journey 1 adoption:** Users need active governance rules generating enforcement events before a timeline has anything worth scrubbing. An empty timeline is a dead feature.
- **Undisk already has undo:** `restore_version` works from any MCP client. The Undo Slider adds a visual layer on top of existing capability. It's a UX enhancement, not a new capability. The delta over "just use Undisk directly" must be proven, not assumed.

**What happens if we're wrong:** Undo Slider is the most deferrable feature because the underlying capability (Undisk undo) exists today. Users who need undo can call `restore_version` directly. The Slider is convenience, not capability.

### DNB-4: Billing/Subscription System (F17, Priority 0.70)

**What it is:** Stripe integration for $19/mo individual, $49/mo team subscriptions.

**Why it seems important:** Revenue. The product needs to make money eventually.

**Why it must die (for now):**
- **Premature monetization:** Building billing before confirming anyone wants the product is the textbook premature optimization. validate.md captures willingness-to-pay via interview Q11-12 and fake Stripe checkout — no production billing needed.
- **Effort sink:** Stripe integration (checkout flow, webhooks, entitlements, customer portal, cancellation) is 1-2 weeks of work. That's 10-20% of the entire MVP budget spent on something that generates zero validation signal.
- **Apple IAP risk:** If iOS ships, Apple may require IAP, invalidating the Stripe integration entirely. Building billing before the platform decision is resolved wastes work.
- **Feasibility.md:** MVP is explicitly "free alpha." Break-even is 3 customers — billing can be manual (send a Stripe invoice) for the first 10 customers.

**What happens if we're wrong:** If someone offers to pay during alpha, send them a Stripe Payment Link (5 minutes to set up). Production billing is a v1.1 feature after demand is confirmed.

### DNB-5: Clock-as-Primary-Governance-UI (F11, Priority 1.69 — conditional)

**What it is:** The signature interaction: tap a timezone block on the clock face → rule drawer opens → set governance parameters.

**Why it seems important:** It's the visual identity. The USP says "clock interface." Differentiation from conventional dashboards depends on this.

**Why it must die as the *primary* UI (for now):**
- **Critic-report.md F3:** 25% kill probability. "No precedent exists for using a clock as a governance control interface... Governance rules require structured data entry: select timezone, specify time windows, choose action, select scope. Clocks display time — they are not input forms."
- **Critic-report.md HG2:** Must pass ≥75% task completion in prototype test. This gate has not been passed.
- **The conventional form (F7) delivers identical governance value.** A timezone dropdown + time picker + action selector creates the same rule with near-zero UX risk. The clock is a presentation layer on top of the same data model. Ship the form. Validate the governance thesis. THEN invest in the clock interaction if prototype testing confirms it.
- **feasibility.md already hedges:** "Prepare fallback to conventional dashboard UI if clock UX tests poorly."

**What we ship instead:** The clock DISPLAYS timezones (F6, Priority 4.00). Rules are CREATED via a conventional form (F7, Priority 3.83). The clock is visual context, not the input mechanism. If HG2 passes, the clock becomes the input mechanism in v1.1.

**What happens if we're wrong:** If the clock-as-UI IS the product (users love it, it's the reason they adopt), we've delayed the wow-factor by one release. But we've validated the *governance concept* independently of the *interaction paradigm*. This separation is critical: it tells us whether the value is in "timezone rules for agents" or in "a cool clock interface." We need to know which.

### DNB-6: Trojan Horse Positioning — "Free Clock Utility" (F34, Priority 0.92)

**What it is:** Positioning Delta-T as a free timezone utility that upsells to governance features.

**Why it seems important:** validate.md Channel 1 describes "a clock app on your phone." steal-differentiate-ignore.md frames the free clock as S1 (Steal). The theory: free clock → downloads → governance upsell.

**Why it must die:**
- **Critic-report.md F5:** "The App Store has hundreds of free world clocks. Why would a developer download Delta-T's clock over World Time Buddy?" The free tier competes in a saturated market (World Time Buddy: millions of users, thousands of reviews). Delta-T starts at zero.
- **Critic-report.md F5 continued:** "If the free tier attracts timezone-curious users (not agent-governance-curious users), the funnel fills with people who will never convert. High download count + zero rule creation = vanity metrics."
- **Anti-pattern: Feature Parity Trap.** Competing with World Time Buddy on "clock quality" is fighting an incumbent on their home turf with fewer resources. The correct move is to avoid the timezone-utility battleground entirely.
- **ICP mismatch:** icp.md primary persona is "The Agentic Timezone Juggler" — a developer running AI agents across timezones. This person doesn't need another clock app. They need agent governance. Lead with governance.

**What we do instead:** Position as a developer governance tool from day 1. The clock display is supporting context, not the product. Landing page headline: "Set timezone rules for your AI agents" not "A beautiful world clock."

**ICP pain-point cross-reference:** This cut does NOT conflict with ICP pain points. Pain #3 (timezone coordination) is about scheduling *actions*, not displaying *clocks*. The governance form (F7) addresses Pain #3 directly.

### DNB-7: Multi-Workspace Support (F21, Priority 0.93)

**What it is:** Governing multiple Undisk workspaces from a single Delta-T instance.

**Why it seems important:** icp.md: persona "works across 3+ timezones regularly" with multiple contexts.

**Why it must die:**
- **Critic-report.md C8:** "Single-workspace MVP vs. multi-timezone ICP... acceptable for validation (test the concept with one workspace first)."
- **Effort multiplier:** Each additional workspace multiplies the testing surface for policy interactions, rule conflicts, and UI state management.
- **Anti-pattern: Kitchen Sink.** Multi-workspace is a scale feature, not a validation feature. If governance doesn't work for one workspace, it doesn't work for ten.

### DNB-8: Team/Org Support (F25, Priority 0.39)

**What it is:** Multi-user access, role-based permissions, team billing.

**Why it must die:**
- **steal-differentiate-ignore.md I8:** "Team features are a v2/v3 concern."
- **icp.md primary persona:** Solo developer. The validation target is an individual, not an organization.
- **Anti-pattern: Premature Platform.** Building team features before proving individual value is building the second floor before the foundation.

### DNB-9: Android App (F20, Priority 0.94)

**What it is:** Android app via Tauri mobile.

**Why it must die:**
- **Already cut in feasibility.md:** "❌ Cut" from MVP scope.
- **icp.md:** Primary persona uses "iPhone primary... macOS for development." Android is secondary platform for the ICP.
- **Double the mobile risk:** If Tauri iOS is beta, Tauri Android is equally beta. Adding a second beta platform doubles the debugging surface for a solo founder.

### DNB-10: Open-Source Clock UI Component (F24, Priority 0.80)

**What it is:** Extract the timezone clock as a standalone npm/crates.io package.

**Why it seems important:** validate.md Channel 6 suggests it as an acquisition channel. "GitHub trending → 200 additional installs."

**Why it must die:**
- **Anti-pattern: Sunk Cost Inclusion.** The value is in governance, not in a clock component. A clock component attracts frontend developers, not agent-governance buyers. The ICP overlap is near-zero.
- **Effort:** "2 weeks to extract and polish" (validate.md). That's 20% of the MVP budget for an acquisition channel with 0.1-0.5% conversion to paid (validate.md's own estimate).
- **Sequencing:** An open-source component makes sense AFTER the product proves value, as a growth lever. Before product-market fit, it's a distraction.

---

## The Hard No List — Never Build, Regardless of Demand

These features are scope poison. They should never enter the roadmap regardless of user requests, competitive pressure, or founder enthusiasm.

### HN-1: Custom MCP Server (F29)

**Why never:** feasibility.md is definitive: "The agent connects directly to Undisk's MCP server... The Cloudflare Worker is NOT an MCP server." Building a custom MCP server creates a parallel infrastructure surface to maintain, diverges from the Undisk ecosystem (which the team owns), and adds zero governance value. The Worker is standard HTTP middleware — that's all it needs to be.

### HN-2: Agent-Level Intelligence / LLM Integration

**Why never:** Delta-T governs *when* agents act. It does not govern *how* they act, *what* they produce, or *whether* their output is correct. Adding LLM-based evaluation ("is this agent's code change safe?") enters the agent safety/evaluation space (competitors: Invariant Labs, E2B, various sandboxing tools). This is a different product category. The moat is temporal governance, not agent evaluation.

### HN-3: Custom Deployment Pipeline / CI/CD Integration

**Why never:** Delta-T controls Undisk workspace policies. It does not deploy code, run builds, or trigger CI/CD. Integrating with GitHub Actions, CircleCI, or Jenkins makes Delta-T a deployment tool — a category with massive incumbents and zero temporal-governance differentiation. The governance happens at the Undisk layer; deployment happens elsewhere.

### HN-4: Direct Agent Communication / Agent SDK

**Why never:** feasibility.md: "governance is transparent to the agent... Delta-T modifies the Undisk workspace's policy, and the agent receives standard PERMISSION_DENIED errors." The agent doesn't need to know Delta-T exists. Building an agent SDK or direct agent communication channel violates this architectural principle and creates maintenance obligations with every agent platform (Cursor, Claude Code, Copilot, etc.).

### HN-5: Social / Collaboration Features (Shared Clocks, Team Chat)

**Why never:** This is not a collaboration tool. It's a governance tool for a solo developer. Adding social features enters Slack/Discord territory. The ICP values autonomy, not social interaction within their governance tools.

### HN-6: Mobile Widgets (iOS Home Screen, Android Widget)

**Why never in v1:** feasibility.md explicitly defers: "Android home screen widgets via Tauri custom plugin or native Kotlin bridge are Phase 2." iOS widgets require WidgetKit (Swift), which Tauri cannot generate. Building native widgets requires native code outside the Tauri framework — a second codebase to maintain. Only justifiable after mobile app proves valuable, which requires mobile app to exist, which is itself deferred.

---

## Sequencing

### Build Order (Risk-First, Dependency-Ordered)

```
Week 0 (pre-build, parallel with validation interviews):
├── HG4: Tauri macOS desktop hello-world build [1 day]
│   └── Gate: Can Tauri build a menu bar app that renders a WebView?
│       If NO → evaluate Electron (heavier but proven) or Swift menu bar
├── HG5: Quantify Undisk active user base [1 hour]
│   └── Gate: How many active Undisk users exist?
│       Grounds ALL market-size and distribution assumptions.
└── Figma prototype of clock-as-governance UI [1 week]
    └── Gate: HG2 — ≥75% task completion on "set a deploy gate for TYO 9AM"
        If FAIL → clock is display-only, governance via form (already planned)

Weeks 1-2: Foundation
├── Cloudflare Worker scaffold (F2) [3-4 days]
│   ├── REST API: POST /rules, GET /rules, DELETE /rules
│   ├── Rule evaluation: GET /check?workspace=X&action=deploy
│   └── Auth: API key validation (simple, no OAuth yet)
├── Cloudflare KV integration (F3) [1-2 days]
│   └── Rule schema: { timezone, window_start, window_end, action, workspace }
└── Undisk set_policy integration (F4) [2-3 days]
    ├── Lock: set_policy(mode: "merge", pattern: "**", permission: "read")
    ├── Unlock: set_policy(mode: "merge", pattern: "**", permission: "read-write")
    └── Integration test: create rule → trigger agent write → verify blocked

Weeks 3-4: Desktop Shell
├── Tauri macOS menu bar app (F5) [4-5 days]
│   ├── System tray with persistent icon
│   ├── Click → dropdown showing configured timezones (F6)
│   └── "Add Rule" → conventional form (F7)
├── Undisk audit_trail integration (F8) [1-2 days]
│   └── Display last 5 enforcement events in the menu bar dropdown
└── Local notifications (F9) [1-2 days]
    └── macOS native notification: "Deploy blocked — TYO locked until 09:00"

Weeks 5-6: Integration + Polish
├── End-to-end Journey 1 flow [3-4 days]
│   └── User creates rule → Agent hits Undisk → Worker intercepts → Policy enforces → Notification fires
├── Basic telemetry (F10) [0.5-1 day]
├── Dogfood on Undisk workspace [ongoing]
│   └── Use Delta-T to govern the Undisk development workspace
└── Bug fixes + edge cases [3-4 days]
    ├── DST transitions
    ├── Half-hour timezone offsets (IST +5:30)
    └── Rule conflict resolution (overlapping windows)

Weeks 7-8: Alpha Testing
├── 10-20 alpha users from Undisk community + interview subjects
├── Track: rules created, rules triggered, time-to-first-rule
├── Collect feedback: 5 user interviews (15 min each)
└── Go/no-go for v1.1 scope (Sleep Fence? iOS? Clock UI?)
```

**Total build time:** 8 weeks (compressed from feasibility.md's 10 weeks by cutting Journey 2, Journey 3, iOS, and billing).

### Dependency Graph

```
F4 (set_policy) ← F2 (Worker) ← F1 (Journey 1) ← F5 (macOS app)
                                     ↑
F3 (KV) ─────────────────────────────┘
F8 (audit_trail) ─────────────────────┘
F7 (rule form) ← F5 (macOS app)
F6 (TZ display) ← F5 (macOS app)
F9 (notifications) ← F5 (macOS app) ← F1 (Journey 1)
F10 (telemetry) ← F2 (Worker)
```

Critical path: F4 → F2 → F3 → F1 → F5 → integration testing. The Worker + Undisk integration must work before any UI is built. **Build from the enforcement layer outward.**

---

## Resource Reality Check

### The Two-Product Problem

The founder runs Undisk MCP in production. This is not a side project — it's a service with users, uptime expectations, and ongoing development needs. Building Delta-T while maintaining Undisk is a bandwidth allocation problem, not a technical one.

#### Realistic Weekly Time Budget

| Activity | Hours/Week | Notes |
|----------|-----------|-------|
| **Undisk maintenance** (bug fixes, support, monitoring) | 5-8 | Production service. Cannot be zero. Incidents are unscheduled. |
| **Undisk feature development** | 3-5 | Users expect progress. Stalling Undisk development to build Delta-T risks Undisk user churn. |
| **Delta-T engineering** | 15-20 | The productive build window. Assumes focus blocks of 3-4 hours. |
| **Delta-T validation** (interviews, prototype tests, community seeding) | 3-5 | validate.md requires 15+ interviews in weeks 1-3. At 30 min each + scheduling overhead = ~2 hours/week. |
| **Overhead** (context switching, planning, admin) | 3-5 | Switching between two codebases, two deployment pipelines, two mental models. |
| **Total** | **29-43** | Realistic productive hours for a disciplined solo founder. |

#### What Gets Neglected

Be honest: something will slip. Rank order of likely neglect:

1. **Undisk feature development** (first to slip) — Maintenance continues but new features freeze for 8 weeks. Acceptable if Undisk's current feature set is sufficient. Risk: Undisk users notice stagnation and evaluate alternatives.

2. **Delta-T validation quality** (second to slip) — Under time pressure, the founder conducts fewer interviews (10 instead of 15), skips the Figma prototype test, or rushes the landing page. This is the most dangerous neglect — it undermines the entire validation thesis.

3. **Delta-T engineering quality** (third to slip) — Corners cut on testing, edge cases ignored, DST handling skipped. Technical debt compounds and creates production bugs during alpha.

#### Mitigation: The 2-Day Undisk Block

Block Monday + Tuesday for Undisk. Wednesday through Friday for Delta-T. This creates predictable context-switching boundaries and prevents Undisk interruptions from fragmenting Delta-T focus blocks.

Exception: Undisk production incidents override everything. Build a 1-week buffer into the timeline (8 weeks becomes 9 weeks with contingency).

#### The Undisk Synergy (Genuine Advantage)

The two-product burden has a genuine upside:

- **Dogfooding:** Use Delta-T to govern the Undisk development workspace during alpha. This generates authentic usage data and surfaces edge cases that no amount of prototype testing would find.
- **Undisk API extensions:** If Delta-T needs an Undisk feature (e.g., real-time event stream for live timeline updates), the founder can build it. No vendor negotiation, no feature request ticket, no waiting. (But each Undisk extension consumes from the same time budget — so this "advantage" is also a resource cost.)
- **Cross-promotion:** Undisk docs/changelog can mention Delta-T. Undisk users receive Delta-T launch emails. Zero acquisition cost for the highest-quality leads.

---

## Platform Tradeoffs

### The Four Options

#### Option A: iOS App via Tauri (Current Plan)

| Dimension | Assessment |
|-----------|------------|
| **Pros** | Validates mobile-first USP. "From your phone" is the differentiator. Tauri 2.10.x has mobile support. |
| **Cons** | Tauri mobile is beta/experimental (feasibility.md). iOS background execution limited to 30s BGTaskScheduler. TestFlight requires Apple Developer ($99/yr) + provisioning. App Store submission has IAP risk (critic-report.md F7, 15% kill). Solo founder debugging WebView quirks on beta framework could consume weeks. |
| **Time to ship** | 12-16 weeks (realistic, not the optimistic 10). |
| **Risk** | HIGH — three compounding unknowns: Tauri mobile stability, Apple approval, mobile UX paradigm acceptance. |

#### Option B: Progressive Web App (PWA)

| Dimension | Assessment |
|-----------|------------|
| **Pros** | Cross-platform instantly (iOS Safari, Android Chrome, desktop). No App Store gatekeeping. No Tauri mobile risk. Web technologies the founder already knows. Push notifications via Web Push API. Installable as home screen app. |
| **Cons** | iOS Safari PWA support is limited: no push notifications until iOS 16.4+, no background sync, no badges. "Install to home screen" UX is friction-heavy on iOS (Share → Add to Home Screen). PWAs feel "second-class" compared to native apps — developer tool credibility may suffer. Cannot access system tray on desktop. |
| **Time to ship** | 6-8 weeks. Significantly faster than native. |
| **Risk** | MEDIUM — iOS PWA limitations may undermine the "mobile-first" positioning. But governance enforcement is server-side, so PWA limitations only affect the control plane UX, not the enforcement quality. |

#### Option C: CLI Tool That Extends Undisk (Fastest to Ship)

| Dimension | Assessment |
|-----------|------------|
| **Pros** | Fastest possible path: the Cloudflare Worker already has REST endpoints. A CLI (`delta-t set-rule --timezone "Asia/Tokyo" --unlock-at "09:00" --workspace production`) is a thin HTTP client. Ship in 1-2 weeks. Matches ICP's "autonomy obsession" and CLI-fluency. No UI risk. No platform risk. Validates governance concept with zero UX noise. |
| **Cons** | Abandons the entire visual identity (clock, timezones rendered graphically). No "wow factor" for HN/Product Hunt launch. Doesn't validate the "mobile-first" or "clock interface" USP — only validates "do developers want timezone rules for agents?" Difficult to differentiate from "just write a cron job." |
| **Time to ship** | 1-2 weeks (CLI client) + 2-3 weeks (Worker backend) = 3-5 weeks total. |
| **Risk** | LOW — but validates only the demand hypothesis, not the UX hypothesis. If the CLI gets traction, it proves demand exists but says nothing about whether a clock UI or mobile app would improve the experience. |

#### Option D: Desktop Menu Bar App via Tauri (Recommended)

| Dimension | Assessment |
|-----------|------------|
| **Pros** | Tauri desktop is production-stable (not beta). Menu bar apps are Tauri's strength — the SystemTray API is documented and proven. Zero App Store risk (direct download). macOS is the ICP's primary dev platform (icp.md). Menu bar persistence matches "persistent background + burst interaction" usage pattern (steal-differentiate-ignore.md S2). Can display clock + timezone info + rule status in a compact dropdown. Can include a conventional rule-creation form. Validates governance concept AND provides a visual product (not just CLI). |
| **Cons** | Not mobile. The USP's "from their phone" claim is deferred. Limits the "mobile-first" positioning to "desktop-first, mobile later." Requires the USP to be softened (critic-report.md C2 recommends this anyway). |
| **Time to ship** | 6-8 weeks. Same as PWA but with native desktop polish (system tray, notifications, auto-start). |
| **Risk** | LOW — Tauri desktop is the framework's sweet spot. The founder has Rust experience (Undisk is Rust-based). No App Store gatekeeping. |

### Decision: **Option D — Desktop Menu Bar App (macOS only)**

**Justification:**

1. **De-risks the unvalidated:** The three highest-risk unknowns are demand (F1, 40% kill), clock-as-UI (F3, 25% kill), and Tauri mobile (F5, 20% kill). Option D eliminates the Tauri mobile risk entirely and isolates the clock-as-UI risk (test via prototype, ship with conventional form). It focuses all validation energy on the demand question — the only question that matters right now.

2. **Plays to Tauri's strength:** Tauri was built for desktop apps. Its mobile support was added later and is explicitly labeled beta. Building on the stable foundation (desktop) rather than the experimental one (mobile) is the engineering-rational choice.

3. **Preserves the mobile option:** If demand validates and interviews show mobile preference (HG3), building iOS in v1.1 is a known-scope, data-informed investment. The Cloudflare Worker + Undisk integration is UI-agnostic — it works identically whether the client is a desktop app, mobile app, PWA, or CLI.

4. **Matches the founder's stack:** Undisk is built in Rust. Tauri is Rust-based. The founder's strongest technical advantage is in the Rust ecosystem. Building a Tauri desktop app leverages existing skills. Building a React Native iOS app (the Tauri mobile fallback) does not.

5. **Critic-report.md alignment:** F2 mitigation explicitly recommends: "Ship macOS desktop first (Tauri desktop is stable), validate governance concept, THEN invest in iOS."

**USP Implication:** The USP must be softened from "controlled entirely from a clock interface on their phone" to "controlled from a temporal governance interface on your Mac — with mobile coming when demand confirms it." This is an honest reframe, not a retreat. Critic-report.md C2 already flags this as a required update.

**Why not Option C (CLI)?** The CLI validates demand but produces no visual artifact for HN/Product Hunt/social media. Developer tools are evaluated from screenshots and demos. A menu bar app with a timezone display and rule-status indicators is dramatically more marketable than a CLI. The marginal effort (4-6 weeks over CLI) buys the visual identity that acquisition channels require.

**Why not Option B (PWA)?** PWAs are a reasonable choice, but iOS Safari PWA limitations (no reliable push, no background sync, friction-heavy install) undermine the "from your phone" experience. If we're going to defer mobile, we should defer it cleanly rather than shipping a compromised mobile experience via PWA. And on desktop, a native Tauri menu bar app is strictly better than a PWA (system tray persistence, native notifications, auto-start).

---

## Rationale

### Prioritization Methodology

Every feature was scored on the four-dimension decision matrix (Impact, Strategic Alignment, Effort, Risk) using the constitutional formula: `Priority = (Impact × 2 + SA × 1.5) / (Effort + Risk)`. The cut line is Priority = 1.0.

Three sources grounded the scoring:

1. **Impact scores** derived from validate.md's North Star Metric (Governance Rules Enforced/Week). Features that directly increase rule creation or enforcement scored highest. Features that improve UX without affecting rule creation scored lower.

2. **Effort scores** derived from feasibility.md's timeline estimates and the Resource Reality Check (solo founder, ~15-20 productive Delta-T hours/week, ~300 total hours for MVP).

3. **Risk scores** derived from critic-report.md's kill probabilities and unresolved risks. Features flagged as unvalidated (clock-as-UI, mobile preference, demand itself) received Risk=4. Features using proven infrastructure (KV, Workers, Undisk API) received Risk=1.

### Key Tradeoff Decisions

**1. Governance-first, not clock-first.**

The pipeline's narrative arc — "beautiful clock → governance upsell" — is reversed. We lead with governance value and use the clock as supporting context. This contradicts validate.md's "trojan horse" strategy but aligns with critic-report.md F5's warning about funnel dilution. The ICP doesn't need another clock. They need agent governance.

**2. Conventional form over clock interaction.**

The clock-as-governance-UI is the product's most distinctive interaction — and its highest UX risk. Shipping a conventional rule-creation form (timezone dropdown + time picker + action selector) delivers identical governance value with near-zero UX risk. The clock remains as a display element. If HG2 prototype testing validates the clock interaction (≥75% task completion), it becomes the primary input in v1.1. This separation lets us validate governance demand independently of governance UX.

**3. macOS desktop over iOS mobile.**

The USP says "from their phone." We're shipping "from your Mac" instead. This is a deliberate, documented deviation justified by: (a) enforcement is server-side regardless of client platform, (b) Tauri mobile is beta, (c) mobile preference is unvalidated (HG3), (d) the founder's time budget cannot absorb iOS platform risk. The USP must be updated to reflect this.

**4. One journey, not three.**

feasibility.md proposes Journey 1 + basic Journey 3. We cut to Journey 1 only. The 10-week timeline is optimistic for a founder maintaining Undisk simultaneously. Cutting to one journey compresses the build to 6-8 weeks and focuses all validation on the simplest possible test: "will a developer create a timezone rule for their Undisk workspace?"

**5. Strategic override review: Sleep Fence (F12, Priority 1.71)**

Sleep Fence addresses ICP Pain #2 (top-3 pain point per icp.md). Under the Strategic Override Protocol, this requires documentation:

- ✅ Addresses a top-3 ICP pain point (Pain #2: "I can't sleep because my agents might break things")
- ✅ Critic-report.md flags it as 10% kill (F6, medium severity), not critical
- ⚠️ However, the demand for Sleep Fence is contingent on demand for governance in general (F1, 40% kill). Building it before validating demand is premature.

**Override denied.** Sleep Fence enters scope in v1.1 only after Journey 1 adoption data confirms governance demand. Building it now violates the principle of validating the atomic hypothesis first.

### Anti-Pattern Audit

| Anti-Pattern | Status | Evidence |
|--------------|--------|----------|
| **Kitchen Sink** | ✅ Avoided | 10 features in MVP vs. 34 total candidates. 24 features cut. |
| **Premature Platform** | ✅ Avoided | No multi-workspace, no team support, no plugin system, no public API. |
| **Feature Parity Trap** | ✅ Avoided | Not competing with World Time Buddy on clock quality. Not competing with Temporal.io on workflow orchestration. Governance-only positioning. |
| **Sunk Cost Inclusion** | ✅ Avoided | Clock-as-UI was the original concept; it's deferred to conditional status despite being the product's visual identity. |
| **Founder Pet Feature** | ⚠️ Watch item | The clock interface is likely a founder pet feature (it's the original product vision). Deferring it to conditional status is the correct decision but may face founder resistance. The decision matrix score (1.69) is documented and defensible. |

---

## Quality Score

| Criterion | Score (1-5) | Notes |
|-----------|-------------|-------|
| **Specificity** | 5 | Every feature scored on 4 dimensions with explicit numbers. Every exclusion cites a specific critic-report.md finding (F1-F7), feasibility.md constraint, or validate.md threshold. Platform decision evaluates 4 options with pros/cons/timeline/risk. Resource reality check quantifies weekly hours and identifies what gets neglected. |
| **Actionability** | 5 | Week-by-week build sequence with dependency graph. Specific gates for conditional features (HG1-HG6). "What we ship instead" for every cut feature. Platform decision is unambiguous (Option D, macOS menu bar). Total effort estimated (21-31 days). |
| **Non-redundancy** | 5 | No artifact in the pipeline answers "what to cut." Critic-report.md identifies risks but doesn't make scope decisions. Feasibility.md evaluates technical viability but doesn't prioritize. Validate.md plans acquisition but doesn't address build scope. This is the only artifact that makes explicit build/cut decisions with cited rationale. |
| **Evidence quality** | 4 | All exclusions cite specific upstream artifacts with section-level references. Kill probabilities from critic-report.md. Effort estimates from feasibility.md timeline. Market data from validate.md projections. Score not 5 because Impact scores for governance features are necessarily model-sourced — no real usage data exists yet (this is a pre-validation product). |
