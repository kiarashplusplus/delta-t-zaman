---
description: "Create launch kit with acquisition channels, content strategy, and sequencing"
handoffs:
  - label: "Next: Analytics Setup"
    agent: "os.analytics"
    prompt: "Set up behavioral analytics tracking for the launched product"
---

## User Input

$ARGUMENTS

## Prerequisites

| Artifact | Path | Producing Agent |
|---|---|---|
| Market Map | `.specify/artifacts/phase-1/market-map.md` | `os.market` |
| ICP | `.specify/artifacts/phase-1/icp.md` | `os.market` |
| Why Now | `.specify/artifacts/phase-1/why-now.md` | `os.market` |
| Competitor Matrix | `.specify/artifacts/phase-1/competitor-matrix.md` | `os.competition` |
| UX Teardown | `.specify/artifacts/phase-1/ux-teardown.md` | `os.competition` |
| Steal/Differentiate/Ignore | `.specify/artifacts/phase-1/steal-differentiate-ignore.md` | `os.competition` |
| USP | `.specify/artifacts/phase-1/usp.md` | `os.usp` |
| Feasibility | `.specify/artifacts/phase-1/feasibility.md` | `os.usp` |
| Growth Plan | `.specify/artifacts/phase-1/growth-plan.md` | `os.usp` |
| Brutal Critique | `.specify/artifacts/phase-1/brutal-critique.md` | `os.critic` |
| Tradeoff Matrix | `.specify/artifacts/phase-1/tradeoff-matrix.md` | `os.tradeoff` |
| Moat | `.specify/artifacts/phase-1/moat.md` | `os.moat` |
| Persona Simulation | `.specify/artifacts/phase-2/persona-sim.md` | Phase 2 agent |
| UX Strategy | `.specify/artifacts/phase-2/ux-strategy.md` | Phase 2 agent |
| Design Tokens | `.specify/artifacts/phase-2/design-tokens.md` | Phase 2 agent |
| Design Constraints | `.specify/artifacts/phase-2/design-constraints.md` | Phase 2 agent |
| QA Report | `.specify/artifacts/phase-3/qa-report.md` | `os.qa` |
| Trust & Safety Report | `.specify/artifacts/phase-3/trust-safety-report.md` | `os.safety` |

Run the pipeline audit to validate all Phase 1, Phase 2, and Phase 3 artifacts exist:

```bash
bash .specify/scripts/bash/pipeline-audit.sh --phase 1 --json
```

```bash
bash .specify/scripts/bash/pipeline-audit.sh --phase 2 --json
```

```bash
bash .specify/scripts/bash/pipeline-audit.sh --phase 3 --json
```

If ANY artifact is missing, HALT immediately and emit a YAML gate violation:

```yaml
violation_type: "missing_artifact"
blocking_gate: "Phase 3 → Phase 4"
missing_artifacts:
  - name: "{artifact}"
    expected_path: "{path}"
    producing_agent: "{agent}"
remediation: "Run /{agent} to generate the missing artifact"
```

Do **not** proceed past this section until all Phase 1, Phase 2, and Phase 3 artifacts are confirmed present.

## Constitutional Authority

**Law II — Anti-Lazy Analysis** (Rule 2.3)

Every channel recommendation must be grounded in data from the prerequisite artifacts. No generic "use social media" advice — every acquisition channel must cite specific CAC estimates from `growth-plan.md` and map to a specific ICP segment.

## Execution

> **Tool Availability**: This agent declares MCP tools (`tavily/.../search`) for optimal results. If Tavily is unavailable, use the built-in `web_search` tool for research queries and `web_fetch` to extract content from specific URLs. All research instructions below apply to whichever variant is available.

1. **READ** `growth-plan.md` in full for channel strategy and CAC estimates. **READ** `moat.md` for positioning and defensibility angles. **READ** `icp.md` for target audience segmentation. Use **Tavily search** (or `web_search` if Tavily is unavailable) to research current best practices for launch sequencing, content distribution strategies, and channel-specific conversion benchmarks relevant to the product category.
2. Define a **Launch Sequence** with explicit phases:
   - **Soft Launch** — limited audience, purpose: validate core assumptions, success criteria, timeline
   - **Beta** — expanded audience, purpose: stress-test at scale, feedback collection, timeline
   - **Public Launch** — full availability, purpose: acquisition at scale, timeline
   - Each phase MUST have entry criteria (what must be true to begin) and exit criteria (what must be true to proceed)
3. Map **Acquisition Channels** to ICP segments:
   - For each channel, cite the CAC estimate from `growth-plan.md`
   - Rank channels by expected ROI
   - Include both paid and organic channels
   - Every channel recommendation MUST cite `growth-plan.md` CAC estimates — no unsubstantiated claims
4. Define a **Content Strategy**:
   - Map content types to launch phases (what content for soft launch vs. beta vs. public?)
   - Tie content themes to USP messaging from `usp.md`
   - Define distribution channels for each content type
5. Create a **Pre-Launch Checklist**:
   - Technical readiness (QA passed, performance benchmarks met per `qa-report.md`)
   - Trust & safety readiness (hallucination mitigations in place per `trust-safety-report.md`)
   - Marketing assets prepared
   - Analytics instrumentation in place
   - Rollback plan defined

## Output

Create the output directory if it does not exist: `.specify/artifacts/phase-4/`

Write the following artifact:

- **Path**: `.specify/artifacts/phase-4/launch-kit.md`
- **Required sections**:
  - `## Launch Sequence` — phased rollout (soft launch → beta → public) with entry/exit criteria, timelines, and success metrics for each phase
  - `## Acquisition Channels` — channels mapped to ICP segments, each citing CAC estimates from `growth-plan.md`, ranked by expected ROI
  - `## Content Strategy` — content types mapped to launch phases, tied to USP messaging, with distribution channels
  - `## Pre-Launch Checklist` — actionable checklist covering technical, trust/safety, marketing, analytics, and rollback readiness

## Gate Criteria

- [ ] Every acquisition channel cites a specific CAC estimate from `growth-plan.md`
- [ ] Launch sequence has at least three phases (soft launch, beta, public) with explicit dates or milestone triggers
- [ ] Each launch phase has entry criteria and exit criteria
- [ ] Content strategy maps content types to specific launch phases
- [ ] Pre-launch checklist covers technical readiness, trust/safety readiness, and rollback plan
- [ ] All four required sections are present in the output artifact

### Citation Verification

- [ ] Every channel recommendation and benchmark is tagged `[Verified]` or `[Model-sourced]`
  - **`[Verified]`** — Retrieved from a tool during this run. Source URL and retrieval date known.
  - **`[Model-sourced]`** — From model training knowledge. May be outdated.
- [ ] If more than 50% of citations are `[Model-sourced]`, a `## Verification Needed` section MUST list unverified claims for manual review


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
