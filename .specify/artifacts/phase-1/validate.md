---
artifact_meta:
  produced_by: "os.validate"
  produced_at: "2026-07-19T22:30:00Z"
  confidence: 0.78
  inputs_used:
    - ".specify/artifacts/phase-1/icp.md"
    - ".specify/artifacts/phase-1/usp.md"
    - ".specify/artifacts/phase-1/feasibility.md"
  stale_after: "on_input_change"
  revision: 2
  quality_scores:
    specificity: 4
    actionability: 5
    non_redundancy: 5
    evidence_quality: 4
  grade: "draft"
---

# User Acquisition & MVP Validation Plan — Delta-T Zaman: The Temporal Command Center

## Executive Summary

This artifact defines how Delta-T Zaman will acquire its first 100 users, validate its core hypothesis before writing production code, and sequence its go-to-market for the first 8 weeks post-MVP. The central hypothesis is falsifiable and quantitative: **solo developers using AI coding agents across 3+ timezones will pay $19+/mo for mobile-first temporal governance.** This must be validated through 15+ customer discovery interviews, a landing-page smoke test, and a clickable prototype test — all before the 10-week MVP build begins [Source — feasibility.md: Required mitigations].

The validation plan is designed for a solo founder with near-zero marketing budget. All P0 channels are free. The go/no-go decision gate requires ≥40% of interviewees confirming temporal agent governance as a top-5 pain AND ≥150 waitlist signups from a cold landing page within 4 weeks.

---

## User Acquisition Strategy

### Channel Analysis

#### Channel 1: Hacker News (Show HN + Follow-up Posts)

| Dimension | Assessment |
|---|---|
| **Expected reach** | 50K–200K impressions per front-page post. Show HN posts from solo developers building novel tools in Rust/Tauri consistently reach 100+ points. [Model-sourced — based on HN front-page traffic patterns for developer tool launches] |
| **Cost** | Free |
| **Conversion likelihood** | **MEDIUM** — HN audience skews toward ICP (senior developers, early adopters, remote workers). However, HN is notoriously skeptical of "invented categories." The temporal-governance framing must be demonstrated, not explained. |
| **Time to first users** | 1–3 days from post. HN traffic is bursty — 80% of traffic arrives within 24 hours. |
| **Priority** | **P0** — Highest-ROI free channel for developer tools. First launch post should be the Show HN. |
| **Tactic** | Post as "Show HN: I built a clock that controls when my AI agents can deploy." Lead with the demo video (30s GIF of clock UI → tap TYO → agent blocked). Avoid jargon ("temporal governance layer"). Follow up 4–6 weeks later with a "lessons learned" post. |

#### Channel 2: Reddit (r/programming, r/devops, r/MachineLearning, r/SideProject)

| Dimension | Assessment |
|---|---|
| **Expected reach** | r/programming: 6.5M members, 2K–10K views per trending post. r/devops: 350K members. r/MachineLearning: 3.1M members. r/SideProject: 200K members. [Model-sourced — Reddit subscriber counts as of mid-2026] |
| **Cost** | Free |
| **Conversion likelihood** | **LOW–MEDIUM** — Reddit communities are hostile to self-promotion. Authentic "building in public" posts perform better than launch announcements. r/devops is highest-conversion for deploy-gate use case; r/MachineLearning is relevant for agent-governance angle. |
| **Time to first users** | 1–7 days. Reddit traffic is less bursty than HN but more sustained. |
| **Priority** | **P1** — Good secondary channel. Must be authentic (account history, community participation) to avoid removal. |
| **Tactic** | Seed 4–6 weeks of genuine participation (answering timezone/agent/MCP questions) before posting. Frame as a problem-first story: "I kept waking up to find my AI agent had deployed at 2AM Tokyo time. So I built this." Cross-post adapted versions to each sub. |

#### Channel 3: AI Agent Ecosystem Communities (Anthropic, OpenAI, Cursor, Windsurf, GitHub Copilot)

| Dimension | Assessment |
|---|---|
| **Expected reach** | Anthropic community Discord: ~50K members. OpenAI Developer Forum: ~100K registered. Cursor community: ~30K active. GitHub Copilot discussions: ~20K threads. [Model-sourced — estimated community sizes mid-2026] |
| **Cost** | Free |
| **Conversion likelihood** | **HIGH** — These users are the ICP by definition: developers running autonomous AI agents who need governance. The "my agent deployed at 2AM" pain point resonates immediately. |
| **Time to first users** | 3–14 days. Community posts have longer shelf life than HN/Reddit. |
| **Priority** | **P0** — Highest-quality leads. These communities contain the exact "Agentic Timezone Juggler" persona [Source — icp.md: Primary Persona]. |
| **Tactic** | Share a short tutorial: "How I added timezone-gated rules to my Claude Code agent workflow." Include a working demo with Undisk MCP. Position Delta-T as a tool that solves a problem the community already discusses (agent safety, overnight costs, unintended deploys). |

#### Channel 4: Undisk / MCP Ecosystem (MCP Server Directories, Undisk Community)

| Dimension | Assessment |
|---|---|
| **Expected reach** | MCP server directories (mcp.run, Smithery, glama.ai): ~5K–15K monthly visitors each. Undisk community: small but deeply aligned. [Model-sourced — MCP ecosystem is nascent; directory traffic estimated from similar early-stage ecosystems] |
| **Cost** | Free |
| **Conversion likelihood** | **HIGH** — Users already in the MCP ecosystem understand agent workspaces and governance gaps. Delta-T's Undisk integration is a direct value-add to their existing stack. |
| **Time to first users** | 7–21 days. Directory listings are evergreen but low-traffic initially. |
| **Priority** | **P0** — Small audience but near-perfect ICP fit. Every user from this channel is a potential paying customer. |
| **Tactic** | List Delta-T's Cloudflare Worker as an MCP-compatible governance proxy on all major MCP directories. Publish an integration guide: "Add temporal governance to your Undisk workspace in 5 minutes." Engage directly with Undisk team for co-marketing (per feasibility.md mitigation: "Contact Undisk team for partnership/API guarantees" [Source — feasibility.md: Required mitigations #3]). |

#### Channel 5: Content Marketing (Blog Posts, Demo Videos, Building-in-Public)

| Dimension | Assessment |
|---|---|
| **Expected reach** | Technical blog posts: 1K–10K views per post (on personal blog/dev.to/Hashnode). Demo videos (YouTube/X): 500–5K views. Building-in-public threads (X/Twitter): 1K–20K impressions per thread for accounts with 1K+ followers. [Model-sourced — developer content benchmarks] |
| **Cost** | Free (time investment: ~4 hours/week) |
| **Conversion likelihood** | **MEDIUM** — Content marketing compounds over time. Individual pieces have low conversion, but a content library builds SEO authority and establishes founder credibility. |
| **Time to first users** | 2–8 weeks. SEO traffic takes 4–8 weeks to materialize. Social content is faster (1–3 days). |
| **Priority** | **P1** — Essential for sustained growth but not a launch channel. Start publishing during pre-build validation phase. |
| **Tactic** | Publish a 3-part series: (1) "The Problem with Unsupervised AI Agents" (pain-focused, no product mention), (2) "How Timezone Governance Works" (concept piece), (3) "Building Delta-T with Tauri + Cloudflare + Undisk" (technical deep-dive). Record a 60-second demo video showing the clock UI blocking a deploy. Post building-in-public updates weekly on X. |

#### Channel 6: Open Source (Clock UI Component, MCP Governance Primitives)

| Dimension | Assessment |
|---|---|
| **Expected reach** | GitHub trending (Rust/TypeScript): 10K–50K views for a trending repo. NPM/crates.io package: 100–1K downloads/month initially. [Model-sourced — open-source developer tool benchmarks; ICP discovery pattern: "will try anything that has a clean GitHub README" — Source: icp.md, Discovery Pattern] |
| **Cost** | Free (significant time investment: ~2 weeks to extract and polish a standalone component) |
| **Conversion likelihood** | **MEDIUM** — Open-source builds trust and awareness. Conversion from GitHub star to paying customer is typically 0.1–0.5% for developer tools. But the GitHub repo becomes a permanent discovery channel. [Model-sourced] |
| **Time to first users** | 2–4 weeks for initial stars. Conversion to paid users is 2–6 months. |
| **Priority** | **P1** — Strategic investment. Open-source the clock UI component (React/Svelte) as a standalone package. Contribute MCP governance utilities to the ecosystem. This earns trust with the ICP's "autonomy obsession" and "automation maximalism" values [Source — icp.md: Psychographics, Core Values #1 and #2]. |
| **Tactic** | Extract the timezone-aware clock slider as a standalone open-source package (`@delta-t/clock-slider`). Publish on npm/crates.io. Contribute a "temporal governance" example to MCP community repos. License the UI component MIT; keep the governance engine proprietary. |

#### Channel 7: Direct Outreach (Newsletters, Podcasts, Partnerships)

| Dimension | Assessment |
|---|---|
| **Expected reach** | Developer newsletters (TLDR, Bytes, DevOps Weekly): 100K–500K subscribers each. Podcasts (The Changelog, Ship It, Indie Hackers): 10K–50K listeners per episode. [Model-sourced] |
| **Cost** | Newsletter sponsorships: $200–$2,000 per placement. Podcast sponsorships: $500–$3,000 per episode. Guest appearances: Free. |
| **Conversion likelihood** | **LOW–MEDIUM** for paid sponsorships (0.1–0.5% CTR typical). **MEDIUM–HIGH** for earned media (guest appearances, product features). |
| **Time to first users** | 1–7 days from placement. |
| **Priority** | **P2** — Defer paid sponsorships until post-MVP validation confirms product-market fit. Pursue free guest appearances on developer podcasts during build phase. |
| **Tactic** | Pitch a guest appearance on The Changelog or Ship It: "Building a temporal command center for AI agents with Tauri + Cloudflare." Reserve paid newsletter placements for post-MVP launch week (budget: $500 max for first placement in TLDR or Bytes). |

#### Channel 8: Dev.to / Hashnode / Medium (Developer Blogging Platforms)

| Dimension | Assessment |
|---|---|
| **Expected reach** | dev.to: 500K–1M monthly active writers/readers. Hashnode: ~200K. Individual post reach: 500–5K views depending on tags and engagement. [Model-sourced] |
| **Cost** | Free |
| **Conversion likelihood** | **LOW–MEDIUM** — Broad developer audience; only a fraction matches the ICP. Best for awareness and SEO. |
| **Time to first users** | 1–4 weeks. |
| **Priority** | **P2** — Supplement to owned blog. Cross-post content marketing pieces. |
| **Tactic** | Cross-post the 3-part blog series from Channel 5. Use platform-specific tags (#ai, #devops, #rust, #tauri, #timezone). Engage in comments to build credibility. |

### Channel Priority Summary

| Priority | Channel | Cost | Expected First Users |
|---|---|---|---|
| **P0** | Hacker News (Show HN) | Free | Days 1–3 |
| **P0** | AI Agent Communities (Anthropic, Cursor, etc.) | Free | Days 3–14 |
| **P0** | Undisk / MCP Ecosystem | Free | Days 7–21 |
| **P1** | Reddit (r/programming, r/devops) | Free | Days 1–7 |
| **P1** | Content Marketing (blog, video, BIP) | Free | Weeks 2–8 |
| **P1** | Open Source (clock UI component) | Free | Weeks 2–4 |
| **P2** | Direct Outreach (newsletters, podcasts) | $200–$2,000 | Days 1–7 |
| **P2** | Dev.to / Hashnode cross-posts | Free | Weeks 1–4 |

---

## MVP Validation Plan

### Core Hypothesis

> **"Solo developers and small teams (≤5 people) using AI coding agents (Cursor, Claude Code, GitHub Copilot Agent Mode) across 3+ timezones will pay $19+/mo for mobile-first temporal governance of their autonomous agents."**

This hypothesis is falsifiable. It will be disproven if any of the following occur:
- Fewer than 8/15 interviewees confirm temporal agent governance as a top-5 pain point → problem does not exist at claimed severity.
- Fewer than 150 waitlist signups from a cold landing page within 4 weeks → insufficient demand signal.
- Fewer than 6/10 prototype testers find the clock-as-governance-UI intuitive → UX paradigm rejected.
- Fewer than 3 "soft commits" ($19/mo stated willingness) from 15 interviewees → pricing rejected.

### Signal 1: Problem Validation

**Question:** Do developers actually experience timezone-related agent governance pain?

**How to measure:**

| Method | Metric | Threshold | Timeline |
|---|---|---|---|
| Customer discovery interviews (15+) | % who rank "agents acting during my off-hours" in top-5 pain points (unprompted) | ≥ 53% (8/15) | Weeks 1–3 of validation |
| Reddit/HN/X sentiment scan | Posts/comments describing overnight agent incidents, timezone coordination pain | ≥ 20 organic complaints found | Week 1 |
| ICP survey (30+ respondents) | "How often have AI agents caused issues outside your working hours?" — weekly/daily vs. rarely/never | ≥ 40% report weekly+ incidents | Weeks 2–4 |

**Evidence from upstream artifacts:**
- 29% of developers trust AI outputs (down from 40%); 46% actively distrust [Source — icp.md, Pain #2, Verified — Stack Overflow 2025]
- 98% of remote workers cite timezone differences as a major hurdle [Source — icp.md, Pain #3, Verified — Buffer 2024]
- No major AI agent offers timezone-aware execution constraints [Source — icp.md, Pain #1, Verified — CB Insights 2025]

### Signal 2: Solution Validation

**Question:** Does the clock-interface metaphor resonate as a governance control plane?

**How to test without building:**

| Method | Metric | Threshold | Timeline |
|---|---|---|---|
| Figma clickable prototype (10+ testers) | Task completion rate: "Set a rule that blocks deploys in Tokyo before 9AM" | ≥ 60% complete without guidance | Weeks 2–4 |
| Prototype testers — NPS-style score | "How likely would you use this daily?" (1–10) | Mean ≥ 7.0 | Weeks 2–4 |
| Prototype testers — preference test | Clock UI vs. conventional dashboard mockup (A/B within prototype) | Clock preferred by ≥ 50% | Weeks 2–4 |
| Landing page hero image | Click-through on "See the clock in action" (video demo link) | ≥ 15% CTR on hero CTA | Weeks 1–4 |

**Prototype specification:**
- 5-screen Figma flow: (1) Clock face with 4 timezone blocks, (2) Tap TYO block → rule drawer opens, (3) Set "Unlock Deploy at 09:00" → confirm, (4) Agent attempts deploy → "BLOCKED" notification, (5) 09:00 arrives → "UNLOCKED" notification
- Test on 10 ICP-matching developers (recruited via X, Indie Hackers, or cold outreach)
- Record screen + audio for qualitative analysis
- Per feasibility.md: "At least 6/10 prototype testers find the clock-as-governance-UI intuitive" [Source — feasibility.md: Proceed-if gate]

### Signal 3: Willingness to Pay

**Question:** Would developers pay $19/mo? $49/mo?

**How to measure pre-product:**

| Method | Metric | Threshold | Timeline |
|---|---|---|---|
| Van Westendorp pricing (in interviews) | Acceptable price range for "mobile temporal governance for AI agents" | Median acceptable ≥ $15/mo | Weeks 1–3 |
| Landing page pricing page | % of visitors who click "Get early access" after seeing $19/mo price | ≥ 5% of pricing page viewers | Weeks 1–4 |
| Interview "soft commit" | "If this existed today at $19/mo, would you subscribe?" — yes / maybe / no | ≥ 3 firm "yes" from 15 interviews | Weeks 1–3 |
| Fake door test | Stripe checkout page that collects email instead of charging ("You'll be first in line — we'll email you when it's live") | ≥ 30 email captures in 4 weeks | Weeks 1–4 |

**Revenue model reference:** $19/mo indie, $49/mo team. Break-even at 3 paying customers on infrastructure costs alone ($47/mo). Sustainability target: $2,000/mo at ~85 customers [Source — feasibility.md: Financial Feasibility].

### Signal 4: Activation

**Question:** What's the "aha moment"?

**Hypothesized activation event:** User creates their first timezone-gated governance rule and sees an agent blocked by it.

**How to identify post-MVP:**

| Metric | Definition | Target |
|---|---|---|
| Time-to-first-rule | Minutes from app install to first governance rule created | ≤ 5 minutes [Source — icp.md: "5-minute evaluation window"] |
| Rule-trigger-witnessed | % of users who see their first rule block or allow an agent action within 48 hours | ≥ 40% |
| Day-7 retention | % of users who open the app on day 7 after install | ≥ 25% (strong for developer tools) [Model-sourced] |
| Rule-modification rate | Users who modify/add a second rule within 14 days | ≥ 30% — signals the tool is becoming embedded in workflow |

**"Aha moment" definition:** The user has experienced the aha moment when they have **created at least 1 rule AND witnessed it enforce (block or allow) at least 1 agent action.** This is the point where the abstract concept of "temporal governance" becomes a felt experience.

### Validation Artifacts

#### A. Landing Page Specification

**URL:** `deltat.dev` or `deltat-zaman.dev` (register during validation phase)

**Structure:**

| Section | Content | What to Measure |
|---|---|---|
| **Hero** | Headline: "Your AI agents don't know what time it is." Subhead: "Delta-T Zaman is a temporal command center — set timezone rules for your autonomous agents from your phone." 30-second demo GIF of clock UI. CTA: "Join the waitlist." | Bounce rate, scroll depth, waitlist conversion |
| **Problem** | Three pain points as cards: (1) "Agents deploy at 2AM" (2) "No kill switch when you sleep" (3) "No undo when you wake up." Each with a one-line description. | Click-through to each card (engagement) |
| **Solution** | 3 feature blocks mapping to Journeys 1–3: Deploy Gate, Sleep Fence, Undo Slider. Each with a static mockup screenshot. | Time-on-section |
| **Social proof** | "Built on Cloudflare edge + Undisk MCP. Sub-50ms undo. 330+ global edge locations." Logo bar: Tauri, Cloudflare, Undisk. | Trust signal — reduces bounce |
| **Pricing** | "$19/mo for indie developers. $49/mo for teams. Free during alpha." | Pricing page view rate, CTA click rate |
| **Waitlist CTA** | Email capture + 2 optional questions: (1) "How many timezones do you work across?" (2) "What AI coding agent do you use?" | Email captures, question response rate |
| **Footer** | "Built by [founder name]. Open-source clock component on GitHub." Link to GitHub. | GitHub click-through |

**Tech stack:** Static site (Astro or plain HTML). Hosted on Cloudflare Pages (free). Analytics: Plausible (privacy-first, $9/mo) or Cloudflare Web Analytics (free).

**Success metric:** ≥150 waitlist emails in 4 weeks from organic traffic (HN, Reddit, X, communities).

#### B. Customer Discovery Interview Script

**Target:** 15+ interviews with ICP-matching developers. Recruit from X, Indie Hackers, Cursor/Anthropic communities, and personal network.

**Duration:** 25–30 minutes. Recorded (with consent) for analysis.

**Script:**

---

**Introduction (2 min)**

> "Thanks for taking the time. I'm researching how developers manage AI coding agents across timezones. I'm not selling anything — I'm trying to understand your workflow. There are no right or wrong answers. Can I record this for my notes?"

**Part 1 — Context (5 min)**

1. "Tell me about your current setup — what AI coding agents or tools do you use day-to-day?"
   - *Listen for:* Cursor, Claude Code, Copilot Agent, autonomous workflows, MCP servers
   
2. "How many timezones do you regularly work across? Walk me through a typical day."
   - *Listen for:* Collaborators in different TZs, deployment targets in specific regions, personal schedule constraints

3. "When you step away from your computer — say, overnight — do your AI agents keep running? What happens?"
   - *Listen for:* Unattended agent execution, overnight incidents, cost spikes, anxiety about unsupervised agents

**Part 2 — Pain Exploration (10 min)**

4. "Can you tell me about a time an AI agent did something unexpected when you weren't actively supervising it?"
   - *Listen for:* Specific incidents — file overwrites, bad deploys, cost overruns, waking up to a mess
   - *Probe:* "What happened? How did you find out? How long did it take to fix? What did it cost you?"

5. "What's your current approach to controlling *when* your agents can or can't act?"
   - *Listen for:* Cron jobs, manual pausing, GitHub Actions schedules, "I just accept the risk"
   - *Probe:* "How well does that work? What's still a problem?"

6. "If I asked you to rank your top 5 pain points with AI coding agents, where would 'controlling when they run' fall?"
   - *Listen for:* Ranking — is temporal governance top-3, top-5, or not on the list at all?
   - *Critical signal:* If <53% rank it top-5, problem validation fails.

7. "Have you ever wished you could just set a 'do not disturb' schedule for your agents — like a sleep mode?"
   - *Listen for:* Enthusiasm, specific use cases, or "not really a problem for me"

**Part 3 — Solution Reaction (8 min)**

8. "Imagine a clock app on your phone where you can tap a timezone block and set a rule like 'block deploys in Tokyo before 9AM' or 'pause all agents from 11PM to 7AM my time.' What's your first reaction?"
   - *Listen for:* "That's exactly what I need" vs. "I'd rather use a CLI/dashboard" vs. "I don't understand why a clock"
   - *Show Figma prototype if available*

9. "Would you prefer to control this from your phone, your laptop, or both? Why?"
   - *Listen for:* Mobile preference validates USP's "from their phone" claim. Laptop preference = USP risk [Source — usp.md: Confidence Limiter #2]

10. "If the agent was blocked by a rule, and you needed to override it urgently at 3AM, how would you want that to work?"
    - *Listen for:* Emergency override expectations, friction tolerance

**Part 4 — Willingness to Pay (5 min)**

11. "If this tool existed today, at what monthly price would it be: (a) so cheap it feels low-quality, (b) a great deal, (c) getting expensive but still worth it, (d) too expensive to consider?"
    - *Van Westendorp pricing — record all four values*

12. "Specifically, would you pay $19/month for this? What about $49/month for a team version?"
    - *Listen for:* Firm yes, conditional yes ("if it integrates with X"), or no
    - *Critical signal:* ≥3 firm "yes" from 15 interviews required

**Closing (2 min)**

13. "Is there anything else about managing AI agents across timezones that I should know about?"
    - *Listen for:* Pain points not covered, feature requests, adjacent problems

14. "Would you be interested in early access when we launch? Can I add you to the list?"
    - *Convert interviewee to waitlist*

15. "Can you recommend 1–2 other developers who manage agents across timezones?"
    - *Snowball recruitment for additional interviews*

---

#### C. Smoke Test Design

**Primary test: Waitlist landing page (fake door)**

| Element | Specification |
|---|---|
| **Type** | Landing page with email capture. No product exists yet. |
| **Traffic source** | Organic: HN Show HN post, X thread, Reddit posts, community posts |
| **Conversion event** | Email submitted on waitlist form |
| **Success threshold** | ≥150 emails in 4 weeks (from organic traffic only) |
| **Failure threshold** | <50 emails in 4 weeks → insufficient demand signal |
| **Data collected** | Email, timezone count (optional), AI agent used (optional), referral source (UTM) |

**Secondary test: Figma prototype usability**

| Element | Specification |
|---|---|
| **Type** | Clickable Figma prototype (5 screens). Remote moderated testing via Zoom. |
| **Participants** | 10 ICP-matching developers |
| **Task** | "Set a rule that blocks deploys in Tokyo before 9AM." |
| **Success threshold** | ≥6/10 complete the task without guidance [Source — feasibility.md: Proceed-if gate] |
| **Failure threshold** | <4/10 complete → clock-as-UI is unintuitive, pivot to conventional UI |
| **Data collected** | Task completion rate, time-on-task, NPS score, qualitative feedback, clock vs. dashboard preference |

**Tertiary test: Pricing validation (Stripe fake checkout)**

| Element | Specification |
|---|---|
| **Type** | Pricing page with "Start free trial" button → Stripe checkout page → email capture (no charge) with message: "You're on the list! We'll email you when the alpha launches." |
| **Traffic source** | Subset of landing page visitors who reach pricing section |
| **Conversion event** | Email captured on Stripe fake checkout page |
| **Success threshold** | ≥30 emails via this path in 4 weeks |
| **Failure threshold** | <10 emails → pricing page is a wall; test lower price or different framing |

#### D. Success / Failure Criteria

| Signal | Metric | PROCEED | ITERATE | PIVOT |
|---|---|---|---|---|
| **Problem validation** | Interviewees ranking temporal governance top-5 pain (unprompted) | ≥ 8/15 (53%) | 5–7/15 (33–47%) — problem exists but is niche | < 5/15 (33%) — problem is not salient enough |
| **Solution validation** | Prototype task completion rate (unguided) | ≥ 6/10 (60%) | 4–5/10 (40–50%) — UI needs iteration, concept resonates | < 4/10 (40%) — clock-as-UI paradigm rejected |
| **Waitlist demand** | Email signups from organic traffic (4 weeks) | ≥ 150 | 50–149 — messaging needs work, demand is present | < 50 — insufficient market pull |
| **Willingness to pay** | Firm "yes" at $19/mo from interviews | ≥ 3/15 (20%) | 1–2/15 — price may need to drop to $9–14/mo | 0/15 — product is not valued enough to pay for |
| **Pricing checkout** | Fake checkout email captures | ≥ 30 | 10–29 — pricing friction; test lower price | < 10 — fundamental pricing/value disconnect |
| **Mobile preference** | Interviewees who prefer phone control over laptop | ≥ 8/15 (53%) | 5–7/15 — mobile-first USP is risky, consider "mobile + desktop" | < 5/15 — mobile-first positioning rejected; pivot to desktop-first |

**Decision rules:**
- **PROCEED** with 10-week MVP build if ≥4 of 6 signals are in the "PROCEED" column AND neither Problem Validation nor Solution Validation is in "PIVOT."
- **ITERATE** on hypothesis/positioning if 2–3 signals are in "ITERATE" — re-run validation after changes.
- **PIVOT** if any 2+ signals are in "PIVOT" column — the core hypothesis is invalidated. Explore alternative positioning (e.g., desktop-first temporal dashboard without mobile-first USP, or timezone utility without agent governance).

---

## Go-to-Market Sequence

### Pre-Launch (Validation Phase — Weeks -4 to 0)

| Week | Activity | Deliverable |
|---|---|---|
| V-Week 1 | Set up landing page. Begin customer discovery interviews. Start Reddit/community seeding. | Landing page live. 3–5 interviews completed. |
| V-Week 2 | Build Figma prototype. Continue interviews. Post first content piece. | Prototype ready for testing. 8–10 interviews completed. |
| V-Week 3 | Run prototype tests (5–7 testers). Analyze interview data. Post HN Show HN (landing page only). | Prototype test results. Demand signal from HN. |
| V-Week 4 | Complete remaining interviews + prototype tests. Compile validation report. Make go/no-go decision. | 15+ interviews, 10+ prototype tests, waitlist count. GO/NO-GO. |

### Post-MVP Launch (Weeks 1–8)

#### Weeks 1–2: Launch Blitz

| Day | Channel | Action |
|---|---|---|
| Day 1 | Hacker News | Show HN post: "Delta-T Zaman — A temporal command center for AI agents (Tauri + Cloudflare + Undisk)" with TestFlight link |
| Day 1 | X / Twitter | Launch thread: 10-tweet thread with demo GIFs. Tag Tauri, Cloudflare, Undisk accounts. |
| Day 1 | Landing page | Update CTA from "Join waitlist" to "Download on TestFlight" (iOS) / "Install" (macOS) |
| Day 2 | Reddit | Post to r/programming, r/devops, r/SideProject (staggered, authentic framing) |
| Day 2 | Indie Hackers | Launch post on Indie Hackers with revenue/metrics transparency |
| Day 3 | AI communities | Post tutorials in Anthropic, Cursor, OpenAI communities |
| Day 3 | MCP ecosystem | List on MCP directories (mcp.run, Smithery, glama.ai) |
| Day 5 | Product Hunt | Launch on Product Hunt (if audience overlap with ICP is sufficient — developer tools category) |
| Day 7 | Email | Send launch email to all waitlist subscribers |
| Week 2 | Content | Publish "Building Delta-T" technical blog post. Record and publish demo video (2–3 min). |

**Week 1–2 targets:**
- 500+ app downloads (TestFlight + macOS)
- 100+ users who open the app and add ≥1 timezone
- 20+ users who create a governance rule (4% activation rate)

#### Weeks 3–4: Iterate on Activation

| Focus | Action | Metric |
|---|---|---|
| **Activation funnel** | Analyze where users drop off: install → add timezone → create rule → rule triggers. Instrument each step. | Conversion between funnel steps |
| **Onboarding** | If activation < 10%, add guided onboarding: "Try setting a sleep fence for tonight." Pre-populate with user's detected timezone. | Time-to-first-rule (target: ≤ 5 min) |
| **Feedback loops** | In-app "What's missing?" prompt after Day 3. Direct outreach to first 20 rule-creators for 15-min feedback calls. | Qualitative themes: missing features, UX pain, bugs |
| **Bug triage** | Fix top 5 reported bugs. Prioritize anything blocking rule creation or Undisk integration. | Crash-free rate ≥ 95% |
| **Content** | Publish "How I use Delta-T to manage my Claude Code agent" (personal use case). Share user testimonials if available. | Content engagement (views, shares) |

**Week 3–4 targets:**
- 50+ users with ≥1 governance rule active
- Time-to-first-rule ≤ 5 minutes for 60% of new users
- 5+ qualitative feedback calls completed
- Net Promoter Score ≥ 30 from early users

#### Weeks 5–6: Scale What's Working

| If This Works | Scale It |
|---|---|
| HN drove >200 installs | Write a follow-up "1 month of Delta-T" post with real metrics. Post to HN again. |
| AI communities drove >50 installs | Create community-specific tutorials (Cursor + Delta-T, Claude Code + Delta-T). Offer to do live demos in community calls. |
| Open-source clock component got >100 stars | Publish v2 with additional timezone features. Cross-promote with the app. |
| Activation rate > 10% | Begin testing paid channels: $200 TLDR newsletter sponsorship. Track cost-per-activated-user. |
| Activation rate < 5% | Do NOT scale. Return to iteration. Redesign onboarding. Consider if the problem is product, not distribution. |

**Week 5–6 targets:**
- 1,000+ total app downloads
- 100+ active users (opened app in last 7 days)
- 50+ users with active governance rules
- First 3–5 paying customers (if billing is live) OR 50+ "would pay" survey responses

#### Weeks 7–8: Evaluate and Decide

| Metric | PROCEED to Phase 2 | ITERATE (extend alpha 4 weeks) | PIVOT |
|---|---|---|---|
| Active users (7-day) | ≥ 100 | 30–99 | < 30 |
| Users with active rules | ≥ 50 | 15–49 | < 15 |
| Day-30 retention | ≥ 15% | 8–14% | < 8% |
| Willingness to pay (survey or actual) | ≥ 10 firm commitments at $19/mo | 3–9 commitments | < 3 commitments |
| NPS | ≥ 30 | 10–29 | < 10 |

**Decision:**
- **PROCEED:** Build Journey 2 (Sleep Fence) + Journey 3 (Undo Slider). Launch Stripe billing. Target App Store release.
- **ITERATE:** Revisit onboarding, UX, and feature set. Extend alpha by 4 weeks. Re-test with modified product.
- **PIVOT:** The temporal agent governance hypothesis is invalidated at this scale. Options: (a) pivot to timezone utility only (drop agent governance), (b) pivot to desktop-first dashboard (drop mobile-first USP), (c) shelve the project.

---

## Metrics Framework

### North Star Metric

> **Governance Rules Enforced Per Week (GRE/W)**

**Definition:** The total number of times a Delta-T governance rule successfully blocked or allowed an agent action across all users in a given week.

**Why this metric:**
- It captures value delivered — a rule that enforces is a rule that protects the developer.
- It encompasses both adoption (rules created) and engagement (agents triggering rules).
- It's leading for revenue: a user whose rules are actively enforcing is a user who experiences the product's value and will pay for it.
- It's not gameable: you can't inflate GRE/W without real agent activity hitting real rules.

[Source — icp.md: "Agent governance rules are set once and modified infrequently (weekly or less)." The value is in enforcement, not creation.]

### Leading Indicators (Predict Future Success)

| Indicator | Definition | Target | Why It Leads |
|---|---|---|---|
| **Rules created per user per week** | Average number of new governance rules created per active user per week | ≥ 0.5 in first month, ≥ 0.2 steady-state | Rule creation = user is investing in the product. More rules → more enforcement → more value delivered → retention. |
| **Time-to-first-rule** | Median minutes from app install to first governance rule creation | ≤ 5 minutes | Fast activation predicts retention. ICP expects 5-minute time-to-value [Source — icp.md: Evaluation Pattern]. Exceeding this = churn risk. |
| **Undisk workspaces connected** | Number of unique Undisk workspaces linked to Delta-T | ≥ 30% of users connect within 7 days | Integration depth predicts retention + willingness to pay. Connected users are 3–5x more likely to convert [Model-sourced — SaaS integration benchmarks]. |
| **Mobile session frequency** | Average app opens per user per week | ≥ 3/week | Validates mobile-first USP. If users only use desktop, mobile-first positioning is wrong [Source — usp.md: Confidence Limiter #2]. |

### Lagging Indicators (Confirm Past Success)

| Indicator | Definition | Target | Why It Lags |
|---|---|---|---|
| **Monthly Recurring Revenue (MRR)** | Total subscription revenue per month | $570/mo by month 3 (30 customers × $19) | Revenue is the ultimate validation but takes months to materialize. Track from billing day 1. |
| **Day-30 retention** | % of users who open the app 30 days after install | ≥ 15% | Developer tools average 10–20% Day-30 retention [Model-sourced]. Below 10% = product-market fit not achieved. |
| **Net Promoter Score (NPS)** | "How likely are you to recommend Delta-T to a colleague?" (0–10) | ≥ 30 | NPS ≥ 30 = "good" for SaaS. Below 0 = detractors outnumber promoters. Survey at Day 14 and Day 30. |
| **Customer Acquisition Cost (CAC)** | Total marketing spend ÷ new paying customers | ≤ $50 (organic) / ≤ $150 (paid) | At $19/mo, LTV:CAC must be ≥ 3:1 for sustainability. 12-month LTV = ~$171 (assuming 25% annual churn) → max CAC = $57. [Model-sourced — SaaS LTV:CAC benchmarks] |

### Red Flag Metrics (Pivot Signals)

| Red Flag | Threshold | What It Means | Action |
|---|---|---|---|
| **< 5% of users create a rule within 7 days** | Activation failure | The product's value isn't being discovered. Onboarding is broken or the problem isn't salient enough. | Redesign onboarding. If still failing after iteration, the problem validation was a false positive. |
| **> 60% of governance rules are never triggered** | Rules created but agents never hit them | Users are setting rules that don't align with their actual agent behavior. The product is aspirational, not functional. | Investigate: are users connecting Undisk? Are agents running? Add telemetry for rule-trigger matching. |
| **Mobile sessions < 30% of total sessions** | Desktop dominance | Mobile-first USP is not reflected in usage. Users prefer desktop. | Consider pivoting to desktop-primary, mobile-secondary positioning. Revisit USP Confidence Limiter #2 [Source — usp.md]. |
| **Day-7 retention < 15%** | Immediate churn | Users try it once and leave. The "aha moment" isn't happening within 7 days. | Accelerate time-to-first-enforcement: auto-suggest a rule based on detected timezone. |
| **NPS < 0** | Net detractors | More users are unhappy than happy. Product is causing frustration (likely UX or reliability issues). | Halt growth. Fix reliability and UX. NPS < 0 means word-of-mouth is negative — growth will be self-defeating. |
| **0 paying customers after 8 weeks post-MVP** | No willingness to pay | The free product is useful but not valuable enough to pay for. The $19/mo barrier is real. | Test lower pricing ($9/mo). If still zero, the monetization hypothesis is invalid. |

---

## Risk-Adjusted User Projections

### Assumptions (Common to All Scenarios)

| Assumption | Value | Basis |
|---|---|---|
| HN Show HN reach (if front page) | 100K impressions | [Model-sourced — HN traffic benchmarks for developer tools] |
| HN → landing page conversion | 5% | [Model-sourced — typical HN CTR for "Show HN" posts] |
| Landing page → waitlist/install conversion | 8% | [Model-sourced — developer tool landing page benchmarks] |
| Waitlist → active user conversion | 40% | [Model-sourced — pre-launch waitlist activation for developer tools] |
| AI community post reach | 5K–15K per community | [Model-sourced — estimated from community sizes in Channel 3] |
| Community → install conversion | 3% | Higher intent but smaller audience |
| Free → paid conversion (month 3+) | 3–5% | Developer SaaS benchmark: GitHub Copilot, Cursor achieve 3–8% [Source — icp.md: Spending Pattern; feasibility.md: Financial Feasibility conversion assumptions] |
| Monthly churn (paid) | 5–8% | Developer tools average 3–10% monthly churn [Model-sourced] |
| Undisk workspace connection rate | 30–50% of installs | [Model-sourced — integration activation for tools requiring external service connection] |

### Scenario 1: Conservative

**Premise:** HN post doesn't reach front page. Community reception is lukewarm. Activation rate is low. Mobile-first positioning creates friction.

| Month | New Installs | Active Users | Paid Customers | MRR |
|---|---|---|---|---|
| Month 1 | 150 | 60 | 0 (free alpha) | $0 |
| Month 3 | 80/mo | 100 | 5 | $95 |
| Month 6 | 50/mo | 120 | 12 | $228 |

**Key assumptions:**
- HN post gets ~30 points, 20K impressions → 200 landing page visits → 16 installs from HN
- Community posts generate ~100 installs total over 8 weeks
- 40% of installs become active (add timezone + open >3 times)
- 3% free-to-paid conversion
- 8% monthly churn on paid
- No paid marketing spend

**Verdict at month 6:** Below break-even ($228 < $2,000 target). ITERATE or PIVOT decision required.

### Scenario 2: Moderate

**Premise:** HN post reaches front page. AI community reception is strong. Clock UI tests well in prototype. Activation rate is healthy.

| Month | New Installs | Active Users | Paid Customers | MRR |
|---|---|---|---|---|
| Month 1 | 500 | 200 | 0 (free alpha) | $0 |
| Month 3 | 200/mo | 350 | 18 | $342 |
| Month 6 | 150/mo | 500 | 40 | $760 |

**Key assumptions:**
- HN front page: 100K impressions → 5K landing page visits → 400 installs from HN
- Community posts generate 300 installs over 8 weeks
- Content marketing adds 50 installs/month by month 3
- 40% of installs become active
- 5% free-to-paid conversion
- 5% monthly churn on paid
- $200 total paid marketing (one newsletter sponsorship in month 2)

**Verdict at month 6:** Below sustainability target ($760 < $2,000) but on trajectory. PROCEED with growth investment.

### Scenario 3: Optimistic

**Premise:** HN post goes viral (>500 points). Undisk co-marketing amplifies reach. Multiple AI community posts gain traction. Strong word-of-mouth from early users.

| Month | New Installs | Active Users | Paid Customers | MRR |
|---|---|---|---|---|
| Month 1 | 1,500 | 600 | 0 (free alpha) | $0 |
| Month 3 | 500/mo | 1,000 | 50 | $950 |
| Month 6 | 400/mo | 1,500 | 120 | $2,280 |

**Key assumptions:**
- HN viral: 300K impressions → 15K landing page visits → 1,200 installs from HN
- Community posts generate 500 installs, amplified by Undisk partnership
- Open-source clock component reaches GitHub trending → 200 additional installs
- 40% of installs become active
- 5% free-to-paid conversion
- 5% monthly churn on paid
- Organic referrals add 10% to monthly installs by month 3
- $500 total paid marketing (newsletter + podcast guest appearance)

**Verdict at month 6:** At sustainability target ($2,280 ≥ $2,000). PROCEED to scale. Begin App Store submission and Journey 2 build.

### Projection Summary

| Metric | Conservative | Moderate | Optimistic |
|---|---|---|---|
| Month 1 installs | 150 | 500 | 1,500 |
| Month 3 active users | 100 | 350 | 1,000 |
| Month 6 paying customers | 12 | 40 | 120 |
| Month 6 MRR | $228 | $760 | $2,280 |
| Months to $2K MRR | 12+ | 9 | 6 |
| Break-even (3 customers) | Month 3 | Month 2 | Month 2 |

---

## Pre-Build Validation Checklist

The following must be TRUE before writing production code for the 10-week MVP. Items are ordered by dependency and priority.

- [ ] **15+ customer discovery interviews completed** — with ICP-matching developers (solo devs / small teams using AI agents across 3+ timezones). At least 8/15 must rank temporal agent governance as a top-5 pain point. [Source — feasibility.md: Required mitigations #4; icp.md: Verification Needed]
- [ ] **Problem validation signal positive** — ≥53% of interviewees confirm the pain unprompted. If <33% confirm, PIVOT. [Source — this artifact: Success/Failure Criteria, Signal 1]
- [ ] **Prototype usability test passed** — ≥6/10 ICP testers complete the "set a deploy gate" task in the Figma prototype without guidance. Clock-as-UI paradigm is accepted. [Source — feasibility.md: "Proceed if: at least 6/10 prototype testers find the clock-as-governance-UI intuitive"]
- [ ] **≥150 waitlist signups** — from organic traffic on the landing page within 4 weeks. Validates demand beyond interview subjects. [Source — this artifact: Smoke Test Design]
- [ ] **≥3 firm pricing commitments** — interviewees who say "yes, I would pay $19/mo for this today." Van Westendorp median acceptable price ≥$15/mo. [Source — this artifact: Signal 3]
- [ ] **Undisk partnership confirmed** — Direct communication with Undisk team confirming: (a) API stability commitment for `set_policy`, `restore_version`, `workspace_checkpoint` tools, (b) no plans to build competing mobile governance UI, (c) willingness to co-market or list Delta-T as an ecosystem partner. [Source — feasibility.md: Required mitigations #3; Risk R1: Undisk single-vendor dependency]
- [ ] **Tauri iOS physical device build confirmed** — A minimal Tauri 2.x app running on a physical iPhone (not just simulator). WebView renders, `fetch()` to external endpoint works, push notifications are received. This is the Week 1 gate. [Source — feasibility.md: Required mitigations #2; Risk R10: Tauri mobile maturity]
- [ ] **Mobile-first preference validated** — ≥53% of interviewees prefer controlling agent governance from their phone over their laptop. If <33% prefer mobile, pivot USP to "desktop + mobile" rather than "mobile-first." [Source — usp.md: Confidence Limiter #2]
- [ ] **Apple Developer account active** — $99/yr Apple Developer Program enrollment confirmed. Provisioning profile generated. TestFlight distribution capability verified. [Source — feasibility.md: iOS App Store distribution]
- [ ] **Founder runway confirmed** — ≥6 months of living expenses available, plus $200 budgeted for pre-launch infrastructure and marketing. [Source — feasibility.md: Solo developer runway — "$100–200 total infrastructure spend"]

---

## Confidence Limiters

### 1. All User Projections Are Model-Sourced (Impact: High)

The user projection scenarios rely on estimated conversion rates (5% HN CTR, 8% landing page conversion, 3–5% free-to-paid) drawn from general developer tool benchmarks, not from Delta-T-specific data. Actual conversion rates could vary by 2–5x in either direction. The projections should be treated as order-of-magnitude estimates, not forecasts. No comparable product (mobile-first temporal governance for AI agents) exists to calibrate against. [Model-sourced]

### 2. Channel Reach Estimates Are Pre-Validation (Impact: Medium)

Community sizes and post reach estimates (HN impressions, Reddit views, AI community engagement) are based on general platform benchmarks. Delta-T's specific framing ("temporal governance") may resonate strongly (novel concept → high engagement) or fall flat (unfamiliar concept → low click-through). The pre-launch landing page test will provide the first real data point. [Model-sourced]

### 3. Interview Recruitment Bias (Impact: Medium)

Recruiting interviewees from X, Indie Hackers, and AI communities introduces selection bias: these developers are more likely to be early adopters, more AI-savvy, and more receptive to novel tools than the general developer population. Interview results may overstate demand. Mitigation: include at least 3 interviewees recruited through personal network (not self-selected from communities) to diversify the sample. [Model-sourced]

### 4. Mobile-First Demand Is Unvalidated (Impact: Medium-High)

The entire go-to-market strategy assumes developers want mobile-first governance, but this is the USP's highest-risk claim. PagerDuty and Datadog mobile apps prove developers *monitor* from phones, but *write-capable governance* from mobile is unprecedented. If interviews reveal strong desktop preference, the launch strategy, channel priorities, and messaging all need revision. [Source — usp.md: Confidence Limiter #2; icp.md: Pain #5]

### 5. Undisk Ecosystem Size Is Unknown (Impact: Medium)

The MCP/Undisk channel is rated P0, but the actual number of developers actively using Undisk MCP workspaces is unknown. If the ecosystem is <1,000 active users, this "perfect fit" channel delivers very few installs. The broader MCP ecosystem is growing but still nascent. [Model-sourced — no public Undisk user counts available]

### 6. Pricing Sensitivity to IAP Routing (Impact: Low-Medium)

The $19/mo pricing assumes web checkout (Stripe) to avoid Apple's 30% IAP cut. If Apple requires IAP for in-app subscription management, the effective price to maintain margin would be ~$25/mo, which may exceed the ICP's willingness to pay. The Van Westendorp test in interviews should stress-test the $25 price point as well. [Source — feasibility.md: Risk R6]

---

## Quality Score

| Criterion | Score (1-5) | Rationale |
|---|---|---|
| **Specificity** | 4 | 8 channels evaluated with specific reach/cost/conversion estimates. Quantitative thresholds for all 6 validation signals. Week-by-week GTM sequence with named platforms and target numbers. 15-question interview script with specific probes and listening-for cues. Score not 5 because channel reach estimates are pre-validation and inherently imprecise. |
| **Actionability** | 5 | Every section produces a concrete deliverable: landing page spec (what to build), interview script (what to ask), smoke test design (what to measure), success/failure criteria (how to decide), GTM sequence (when to do what), pre-build checklist (what must be true). A solo founder can execute this plan starting tomorrow with zero budget. |
| **Non-redundancy** | 5 | No overlap with icp.md (profiles users), usp.md (defines positioning), or feasibility.md (evaluates technical viability). This artifact uniquely answers: "How do we find users, validate demand, and sequence our launch?" — a function none of the upstream artifacts perform. The interview script operationalizes icp.md's verification needs; the GTM sequence operationalizes feasibility.md's timeline. |
| **Evidence quality** | 4 | Channel analysis cites ICP discovery patterns from icp.md. Success thresholds cite feasibility.md's proceed-if gates. Pricing thresholds cite feasibility.md's financial model. User projections are clearly tagged [Model-sourced] with explicit assumptions. Score not 5 because conversion rate benchmarks are drawn from general developer tool data, not from comparable temporal-governance products (none exist). |
| **Overall** | **4.5** | Comprehensive validation plan grounded in upstream artifacts with quantitative decision gates. Primary weakness is reliance on model-sourced conversion benchmarks with no comparable product to calibrate against. This is structurally unavoidable for a category-creation product — the validation plan itself is designed to generate the missing data. |
