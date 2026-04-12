---
description: "Force complexity reduction — cut features justified only by data"
handoffs:
  - label: "Next: Iterate"
    agent: "os.iterate"
    prompt: "Plan the next iteration based on simplification decisions"
---

## User Input

$ARGUMENTS

## Prerequisites

| Artifact | Path | Producing Agent |
|---|---|---|
| Analytics Report | `.specify/artifacts/phase-4/analytics-report.md` | `os.analytics` |

Run the pipeline audit to validate all prerequisite artifacts exist:

```bash
bash .specify/scripts/bash/pipeline-audit.sh --phase 1 --json
```

```bash
bash .specify/scripts/bash/pipeline-audit.sh --phase 2 --json
```

```bash
bash .specify/scripts/bash/pipeline-audit.sh --phase 3 --json
```

Verify that `analytics-report.md` exists in `.specify/artifacts/phase-4/`. If ANY prerequisite artifact is missing, HALT immediately and emit a YAML gate violation:

```yaml
violation_type: "missing_artifact"
blocking_gate: "Phase 3 → Phase 4"
missing_artifacts:
  - name: "{artifact}"
    expected_path: "{path}"
    producing_agent: "{agent}"
remediation: "Run /{agent} to generate the missing artifact"
```

Do **not** proceed past this section until all prerequisites are confirmed present.

## Constitutional Authority

**Law II — Anti-Lazy Analysis** (Rules 2.2, 2.5)

- **Rule 2.2**: Complexity is the enemy. Every feature must justify its existence with data. If it cannot, it is a candidate for removal.
- **Rule 2.5**: Simplification is not optional — it is a recurring obligation. The product must get simpler over time, not more complex.

## Execution

1. **READ** `analytics-report.md` in full. Extract all KPIs, drop-off data, and behavioral hypotheses.
2. **IDENTIFY underperforming features**:
   - Features with low adoption (used by < X% of active users — define the threshold based on the product's scale)
   - Features with high drop-off (users start but don't complete)
   - Features with disproportionate error rates
   - Features that add complexity without measurable value
3. **PROPOSE cuts** with mandatory data justification:
   - Every proposed cut MUST cite a specific metric or user behavior from `analytics-report.md`
   - No "gut feel" cuts allowed — if a cut cannot be justified by data, it is not proposed
   - For each cut, state what metric will improve and by how much (estimated)
4. **QUANTIFY engineering time recovered** by each cut:
   - Estimate maintenance burden removed (hours/sprint or equivalent)
   - Estimate cognitive complexity reduction for the team
   - Estimate reduction in test surface area
5. **RISK ASSESSMENT** for each proposed cut:
   - What is the worst case if this feature is removed?
   - Is the cut reversible? What would re-implementation cost?
   - Are there users who depend critically on this feature?

## Output

Write the following artifact to `.specify/artifacts/phase-4/`:

- **Path**: `.specify/artifacts/phase-4/simplification-plan.md`
- **Required sections**:
  - `## Underperforming Features` — list of features identified as underperforming, each with the specific metric or behavior that flagged them
  - `## Proposed Cuts` — features recommended for removal, each with a clear data-backed rationale
  - `## Data Justification` — for each proposed cut, the specific metric, user behavior, or analytics data point that justifies it
  - `## Engineering Time Recovered` — estimated time savings per cut, including maintenance burden, complexity reduction, and test surface reduction

## Gate Criteria

- [ ] Every proposed cut cites a specific metric or user behavior from `analytics-report.md`
- [ ] No unjustified removals — every cut has a data-backed rationale in the `## Data Justification` section
- [ ] Engineering time recovered is quantified for each proposed cut
- [ ] Risk assessment is present for each proposed cut (worst case, reversibility, dependent users)
- [ ] All four required sections are present in the output artifact


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
