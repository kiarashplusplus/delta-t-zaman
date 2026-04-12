---
description: "Define the long-term competitive advantage that compounds over time"
---

## User Input

$ARGUMENTS

## Prerequisites

| Artifact | Path | Producing Agent |
|---|---|---|
| Competitor Matrix | `.specify/artifacts/phase-1/competitor-matrix.md` | `os.competition` |
| USP | `.specify/artifacts/phase-1/usp.md` | `os.usp` |
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

**Law III — Brutal Critic** (Rule 3.4)

A product without a moat is a feature waiting to be cloned. Defensibility is not optional — it is existential.

## Moat Taxonomy

Classify every identified moat under one or more of the following archetypes:

| # | Archetype | Definition |
|---|-----------|------------|
| 1 | **Data Network Effects** | Usage generates proprietary data that improves the product, creating a flywheel competitors cannot bootstrap. |
| 2 | **Community / Marketplace Network Effects** | Each new user adds value for every other user, making the product exponentially harder to displace. |
| 3 | **Switching Costs** | Users accumulate data, habits, integrations, or workflows that make leaving painful and expensive. |
| 4 | **Proprietary Technology** | Patents, trade secrets, or unique algorithms that provide a capability competitors cannot legally or practically replicate. |
| 5 | **Brand & Trust** | Reputation compounds over time through consistent delivery; hard to replicate because trust is earned, not bought. |
| 6 | **Regulatory / Compliance Moats** | Certifications, licenses, or regulatory approvals that act as structural barriers to entry for new competitors. |
| 7 | **Scale Economies** | Unit economics improve with volume — the bigger you get, the cheaper each marginal unit becomes relative to competitors. |

When analyzing moat mechanisms in the Execution phase, tag each mechanism with its archetype number(s). A single mechanism may map to multiple archetypes.

## Moat Scoring Framework

For **each** moat mechanism identified during execution, produce a scored assessment using the following dimensions:

- **Strength (1–5)**: How powerful is this moat today? (1 = trivial, 5 = nearly insurmountable)
- **Durability (1–5)**: Will this moat strengthen or weaken over the next 3 years? (1 = eroding fast, 5 = compounding aggressively)
- **Time-to-replicate**: Estimated months a well-funded competitor would need to replicate this moat from scratch.
- **Compounding**: Does this moat get stronger with usage or time? (yes/no + one-sentence explanation)

Include the following table template in the output artifact under `## Moat Mechanisms`:

```markdown
| Mechanism | Archetype(s) | Strength (1–5) | Durability (1–5) | Time-to-Replicate (months) | Compounding |
|-----------|-------------|-----------------|-------------------|----------------------------|-------------|
| {name}    | {#}         | {score}         | {score}           | {months}                   | {yes/no}: {explanation} |
```

**Scoring rules**:
- A moat with Strength < 3 AND Durability < 3 is **not credible** — do not count it toward gate criteria.
- A moat with Compounding = "yes" and Durability ≥ 4 qualifies as a **compounding moat** — flag it explicitly in the Defensibility Thesis.
- If no mechanism scores Strength ≥ 4, include a warning that the overall moat position is weak.

## Execution

1. **READ** the competitor matrix, USP, and growth plan in full.
2. Define **defensibility mechanisms** across the following categories:
   - **Data flywheel** — does usage generate data that makes the product better, creating a compounding advantage?
   - **Network effects** — does each new user make the product more valuable for existing users?
   - **Switching costs** — what does a user lose by leaving? Is it painful enough to prevent churn?
   - **Proprietary knowledge** — does the team possess unique domain expertise, datasets, or algorithms that cannot be easily replicated?
3. For each mechanism identified, assess **replicability**:
   - How long would it take a well-funded competitor to replicate this?
   - What resources would they need?
   - Is there a structural reason it cannot be replicated (first-mover data, regulatory capture, network density)?
4. **Tag each mechanism** with its archetype(s) from the Moat Taxonomy and **score it** using the Moat Scoring Framework.
5. At least **one moat MUST be non-replicable** — a structural advantage that compounds over time and cannot be bought or copied.
6. If **no credible moat exists**, HALT execution and recommend a strategic review. Output a clear warning: a product without a moat is a feature waiting to be cloned. Do not proceed with false confidence.

## Output

Write the following artifact:

- **Path**: `.specify/artifacts/phase-1/moat.md`
- **Required sections**:
  - `## Defensibility Thesis` — the core argument for why this product can sustain a long-term competitive advantage
  - `## Moat Mechanisms` — detailed analysis of each defensibility mechanism (data flywheel, network effects, switching costs, proprietary knowledge), including the scoring table from the Moat Scoring Framework
  - `## Replicability Assessment` — for each mechanism, how easily a competitor could replicate it and the structural barriers preventing replication

## Gate Criteria

- [ ] ≥1 moat mechanism is identified as non-replicable with structural reasoning
- [ ] All three required sections (`## Defensibility Thesis`, `## Moat Mechanisms`, `## Replicability Assessment`) are present in the output artifact
- [ ] Each moat mechanism has a replicability assessment with timeline and resource estimates
- [ ] Every moat mechanism is tagged with an archetype and scored using the Moat Scoring Framework
- [ ] At least one mechanism qualifies as a compounding moat (Compounding = yes, Durability ≥ 4)
- [ ] If no credible moat is found, execution is halted with a strategic review recommendation instead of producing a false-confidence artifact


## Moat Validation Tests

Before finalizing the artifact, stress-test every claimed moat against these four litmus tests. Document the answers in a `## Validation` section of the output artifact.

### The Clone Test

> If a YC-funded startup copied your product tomorrow, what would they **NOT** be able to replicate?

Any moat that fails this test (i.e., the clone could fully replicate it within 6 months) is not a real moat. Downgrade its Strength score to ≤ 2.

### The Substitution Test

> Can the user achieve the same outcome with a spreadsheet + manual effort?

If yes, the product solves a convenience problem, not a defensibility problem. This does not disqualify the moat, but it must be acknowledged — and the moat must rely on something beyond the core workflow (data, network, integrations).

### The Indifference Test

> If you shut down for 30 days, would users wait or switch?

If users would switch without hesitation, switching costs are near zero. Any claimed switching-cost moat must be re-evaluated or removed.

### The Funding Test

> Could $10M in competitor funding neutralize your advantage?

If a single seed round could eliminate the moat, it is a **speed advantage**, not a structural moat. Flag it as time-limited and ensure at least one other mechanism provides durable defensibility.

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
