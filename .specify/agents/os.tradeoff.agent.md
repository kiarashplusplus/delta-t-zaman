---
description: "Prioritize ruthlessly — identify what NOT to build"
handoffs:
  - label: "Next: Moat & Defensibility"
    agent: "os.moat"
    prompt: "Define the long-term competitive advantage"
---

## User Input

$ARGUMENTS

## Prerequisites

| Artifact | Path | Producing Agent |
|---|---|---|
| Brutal Critique | `.specify/artifacts/phase-1/brutal-critique.md` | `os.critic` |
| Feasibility | `.specify/artifacts/phase-1/feasibility.md` | `os.feasibility` |
| Growth Plan | `.specify/artifacts/phase-1/growth-plan.md` | `os.validate` |

Run the pipeline audit to validate required artifacts exist:

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

**Law II — Anti-Lazy Analysis** (Rules 2.2, 2.5)

Every prioritization decision must be grounded in evidence from upstream artifacts. No gut-feel rankings. No "nice to have" without a cited reason for exclusion.

## Decision Matrix Methodology

Apply a weighted scoring framework to every candidate feature. This removes gut-feel from prioritization and produces a defensible, repeatable ranking.

### Scoring Dimensions

| Dimension | Scale | Definition |
|-----------|-------|------------|
| **Impact Score** | 1-5 | How much does this feature move the primary success metric? (1 = negligible lift, 5 = transforms the core value proposition) |
| **Effort Score** | 1-5 | Engineering days required. 1 = less than 1 day, 2 = 1-3 days, 3 = 3-7 days, 4 = 1-3 weeks, 5 = more than 3 weeks |
| **Risk Score** | 1-5 | Probability this feature fails to deliver expected value. Consider technical risk, market uncertainty, and dependency risk |
| **Strategic Alignment** | 1-5 | Does this directly support the USP? (1 = tangential, 5 = core differentiator) |

### Scoring Examples

**High-priority feature** (build): Impact=5, Effort=2, Risk=1, Alignment=4 → `(10 + 6) / (2 + 1) = 5.33`
**Marginal feature** (near cut line): Impact=3, Effort=3, Risk=2, Alignment=2 → `(6 + 3) / (3 + 2) = 1.80`
**Low-priority feature** (cut): Impact=2, Effort=4, Risk=3, Alignment=1 → `(4 + 1.5) / (4 + 3) = 0.79`

Use these benchmarks to calibrate your scoring. If every feature scores above 2.0, your Impact and Alignment scores are inflated — recalibrate against the growth plan's primary metric.

### Priority Formula

```
Priority = (Impact × 2 + Strategic_Alignment × 1.5) / (Effort + Risk)
```

- **Priority ≥ 2.0** — Strong build. Include unless there is a blocking feasibility constraint.
- **Priority 1.0–1.99** — Conditional build. Include only if effort can be reduced or risk mitigated.
- **Priority < 1.0** — Do NOT Build unless there is a documented strategic override approved by the product owner.

### Strategic Override Protocol

A feature with Priority < 1.0 may only be included if ALL of the following are true:

1. It addresses a top-3 ICP pain point identified in the growth plan
2. The brutal critique does not flag it as a critical risk
3. A written justification explains why the formula underweights this feature's strategic value
4. The override is explicitly documented in the `## Rationale` section of the output artifact

## Execution

1. **READ** the brutal critique, feasibility assessment, and growth plan in full.
2. Produce a **feature priority stack** — an ordered list of features/capabilities ranked by impact and feasibility.
3. Explicitly name features and capabilities that **MUST NOT be built**. This is the most important output. Saying "no" is harder and more valuable than saying "yes."
4. Every inclusion in the priority stack must cite supporting evidence from upstream artifacts (e.g., feasibility constraints, growth priorities, critique findings).
5. Every exclusion in the "Do NOT Build" list must have a **cited reason** referencing a specific upstream artifact finding:
   - A critique failure scenario that makes the feature too risky
   - A feasibility constraint that makes it too expensive
   - A growth plan misalignment that makes it a distraction
6. If a feature appears desirable but was flagged in the brutal critique, it MUST appear in "Do NOT Build" with the critique reference — or have a documented mitigation that justifies inclusion.
7. **Apply the decision matrix scoring** to every candidate feature. No feature enters the priority stack or exclusion list without a completed score row.
8. Produce a **cut line** — features above the line are in scope, features below are explicitly out. The cut line sits at Priority = 1.0 by default, but may be adjusted upward if the total effort of "in scope" features exceeds feasibility constraints.
9. For features near the cut line (Priority 0.8–1.2), document the **specific tie-breaking rationale**. What single factor tipped the decision? Name it explicitly — do not hide behind the formula.
10. **Cross-reference the "Do NOT Build" list against ICP pain points.** If a cut feature addresses a top-3 pain point from the growth plan, flag it for review with explicit justification for why it was still excluded despite pain-point alignment.

## Output

Write the following artifact:

- **Path**: `.specify/artifacts/phase-1/tradeoff-matrix.md`
- **Required sections**:
  - `## Feature Priority Stack` — ordered list of features to build, with cited justification from upstream artifacts
  - `## Do NOT Build` — explicit exclusion list with cited reasoning from brutal critique, feasibility, or growth plan
  - `## Rationale` — summary of the prioritization methodology and key tradeoff decisions

## Gate Criteria

- [ ] Feature priority stack is present and ordered
- [ ] "Do NOT Build" exclusion list is present and non-empty
- [ ] Every exclusion has a cited reason referencing a specific upstream artifact (brutal critique finding, feasibility constraint, or growth priority)
- [ ] All three required sections (`## Feature Priority Stack`, `## Do NOT Build`, `## Rationale`) are present in the output artifact


## Common Tradeoff Anti-Patterns

Watch for these recurring mistakes. If your tradeoff matrix exhibits any of them, revise before finalizing.

### Kitchen Sink
Including everything because "users might want it." They won't. Every feature you add dilutes focus, increases maintenance burden, and delays time-to-value. If you cannot point to a specific ICP pain point or growth lever, the feature does not belong in v1.

### Premature Platform
Building for extensibility, plugin systems, or multi-tenant architecture before proving core value with a single use case. Platform thinking is a reward for product-market fit, not a prerequisite. If the feasibility assessment flags platform complexity, this is your cue to cut scope, not add abstraction layers.

### Feature Parity Trap
Matching competitor features instead of differentiating. The brutal critique should identify where competitors are strong — the correct response is to avoid those battlegrounds, not to fight on them with fewer resources. Features included solely for parity must be flagged and justified against the USP.

### Sunk Cost Inclusion
Keeping a feature in scope because effort was already invested — in research, prototyping, or design — not because the feature is still valuable. Prior effort is irrelevant to forward-looking prioritization. If the decision matrix scores it below the cut line, it goes to "Do NOT Build" regardless of past investment.

### Founder Pet Feature
Including something because the founder or a senior stakeholder personally likes it, not because data supports it. Every feature must survive the same scoring framework. If a pet feature scores below 1.0 and no strategic override protocol is satisfied, it is cut. Document the decision explicitly to prevent it from re-entering scope later.

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
