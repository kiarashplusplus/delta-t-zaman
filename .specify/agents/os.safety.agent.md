---
description: "Generate hallucination mitigation scoring and source attribution rules"
---

## User Input

$ARGUMENTS

## Prerequisites

| Artifact | Path | Producing Agent |
|---|---|---|
| QA Report | `.specify/artifacts/phase-3/qa-report.md` | `os.qa` |
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

Run the pipeline audit to validate all Phase 1 and Phase 2 artifacts exist:

```bash
bash .specify/scripts/bash/pipeline-audit.sh --phase 1 --json
```

```bash
bash .specify/scripts/bash/pipeline-audit.sh --phase 2 --json
```

Verify that `qa-report.md` exists in `.specify/artifacts/phase-3/`. If ANY prerequisite artifact is missing, HALT immediately and emit a YAML gate violation:

```yaml
violation_type: "missing_artifact"
blocking_gate: "Phase 2 → Phase 3"
missing_artifacts:
  - name: "{artifact}"
    expected_path: "{path}"
    producing_agent: "{agent}"
remediation: "Run /{agent} to generate the missing artifact"
```

Do **not** proceed past this section until all prerequisites are confirmed present.

## Constitutional Authority

**Law IV — Hardened Engineering** (Rule 4.5)

AI-generated content is guilty until proven innocent. Every output must carry attribution, a confidence score, and a verification path. Users must never be exposed to unverified AI claims without clear trust indicators.

## Execution

1. **READ** the `qa-report.md` in full, especially the `## AI Hallucination Tests` section. Also read `icp.md` to understand the target user's trust expectations.
2. Define a **Trust Scoring Framework**:
   - Establish scoring dimensions (source quality, recency, corroboration count, domain authority)
   - Define a numerical scale (e.g., 0.0–1.0) with labeled tiers (e.g., `verified`, `high-confidence`, `uncertain`, `unverified`)
   - Map each tier to a specific user-facing behavior (display as-is, display with caveat, require confirmation, block display)
3. Define **Source Attribution Rules**:
   - Every AI-generated output MUST trace to at least one source
   - Sources must include: origin (API, database, user input, model inference), timestamp, and reliability tier
   - Outputs with no traceable source MUST be flagged as `unattributed` and suppressed or caveated
4. Define **Hallucination Detection Heuristics**:
   - Internal consistency checks (does the output contradict other outputs or known data?)
   - Temporal plausibility (does the output reference events or data outside its training window?)
   - Confidence distribution anomalies (suspiciously uniform high confidence across diverse claims)
   - Cross-reference validation (can claims be verified against independent sources?)
5. Define **Confidence Thresholds** (numeric):
   - Threshold for auto-display (e.g., ≥ 0.85)
   - Threshold for display-with-caveat (e.g., 0.60–0.84)
   - Threshold for suppression (e.g., < 0.60)
   - Thresholds MUST be numeric — no vague qualitative labels without numbers
   - **Scoring Methodology**: Define how confidence scores are calculated. Use one or more of the following techniques, adapted to the project's tech stack:
     - **Source overlap %**: Fraction of claims in the output that can be traced to a cited source document. Score = (attributed claims / total claims).
     - **Retrieval confidence**: If using vector search or RAG, use the similarity score from the retrieval step as a proxy for factual grounding.
     - **Ensemble agreement**: Run the same query through multiple models or prompts. Score = fraction of outputs that agree on the same factual claims.
     - **Cross-reference hit rate**: Fraction of claims that can be independently verified against a known-good dataset (e.g., knowledge base, database records).
     - **Temporal validity**: Penalize claims that reference data beyond the model's training cutoff or the source's freshness window.
   - The chosen methodology MUST be documented in the output artifact so downstream agents (e.g., `os.qa`) can validate scores reproducibly.
6. Define **User-Facing Trust Indicators**:
   - Visual indicators for each confidence tier (icons, colors, labels)
   - Disclosure language for AI-generated content
   - User override mechanism (allow users to request source details on any output)

## Output

Write the following artifact to `.specify/artifacts/phase-3/`:

- **Path**: `.specify/artifacts/phase-3/trust-safety-report.md`
- **Required sections**:
  - `## Trust Scoring Framework` — scoring dimensions, numerical scale with labeled tiers, and tier-to-behavior mapping
  - `## Source Attribution Rules` — requirements for every AI output to trace to a source, including origin, timestamp, and reliability tier
  - `## Hallucination Detection` — heuristics for detecting fabricated content: consistency checks, temporal plausibility, confidence anomalies, cross-reference validation
  - `## Confidence Thresholds` — numeric thresholds for auto-display, display-with-caveat, and suppression with specific cutoff values
  - `## User-Facing Trust Indicators` — visual indicators, disclosure language, and user override mechanisms for each confidence tier

## Gate Criteria

- [ ] All AI outputs have explicit attribution requirements (origin, timestamp, reliability tier)
- [ ] Confidence thresholds are numeric with specific cutoff values (not vague qualitative labels)
- [ ] Hallucination detection heuristics cover at least: internal consistency, temporal plausibility, and cross-reference validation
- [ ] Trust scoring framework maps every tier to a concrete user-facing behavior
- [ ] User-facing trust indicators are defined for each confidence tier
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
