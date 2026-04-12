---
description: "Post-implementation QA focusing on offline failures, AI hallucinations, and edge cases"
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

Implementation must have been run — verify that `specs/*/tasks.md` files exist with completed tasks.

Run the pipeline audit to validate all Phase 1 and Phase 2 artifacts exist:

```bash
bash .specify/scripts/bash/pipeline-audit.sh --phase 1 --json
```

```bash
bash .specify/scripts/bash/pipeline-audit.sh --phase 2 --json
```

If ANY Phase 1 or Phase 2 artifact is missing, HALT immediately and emit a YAML gate violation:

```yaml
violation_type: "missing_artifact"
blocking_gate: "Phase 2 → Phase 3"
missing_artifacts:
  - name: "{artifact}"
    expected_path: "{path}"
    producing_agent: "{agent}"
remediation: "Run /{agent} to generate the missing artifact"
```

Do **not** proceed past this section until all Phase 1 and Phase 2 artifacts are confirmed present and implementation tasks exist.

## Constitutional Authority

**Law IV — Hardened Engineering** (Rules 4.1–4.5)

Every feature must survive hostile conditions. Offline-first is not optional — it is the default. AI outputs must be verifiable, attributed, and scored for confidence. Edge cases are not afterthoughts; they are first-class test scenarios derived from the brutal critique.

## Execution

1. **READ** all Phase 1 and Phase 2 artifacts in full. Pay special attention to `brutal-critique.md` failure scenarios, `design-constraints.md` for technical boundaries, and `feasibility.md` for known limitations.
2. **CHECK** that implementation has been run by verifying `specs/*/tasks.md` files contain completed tasks. If no implementation evidence exists, HALT and report.
3. Generate an **Offline Test Matrix** covering:
   - Connectivity loss mid-operation (what happens to in-flight requests?)
   - Stale cache behavior (how old can cached data be before it's dangerous?)
   - Sync conflict resolution (what happens when offline edits conflict with server state?)
   - Graceful degradation paths (what features are available with zero connectivity?)
4. Generate **AI Hallucination Tests** covering:
   - Source attribution verification (every AI output must trace to a cited source)
   - Confidence scoring validation (outputs below threshold must be flagged)
   - Factual verification checks (cross-reference AI claims against known data)
   - Hallucination detection heuristics (patterns that indicate fabricated content)
5. Generate **Edge Case Tests** derived from `brutal-critique.md` failure scenarios:
   - For each failure scenario with severity `critical` or `high`, create at least one test case
   - Include boundary conditions from `design-constraints.md`
   - Cover error recovery paths

   **Failure-Scenario-to-Test-Case Translation Examples**:

   | Failure Scenario (from brutal-critique.md) | Derived Test Case |
   |---------------------------------------------|-------------------|
   | "User loses connectivity mid-sync" (critical) | **Test**: Simulate network drop during data upload. **Steps**: 1) Start a sync operation, 2) Kill network after 50% transfer, 3) Restore network. **Expected**: Local queue persists, sync resumes automatically, no data loss or duplication. |
   | "Competitor launches free tier undercutting pricing" (high) | **Test**: Validate core value proposition works without premium features. **Steps**: 1) Disable all premium features, 2) Complete the primary user journey. **Expected**: Core experience remains compelling; free-tier users can complete key tasks. |
   | "User enters 10,000-character input in search field" (high) | **Test**: Boundary input test. **Steps**: 1) Submit max-length input to search, 2) Submit input with special characters (Unicode, RTL, emoji). **Expected**: Input is truncated or rejected gracefully with user-facing message; no crash, no SQL injection, no infinite spinner. |
6. Define **Performance Benchmarks**:
   - First paint MUST be < 2 seconds
   - Offline response MUST be < 100ms
   - Sync resolution MUST complete within 5 seconds of connectivity restoration
   - Include measurement methodology for each benchmark

## Output

Create the output directory if it does not exist: `.specify/artifacts/phase-3/`

Write the following artifact:

- **Path**: `.specify/artifacts/phase-3/qa-report.md`
- **Required sections**:
  - `## Offline Test Matrix` — structured table of offline scenarios with expected behavior, test steps, and pass/fail criteria
  - `## AI Hallucination Tests` — test cases for source attribution, confidence scoring, and factual verification
  - `## Edge Case Tests` — test cases derived from brutal-critique.md failure scenarios, each linked to its source scenario
  - `## Performance Benchmarks` — measurable targets (first-paint <2s, offline response <100ms) with measurement methodology
  - `## Test Results` — placeholder template for recording actual test outcomes with pass/fail status

## Gate Criteria

- [ ] Offline test scenarios are present covering: connectivity loss, stale cache, sync conflicts, and graceful degradation
- [ ] AI hallucination mitigation tests are present covering: source attribution, confidence scoring, and factual verification
- [ ] Every `critical` or `high` severity failure scenario from `brutal-critique.md` has at least one corresponding edge case test
- [ ] All tests have explicit pass/fail criteria
- [ ] Performance benchmarks include first-paint <2s and offline response <100ms with measurement methodology
- [ ] All five required sections are present in the output artifact


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
