# Iteration Scope: [FEATURE / CYCLE NAME]

**Iteration**: [Cycle number, e.g., v2, v3]  
**Date**: [DATE]  
**Source**: `iteration-brief.md` from Phase 4  
**Target**: Input for `/speckit.specify`

## Context from Previous Iteration

<!-- Summarize what was built, launched, and measured in the previous cycle. -->

- **What was shipped**: [Brief description of features/changes delivered]
- **Launch date**: [When it went live]
- **Data collection period**: [How long production data was gathered]
- **Key metrics observed**: [Top 3-5 metrics from analytics-report.md]

## Validated Assumptions

<!-- Assumptions from Phase 1 that production data confirmed. These do NOT need re-evaluation. -->

| Assumption | Evidence | Source Artifact |
|-----------|----------|-----------------|
| [e.g., "Users want offline access"] | [e.g., "73% of sessions include offline usage"] | `analytics-report.md` §Metrics |
| | | |

## Changed Assumptions

<!-- Assumptions that production data contradicted or modified. These MUST be reflected in the new spec. -->

| Original Assumption | What Changed | Evidence | Impact on Next Iteration |
|---------------------|-------------|----------|--------------------------|
| [e.g., "Users prefer search-first discovery"] | [e.g., "80% use browse, not search"] | `analytics-report.md` §Drop-off | [e.g., "Redesign home screen to prioritize browsing"] |
| | | | |

## Proposed Scope

<!-- Prioritized list of features, experiments, and simplifications for the next cycle. Each item should be concrete enough for /speckit.specify to turn into a spec. -->

### New Features

1. **[Feature Name]** — [One-sentence description]. Motivated by: [data point or learning].
2. **[Feature Name]** — [One-sentence description]. Motivated by: [data point or learning].

### Experiments to Run

1. **[Experiment]** — Hypothesis: [statement]. Measure: [metric]. Success if: [threshold].

### Simplifications (from simplification-plan.md)

1. **Remove/simplify [X]** — Justification: [usage data or maintenance cost from simplification-plan.md].

## Success Criteria Revisions

<!-- Updated success criteria for the next iteration, informed by what was learned. -->

| Criterion | Previous Target | Revised Target | Reason for Change |
|-----------|----------------|----------------|-------------------|
| [e.g., "Onboarding completion rate"] | [e.g., "80%"] | [e.g., "70% — baseline was unrealistic"] | [e.g., "Actual baseline was 55%; 70% is ambitious but achievable"] |
| | | | |

## Stale Artifacts to Refresh

<!-- Artifacts from previous phases that need updating before the next cycle begins. -->

| Artifact | Why Stale | Priority |
|----------|-----------|----------|
| [e.g., `icp.md`] | [e.g., "Production data revealed a new user segment not in original ICP"] | High |
| | | |
