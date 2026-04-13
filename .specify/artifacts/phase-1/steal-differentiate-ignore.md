---
artifact_meta:
  produced_by: "os.competition"
  produced_at: "2026-04-12T04:33:00Z"
  confidence: 0.80
  inputs_used:
    - ".specify/artifacts/phase-1/market-map.md"
    - ".specify/artifacts/phase-1/icp.md"
    - ".specify/artifacts/phase-1/why-now.md"
  stale_after: "on_input_change"
  revision: 2
  quality_scores:
    specificity: 4
    actionability: 5
    non_redundancy: 4
    evidence_quality: 4
  grade: "draft"
---

# Steal · Differentiate · Ignore — Delta-T Zaman: The Temporal Command Center

## Classification Methodology

Every feature observed across all 15 competitors in the competitor matrix is classified into exactly ONE of three categories:

- **Steal** — Adopt this feature as-is. It works, users expect it, and reimagining it adds no value for the ICP.
- **Differentiate** — This capability exists in competitors, but Delta-T must do it meaningfully better with a specific angle tied to the product's temporal governance thesis.
- **Ignore** — Skip this feature. It serves a different user, solves a different problem, or conflicts with Delta-T's focused product philosophy.

Every classification references the ICP ("Agentic Timezone Juggler" — solo/indie developer, 25-44, remote, 3+ timezones, manages AI agents daily, iPhone + macOS, values autonomy and minimal UI).

---

## Steal

### S1: Visual timezone slider/timeline (from Every Time Zone)

**What it is**: A horizontal timeline where each row is a timezone, a vertical line marks "now," and dragging the slider shows future/past times across all zones.

**Why steal it**: The ICP has a mental model of "time as a slider" from years of using Every Time Zone and similar tools. Reinventing the timezone visualization adds no value. The ICP expects this interaction pattern. [Verified — Every Time Zone's slider UX is the de facto standard for timezone comparison]

**ICP justification**: Per ICP psychographic "global-first mindset" — the user thinks in multiple timezones simultaneously. A visual slider is the fastest way to answer "what time will it be in Tokyo when it's 3 PM here?"

**Implementation note**: The Delta-T twist is that the slider doesn't just *show* time — it *sets governance rules*. Drag the slider to define execution windows. But the base interaction should be familiar.

---

### S2: Persistent menu bar / widget presence (from Dato) — ⚡ PROMOTED TO CORE per tradeoff.md r3

**What it is**: A timezone clock that lives permanently in the macOS menu bar, always visible without opening an app.

**[REVISED r3]:** Originally classified as Steal. tradeoff.md r3 selected the macOS menu bar app as THE primary product form factor for MVP. S2 is no longer a Steal feature — it IS the product. The menu bar widget is the control surface for Deploy Gate rules.

**Why it's core**: The ICP's timezone needs are "persistent background + burst interaction" (per ICP behavioral pattern). Dato proved that a persistent clock drives daily utility. The world clock is Delta-T's trojan horse — the always-on utility that justifies the app's presence on the user's device. [Verified — Dato's menu bar UX is cited in App Store reviews as its #1 feature]

**ICP justification**: Per ICP usage pattern "persistent background + burst interaction" — the clock runs always; rule management is accessed via click. Mobile widget (iOS/Android) is a Phase 2 expansion of this same pattern.

---

### S3: Step-level retry / durable execution (from Inngest / Temporal)

**What it is**: When a multi-step operation fails, only the failed step is retried, not the entire operation. State is preserved between steps.

**Why steal it**: If Delta-T's governance rules trigger actions (e.g., "at TYO 09:00, unlock Undisk deploy"), those actions should be durable. If the Cloudflare Worker that pushes state to KV fails, it should retry that specific step, not re-evaluate all governance rules. This is table-stakes infrastructure reliability. [Verified — Inngest's step-level retry is a core differentiator; Temporal's durable execution is proven at scale]

**ICP justification**: Per ICP pain point "My agents don't know what time it is" — if governance rule execution itself is unreliable, the trust problem compounds. Durability is non-negotiable for a governance product.

---

### S4: Execution-based pricing model (from n8n)

**What it is**: Charge per governance rule evaluation / per month, not per individual API call or per node in a workflow. A developer pays the same whether they have 3 timezone rules or 30.

**Why steal it**: n8n proved that execution-based pricing eliminates bill shock and aligns costs with user-perceived value. For the ICP, a predictable $10-20/month is far more acceptable than usage-based pricing that could spike unpredictably. [Verified — n8n's execution-based pricing is consistently cited as a competitive advantage in comparison articles]

**ICP justification**: Per ICP spending pattern "low-friction, subscription-tolerant" — the ICP already pays for 3-5 dev SaaS subscriptions. They expect flat monthly pricing with no surprises, not per-action metering.

---

### S5: OpenTelemetry / structured observability (from Trigger.dev)

**What it is**: Built-in tracing with OpenTelemetry compatibility, custom dashboards, and a SQL-style query language for searching execution history.

**Why steal it**: When a governance rule fires (or fails to fire), the developer needs to understand what happened. Structured observability (not just logs) is the standard for modern developer tools. [Verified — Trigger.dev includes "OpenTelemetry observability with custom dashboards and a SQL-style query language"]

**ICP justification**: Per ICP pain point "I have no temporal audit trail for my agents" — the audit trail needs to be queryable and explorable, not just a raw log dump. OTel-compatible traces let the developer use tools they already know (Grafana, Honeycomb, Datadog).

---

### S6: Free tier with generous limits (from Trigger.dev / Inngest / n8n)

**What it is**: A functional free tier that covers individual developer usage (50K runs/month for Inngest/Trigger.dev; self-hosted free for n8n).

**Why steal it**: The ICP adopts tools bottom-up via product-led growth (per ICP adoption pattern). A free tier that includes the timezone utility + basic governance rules (e.g., 3 timezone rules, 1 sleep fence) converts users who would never sign up for a trial. [Verified — Inngest: 50K runs/mo free; Trigger.dev: 50K runs/mo free; n8n: self-hosted free]

**ICP justification**: Per ICP adoption pattern "bottom-up, product-led" — the timezone features are the hook; agent governance is the monetization. The free tier must include a fully functional world clock with the ability to try governance features.

---

## Differentiate

### D1: Timezone-gated governance windows (vs. Temporal/Inngest cron scheduling)

**What exists**: Temporal and Inngest offer cron-based scheduling with timezone parameters. E.g., "Run this workflow at 9 AM JST every weekday."

**Delta-T's angle**: Transform scheduling from a *trigger* into a *constraint*. Instead of "start workflow at time T," express "agent may only operate within window W in timezone Z." This is a fundamentally different primitive:

| Competitor | Primitive | Semantics |
|---|---|---|
| Temporal/Inngest | `schedule.start(cron: "0 9 * * *", tz: "Asia/Tokyo")` | "Run at this time" |
| **Delta-T** | `rule.window(allow: "09:00-18:00", tz: "Asia/Tokyo", target: "undisk-deploy")` | "Allow only during this window" |

**ICP justification**: Per ICP pain point "My agents don't know what time it is" — the ICP doesn't want to schedule agent starts; they want to constrain agent behavior. "Don't deploy outside business hours" is a governance rule, not a cron job.

**Differentiation evidence**: No competitor offers constraint-window semantics. All offer point-in-time triggers. [Verified — Temporal Schedules docs; Inngest cron docs; Trigger.dev scheduling docs]

---

### D2: Mobile-native control plane (vs. everyone's web-only dashboards)

**What exists**: Every orchestration tool (Temporal, Inngest, Trigger.dev, Windmill, n8n, Retool) has a web dashboard. None have mobile apps. The closest mobile developer tool is Linggen (open-source, P2P agent access from phone, launched 2026). [Verified — Epsilla blog, April 2026]

**Delta-T's angle**: The iPhone IS the control plane. Not a responsive web dashboard crammed onto a small screen, but a purpose-built iOS app where the primary interaction is a clock interface. Tap a timezone to set rules. Swipe to scrub workspace history. Toggle a sleep fence from the lock screen widget.

**ICP justification**: Per ICP pain point "There's no mobile control plane for my dev infrastructure" — the ICP monitors infrastructure from their phone (PagerDuty, Datadog mobile) but cannot *control* agent behavior from mobile. SSH from phone (Termius, Blink Shell) is high-friction. Delta-T should be as simple as setting a phone alarm.

**Differentiation evidence**: Zero orchestration competitors have mobile apps. Linggen is the only mobile agent control tool, but it's P2P infrastructure access, not governance. [Verified — product analysis across all competitors]

---

### D3: Temporal undo slider with "Revert to Intent" (vs. Temporal.io workflow replay / Git history)

**What exists**: Temporal.io offers workflow replay (re-execute from event history). Git offers `git log` and `git revert`. Undisk offers `list_versions` and `restore_version` via API/CLI.

**Delta-T's angle**: A visual, mobile-friendly timeline scrubber that shows workspace state over time. The developer drags a slider to "3 hours ago" and sees a diff of what changed. "Revert to Intent" uses `workspace_checkpoint` to restore an atomic multi-file state, not just individual file versions.

| Competitor | Undo mechanism | UX |
|---|---|---|
| Temporal.io | Workflow replay from event history | CLI/Web; replays workflow code, not workspace state |
| Git | `git log && git revert` | Terminal-only; per-commit, not per-timestamp |
| Undisk (raw) | `list_versions` + `restore_version` API | CLI/API; requires knowing file paths and version IDs |
| **Delta-T** | Visual timeline slider → workspace checkpoint restore | Mobile touch UX; scrub time, preview diff, tap to revert |

**ICP justification**: Per ICP pain point "I have no temporal audit trail for my agents" — the ICP wakes up to an agent-caused mess and needs to answer "what happened while I slept?" and then "undo the damage." This should take 30 seconds from their phone, not 30 minutes in a terminal.

---

### D4: Circadian sleep fence (vs. basic pause/resume)

**What exists**: Temporal.io supports workflow pause/resume via signals. Trigger.dev has waitpoints. These are manual, point-in-time controls.

**Delta-T's angle**: An automatic, recurring "kill switch" that activates every night during the developer's local sleep hours and deactivates every morning. Set it once, forget it. The sleep fence is not a manual pause — it's a circadian rhythm for your agents.

**ICP justification**: Per ICP pain point "I can't sleep because my agents might break things" — the ICP doesn't want to remember to pause their agents every night. They want to set "sleep hours: 11 PM - 7 AM PST" once, and trust that no agent will write to their workspace during that window. The key word is *trust*.

**Differentiation evidence**: No competitor offers an automatic, recurring pause/resume tied to local time. Temporal's signals are imperative (send a signal each time); Delta-T's sleep fence is declarative (set it once). [Verified — Temporal docs show signal-based pause requiring explicit signal sends]

---

### D5: Edge-state sync for governance rules (vs. centralized server architectures)

**What exists**: Every orchestration platform (Temporal, Inngest, Trigger.dev) uses centralized cloud infrastructure. When a governance rule changes, the change propagates through a single-region server.

**Delta-T's angle**: Governance rules live in Cloudflare KV, replicated across 330+ global edge locations with 1-5ms read latency. When the ICP toggles a sleep fence from their iPhone in San Francisco, an agent reading from Tokyo sees the updated state within 60 seconds, reading it in under 5ms. No orchestration competitor can match this propagation speed.

**ICP justification**: Per ICP psychographic "global-first mindset" — the ICP's agents may run anywhere in the world. Governance rules must propagate globally, not sit in a single US-East datacenter.

**Differentiation evidence**: No orchestration competitor uses edge state for rule distribution. [Verified — Temporal Cloud is centralized; Inngest is centralized; Cloudflare KV docs confirm 1-5ms reads across 330+ PoPs]

---

### D6: Undisk-native workspace integration (vs. generic tool integrations)

**What exists**: n8n has 400+ integrations. Temporal connects to anything via activities. But none are purpose-built for MCP workspace governance.

**Delta-T's angle**: Deep, first-class integration with Undisk's 25 MCP tools — `list_versions`, `restore_version`, `workspace_checkpoint`, `audit_trail`, `workspace_collaborate` (locks). Delta-T doesn't just "connect" to Undisk; it understands Undisk's temporal data model and presents it as a governance UI.

**ICP justification**: Per ICP pain point "I have no temporal audit trail for my agents" — the ICP uses Undisk (or similar MCP workspace) for agent-driven development. Delta-T is the governance UI that Undisk lacks.

---

### D7: "Stark, zero-CSS" design language (vs. feature-rich dashboards)

**What exists**: Every orchestration competitor has a complex dashboard with navigation menus, data tables, charts, filters, and settings panels.

**Delta-T's angle**: A clock. That's it. The interface IS a clock. Timezone rules are set by interacting with the clock. The undo slider IS a timeline. The sleep fence IS a toggle. The product's visual identity is radical simplicity — closer to a high-end watch face than a SaaS dashboard.

**ICP justification**: Per ICP psychographic "minimal UI preference" — "They distrust bloated dashboards. They want tools that do one thing well. A 'stark, zero-CSS clock' resonates because it signals focus and intentionality."

---

## Ignore

### I1: Complex multi-step workflow orchestration (Temporal, Inngest, Trigger.dev)

**What to skip**: Saga patterns, compensation handlers, distributed task queues, multi-step workflow builders, activity retry policies.

**Why**: Delta-T is a governance layer, not a workflow engine. The ICP's use case is "control when agents can act," not "orchestrate multi-step business processes." Building workflow features would put Delta-T in direct competition with Temporal ($5B valuation, 300M Series D) and Inngest/Trigger.dev — a fight Delta-T cannot win and doesn't need to. [Verified — Temporal's $5B valuation and deep feature set; per ICP the user wants "a light switch, not a 747"]

**ICP justification**: Per ICP evaluation pattern "5-minute test" — workflow orchestration has a multi-day learning curve. Delta-T must be usable in 5 minutes.

---

### I2: 400+ pre-built integrations (n8n, Retool)

**What to skip**: Building connectors for Slack, PostgreSQL, Salesforce, Gmail, Stripe, and hundreds of other services.

**Why**: Delta-T has exactly three integrations that matter: (1) Cloudflare KV/D1, (2) Undisk MCP, (3) iOS app. Breadth of integration is n8n's competitive advantage; Delta-T's advantage is *depth* of integration with its specific stack. Every integration added dilutes focus. [Verified — n8n has 400+ integrations; Delta-T's architecture needs only 3]

**ICP justification**: Per ICP psychographic "minimal UI preference" — the user doesn't want a Swiss Army knife; they want a precision instrument. "Do one thing well" philosophy.

---

### I3: Visual flow/node builder (n8n, Windmill, Retool)

**What to skip**: Drag-and-drop workflow editors, node graphs, visual flow designers.

**Why**: Agent governance rules are not workflows — they're constraint declarations. "Block writes during sleep hours" is a rule, not a flowchart. A visual builder would add complexity without matching the user's mental model. [Model-sourced — inferred from governance-vs-workflow distinction]

**ICP justification**: Per ICP identity markers — the user has "a `.zshrc` longer than most people's resumes." They express rules as declarations, not diagrams.

---

### I4: GPU/compute execution (Modal, E2B, Replit)

**What to skip**: Running agent code, providing sandboxed execution environments, offering GPU compute.

**Why**: Delta-T governs *when* agents execute, not *where* or *how*. E2B provides sandboxes. Modal provides GPUs. Trigger.dev provides managed compute. Delta-T provides temporal boundaries. These are complementary, not competitive. Building compute would require massive infrastructure investment for no governance differentiation. [Verified — E2B, Modal, Trigger.dev are compute platforms]

**ICP justification**: Per ICP usage — the user already has compute (Cursor, Claude Code, Copilot Agent Mode run on their own infrastructure). They don't need another compute provider; they need a governance layer.

---

### I5: AI agent builder (n8n AI Agent node, Inngest agent harness)

**What to skip**: Building new AI agents, agent memory systems, tool-use frameworks, prompt chaining.

**Why**: Delta-T does not build agents — it governs existing agents. The ICP already uses GitHub Copilot, Cursor, or Claude Code. They don't need another agent framework. Delta-T's value is that it works with *any* MCP-connected agent without requiring the agent to be built on a specific platform. [Verified — n8n's AI Agent builder creates new agents; Delta-T governs existing ones]

**ICP justification**: Per ICP psychographic "automation maximalism" — the user already runs multiple agents. They need governance, not yet another agent to manage.

---

### I6: Calendar/meeting scheduling (World Time Buddy, Dato)

**What to skip**: Finding optimal meeting times, sending calendar invites, integrating with Google Calendar events.

**Why**: Meeting scheduling is a solved problem (Calendly, Cal.com, World Time Buddy). Adding scheduling features would move Delta-T toward the crowded consumer productivity space and away from the developer governance niche. The timezone display should *show* times; it should not become a scheduling app. [Verified — Calendly, Cal.com are established meeting schedulers]

**ICP justification**: Per ICP persona hierarchy — the tertiary persona ("Timezone-Aware Manager") might want meeting scheduling, but the primary persona ("Agentic Timezone Juggler") does not. Product decisions should serve the primary persona.

---

### I7: External user portals / end-user apps (Retool, Replit)

**What to skip**: Building customer-facing applications, embedding internal tools for external users, white-label solutions.

**Why**: Delta-T is a developer tool for the developer's own agent governance. There are no "end users" beyond the developer themselves (and possibly their small team). Building multi-tenant, customer-facing features is an enterprise distraction. [Model-sourced — inferred from ICP: "Solo developer, indie hacker, or senior IC at a startup (1-50 employees)"]

**ICP justification**: Per ICP role — "Solo developer, indie hacker, or senior IC." They are both the builder and the user.

---

### I8: Team collaboration features (Windmill multiplayer, Retool team features)

**What to skip**: Real-time collaborative editing, team role management, shared workspace permissions, organizational hierarchy.

**Why**: Delta-T v1 targets the individual developer managing their own agents. Team features are a v2/v3 concern if individual adoption proves the concept. Premature team features add UX complexity and development cost without validating the core governance thesis. [Model-sourced — per ICP adoption pattern: "adopts tools individually before bringing them to their team"]

**ICP justification**: Per ICP adoption pattern "bottom-up, product-led" — individual adoption first, team features later.

---

### I9: Enterprise compliance features (Temporal SOC 2, SSO/SAML)

**What to skip**: SOC 2 certification, SAML SSO, RBAC, audit logging for compliance, 99.9% SLA.

**Why**: Enterprise features cost $100K+ to implement and maintain. The ICP is a solo developer paying $10-20/month. Enterprise compliance is a requirement for scaling past ~$1M ARR, not for validating the product thesis. Build it when enterprise customers demand it and are willing to pay enterprise prices. [Model-sourced — standard enterprise feature cost estimates]

**ICP justification**: Per ICP spending pattern — the target price is $10-20/month. Enterprise features are antithetical to this price point at launch.

---

## Feature Classification Summary

| Feature | Source Competitor(s) | Classification | ICP Rationale (Short) |
|---|---|---|---|
| Visual timezone slider | Every Time Zone | **Steal** | Mental model alignment; expected interaction |
| Menu bar / widget presence | Dato | **Steal** | Persistent background utility pattern |
| Step-level retry / durability | Inngest, Temporal | **Steal** | Governance must be reliable |
| Execution-based pricing | n8n | **Steal** | Flat monthly; no bill shock |
| OpenTelemetry observability | Trigger.dev | **Steal** | Queryable audit trail |
| Generous free tier | Trigger.dev, Inngest, n8n | **Steal** | Product-led adoption hook |
| Timezone-gated governance windows | Temporal, Inngest (cron) | **Differentiate** | Constraints > triggers |
| Mobile-native control plane | None (gap) | **Differentiate** | Control from bed/phone |
| Temporal undo slider | Temporal (replay), Git | **Differentiate** | Visual mobile time-travel |
| Circadian sleep fence | Temporal (signals) | **Differentiate** | Automatic, not manual |
| Edge-state sync | None (gap) | **Differentiate** | Global <60s propagation |
| Undisk-native integration | None (gap) | **Differentiate** | Purpose-built workspace governance |
| Stark clock design language | None (gap) | **Differentiate** | Radical simplicity |
| Multi-step workflow orchestration | Temporal, Inngest, Trigger.dev | **Ignore** | Governance ≠ orchestration |
| 400+ integrations | n8n, Retool | **Ignore** | Depth > breadth |
| Visual flow/node builder | n8n, Windmill | **Ignore** | Rules ≠ flowcharts |
| GPU/compute execution | Modal, E2B, Replit | **Ignore** | Governance layer, not compute |
| AI agent builder | n8n, Inngest | **Ignore** | Govern existing agents |
| Calendar/meeting scheduling | World Time Buddy, Dato | **Ignore** | Solved problem; wrong niche |
| External user portals | Retool, Replit | **Ignore** | Solo dev tool, not SaaS builder |
| Team collaboration features | Windmill, Retool | **Ignore** | v1 is individual-first |
| Enterprise compliance (SOC 2, SSO) | Temporal | **Ignore** | Post-$1M ARR concern |

**Classification count**: 6 Steal · 7 Differentiate · 9 Ignore — total 22 features classified.

---

## Strategic Implications

### 1. The "Governance Layer" Positioning Is Defensible

By explicitly **ignoring** workflow orchestration, compute execution, and agent building, Delta-T avoids competing with well-funded incumbents ($5B Temporal, $3.35B Retool) on their home turf. The governance layer sits *above* these tools, not alongside them. Delta-T is the "temporal RBAC for agents" — complementary to everyone, competitive with no one.

### 2. The Free Timezone Utility Is the Growth Engine

By **stealing** the visual slider (Every Time Zone) and persistent widget (Dato), Delta-T gets a compelling free product that stands alone as a developer-friendly timezone tool. The agent governance features are the monetization upsell, not the acquisition hook. This mirrors Cursor's strategy: free editor → paid AI features.

### 3. The Seven Differentiators Are the Moat

The combination of timezone-gated governance + mobile control + temporal undo + sleep fence + edge sync + Undisk integration + stark design is not one feature — it's an integrated system. Any competitor could build one of these; building all seven in a coherent product is the defensible position.

---

## Quality Score

| Criterion | Score (1-5) | Rationale |
|---|---|---|
| Specificity | 4 | 22 features classified with named competitor sources, specific ICP justifications, and implementation notes. |
| Actionability | 5 | Every classification directly maps to a build/skip decision. The summary table is a product roadmap filter. Strategic implications are immediately executable. |
| Non-redundancy | 4 | Clear separation between steal (adopt), differentiate (do better), and ignore (skip). No feature appears in multiple categories. |
| Evidence quality | 4 | Steal/Differentiate claims reference verified competitor features. Ignore claims reference ICP psychographics and market positioning logic. |
| **Overall** | **4.25** | Highest-actionability artifact: directly informs v1 feature scope, pricing strategy, and competitive positioning. |
