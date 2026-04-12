---
description: "Distill the product's unique value into one falsifiable sentence"
handoffs:
  - label: "Next: Feasibility Check"
    agent: "os.feasibility"
    prompt: "Validate technical and financial constraints for the USP"
---

## User Input

$ARGUMENTS

## Prerequisites

| Artifact | Path | Producing Agent |
|---|---|---|
| `competitor-matrix.md` | `.specify/artifacts/phase-1/competitor-matrix.md` | `os.competition` |
| `steal-differentiate-ignore.md` | `.specify/artifacts/phase-1/steal-differentiate-ignore.md` | `os.competition` |
| `icp.md` | `.specify/artifacts/phase-1/icp.md` | `os.market` |

Run the pipeline audit to validate all prerequisite artifacts exist:

```bash
bash .specify/scripts/bash/pipeline-audit.sh --phase 1 --json
```

Verify that `competitor-matrix.md`, `steal-differentiate-ignore.md`, and `icp.md` are all present in the audit output. If **any** artifact is missing, **halt immediately** and emit the following YAML violation block (substituting the actual missing artifact names):

```yaml
violation_type: "missing_artifact"
blocking_gate: "Phase 0 → Phase 1"
missing_artifacts:
  - name: "competitor-matrix.md"
    expected_path: ".specify/artifacts/phase-1/competitor-matrix.md"
    producing_agent: "os.competition"
  - name: "steal-differentiate-ignore.md"
    expected_path: ".specify/artifacts/phase-1/steal-differentiate-ignore.md"
    producing_agent: "os.competition"
  - name: "icp.md"
    expected_path: ".specify/artifacts/phase-1/icp.md"
    producing_agent: "os.market"
remediation: "Run /os.competition to generate the missing artifact, or /os.market for icp.md"
```

Do **not** proceed past this section until all three artifacts are confirmed present.

## Constitutional Authority

- **Law II (Anti-Lazy Analysis) Rules 2.1–2.3** — The USP must be derived from rigorous analysis, not aspirational marketing copy. Every claim must be traceable to evidence in the competitive analysis. Vague differentiators like "better UX" or "more innovative" are rejected.

## Positioning Matrix

Before writing the USP, construct a 2×2 positioning matrix to identify defensible whitespace.

**Step 1 — Choose axes.** Pick the two dimensions that matter most to the ICP. Derive these directly from the pain points and desires documented in `icp.md`. Good axes are measurable spectrums (e.g., "Self-serve ↔ Sales-assisted", "Single-player ↔ Collaborative").

**Step 2 — Plot players.** Place the product and every competitor from `competitor-matrix.md` on the matrix using High/Low scores for each axis.

**Step 3 — Find whitespace.** Identify the unoccupied or under-served quadrant. This is the positioning opportunity.

**Step 4 — Validate alignment.** The USP you produce in the Execution section must describe the product's position in the whitespace quadrant. If the USP doesn't map to the whitespace, either the axes are wrong or the USP is wrong — revise one of them.

Use this template in the output artifact:

```
Axis X: [dimension 1 derived from ICP pain points]
Axis Y: [dimension 2 derived from ICP pain points]

| Player | X Score | Y Score | Quadrant |
|--------|---------|---------|----------|
| Our Product | ? | ? | ? |
| Competitor A | ? | ? | ? |
| Competitor B | ? | ? | ? |
| Competitor C | ? | ? | ? |

Whitespace quadrant: [identify the unoccupied quadrant]
Positioning opportunity: [one sentence describing why this quadrant is valuable to the ICP]
```

## Execution

1. Read all three prerequisite artifacts in full: `competitor-matrix.md`, `steal-differentiate-ignore.md`, `icp.md`.
2. Extract the **differentiate** bucket from `steal-differentiate-ignore.md` — these are the positioning opportunities.
3. Cross-reference differentiation opportunities against `icp.md` to identify which matter most to the target user.
4. Synthesize a **single-sentence USP** that:
   - Is **falsifiable** — it makes a concrete claim that could be proven wrong with evidence.
   - Is **unique** — it does not apply to any of the competitors listed in the competitor matrix.
   - Addresses a real pain point or desire from the ICP.
5. Produce an **"Only we..."** framing of the USP to stress exclusivity.
6. Write a **positioning statement** in the format: "For [ICP], [product] is the [category] that [USP] unlike [competitors] who [alternative]."
7. Define a **falsifiability test**: describe the specific evidence, metric, or experiment that would disprove the USP. If the USP cannot be disproven, it is too vague — revise it.
8. Validate the USP against every competitor: if the sentence could truthfully describe any listed competitor, the USP is not unique — revise it.

### USP Stress Tests

Run every test below before finalizing. If any test fails, revise the USP and re-run all tests.

1. **The Competitor Swap Test** — Replace your product name with each competitor's name in the USP. Read the sentence aloud. If it still sounds true for *any* competitor, the USP is not unique. Go back to the differentiate bucket and find a stronger angle.

2. **The "So What?" Test** — Read the USP to a non-technical person (or simulate one). Ask them to explain why it matters in one sentence. If they can't, the USP is too abstract or too jargon-heavy. Rewrite it in plain language that connects to a tangible outcome.

3. **The Negation Test** — Negate the USP. If the negation is absurd (e.g., "We DON'T help users save time"), the original is too generic — every competitor could claim it. A good USP's negation should be a *valid but different* strategy that a reasonable competitor might actually choose.

4. **The Time Test** — Ask: will this USP still differentiate the product in 18 months? If the answer is no (because competitors can trivially copy the feature), the USP describes a *feature advantage*, not a *positioning advantage*. Reframe around the structural reason the advantage persists (e.g., data network effects, architectural decisions, ecosystem lock-in).

## Output

Write the following artifact to `.specify/artifacts/phase-1/`:

### `usp.md`

```markdown
## USP Statement
<!-- Single falsifiable sentence -->

## Only We Framing
<!-- "Only we..." version of the USP -->

## Positioning Statement
<!-- For [ICP], [product] is the [category] that [USP] unlike [competitors] who [alternative]. -->

## Falsifiability Test
<!-- How you would prove this USP wrong: specific metric, experiment, or evidence -->
```

### Anti-Patterns

Reject any USP that falls into these traps:

- **"Best-in-class [X]"** — Unmeasurable superlative. Every competitor claims it, so it differentiates no one. Replace with a specific, falsifiable claim about *what* makes it better and *by how much*.
- **"AI-powered"** — AI is table stakes in 2025, not a moat. If the USP relies on "AI-powered" as the differentiator, ask: what does the AI *enable* that competitors' AI doesn't? Lead with the outcome, not the technology.
- **"Simple and intuitive"** — Subjective and unfalsifiable. No user has ever demanded a product be *more* confusing. Replace with a concrete claim about time-to-value, onboarding steps, or learning curve.
- **"All-in-one platform"** — The opposite of a USP. Claiming to do everything says nothing about what you do *uniquely well*. A USP must narrow, not broaden.

## Gate Criteria

- [ ] USP is exactly one sentence
- [ ] USP is falsifiable — a concrete test to disprove it is documented
- [ ] USP does not apply to any competitor listed in the competitor matrix (verified per-competitor)
- [ ] "Only we..." framing is present and consistent with the USP statement
- [ ] Positioning statement follows the prescribed format and references the ICP
- [ ] All claims in the USP trace back to evidence in the prerequisite artifacts


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
