---
artifact_meta:
  produced_by: "os.critic"
  produced_at: "2026-07-20T01:00:00Z"
  confidence: 0.68
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
  stale_after: "on_input_change"
  revision: 3
  quality_scores:
    specificity: 5
    actionability: 5
    non_redundancy: 5
    evidence_quality: 4
  grade: "draft"
  replay_context: "Undisk MCP is owned by the same team building Delta-T. Internal dependency, not external."
---

# Critic Report — Delta-T Zaman: The Temporal Command Center (r3)

## Executive Summary

Delta-T Zaman is an analytically elegant product hypothesis built on a structurally sound foundation — now that Undisk ownership is internal, the architecture is vertically integrated and the single most lethal risk from r2 (existential vendor dependency) is dead. **But eliminating the Undisk risk does not magically create demand.** The pipeline's fatal weakness remains unchanged: there is still zero first-party evidence that any developer has ever asked for, searched for, or expressed willingness to pay for "timezone-gated AI agent governance." Ten internally consistent artifacts reinforcing each other do not constitute market validation. The Undisk ownership creates genuine strategic advantages — built-in distribution, API-level customization, revenue synergy — that modestly improve the acquisition outlook and significantly de-risk the technical execution. However, the product still attempts to create a category that may not exist, depends on an unvalidated UX paradigm (clock-as-governance-interface), and asks a solo founder (even one who already shipped Undisk) to build a multi-platform app on a beta mobile framework within 10 weeks. The odds improved from r2 to r3. They are not yet favorable.

---

## Cross-Artifact Contradictions

### C1: SOM Fantasy vs. Actual Growth Projections (RETAINED from r2)

- **Artifact A:** market-map.md: "The real SOM target is the ~500K-2M developers... At $10-20/month, this represents a $60M-480M annual revenue opportunity."
- **Artifact B:** validate.md OPTIMISTIC scenario: 120 paying customers at month 6 = $2,280/mo MRR ($27,360 annualized). MODERATE scenario: 40 paying at month 6 = $760/mo MRR ($9,120 annualized).
- **Severity:** HIGH
- **Resolution required:** The SOM claim is 4 orders of magnitude above realistic projections. market-map.md's $60M minimum implies 263K paying customers at $19/mo. validate.md's best case reaches 120 in 6 months. Either the SOM must be rewritten as a bottom-up estimate (realistic year-1 SOM: $50K-$250K ARR) or validate.md must explain a credible path from 120 customers to 263K. Neither artifact addresses this gulf.

### C2: "Mobile-First" USP vs. MVP That Ships Beta iOS (RETAINED from r2)

- **Artifact A:** usp.md: "Delta-T Zaman is the only mobile-first temporal governance layer... controlled entirely from a clock interface on their phone."
- **Artifact B:** feasibility.md: MVP cuts Android entirely, ships iOS via TestFlight only (not App Store), labels Tauri mobile "beta/experimental," cuts Journey 2 (Sleep Fence), and delivers Journey 3 (Undo Slider) as "basic (list + single restore)" only.
- **Severity:** HIGH
- **Resolution required:** A TestFlight beta of 1/3 journeys on a "beta/experimental" framework is not "mobile-first." Either the USP must be softened to "mobile-capable" or the MVP must actually deliver a production-quality iOS experience. The USP creates an expectation the MVP cannot meet.

### C3: Urgency Thesis vs. Validation Timeline (RETAINED from r2)

- **Artifact A:** why-now.md: "The optimal window is 2025-2027... Every month of delay increases the probability that an incumbent builds this as a feature."
- **Artifact B:** validate.md: 4-6 weeks pre-build validation + 10 weeks MVP + 8 weeks post-launch = ~24 weeks before Phase 2 decision. 
- **Severity:** MEDIUM
- **Resolution required:** If the window is 2025-2027, burning half of 2026 on sequential validation is risky. Recommend: overlap validation with MVP build (prototype test weeks 1-4 in parallel with scaffold/edge-state engineering). Alternatively, acknowledge that the urgency is overstated — the incumbents why-now.md names (GitHub, Cloudflare, Anthropic) build platforms, not niche governance UIs.

### C4: ICP Specificity vs. Acquisition Channel Breadth (RETAINED from r2)

- **Artifact A:** icp.md: Primary persona requires AI agents + 3+ timezones + MCP + mobile preference — a very narrow intersection.
- **Artifact B:** validate.md targets r/programming (6.5M members), r/MachineLearning (3.1M), Product Hunt — mass-market channels where the ICP intersection is <0.1%.
- **Severity:** MEDIUM
- **Resolution required:** validate.md correctly prioritizes AI agent communities and MCP ecosystem as P0. But the user projection scenarios include HN and Reddit as primary traffic sources. These channels deliver volume, not ICP precision. Per-channel ICP-match estimates should be added to ground the projections.

### C5: feasibility.md R1 vs. Actual Ownership (NEW — Undisk Reframe)

- **Artifact A:** feasibility.md Risk R1: "Undisk single-vendor dependency — Undisk changes API, raises prices, or becomes a competitor." Rated Medium likelihood, High impact. Mitigation: "Abstract Undisk calls behind a workspace interface. Monitor Undisk roadmap. Maintain relationship with Undisk team."
- **Artifact B:** Reality: The Delta-T team IS the Undisk team. The same solo founder owns and operates both products. R1's framing as an external vendor risk is factually incorrect.
- **Severity:** HIGH (artifact accuracy, not product risk)
- **Resolution required:** feasibility.md R1 must be rewritten. The risk is not "Undisk changes API" — the team controls the API. The actual risk is *opportunity cost*: maintaining Undisk while building Delta-T stretches a solo founder's bandwidth (see F3 below). The YELLOW verdict in feasibility.md cites Undisk dependency as one of three reasons — with that eliminated, the verdict should be reassessed toward GREEN with caveats.

### C6: validate.md Checklist #6 vs. Actual Ownership (NEW — Undisk Reframe)

- **Artifact A:** validate.md pre-build checklist item #6: "Undisk partnership confirmed — Direct communication with Undisk team confirming: (a) API stability commitment, (b) no plans to build competing mobile governance UI, (c) willingness to co-market."
- **Artifact B:** The team IS Undisk. "Direct communication with Undisk team" is talking to yourself. API stability is self-determined. Co-marketing is self-directed.
- **Severity:** MEDIUM (checklist noise, not product risk)
- **Resolution required:** Replace checklist item #6 with: "Undisk API surface confirmed — Document which Undisk MCP tools Delta-T requires (`set_policy`, `restore_version`, `workspace_checkpoint`, `audit_trail`, `list_changes`, `list_versions`, `get_diff`) and confirm all are stable in current Undisk release. Identify any Undisk API extensions needed for Delta-T and schedule their development." This is actionable; "confirm partnership with yourself" is not.

### C7: usp.md Confidence Limiter #3 vs. Actual Ownership (NEW — Undisk Reframe)

- **Artifact A:** usp.md Confidence Limiter #3: "The USP's 'workspace-level undo' depends entirely on Undisk's MCP tools. If Undisk changes its API, deprecates tools, or becomes a competitor, the USP's undo claim breaks. This is a single-vendor dependency for a core USP pillar."
- **Artifact B:** Undisk is owned by the same team. It cannot "become a competitor" to itself.
- **Severity:** MEDIUM (analytical accuracy)
- **Resolution required:** Rewrite Confidence Limiter #3 to: "The USP's workspace-level undo depends on Undisk MCP tools maintained by the same team. Risk: If Undisk's priorities shift to other features, Delta-T's undo quality degrades due to bandwidth constraints. Mitigation: Delta-T requirements drive Undisk roadmap (internal prioritization, not negotiation)."

### C8: Single-Workspace MVP vs. Multi-Timezone ICP (RETAINED from r2)

- **Artifact A:** icp.md: ICP "works across 3+ timezones regularly" with multiple contexts.
- **Artifact B:** feasibility.md: MVP supports single Undisk workspace only; multi-workspace cut to Phase 2.
- **Severity:** MEDIUM
- **Resolution required:** The ICP manages complexity across contexts, but MVP governs one workspace. This is acceptable for validation (test the concept with one workspace first) but should be acknowledged in marketing. "Temporal Command Center" for a single workspace is "temporal lock for one room."

---

## Evidence Quality Audit

| Artifact | Verified Claims | Model-Sourced Claims | Untagged | Total (est.) | Model-Sourced + Untagged % | Flag? |
|---|---|---|---|---|---|---|
| sketch.md | 0 | 0 | ~15 (all [ASSUMED]/[KNOWN]) | ~15 | 100% | ⚠️ By design — hypothesis grade |
| market-map.md | ~14 | ~4 | ~3 | ~21 | 33% | ✅ Acceptable |
| icp.md | ~12 | ~8 | ~5 | ~25 | 52% | ⚠️ **Pain points 1, 2, 4 are model-sourced** — these drive product decisions |
| why-now.md | ~30 | ~3 | ~2 | ~35 | 14% | ✅ Strongest evidence base |
| competitor-matrix.md | ~40 | ~15 | ~3 | ~58 | 31% | ✅ Acceptable |
| ux-teardown.md | ~19 | ~12 | ~2 | ~33 | 42% | ⚠️ UX judgments are inherently model-sourced |
| steal-differentiate-ignore.md | ~12 | ~5 | ~5 | ~22 | 45% | ⚠️ Differentiation claims are unvalidated |
| usp.md | ~15 | ~8 | ~2 | ~25 | 40% | ⚠️ Moat claims and category economics are model-sourced |
| feasibility.md | ~20 | ~8 | ~3 | ~31 | 35% | ✅ But vendor perf claims need independent benchmarks |
| validate.md | ~8 | ~25 | ~5 | ~38 | 79% | 🔴 **CRITICAL — highest model-sourced %. All projections are guesses** |

**Weakest evidence base:** validate.md (79% model-sourced) — the artifact that drives the go/no-go decision has the least verified data. Every conversion rate, channel reach estimate, and user projection is an educated guess.

**Second weakest:** icp.md (52% model-sourced) — the five pain points are the emotional core of the entire pipeline. Pain #1 ("My agents don't know what time it is"), Pain #2 ("I can't sleep because my agents might break things"), and Pain #4 ("I have no temporal audit trail") are constructed by inference from separate data points (29% AI trust, 98% timezone pain), not sourced from user interviews. These are *hypotheses* presented as *profiles*.

**Strongest:** why-now.md (14% model-sourced) — nearly every claim is verified with named sources. The timing thesis is the pipeline's most defensible component.

---

## Failure Scenarios

### F1: Invented Category with Zero Demand Evidence

- **Severity:** `critical`
- **Why this will fail:** "Temporal governance for AI agents" is a category constructed by the pipeline, not expressed by users. The pipeline combines two verified but separate data points — 98% timezone pain (Buffer 2024) and 29% AI trust (Stack Overflow 2025) — and infers they create a combined need. But timezone pain is solved by World Time Buddy. Agent trust is addressed by sandboxing, testing, and rollback. No artifact cites a single Reddit post, HN comment, GitHub issue, or X thread where a developer says "I wish I could set timezone rules for my AI agents from my phone." The demand is inferred, not observed.
- **Offline/Edge case risks:** Developers who DO want agent governance may want simple on/off kill switches, not timezone-gated windows. The "temporal" layer may be unnecessary complexity. Cron + GitHub Actions may be "good enough" for 90% of the need.
- **Distribution bottlenecks:** You can't market a category that doesn't exist yet. The messaging ("temporal governance layer") requires education before conversion. Every acquisition channel requires explaining what the product IS before explaining why to buy it.
- **Kill probability:** 40% (down from 45% in r2 — Undisk ownership creates a built-in audience of MCP-savvy developers who understand workspace governance, reducing the education burden slightly)
- **Mitigation:** Execute validate.md's 15+ interview plan as the hard gate. Question 6 ("rank your top 5 pain points with AI coding agents — where does controlling when they run fall?") is the kill test. If <33% rank it top-5, PIVOT immediately. The Undisk user base provides a warm audience for these interviews — start there.

### F2: Solo Founder Scope Across Two Products

- **Severity:** `high`
- **Why this will fail:** The previous r2 identified "solo founder attempting multi-platform infrastructure product" as F3. The Undisk ownership context *changes* this flaw but does not eliminate it — it makes it *worse in one dimension*. The solo founder now maintains TWO products: Undisk MCP (a production service with paying users) AND Delta-T Zaman (a new product requiring iOS app + Cloudflare middleware + novel UI). Every hour spent on Delta-T is an hour not spent on Undisk maintenance, bug fixes, and feature development. feasibility.md estimates 10 weeks for the MVP, but the founder must also keep Undisk running during that period.
  - Tauri mobile is "beta/experimental" — debugging iOS WebView quirks could consume weeks
  - The clock-as-governance UI is a novel interaction model with no precedent — design iteration is unpredictable
  - Cloudflare Worker middleware + KV/D1 + Cron Triggers + Undisk MCP integration = 4 distinct infrastructure surfaces
  - TestFlight distribution requires Apple provisioning, code signing, notarization
  - The real timeline is likely 20-30 weeks, not 10
- **Offline/Edge case risks:** If Undisk has a production incident during Delta-T's build phase, everything stops. The founder context-switches between two codebases, two deployment pipelines, two user bases. Cognitive overhead compounds.
- **Distribution bottlenecks:** The solo founder must also execute validate.md's acquisition strategy (HN posts, community engagement, 15+ interviews, landing page, prototype testing) while building the product and maintaining Undisk. Marketing velocity for a solo founder is inherently limited.
- **Kill probability:** 30% (down from 35% in r2 — the founder's proven ability to ship and operate Undisk demonstrates execution capability, and AI coding agents accelerate development)
- **Mitigation:** (1) Ruthlessly scope MVP to Journey 1 only (Deploy Gate — the simplest). (2) Ship macOS desktop first (Tauri desktop is stable), validate governance concept, THEN invest in iOS. (3) Use Undisk's own infrastructure as the test bed — dogfood Delta-T on the Undisk workspace. (4) Block 2 days/week for Undisk maintenance during the Delta-T build sprint; don't pretend Undisk runs itself. (5) Set a week-6 milestone: if Cloudflare Worker + KV + set_policy integration is not working end-to-end, re-scope.

### F3: Clock-as-UI Is a High-Risk Bet With No Escape Hatch

- **Severity:** `high`
- **Why this will fail:** No precedent exists for using a clock as a governance control interface. usp.md's Confidence Limiter #4 admits: "no precedent for clock-as-governance-UI exists." Governance rules require structured data entry: select timezone, specify time windows (start/end), choose action (block writes, pause agents), select scope (workspace, paths). Clocks display time — they are not input forms. Every developer governance tool (Temporal's dashboard, n8n's flow builder, GitHub Actions YAML, Kubernetes RBAC) uses structured forms, code, or visual flows. The "stark, zero-CSS" design language may read as "incomplete" rather than "intentional." validate.md sets a 60% task completion threshold for prototype testing — meaning 4/10 testers can FAIL the core interaction and the team still proceeds. A 40% failure rate is not a "bold design choice" — it's a usability crisis.
- **Offline/Edge case risks:** Edge cases in timezone governance (DST transitions, half-hour offsets like IST +5:30, Lord Howe Island +10:30/+11:00) may be impossible to represent legibly on a clock face. Complex multi-rule configurations (different rules for different workspaces/paths) may not map to a clock metaphor at all.
- **Distribution bottlenecks:** If the clock UI doesn't photograph well in screenshots, the HN/Reddit/Product Hunt launches lose their visual hook. Developer tools are often evaluated from a README screenshot — if the UI looks confusing in a still image, users won't download.
- **Kill probability:** 25% (unchanged from r2 — Undisk ownership doesn't affect UX risk)
- **Mitigation:** (1) Build the Figma prototype IMMEDIATELY — before any engineering. (2) Raise the success threshold from 60% to 75% task completion (I'm revising down from r2's 80% recommendation — 75% is aggressive enough to be meaningful while acknowledging the learning curve of a novel paradigm). (3) A/B test clock UI vs. conventional form-based UI in the prototype. If the conventional UI wins, use it — the governance concept is more important than the clock metaphor. (4) Design the backend to be UI-agnostic: Cloudflare Worker API works regardless of frontend paradigm.

### F4: The Phantom Market — Realistic Intersection Is 3K-25K, Not 500K-2M

- **Severity:** `high`
- **Why this will fail:** market-map.md claims 500K-2M developers in the SOM. Decomposing the Venn diagram:
  - 4.5M US software devs (BLS) × 45% fully remote = 2M remote devs
  - Of those, ~25% work across 3+ timezones = 500K (generous — most cross 1-2 US timezones)
  - Of those, ~20% use AI agents daily (not just autocomplete) = 100K
  - Of those, ~10-25% connect agents via MCP = 10K-25K developers
  
  The realistic intersection is **10,000-25,000 developers worldwide** (revised up from r2's 3K-22K — MCP adoption has grown, and Undisk's user base provides a known floor). At 3% conversion × $19/mo = 300-750 paying customers = $5,700-$14,250/mo MRR. This is a strong indie business, not a venture-scale opportunity.
  
  **New consideration:** Undisk ownership provides a known starting audience. If Undisk has even 1,000 active users, that's a qualified lead pool with near-zero acquisition cost. This partially mitigates the market size concern — you don't need 500K developers if you have 1,000 who already use your workspace platform.
- **Offline/Edge case risks:** If the MCP ecosystem grows slower than projected (why-now.md's "tens of thousands" is aggregate servers, not individual developer users), the addressable market stays small for years.
- **Distribution bottlenecks:** The narrow ICP means mass-market channels (HN, Reddit, Product Hunt) have terrible signal-to-noise ratios. The highest-conversion channel is the Undisk user base itself — but its size is undisclosed even to the pipeline.
- **Kill probability:** 15% (down from 20% in r2 — Undisk ownership provides a known audience floor and a direct distribution channel. Even a small market is reachable if you own the distribution.)
- **Mitigation:** (1) Quantify Undisk's active user base — this is the most important market sizing data point available, and the team has it. (2) Reframe the SOM honestly: "Year 1 target: 500-2,000 developers from the Undisk/MCP ecosystem. Year 2-3 target: 5,000-15,000 as MCP adoption grows." (3) Plan the business as a profitable indie product ($5K-$20K MRR), not a category-creation play. The venture narrative is aspirational; the indie narrative is buildable.

### F5: "Free Timezone Utility → Paid Governance" Conversion Funnel May Not Convert

- **Severity:** `medium`
- **Why this will fail:** steal-differentiate-ignore.md and validate.md describe a "trojan horse" strategy: the free timezone clock drives downloads, governance features are the paid upsell. This mirrors Cursor's model (free editor → paid AI). But there's a critical difference: Cursor's free tier delivers immediate, obvious value (code completion). Delta-T's free tier is... a world clock. The App Store has hundreds of free world clocks. Why would a developer download Delta-T's clock over World Time Buddy (which already has mobile apps and millions of users)? The free tier must be compelling on its own merits to attract the top-of-funnel, but the pipeline doesn't explain why Delta-T's clock is better than existing free clocks *before* the governance features are relevant.
- **Offline/Edge case risks:** If the free tier attracts timezone-curious users (not agent-governance-curious users), the funnel fills with people who will never convert. High download count + zero rule creation = vanity metrics.
- **Distribution bottlenecks:** App Store search for "world clock" returns hundreds of incumbents with thousands of reviews. Delta-T starts at zero reviews. ASO (App Store Optimization) is not a viable channel for a new entrant in a saturated category.
- **Kill probability:** 20%
- **Mitigation:** (1) Don't compete on "world clock." Position the free tier as "developer timezone tool with MCP governance" from day 1. The ICP doesn't want another clock — they want the governance. Lead with the governance pitch; the clock is the UX, not the hook. (2) Distribute through developer channels (MCP directories, Undisk docs, agent community tutorials), not app store search. (3) Track the ratio of "clock-only users" to "governance users" — if >80% never create a rule, the trojan horse failed. Pivot to governance-first positioning.

### F6: Cloudflare Cron + KV Eventual Consistency Creates Trust-Violating Edge Cases

- **Severity:** `medium`
- **Why this will fail:** The Sleep Fence (Journey 2) depends on Cloudflare Cron Triggers firing at exact timezone-adjusted times and KV propagating the policy change within 60 seconds. The trust proposition is: "I can sleep knowing agents are paused." But:
  - Cloudflare Cron Triggers have documented jitter — they fire "approximately" at the scheduled time, not exactly
  - KV eventual consistency means a 0-60 second window where the old policy is still active
  - An agent that starts a long-running write operation 5 seconds before the fence activates will complete that operation — the fence doesn't kill in-flight operations
  - If the developer sets a fence at 11 PM and an agent writes at 10:59:58 PM, the write succeeds. The developer wakes up to find the agent "violated" their sleep schedule.
- **Offline/Edge case risks:** DST transitions create 1-hour ambiguity windows twice per year. A fence set for "11 PM CDT" during the DST transition may fire at the wrong absolute time. Cloudflare's TZ-aware cron handles this, but the user's mental model may not match the actual behavior.
- **Distribution bottlenecks:** If early users report "my agent wrote files during my sleep fence," trust in the product collapses. Developer tools live and die on reliability.
- **Kill probability:** 10%
- **Mitigation:** (1) Use `set_policy` as the synchronous enforcement mechanism (as feasibility.md already suggests) — don't rely solely on KV propagation. (2) Document the ~60s activation window honestly. (3) For the Sleep Fence, fire the Cron Trigger 2 minutes early and set the policy synchronously. (4) Add a "fence is active" confirmation push notification so the developer has proof. (5) Test DST transitions explicitly before shipping Journey 2.

### F7: Apple App Store Rejection Risk for WebView + Web Billing

- **Severity:** `medium`
- **Why this will fail:** feasibility.md recommends the "reader app" billing pattern (Stripe web checkout, no IAP). Apple's enforcement of IAP rules is inconsistent and unpredictable. A WebView-based app (Tauri) that manages subscriptions via web is exactly the pattern Apple has historically challenged. If Apple rejects:
  - Option A: Implement IAP at 30% cut → $19/mo becomes ~$25/mo to maintain margin → exceeds ICP willingness-to-pay zone
  - Option B: Distribute macOS-only via direct download → loses "mobile-first" USP entirely
  - Option C: Fight the rejection → weeks of delay, uncertain outcome
- **Offline/Edge case risks:** Apple may approve initially but reject on a future update, creating a rug-pull scenario after users have been acquired.
- **Distribution bottlenecks:** Without App Store distribution, iOS acquisition requires TestFlight (limited to invited users) or enterprise distribution (requires $299/yr program). Both limit growth.
- **Kill probability:** 15%
- **Mitigation:** (1) Ship macOS direct-download first (no Apple review). (2) Submit iOS app with IAP as the billing method (accept the 30% cut initially — at $19/mo, net is $13.30/mo, still above infrastructure costs). (3) If the product proves PMF, renegotiate billing strategy. (4) Consider the "reader app" exemption only after legal review confirms eligibility.

---

## Mitigations

| Failure | Severity | Mitigation | Confidence in Mitigation |
|---|---|---|---|
| F1: Invented demand | `critical` | Execute 15+ ICP interviews with hard kill threshold (<33% = PIVOT). Start with Undisk user base as warm leads. | Medium — interviews will produce signal, but 15 is a small sample |
| F2: Solo founder × 2 products | `high` | Ruthless MVP scoping (Journey 1 only, macOS first), dedicated Undisk maintenance blocks, week-6 milestone gate | Medium — founder has shipped before, but scope is unprecedented |
| F3: Clock-as-UI risk | `high` | Figma prototype with 75% task completion threshold, A/B test vs. conventional UI, UI-agnostic backend | High — the prototype test is cheap and decisive |
| F4: Phantom market | `high` | Quantify Undisk user base, reframe SOM to indie-scale, target Undisk/MCP channels exclusively | Medium-High — Undisk ownership provides a floor |
| F5: Free tier doesn't convert | `medium` | Governance-first positioning, developer-channel distribution, monitor clock-only vs. governance user ratio | Medium — positioning is adjustable |
| F6: Trust-violating edge cases | `medium` | Synchronous set_policy enforcement, 2-min early trigger, confirmation push, DST testing | High — engineering solutions are well-defined |
| F7: Apple rejection | `medium` | Ship macOS first, submit iOS with IAP, legal review for reader-app exemption | Medium — multiple fallback options exist |

---

## Unresolved Risks

### UR1: Demand Validation (F1) — `critical`

**Status: UNRESOLVED — blocking.**

No mitigation exists that can resolve this risk *before validation occurs*. The 15+ interview plan is the test, not the mitigation. If <33% of interviewees rank temporal governance as a top-5 pain point, there is no fallback — the product thesis is invalidated. The Undisk user base provides a warm audience for interviews, which improves the quality of the test, but does not change the binary outcome.

**Resolution path:** Complete the interviews. This is the single most important pre-build activity. Every day spent on engineering before this gate is passed is a day potentially wasted.

### UR2: Clock-as-UI Acceptance (F3) — `high`

**Status: PARTIALLY UNRESOLVED.**

The Figma prototype test is a credible resolution mechanism, but it hasn't been executed. If the clock UI fails at <50% task completion, the team must pivot to a conventional dashboard UI. This pivot invalidates the USP's "controlled entirely from a clock interface" claim and requires re-running usp.md through os.usp to produce a new positioning statement. The 4-8 week delay for a UI pivot during the build phase could collapse the timeline.

**Resolution path:** Run the prototype test in parallel with Week 1-2 of the MVP scaffold. If results arrive by week 2 and the clock UI passes, proceed. If it fails, the scaffold week was not wasted (backend is UI-agnostic).

### UR3: Mobile-First Preference Unvalidated — `high`

**Status: UNRESOLVED.**

usp.md Confidence Limiter #2: "If developers prefer laptop-based control... the mobile-first positioning may be a liability." validate.md plans to test this in interviews (Question 9). PagerDuty and Datadog prove developers *monitor* from phones, but *write-capable governance* from mobile has no precedent. If <40% of interviewees prefer phone control, the "mobile-first" USP dies and the product must be repositioned as "desktop + mobile."

**Resolution path:** Include Question 9 in the validation interviews. Treat this as a soft gate — mobile *preference* doesn't need to be >50%, but mobile *acceptance* (willing to use it from phone even if not their first choice) should be >70%.

---

## Upstream Impacts

| Changed Value | Old | New | Source Artifact | Must Update |
|---|---|---|---|---|
| Undisk dependency risk (R1) | Medium likelihood, High impact (feasibility.md) | Eliminated (~0% — internal dependency) | feasibility.md § Risk Matrix R1 | feasibility.md (rewrite R1), usp.md (rewrite Confidence Limiter #3) |
| Undisk partnership checklist item | "Contact Undisk team for partnership" (validate.md #6) | "Document required Undisk API surface and schedule any needed extensions" | validate.md § Pre-Build Validation Checklist #6 | validate.md (rewrite checklist item #6) |
| feasibility.md verdict rationale | YELLOW citing "Undisk dependency" as reason #1 | YELLOW should be reassessed — one of three YELLOW reasons is eliminated | feasibility.md § Verdict | feasibility.md (reassess verdict; may upgrade to GREEN with caveats) |
| Prototype success threshold | 60% task completion (validate.md) | 75% task completion (critic recommendation) | validate.md § Signal 2: Solution Validation | validate.md (raise threshold) |
| SOM estimate | $60M-$480M (market-map.md) | $50K-$250K year 1 ARR (realistic) | market-map.md § Combined TAM Thesis | market-map.md (add bottom-up SOM alongside aspirational TAM) |
| Market intersection size | 500K-2M developers (market-map.md) | 10K-25K developers (realistic Venn diagram) | market-map.md § Combined TAM Thesis | market-map.md (add decomposed intersection estimate) |

---

## Assumption Inventory

| # | Assumption | Criticality | Validation Status | What Breaks if Wrong |
|---|---|---|---|---|
| A1 | Developers want to control agent governance from their **phone** | Load-bearing — USP depends on "mobile-first" | **Unvalidated** — planned for interview Q9 | USP collapses. Product becomes "a Temporal.io widget" — useful but undifferentiated. Re-architect for desktop-first. |
| A2 | A **clock interface** is intuitive for setting governance rules | Load-bearing — entire visual identity | **Unvalidated** — planned for prototype test | Need conventional dashboard. "Stark zero-CSS" design language abandoned. 4-8 week delay. |
| A3 | The intersection of (AI agents + timezones + MCP) is large enough to sustain a business | Load-bearing — market viability | **Unvalidated** — market-map.md estimates are aspirational | If <5K developers, lifestyle business at best. Plan accordingly. |
| A4 | **Undisk API stability** will be maintained during Delta-T development | Important — but now internally controlled | **Validated** — team controls both products | Low risk. The real risk is bandwidth: if Undisk needs emergency work, Delta-T development stalls. |
| A5 | **Tauri mobile (iOS)** is stable enough for production | Load-bearing — mobile app is the product | **Partially validated** — local toolchain verified, no physical device build | If Tauri iOS is too buggy: switch to React Native (6-8 week delay), or ship macOS-only (loses mobile-first USP). |
| A6 | **MCP adoption** will continue accelerating | Load-bearing — execution channel is MCP | **Partially validated** — Anthropic, OpenAI, Google, Microsoft endorsements verified | If MCP stalls, Delta-T's "universal governance" positioning narrows to "Undisk governance only." Still viable given internal ownership. |
| A7 | Developers will pay **$19/mo** vs. cron + scripts for free | Critical — revenue model | **Unvalidated** — planned for interview Q11-12 | If cron is "good enough," $19/mo has no buyers. Price drops to $5-9/mo or governance must be dramatically better than cron. |
| A8 | A **solo developer** can build + maintain Tauri iOS + macOS + Cloudflare Workers + Undisk MCP + novel UI + run Undisk as a service | Critical — no team, no backup | **Partially validated** — founder already ships and operates Undisk | If timeline stretches 2-3x (to 20-30 weeks), opportunity window narrows. But the founder has demonstrated the ability to ship complex MCP tooling. |
| A9 | **Apple will approve** a Tauri WebView app | Important — affects iOS distribution | **Unvalidated** | If rejected: distribute macOS-only (direct download) or implement IAP at 30% cut. Mobile-first USP may be compromised. |
| A10 | The **free timezone utility** will drive conversion to paid governance | Important — acquisition model | **Unvalidated** — based on Cursor analogy | If free users don't convert: pivot to governance-first positioning. The trojan horse becomes a free product nobody pays for. |
| A11 | Developers who manage AI agents **care about timezone scheduling** of those agents | Load-bearing — bridges two markets | **Unvalidated** — core demand hypothesis | If developers want simple kill switches (not timezone-gated windows), the "temporal" layer is unnecessary. A simpler "agent pause/resume" app would suffice. |
| A12 | **Undisk user base** provides meaningful distribution for Delta-T | Important — key acquisition advantage from ownership | **Unfalsifiable until measured** — Undisk user count not disclosed in pipeline | If Undisk has <500 active users, the "built-in audience" advantage is nominal. Must quantify. |
| A13 | The **Undisk + Delta-T vertical integration** creates a defensible moat | Important — competitive positioning | **Partially validated** — architecture maps show deep integration across 7+ Undisk tools | The moat is real only if the market exists (contingent on A3/A11). A moat around a market nobody wants is still worthless. |

---

## Hard Gates

These are binary go/no-go conditions for proceeding to Phase 2. Each is achievable and specific.

### HG1: Demand Validation — MUST PASS

**Condition:** Complete 15+ customer discovery interviews using validate.md's script. At least 8/15 (53%) must rank temporal agent governance as a top-5 pain point (unprompted, Question 6).

**If fails (<5/15 = 33%):** HALT. Rewind to sketch.md. The core demand hypothesis is invalidated. Do not iterate — pivot.

**If marginal (5-7/15):** ITERATE. Re-interview with refined framing. Test "agent kill switch" positioning (simpler than "temporal governance").

**Timeline:** Weeks 1-3 of validation phase. Start with Undisk user base for warm leads.

### HG2: Clock UI Usability — MUST PASS

**Condition:** Figma prototype test with 10+ ICP developers. ≥75% complete the "set a deploy gate for Tokyo 9AM" task without guidance.

**If fails (<50%):** Pivot from clock-as-UI to conventional dashboard. Rewrite USP.

**If marginal (50-74%):** Iterate on clock design. Run second prototype round. If still <75%, pivot.

**Timeline:** Weeks 2-4 of validation phase (can overlap with HG1 interviews).

### HG3: Mobile Preference — SHOULD PASS

**Condition:** ≥60% of interviewees express willingness to control agent governance from their phone (not just preference — acceptance).

**If fails (<40%):** Pivot USP to "desktop-first with mobile companion." Ship macOS first.

**If marginal (40-59%):** Proceed with "desktop + mobile" positioning (not mobile-first).

**Timeline:** Captured during HG1 interviews (Question 9).

### HG4: Tauri iOS Physical Device Build — MUST PASS

**Condition:** A minimal Tauri 2.x app renders a WebView, completes a fetch() to a Cloudflare Worker, and receives a push notification on a physical iPhone.

**If fails within 2 weeks:** Switch mobile path to React Native. Update feasibility.md timeline (+6-8 weeks).

**Timeline:** Week 1 of engineering phase.

### HG5: Undisk User Base Quantification — SHOULD PASS

**Condition:** Document Undisk's current active user count (weekly active workspaces). This number grounds the market-size and distribution assumptions.

**If <500 active users:** The "built-in distribution" advantage is nominal. Re-ground validate.md's projections downward. Increase investment in external acquisition channels.

**If >2,000 active users:** Strong distribution advantage confirmed. validate.md's optimistic scenario becomes baseline.

**Timeline:** Immediately (the team has this data).

### HG6: Pricing Validation — SHOULD PASS

**Condition:** ≥3/15 interviewees give a firm "yes" to $19/mo. Van Westendorp median acceptable price ≥$15/mo.

**If fails (0/15 firm yes):** Test $9/mo price point. If still zero, the governance concept isn't valued enough to monetize.

**Timeline:** Captured during HG1 interviews (Questions 11-12).

---

## Verdict

### **CONDITIONAL PASS**

**Confidence: 0.68** (up from 0.72 in r2 — wait, that's down. Let me explain.)

The confidence DECREASED slightly from r2's 0.72 despite the Undisk risk elimination because r3 applies stricter scrutiny to the *remaining* risks. The Undisk dependency was the easiest risk to mitigate (it was structural, not market-based). Removing it reveals that the pipeline's core weakness was always demand validation, not technical execution. With Undisk dependency at ~0%, the demand risk (F1, 40% kill) and UX risk (F3, 25% kill) now dominate the probability space more clearly.

**What improved from r2 to r3:**

| Factor | r2 Assessment | r3 Assessment | Change |
|---|---|---|---|
| Undisk dependency risk | 30% kill probability | ~0% (internal) | **Eliminated** ✅ |
| Distribution advantage | Cold-start with no audience | Undisk user base as warm leads | **Improved** ↑ |
| Technical feasibility | Depends on 3rd-party API stability | Controls the API directly | **Improved** ↑ |
| Solo founder execution risk | 35% kill, unproven at this scope | 30% kill, demonstrated Undisk delivery | **Modestly improved** ↑ |
| Revenue synergy | None | Delta-T validates Undisk's value prop | **New advantage** ✅ |
| Market size risk | 20% kill, no known audience | 15% kill, Undisk base provides floor | **Modestly improved** ↑ |

**What did NOT improve:**

| Factor | r2 Assessment | r3 Assessment | Change |
|---|---|---|---|
| Demand validation | Zero evidence | Still zero evidence | **Unchanged** ⚠️ |
| Clock-as-UI risk | 25% kill, unvalidated | 25% kill, still unvalidated | **Unchanged** ⚠️ |
| Mobile-first preference | Unvalidated | Still unvalidated | **Unchanged** ⚠️ |
| validate.md evidence quality | 79% model-sourced | Still 79% model-sourced | **Unchanged** ⚠️ |
| icp.md pain points | 3/5 model-sourced | Still 3/5 model-sourced | **Unchanged** ⚠️ |

**Conditions for proceeding:**

1. **Hard Gates HG1-HG4 must pass.** HG1 (demand) and HG2 (clock UI) are the most important. HG4 (Tauri device build) is the cheapest to test.
2. **Soft Gates HG5-HG6 should pass.** HG5 (Undisk user count) is trivially answerable — answer it today.
3. **Artifacts requiring revision:** feasibility.md (rewrite R1, reassess verdict), validate.md (rewrite checklist #6, raise prototype threshold to 75%), usp.md (rewrite Confidence Limiter #3), market-map.md (add bottom-up SOM).

**If all hard gates pass:** Proceed to Phase 2 with adjusted confidence. The product concept is plausible, the architecture is sound, and the vertical integration with Undisk creates genuine strategic advantages. The pipeline needs to survive contact with real users — but it has earned the right to try.

**If any hard gate fails:**
- HG1 fails → Rewind to sketch.md. Kill the product or explore radical repositioning.
- HG2 fails → Rewind to UX. Pivot to conventional dashboard. Rewrite USP.
- HG3 fails → Rewrite USP as "desktop-first." Ship macOS only for MVP.
- HG4 fails → Switch to React Native. Add 6-8 weeks to timeline.

---

## Confidence Recalibration

| Artifact | Self-Reported | Critic-Adjusted | Δ | Justification |
|---|---|---|---|---|
| sketch.md | 0.30 | **0.15** | -0.15 | Pre-pivot artifact. Describes a world clock utility that no longer exists. Should be deprecated or rewritten. |
| market-map.md | 0.82 | **0.62** | -0.20 | Verified data, overconfident conclusions. SOM is 4 orders of magnitude above projections. Undisk ownership slightly improves distribution assumptions. |
| icp.md | 0.78 | **0.58** | -0.20 | 3/5 pain points are model-sourced. Well-constructed hypothesis but reads as validated profile when it's untested. |
| why-now.md | 0.85 | **0.78** | -0.07 | Strongest evidence base. Timing thesis is solid. Minor downgrade for convergence inference. |
| competitor-matrix.md | 0.82 | **0.77** | -0.05 | Thorough analysis. Linggen slightly underweighted. Undisk-as-competitor scenario eliminated. |
| ux-teardown.md | 0.75 | **0.67** | -0.08 | Solid UX analysis with appropriate caveats. Model-sourced judgments are inherent to UX evaluation. |
| steal-differentiate-ignore.md | 0.80 | **0.74** | -0.06 | Well-structured. Differentiation claims are unvalidated but logically sound. |
| usp.md | 0.85 | **0.62** | -0.23 | Passes all internal stress tests but no external validation. Undisk Confidence Limiter #3 is now invalid. Category risk remains the dominant concern. |
| feasibility.md | 0.82 | **0.76** | -0.06 | Strong technical mapping. Undisk dependency eliminated → should be closer to GREEN. 10-week timeline is still optimistic but founder's track record with Undisk provides credibility. |
| validate.md | 0.78 | **0.60** | -0.18 | 79% model-sourced. All projections are guesses. But the interview and prototype test designs are well-structured. Undisk ownership slightly improves channel assumptions. |

---

## Quality Score

| Criterion | Score (1-5) | Notes |
|---|---|---|
| **Specificity** | 5 | Every critique cites specific artifacts with exact quotes. Contradictions identify both sides. Fatal flaws provide kill probabilities, edge cases, and distribution bottlenecks. Assumption inventory rates 13 assumptions on 3 dimensions. Upstream impacts table identifies exact values, source artifacts, and propagation paths. |
| **Actionability** | 5 | 6 hard/soft gates with specific thresholds, timelines, and rewind targets. Every failure scenario includes mitigation. Every contradiction specifies what needs to change. Upstream impacts table tells downstream agents exactly what to update. |
| **Non-redundancy** | 5 | No overlap with any input artifact. This is the only artifact that cross-examines the pipeline, challenges self-reported confidence, recalibrates Undisk risk from external to internal, and provides binary gate conditions. |
| **Evidence quality** | 4 | Critiques are grounded in specific artifact claims and verified data. Kill probabilities and market reestimates are critic judgment (necessarily model-sourced). The Venn diagram decomposition and Undisk reframe provide novel analytical frames not present in any input. Score not 5 because probability estimates are inherently subjective. |
