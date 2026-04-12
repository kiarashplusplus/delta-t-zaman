---
description: "Synthesize learnings into an iteration brief that feeds back to speckit.specify"
---

## User Input

$ARGUMENTS

## Prerequisites

| Artifact | Path | Producing Agent |
|---|---|---|
| Analytics Report | `.specify/artifacts/phase-4/analytics-report.md` | `os.analytics` |
| Simplification Plan | `.specify/artifacts/phase-4/simplification-plan.md` | `os.simplify` |

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

Verify that `analytics-report.md` and `simplification-plan.md` exist in `.specify/artifacts/phase-4/`. If ANY prerequisite artifact is missing, HALT immediately and emit a YAML gate violation:

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

**Law I — Pipeline Gating** (Rule 1.6 — Loop-Back)

The pipeline is a cycle, not a line. After iteration, learnings feed back to Phase 1 for re-evaluation. Stale assumptions must be flagged and refreshed. The product evolves through disciplined loops, not linear marches.

**Law II — Anti-Lazy Analysis** (Rules 2.1–2.5)

Every learning must be actionable. Every assumption re-evaluation must cite evidence. The iteration brief is not a summary — it is a directive for the next cycle.

## Execution

1. **READ** `analytics-report.md` and `simplification-plan.md` in full.
2. **SYNTHESIZE key learnings**:
   - What did the data confirm about original assumptions?
   - What did the data contradict?
   - What surprised us (unexpected user behaviors, metrics, or outcomes)?
   - What did the simplification process reveal about product complexity?
3. **EVALUATE Phase 1 assumptions** for staleness:
   - Re-read each Phase 1 artifact header (market-map, icp, why-now, competitor-matrix, usp, feasibility, growth-plan, moat)
   - For each artifact, determine if the core assumptions still hold given post-launch data
   - Flag artifacts where assumptions need re-evaluation with specific reasons
4. **DEFINE next iteration scope**:
   - What features or improvements should the next cycle prioritize?
   - What experiments should be run to validate remaining hypotheses from `analytics-report.md`?
   - What simplifications from `simplification-plan.md` should be executed first?
   - Frame the scope as input to `/speckit.specify` — it must be structured enough to feed directly into the specification agent
   - Use the **iteration scope template** at `.specify/templates/iteration-scope-template.md` to structure the `## Next Iteration Scope` section. The template defines required fields: Context from Previous Iteration, Validated Assumptions, Changed Assumptions, Proposed Scope (features, experiments, simplifications), Success Criteria Revisions, and Stale Artifacts to Refresh.
5. **FLAG stale artifacts**:
   - For each artifact that may be stale, state what has changed and why a refresh is needed
   - Prioritize staleness by impact (which stale artifact, if left unrefreshed, poses the greatest risk?)

## Output

Write the following artifact to `.specify/artifacts/phase-4/`:

- **Path**: `.specify/artifacts/phase-4/iteration-brief.md`
- **Required sections**:
  - `## Key Learnings` — confirmed assumptions, contradicted assumptions, and surprises from post-launch data
  - `## Assumption Re-evaluation` — per-artifact assessment of whether Phase 1 assumptions still hold, with evidence from analytics and simplification data
  - `## Next Iteration Scope` — prioritized list of features, experiments, and simplifications for the next development cycle, structured as input for `/speckit.specify`
  - `## Stale Artifact Flags` — artifacts that need refresh, what has changed, why a refresh is needed, and impact-based priority ranking

## Gate Criteria

- [ ] Next iteration scope is defined with prioritized items
- [ ] Stale artifacts are explicitly flagged with reasons and impact priority
- [ ] Every key learning cites specific data from `analytics-report.md` or `simplification-plan.md`
- [ ] Assumption re-evaluation covers all Phase 1 artifacts (market-map, icp, why-now, competitor-matrix, usp, feasibility, growth-plan, moat)
- [ ] Next iteration scope is structured as actionable input for `/speckit.specify`
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
