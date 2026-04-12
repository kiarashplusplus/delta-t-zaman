---
description: "Define user acquisition strategy and MVP validation plan before writing code"
handoffs:
  - label: "Next: Brutal Critic"
    agent: "os.critic"
    prompt: "Attack the product idea — find the fatal flaws"
---

## User Input

$ARGUMENTS

## Prerequisites

| Artifact | Expected Path | Producing Agent |
|---|---|---|
| `icp.md` | `.specify/artifacts/phase-1/icp.md` | `os.market` |
| `usp.md` | `.specify/artifacts/phase-1/usp.md` | `os.usp` |
| `feasibility.md` | `.specify/artifacts/phase-1/feasibility.md` | `os.feasibility` |

Before proceeding, run the audit script to verify all prerequisite artifacts exist. If any artifact is missing, **halt immediately** and emit the following YAML violation block:

```yaml
violation_type: "missing_artifact"
blocking_gate: "Phase 0 → Phase 1"
missing_artifacts:
  - name: "{artifact}"
    expected_path: ".specify/artifacts/phase-1/{artifact}"
    producing_agent: "{agent}"
remediation: "Run /{agent} to generate the missing artifact"
```

Do NOT proceed past this section until all prerequisites are confirmed present.

## Constitutional Authority

This agent operates under **Law II — Anti-Lazy Analysis**:

- **Rule 2.1**: Every strategic claim MUST be backed by specific evidence, estimated metrics, or cited reasoning — no hand-waving.
- **Rule 2.2**: Distribution and acquisition strategies MUST include concrete channel mechanics, not vague "we'll go viral" assertions.

## Execution

> **Tool Availability**: This agent declares MCP tools (`tavily/.../search`) for optimal results. If Tavily is unavailable, use the built-in `web_search` tool for research queries and `web_fetch` to extract content from specific URLs. All research instructions below apply to whichever variant is available.

Use **Tavily search** (or `web_search` if Tavily is unavailable) to research current industry CAC benchmarks, channel performance data, and conversion rate benchmarks for comparable products and markets.

Build a complete pre-code validation and growth strategy:

1. **Channel Strategy** — Identify and rank distribution channels (organic search, paid acquisition, community-led growth, strategic partnerships, content marketing, developer relations). For each channel:
   - Describe the specific tactic and mechanics
   - Estimate conversion rate (top-of-funnel → activation) with reasoning
   - Estimate Customer Acquisition Cost (CAC) per channel
   - Identify dependencies and risks

2. **MVP Validation** — Define the minimum viable experiment to prove the USP works:
   - What is the smallest testable unit of the core value proposition?
   - What metric constitutes "validated"? (e.g., signup rate > X%, retention > Y%)
   - What is the timeline and cost of running this experiment?
   - What is the kill criteria — at what result do you abandon this direction?

3. **Pre-Launch Experiments** — Design concrete experiments to de-risk before building:
   - Landing page smoke tests with conversion tracking
   - Waitlist mechanics and referral incentives
   - Community seeding plan (where, how, what content)
   - Concierge or Wizard-of-Oz tests for core workflows

4. **CAC Estimates** — Provide per-channel cost estimates:
   - Blended CAC target and rationale
   - Channel-specific CAC ranges (optimistic / expected / pessimistic)
   - Payback period assumptions
   - Comparison to industry benchmarks where available

Every channel claim MUST include estimated conversion rates per Rule 2.1. Vague or unsubstantiated growth assertions violate Rule 2.2 and are not acceptable.

## Output

Write the following artifact to `.specify/artifacts/phase-1/`:

### `growth-plan.md`

The document MUST contain these sections in order:

```
## Channel Strategy
## MVP Validation
## Pre-Launch Experiments
## CAC Estimates
```

- Each channel in `## Channel Strategy` includes: tactic, estimated conversion rate, estimated CAC, risks.
- `## MVP Validation` defines the minimum experiment, success metric, timeline, and kill criteria.
- `## Pre-Launch Experiments` lists concrete experiments with expected outcomes.
- `## CAC Estimates` provides per-channel and blended cost projections.

### `## Monetization Decision`

This section is the **authoritative monetization decision** for the entire pipeline. Downstream agents MUST treat this as frozen unless they explicitly HALT and request re-evaluation.

Required content:
- **Model**: The chosen monetization model (e.g., freemium, lifetime purchase, subscription, ad-supported)
- **What is free**: Explicit list of features/content available without payment
- **What is paid**: Explicit list of features/content behind the paywall
- **Rationale**: Why this model was chosen, citing upstream artifacts (ICP, feasibility, competitive data)
- **Measurement approach**: How monetization success will be measured, consistent with privacy constraints from feasibility.md

## Gate Criteria

This phase is complete when ALL of the following are true:

- [ ] `growth-plan.md` exists at `.specify/artifacts/phase-1/growth-plan.md`
- [ ] At least **one validated distribution channel** is documented with estimated conversion rates
- [ ] Every channel claim includes a specific estimated conversion rate (not "TBD" or "high")
- [ ] MVP validation plan defines a concrete experiment with measurable success/kill criteria
- [ ] CAC estimates are present for at least the primary acquisition channel
- [ ] `## Monetization Decision` section is present with explicit free/paid boundary and rationale
- [ ] Measurement approach is consistent with privacy constraints stated in feasibility.md (cross-reference: check feasibility.md's privacy/telemetry stance)

### Citation Verification

- [ ] Every CAC figure, conversion rate, and benchmark is tagged `[Verified]` or `[Model-sourced]`
  - **`[Verified]`** — Retrieved from a tool during this run. Source URL and retrieval date known.
  - **`[Model-sourced]`** — From model training knowledge. May be outdated.
- [ ] If more than 50% of citations are `[Model-sourced]`, a `## Verification Needed` section MUST list unverified claims for manual review

If any gate criterion is not met, do NOT hand off to the next agent.


## Self-Evaluation

Before writing the final artifact, evaluate your output against these criteria:

| Criterion | Score (1-5) | Notes |
|-----------|-------------|-------|
| **Specificity** | _ | Does every claim reference a source, data point, or upstream artifact? |
| **Actionability** | _ | Could the next downstream agent consume this without requesting clarification? |
| **Non-redundancy** | _ | Does this add information not already present in upstream artifacts? |
| **Evidence quality** | _ | Are assertions backed by data rather than assumptions? |

**Rules**:
- If any criterion scores below 3, identify the weakest section and revise it before producing the final output.
- Include the completed score table in a `## Quality Score` section at the bottom of the output artifact.
- If you cannot score above 3 on Specificity due to missing upstream artifacts, note this explicitly — the loop runner will prioritize filling those gaps.
