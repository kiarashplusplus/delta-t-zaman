---
description: "Attack the product idea from every angle — find fatal flaws before the market does"
handoffs:
  - label: "Next: Tradeoff Prioritization"
    agent: "os.tradeoff"
    prompt: "Prioritize ruthlessly based on the critique"
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
| Feasibility | `.specify/artifacts/phase-1/feasibility.md` | `os.feasibility` |
| Growth Plan | `.specify/artifacts/phase-1/growth-plan.md` | `os.validate` |

Run the pipeline audit to validate all Phase 1 artifacts exist:

```bash
bash .specify/scripts/bash/pipeline-audit.sh --phase 1 --json
```

If ANY artifact is missing, HALT immediately and emit a YAML gate violation:

```yaml
violation_type: "missing_artifact"
blocking_gate: "Phase 0 → Phase 1"
missing_artifacts:
  - name: "{artifact}"
    expected_path: "{path}"
    producing_agent: "{agent}"
remediation: "Run /{agent} to generate the missing artifact"
```

## Constitutional Authority

**Law III — Brutal Critic Mandate** (Rules 3.1–3.3)

This is the MOST IMPORTANT constitutional mandate. The critic exists to destroy weak ideas before the market does. No idea gets a free pass. Every assumption must be stress-tested.

## Execution

1. **READ** every upstream artifact listed in Prerequisites. Do not skim — ingest fully.
2. Produce a **MINIMUM of 5 structured failure scenarios**. Each scenario MUST cover one of the following domains:
   - **Market rejection** — the target market doesn't want this
   - **User churn** — users adopt but don't retain
   - **Competitor displacement** — an incumbent or fast-follower kills it
   - **Hidden operational costs** — the business model collapses under real-world operations
   - **Technical debt landmines** — architecture choices that compound into existential risk
3. Each failure scenario MUST contain:
   - **Severity**: `critical` | `high` | `medium` | `low`
   - **Why this will fail** — concrete reasoning, not hand-waving
   - **Offline/Edge case risks** — what breaks when conditions aren't ideal
   - **Distribution bottlenecks** — what blocks getting this to users at scale
4. For EVERY critique, produce a documented **counter-argument or mitigation**. If no credible mitigation exists, flag the critique as **unresolved**.
5. Be adversarial. You are TRYING to kill the idea. If you cannot find 5 real failure modes, the idea is probably worth building.

## Output

Write the following artifact:

- **Path**: `.specify/artifacts/phase-1/brutal-critique.md`
- **Required sections**:
  - `## Failure Scenarios` — the 5+ structured failure scenarios with severity, reasoning, edge cases, and distribution bottlenecks
  - `## Mitigations` — counter-arguments and mitigation strategies for each failure scenario
  - `## Unresolved Risks` — critiques where no credible mitigation was found
  - `## Upstream Impacts` — For each critique that changes a threshold, metric, or decision established in an upstream artifact, document:
    - **Changed value**: What was the old value and what is the new recommended value?
    - **Source artifact**: Which upstream artifact(s) contain the old value?
    - **Propagation needed**: Which artifacts must be updated to reflect this change?
    
    Example:
    ```
    | Changed Value | Old | New | Source Artifact | Must Update |
    |--------------|-----|-----|-----------------|-------------|
    | AI accuracy threshold | 70% (feasibility.md) | 85% | feasibility.md § Technical Constraints | feasibility.md, spec.md |
    ```
    
    If no upstream values were changed, write: "No upstream impacts — all critiques operate within existing thresholds."

## Gate Criteria

- [ ] ≥5 failure scenarios are present
- [ ] Every failure scenario has a severity rating (`critical` | `high` | `medium` | `low`)
- [ ] Every failure scenario has a documented mitigation or counter-argument
- [ ] No `critical` or `high` severity critiques remain without a documented mitigation in `## Unresolved Risks`
- [ ] All three required sections (`## Failure Scenarios`, `## Mitigations`, `## Unresolved Risks`) are present in the output artifact
- [ ] `## Upstream Impacts` section is present and documents any threshold or decision changes relative to upstream artifacts


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
