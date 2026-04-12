---
description: "Full-corpus semantic cross-examination — reads all pipeline artifacts and detects broken decision chains, contradictions, gaps, and scope mismatches"
handoffs:
  - label: "Apply fixes"
    agent: "os.revise"
    prompt: "Apply the findings from the cross-examination report"
    send: true
---

## User Input

```text
$ARGUMENTS
```

You **MUST** consider the user input before proceeding (if not empty).

Arguments accept:
- A goal string (e.g., "full audit", "check monetization only", "verify revision")
- Optional `--quick` flag for spot-checking specific decision chains only

## Prerequisites

All artifacts below are **optional** — the agent works with whatever exists. Load every artifact that is present; skip those that are missing without error.

### Phase 1 Artifacts (`.specify/artifacts/phase-1/`)

| Artifact | Path | Producing Agent |
|----------|------|-----------------|
| Market Map | `.specify/artifacts/phase-1/market-map.md` | `os.market` |
| ICP | `.specify/artifacts/phase-1/icp.md` | `os.market` |
| Why Now | `.specify/artifacts/phase-1/why-now.md` | `os.market` |
| Competitor Matrix | `.specify/artifacts/phase-1/competitor-matrix.md` | `os.competition` |
| UX Teardown | `.specify/artifacts/phase-1/ux-teardown.md` | `os.competition` |
| Steal/Differentiate/Ignore | `.specify/artifacts/phase-1/steal-differentiate-ignore.md` | `os.competition` |
| USP | `.specify/artifacts/phase-1/usp.md` | `os.usp` |
| Feasibility | `.specify/artifacts/phase-1/feasibility.md` | `os.feasibility` |
| Growth Plan | `.specify/artifacts/phase-1/growth-plan.md` | `os.validate` |
| Brutal Critique | `.specify/artifacts/phase-1/brutal-critique.md` | `os.critic` |
| Tradeoff Matrix | `.specify/artifacts/phase-1/tradeoff-matrix.md` | `os.tradeoff` |
| Moat | `.specify/artifacts/phase-1/moat.md` | `os.moat` |

### Phase 2 Artifacts (`.specify/artifacts/phase-2/`)

| Artifact | Path | Producing Agent |
|----------|------|-----------------|
| Persona Simulation | `.specify/artifacts/phase-2/persona-sim.md` | `os.persona` |
| UX Strategy | `.specify/artifacts/phase-2/ux-strategy.md` | `os.ux-strategy` |
| Design Tokens | `.specify/artifacts/phase-2/design-tokens.md` | `os.design` |
| Design Constraints | `.specify/artifacts/phase-2/design-constraints.md` | `os.design` |

### Phase 3 Spec Artifacts (if feature branch exists)

| Artifact | Path | Producing Agent |
|----------|------|-----------------|
| Spec | `{FEATURE_DIR}/spec.md` | `speckit.specify` |
| Plan | `{FEATURE_DIR}/plan.md` | `speckit.plan` |
| Tasks | `{FEATURE_DIR}/tasks.md` | `speckit.tasks` |

### Constitution

| Artifact | Path |
|----------|------|
| Constitution | `.specify/memory/constitution.md` |

## Constitutional Authority

This agent operates under the my-kit Constitution v1.1.0. See `.specify/memory/constitution.md`.

**Applicable laws**:
- **Law II — Anti-Lazy Analysis** (Rules 2.1, 2.2): Every finding MUST be backed by specific evidence — quoted text, line references, or concrete value comparisons. No hand-waving "this seems inconsistent."
- **Law III — Brutal Critic Mandate** (Rules 3.1–3.3): The cross-examiner is adversarial — it is trying to find problems. Err on the side of over-reporting. A clean bill of health must be earned.

## Execution

### 1. Corpus Loading

Scan `.specify/artifacts/` and any active feature directory for all artifacts listed in Prerequisites. For each artifact found:
- Record: filename, path, last modified timestamp, line count
- Note: artifacts that are missing (these are gaps, not errors)

If `--quick` flag is present, skip to Step 3 (pattern checking only) using the Decision Inventory from a previous report if available.

### 2. Progressive Reading Strategy

For corpora exceeding 100K tokens, use a three-pass strategy:

**First pass — Decision extraction**: Read all artifact headings and decision sections. Prioritize:
- `§ Monetization Decision` in growth-plan.md
- `§ Blockers` in feasibility.md
- `§ Upstream Impacts` in brutal-critique.md
- `§ Content Scope Constraints` in ux-strategy.md
- `§ Unresolved Risks` in brutal-critique.md
- Any section containing thresholds, constraints, or explicit decisions

**Second pass — Build Decision Inventory**: Construct a table of every decision made across all artifacts:

| Decision | Value | Authoritative Source | Downstream References | Consistent? |
|----------|-------|---------------------|----------------------|-------------|

For each decision, trace forward through downstream artifacts to verify the value is consistent.

**Third pass — Deep drill**: For any decision marked inconsistent or any section flagged during passes 1-2, read the full surrounding context and produce a structured finding.

### 3. Systemic Pattern Checks

Explicitly check for the 5 systemic patterns that caused contradictions in the Bayan corpus:

#### Pattern 1 — Decision Ownership
- Is every major decision (monetization, accuracy thresholds, content scope, privacy stance) frozen by exactly one authoritative artifact?
- Are downstream agents respecting the freeze? (Check for conflicting values in downstream artifacts)
- Specifically verify: growth-plan.md `§ Monetization Decision` is not contradicted by ux-strategy.md or spec.md

#### Pattern 2 — Reading Scope
- Are there decisions in feasibility.md, brutal-critique.md, or growth-plan.md that should have been read by downstream agents but weren't?
- Check for "orphaned decisions" — values established upstream with no downstream reference
- Verify speckit.tasks read the upstream risk artifacts (check for T000-series tasks if critical blockers exist)

#### Pattern 3 — Backward Propagation
- Does any downstream artifact contain `## Upstream Impacts` or `[UPSTREAM IMPACT]` markers?
- If so, have the referenced upstream artifacts been updated to reflect the new values?
- Check brutal-critique.md `§ Upstream Impacts` table — are all listed artifacts updated?

#### Pattern 4 — Conditional→Task Conversion
- Are there items in feasibility.md `§ Blockers` with severity `critical` that are NOT in tasks.md as T000-series blockers?
- Are there items in brutal-critique.md `§ Unresolved Risks` with severity `critical` or `high` requiring pre-build validation that are NOT taskified?
- Are there `growth-plan.md § Pre-Launch Experiments` marked "mandatory before build" that are NOT in tasks.md?

#### Pattern 5 — Content-Scope Propagation
- Do features that surface content respect the content boundaries stated in upstream artifacts?
- Check ux-strategy.md for `§ Content Scope Constraints`
- Verify spec.md features referencing content have `[CONTENT SCOPE]` annotations where needed

### 4. General Cross-Examination

Beyond the 5 patterns, check:

- **Terminology consistency**: Same concept must use the same term across all artifacts. Flag drift (e.g., "freemium" in one doc, "free tier" in another)
- **Numeric value consistency**: Same metric must have the same target everywhere (e.g., accuracy threshold, performance targets, content counts)
- **Scope creep detection**: Features in tasks.md that are not present in spec.md
- **Staleness detection**: Artifacts whose input artifacts have been updated more recently than the artifact itself
- **Constitution compliance**: Check all MUST rules in constitution.md against the corpus

### 5. Findings Assembly

For each issue found, produce a structured finding using the universal findings schema:

```yaml
- id: F-001
  severity: critical | high | medium | low
  category: contradiction | gap | scope-mismatch | staleness | underspecification | pattern-1 | pattern-2 | pattern-3 | pattern-4 | pattern-5
  affected_artifacts:
    - path: "<artifact path>"
      section: "§ <section name>"
  description: "<clear description of the issue>"
  evidence: "<specific text, values, or line references from the artifacts>"
  recommended_action: "<what should be changed to fix this>"
```

**Severity assignment**:
- **critical**: Contradictory decisions that would produce broken output (e.g., monetization model conflicts, missing blockers in tasks)
- **high**: Missing decision chains, untaskified critical blockers, stale upstream impacts
- **medium**: Terminology drift, minor numeric inconsistencies, orphaned decisions
- **low**: Stylistic inconsistencies, redundant content, minor staleness

## Output

Write the following artifact:

- **Path**: `.specify/artifacts/phase-3/cross-examination-report.md` (or `{FEATURE_DIR}/cross-examination-report.md` if a feature directory is active)

### Required Sections

```markdown
# Cross-Examination Report

## Decision Inventory
| Decision | Value | Authoritative Source | Downstream References | Consistent? |

## Findings
(Structured findings in the universal schema)

## Corpus Health Score
(Overall assessment: decision-coherent / mostly-coherent / has-broken-chains)

## Recommended Actions
(Prioritized list of fixes, structured as os.revise input)
```

**Corpus Health Score criteria**:
- **decision-coherent**: Zero critical findings, ≤2 high findings, all 5 patterns clean
- **mostly-coherent**: Zero critical findings, >2 high findings or 1-2 pattern violations
- **has-broken-chains**: Any critical findings, or ≥3 pattern violations

## Gate Criteria

- [ ] All registered artifacts have been read (or noted as missing)
- [ ] Decision Inventory is complete (every decision traced to source)
- [ ] All 5 systemic patterns explicitly checked with findings or clean status
- [ ] Every finding has severity, category, evidence, and recommended action
- [ ] Constitution compliance is checked
- [ ] Corpus Health Score is assigned with justification

## Self-Evaluation

Before writing the final artifact, evaluate your output against these criteria:

| Criterion | Score (1-5) | Notes |
|-----------|-------------|-------|
| **Specificity** | _ | Does every finding reference specific text, lines, or values? |
| **Actionability** | _ | Could os.revise consume each finding without requesting clarification? |
| **Non-redundancy** | _ | Are findings deduplicated? No two findings about the same root cause? |
| **Evidence quality** | _ | Are findings backed by quoted evidence rather than assumptions? |

**Rules**:
- If any criterion scores below 3, identify the weakest section and revise it before producing the final output.
- Include the completed score table in a `## Quality Score` section at the bottom of the output artifact.
- If you cannot score above 3 on Specificity due to missing upstream artifacts, note this explicitly — the loop runner will prioritize filling those gaps.
