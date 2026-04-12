---
description: "Simulate target user interactions to stress-test UX assumptions before design"
handoffs:
  - label: "Next: UX Strategy"
    agent: "os.ux-strategy"
    prompt: "Define the UX strategy based on persona simulations"
---

## User Input

$ARGUMENTS

## Prerequisites

| # | Artifact | Path | Producing Agent |
|---|----------|------|-----------------|
| 1 | Market Map | `.specify/artifacts/phase-1/market-map.md` | os.market |
| 2 | ICP | `.specify/artifacts/phase-1/icp.md` | os.market |
| 3 | Why Now | `.specify/artifacts/phase-1/why-now.md` | os.market |
| 4 | Competitor Matrix | `.specify/artifacts/phase-1/competitor-matrix.md` | os.competitions |
| 5 | UX Teardown | `.specify/artifacts/phase-1/ux-teardown.md` | os.competition |
| 6 | Steal/Differentiate/Ignore | `.specify/artifacts/phase-1/steal-differentiate-ignore.md` | os.steal |
| 7 | USP | `.specify/artifacts/phase-1/usp.md` | os.usp |
| 8 | Feasibility | `.specify/artifacts/phase-1/feasibility.md` | os.feasibility |
| 9 | Growth Plan | `.specify/artifacts/phase-1/growth-plan.md` | os.validate |
| 10 | Brutal Critique | `.specify/artifacts/phase-1/brutal-critique.md` | os.critique |
| 11 | Tradeoff Matrix | `.specify/artifacts/phase-1/tradeoff-matrix.md` | os.tradeoffs |
| 12 | Moat | `.specify/artifacts/phase-1/moat.md` | os.moat |

**Gate Check**: Run `bash .specify/scripts/bash/pipeline-audit.sh --phase 1 --json` and verify ALL 12 artifacts are present. If any artifact is missing, emit a gate violation and halt:

```yaml
violation_type: "missing_artifact"
blocking_gate: "Phase 1 → Phase 2"
missing_artifacts:
  - name: "{artifact}"
    expected_path: "{path}"
    producing_agent: "{agent}"
remediation: "Run /{agent} to generate the missing artifact"
```

## Constitutional Authority

- **Law V** (UX as Product Strategy) — Rules 5.1, 5.2
- **Law I** (Pipeline Gating) — Rule 1.2

Phase 2 work MUST NOT begin until Phase 1 is complete. Every persona simulation MUST be grounded in validated Phase 1 research.

## Execution

1. **Read foundational artifacts**: Parse `icp.md` for target user demographics, behaviors, and constraints. Parse `ux-teardown.md` for competitive UX patterns. Parse `brutal-critique.md` for known weaknesses.

2. **Simulate 3+ distinct user personas**: For each persona, define:
   - Full name, age, occupation, location
   - Context of use (when, where, why they reach for the product)
   - Pain points (specific, cited from ICP data)
   - Tech literacy level (scale 1–5 with justification)
   - Primary goal and secondary goals

3. **Walk each persona through the core product flow**: Narrate the step-by-step experience from first discovery through core task completion. Identify:
   - **Friction points** — where the persona hesitates, gets confused, or abandons
   - **Confusion points** — where mental model clashes with product model
   - **Delight moments** — where the product exceeds expectations

4. **Simulate OFFLINE scenarios for each persona**: For every persona, describe:
   - What happens when connectivity drops mid-task
   - What data is stale and how the persona perceives it
   - What actions are blocked vs. queued
   - How the persona recovers when connectivity returns

5. **Cross-reference**: Every friction point and design assumption MUST cite specific ICP data or competitive teardown findings.

## Output

Write the following artifact to `.specify/artifacts/phase-2/`:

### `persona-sim.md`

Required sections:

- `## Personas` — Each persona with name, context, pain points, tech literacy, goals. Each persona MUST cite ICP data.
- `## Scenario Walkthroughs` — Step-by-step narration of each persona through the core flow.
- `## Friction Points` — Consolidated friction/confusion points across all personas with severity ratings.
- `## Offline Scenarios` — Per-persona offline simulation with stale data handling and recovery flows.

## Gate Criteria

- [ ] ≥3 distinct personas simulated
- [ ] Each persona includes an offline scenario
- [ ] All friction points cite ICP data as source evidence
- [ ] Persona contexts are diverse (not just variations of the same user)
- [ ] Offline scenarios address stale data perception and recovery


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
