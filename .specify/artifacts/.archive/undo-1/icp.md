---
artifact_meta:
  produced_by: "os.market"
  produced_at: "2025-07-27T02:30:00Z"
  confidence: 0.50
  inputs_used:
    - ".specify/artifacts/sketch.md"
    - ".specify/artifacts/phase-1/market-map.md"
  stale_after: "on_input_change"
  revision: 1
  quality_scores:
    specificity: 3
    actionability: 4
    non_redundancy: 5
    evidence_quality: 3
  grade: "draft"
---

# Ideal Customer Profile: Delta-T Zaman

## Demographics

### Primary ICP: The Timezone Juggler

| Attribute | Profile | Evidence |
|-----------|---------|----------|
| **Role** | Software engineer, product manager, designer, or freelancer working on a distributed team | Stack Overflow 2025: 45% of US developers work fully remote, highest adoption rate globally; only 16.2% are fully in-office `[Verified]` |
| **Age** | 25–45 (Millennials and older Gen Z) | 37% of digital nomads are Millennials `[Verified]` (Pumble, 2025); tech workforce skews 25–45 for remote-eligible roles `[Model-sourced]` |
| **Education** | Bachelor's degree or higher | 42.8% of employees with advanced degrees work remotely `[Verified]` (WorkTime, 2025); 91% of digital nomads report higher education `[Verified]` (Pumble) |
| **Platform** | macOS primary; may also use Linux or Windows secondary | Mac adoption among developers is ~30–35% `[Model-sourced]` (Stack Overflow historical data); significantly higher among designers, product managers, and indie devs |
| **Geography** | US (primary), Western Europe, Australia, Canada | 46% of digital nomads are American `[Verified]` (Pumble); remote work adoption highest in North America, Europe, and Australia `[Verified]` (Gable.to, 2025) |
| **Company size** | 10–500 employees (distributed startup/scaleup) OR freelancer with international clients | Fully remote companies like GitLab, Automattic, Zapier represent the archetype; 36% of employed Americans are independent workers, up from 27% in 2016 `[Verified]` (McKinsey, via Compunnel) |
| **Income** | $80K–$200K USD (knowledge worker with disposable income for productivity tools) | 82% of digital nomads report being very satisfied with their income `[Verified]` (Pumble); tech worker salaries support $5–$15 tool purchases trivially `[Model-sourced]` |

### Secondary Demographics

| Segment | Size Estimate | Notes |
|---------|---------------|-------|
| **Managers of distributed teams** | Subset of primary ICP | Need timezone awareness for scheduling 1:1s and standups across zones. HBR published "Managing a Team Across 5 Time Zones" addressing this pain `[Verified]` |
| **International students / academics** | Unknown | Collaborate with advisors and peers in different countries. Lower willingness to pay. |
| **Long-distance relationship maintainers** | Unknown | Clocker App Store reviews cite "keep track of friends in different time zones" as a use case `[Verified]` |

## Psychographics

### Values and Priorities

| Trait | Evidence |
|-------|----------|
| **Values tool craftsmanship** | This user already uses apps like Raycast, Arc Browser, or Notion — tools chosen for design quality and native performance, not just functionality. They pay for well-made software. `[Model-sourced]` — inferred from Mac indie app ecosystem behavior |
| **Prefers ownership over rental** | Strong documented preference among Mac users for one-time purchases: "The feeling of ownership matters, especially for desktop software that runs locally" `[Verified]` (fazm.ai, 2025). Dato explicitly markets "one-time purchase, never subscription-based" as a feature `[Verified]` |
| **Privacy-conscious** | 77% of digital nomads are early tech adopters `[Verified]` (Pumble); the Mac indie app audience specifically values "no data collection" — menubartime.com lists "Privacy-focused: no data collection" as a feature `[Verified]` |
| **Efficiency-oriented** | Workers in collaborative non-routine roles shift work to early morning/late evening to match colleagues' hours when temporal distance increases `[Verified]` (HBS/Choudhury, 2024). They actively manage timezone friction rather than ignore it. |
| **Aesthetically sensitive** | Mac users in the target segment choose tools partly on design quality. A Clocker review praised the developer's "good sense of design/usability" `[Verified]` (App Store review). Dato's MacStories review and polished UI are central to its appeal `[Verified]` |

### Decision-Making Patterns

- **Discovery**: Finds tools through developer Twitter/X, Hacker News, Product Hunt, Reddit (r/macapps, r/remotework, r/digitalnomad), or word-of-mouth from colleagues `[Model-sourced]`
- **Evaluation**: Downloads a free trial; decides within 1–3 sessions whether it earns a permanent menu bar slot. The menu bar is prime real estate — apps that don't justify their space get removed quickly `[Model-sourced]`
- **Purchase trigger**: "This just saved me from scheduling a meeting at 3am someone's time" — a single moment of friction avoided justifies the $5–$10 price `[Model-sourced]`
- **Switching cost**: Low. World clock apps are not data-locked. Users can and do switch easily. Loyalty is earned by UX quality and reliability, not lock-in `[Model-sourced]`

## Pain Points

### P1: "What time is it there right now?" (Reactive lookup friction)
- **Behavior**: User opens a new browser tab, types "time in Tokyo," reads result, closes tab. Repeated 3–10x per day.
- **Friction**: Context switch away from current work. Each lookup takes 10–15 seconds and breaks flow.
- **Evidence**: Google "time in [city]" is one of the most common time-related searches globally `[Model-sourced]`. Clocker and Menu World Time reviews cite this exact use case: "click on the menu bar and instantly see the current time in all the locations" `[Verified]` (App Store review)
- **Severity**: Low-medium per instance, but cumulative across a workday of 5–10 lookups

### P2: "If I schedule this for 3pm my time, what time is it for them?" (Planning friction)
- **Behavior**: User does mental UTC math, or opens worldtimebuddy.com, or checks their phone's world clock, or opens Google and types a conversion query
- **Friction**: Mental math is error-prone (especially across DST boundaries). Web tools require a separate tab and context switch.
- **Evidence**: Clocker App Store review: "click and drag the scroll bar... you can see past/future dates and times as well. Perfect for planning calls, meetings, etc." `[Verified]`. Dato calls this feature "time travel" `[Verified]`. The "Time" app from menubartime.com calls it "time scroller preview" `[Verified]`.
- **Severity**: Medium. Errors cause meetings scheduled at wrong times — a high-cost mistake when it happens.

### P3: "I forgot they're 8 hours ahead and messaged at midnight their time" (Awareness gap)
- **Behavior**: User sends a Slack message or email without checking the recipient's local time
- **Friction**: HBS research confirms: "when people are spread out across time zones, communication patterns shift" and workers in collaborative roles shift their schedules into personal time to compensate `[Verified]` (Choudhury, 2024)
- **Evidence**: A 1-hour increase in temporal distance reduces synchronous communication by 11% `[Verified]` (Rice Business / Pan Fang). Workers who lose even one hour of timezone overlap shift their workday into early morning or late evening `[Verified]` (HBS)
- **Severity**: High for team culture and individual well-being, but many users don't attribute this pain to a solvable problem

### P4: "I missed the window — their workday ended while I was in meetings" (Overlap window blindness)
- **Behavior**: User doesn't have ambient awareness of when a collaborator's workday starts/ends
- **Friction**: Without visual indicators, overlap windows are missed, delaying decisions by 24 hours
- **Evidence**: Deel's remote team management guide recommends "visual tools such as Timezone.io and Everytimezone.com to understand team member locations and time zones" `[Verified]`
- **Severity**: Medium. Primarily affects managers and leads coordinating across teams.

## Behavioral Patterns

### How They Currently Solve It

| Solution | Usage Pattern | Satisfaction | Switching Likelihood |
|----------|---------------|--------------|---------------------|
| **Google search** ("time in Tokyo") | 3–10x/day, reactive | Adequate for one-off lookups; frustrating for habitual users | Low — zero friction, zero cost, but no ambient awareness |
| **iPhone/Android built-in World Clock** | Check phone when needed | Good enough for personal use; awkward when working on laptop | Low — always available on phone |
| **macOS Notification Center World Clock widget** | Opened occasionally | Limited: requires opening Notification Center; no slider; no custom labels | Medium — users who want more already look for alternatives |
| **Clocker** (free, open-source) | Menu bar persistent; has slider | Satisfied users exist; reviews are positive but app updates infrequent (last: April 2024) `[Verified]` | Medium — users may switch if a better-maintained alternative appears |
| **Dato** ($16) | Menu bar persistent; time travel; calendar | High satisfaction — Dato reviews are very positive `[Verified]`; users explicitly praise "no subscription" model | Low — Dato is hard to displace. Users are invested and satisfied. |
| **Web tools** (worldtimebuddy.com, everytimezone.com) | Opened in browser as needed | Good for one-off planning; no native integration | Medium — users who want native experience already look for apps |
| **Slack timezone display** | See local time on colleague profiles | Adequate for "what time is it for this specific person right now" | Low — built into existing workflow |

### Key Behavioral Insight

The target user has **already tried to solve this problem**. They've configured world clocks on their phone, bookmarked worldtimebuddy.com, or installed Clocker. The question is not "do they have this pain?" (yes) but "is their current solution unsatisfying enough to make them switch to a new, paid app?" That is the critical unknown.

## Anti-Personas (Who This Is NOT For)

| Anti-Persona | Why Not | Notes |
|--------------|---------|-------|
| **Single-timezone office workers** | No timezone friction to solve | Vast majority of workers; not addressable |
| **Casual travelers** | Need timezone info for 1–2 weeks/year, not daily | Google search is sufficient for occasional use |
| **Enterprise IT admins** | Need server timezone management, not personal awareness | Different problem domain (UTC timestamps, cron jobs) |
| **Dato power users** | Already invested in a superior product with calendar + time travel + 50+ integrations | Would need a compelling reason to switch; Delta-T Zaman offers less |
| **Non-Mac users** | MVP is macOS-only | ~65–70% of potential users excluded at launch |
| **Price-sensitive users** | Clocker and Menu World Time are free | Users unwilling to pay $5–$10 for a utility will use free alternatives |

## Willingness to Pay Analysis

### Evidence For Payment

- Dato sells at **$16** and has strong reviews — users pay for polished timezone tools `[Verified]`
- Time (menubartime.com) charges **$24.99** — even newer entrants price above $15 `[Verified]`
- The Clock and MiniZones sell at **$3.99–$4.99** `[Verified]`
- Mac indie app ecosystem demonstrates willingness to pay for quality utilities: DevUtils, Raycast Pro, CleanMyMac, etc. `[Verified]` (Market Clarity)
- 82% of digital nomads report income satisfaction `[Verified]` — $5–$10 is trivial for the ICP

### Evidence Against Payment

- **Clocker is free and has a slider** — the core differentiator is available at no cost `[Verified]`
- Median app revenue is **< $50/month** at 1 year post-launch `[Verified]` (RevenueCat 2024) — most apps fail to monetize meaningfully
- Google search is free, instant, and habit-formed — overcoming this default behavior is hard
- **No evidence of unmet demand**: No forums or communities are clamoring for "a better world clock app" — the pain is real but the urgency is low

### Assessment

Willingness to pay exists in the $5–$10 range for a **polished, well-designed** macOS timezone tool — but only if the product visibly exceeds free alternatives in UX quality and feature cohesion. The burden of proof is on the product, not the market.

## Acquisition Channel Hypotheses

| Channel | Reach | Fit | Cost | Priority |
|---------|-------|-----|------|----------|
| **Product Hunt launch** | 5K–50K views day-of | High — indie Mac app audience | Free (time investment) | P0 — primary launch channel |
| **Hacker News "Show HN"** | 1K–20K views if it hits front page | High — developer audience, appreciates Tauri/Rust | Free | P0 — high-value if it resonates |
| **Reddit** (r/macapps, r/remotework, r/digitalnomad) | 500–5K per post | Medium-High — targeted subreddits | Free | P1 — sustained discovery |
| **Twitter/X indie dev community** | Depends on network; 100–10K | High — build-in-public audience | Free | P1 — relationship-driven |
| **Setapp inclusion** | Access to Setapp's user base | Medium — would compete with Dato on same platform | Revenue share (~70/30) | P2 — evaluate after launch |
| **Mac App Store organic** | Low discoverability for "world clock" | Low — dominated by incumbents with more reviews | 30% commission | P2 — necessary but not primary discovery |
| **Indie App Santa / similar promotions** | 5K–50K downloads per campaign | Medium | $300–$800 per campaign `[Verified]` | P2 — post-launch boost |

## Jobs-to-be-Done

### Primary JTBD
**When** I'm working on my Mac and need to coordinate with colleagues or clients in other time zones, **I want to** instantly see what time it is in their locations and quickly check what any proposed meeting time translates to across all relevant zones, **so that** I can communicate at appropriate hours and schedule meetings without timezone errors.

### Secondary JTBDs

1. **When** I'm about to message a teammate, **I want to** see at a glance whether they're in their working hours, **so that** I don't disturb them or expect an immediate response outside their hours.

2. **When** I'm planning a meeting for later this week, **I want to** slide through future hours and see how they map across all my team's zones, **so that** I can find a time that works for everyone without using a separate web tool.

3. **When** a timezone change is approaching (DST transition), **I want to** be notified that my usual overlap window with a colleague will shift, **so that** I can proactively adjust scheduled calls.

### JTBD Tension

The primary job is **already done well** by Dato, Clocker, and Google search. Delta-T Zaman's challenge is not "can we do this job?" but "can we do it well enough to displace existing solutions or capture users who haven't yet committed to one?" The honest answer: probably only if the UX is dramatically better, or if cross-platform (Windows/Linux) support — which no competitor offers natively — becomes the differentiator.

## Verification Needed

> ⚠️ More than 50% of psychographic and behavioral claims rely on `[Model-sourced]` inference rather than direct research. The following should be validated through primary research:

1. **Decision-making patterns**: How the ICP actually discovers, evaluates, and purchases Mac menu-bar utilities. No direct survey data sourced.
2. **Switching likelihood from existing solutions**: Assumed from competitive analysis, not from user interviews.
3. **"Menu bar real estate" scarcity assumption**: Inferred from the Mac power-user experience, not measured.
4. **Purchase trigger specificity**: The "single moment of friction avoided" trigger is a hypothesis, not an observed pattern.
5. **Discovery channels**: Reddit/HN/Product Hunt ranking is based on general indie dev patterns, not specific to timezone tools.

## Quality Score

| Criterion | Score (1-5) | Notes |
|-----------|-------------|-------|
| **Specificity** | 3 | Demographics well-sourced; psychographics and behavioral patterns lean on inference. Pain points are grounded in research but severity ratings are estimated. |
| **Actionability** | 4 | Clear ICP definition, anti-personas, channel priorities, and JTBD framing. Downstream agents can build persona cards and validation experiments from this. |
| **Non-redundancy** | 5 | First ICP artifact in pipeline; extends market-map.md with user-level detail. |
| **Evidence quality** | 3 | Strong on demographic data (BLS, Stack Overflow, MBO Partners). Weak on psychographics and behavioral patterns — no primary research conducted. |

**Weakest section**: Psychographics section relies heavily on `[Model-sourced]` inferences about Mac user preferences. Would benefit from 10–20 user interviews with current Clocker/Dato users to validate decision-making patterns and switching triggers.
