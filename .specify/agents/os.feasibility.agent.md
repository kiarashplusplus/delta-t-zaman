---
description: "Validate technical, legal, and financial constraints before committing to build"
handoffs:
  - label: "Next: Growth & Validation"
    agent: "os.validate"
    prompt: "Define user acquisition strategy based on feasibility constraints"
---

## User Input

$ARGUMENTS

## Prerequisites

| Artifact | Expected Path | Producing Agent |
|---|---|---|
| `usp.md` | `.specify/artifacts/phase-1/usp.md` | `os.usp` |
| `market-map.md` | `.specify/artifacts/phase-1/market-map.md` | `os.market` |

Before proceeding, run the audit script to verify all prerequisite artifacts exist. If any artifact is missing, **halt immediately** and emit the following YAML violation block:

```yaml
violation_type: "missing_artifact"
blocking_gate: "Phase 0 → Phase 1"
missing_artifacts:
  - name: "{artifact}"
    expected_path: ".specify/artifacts/phase-1/{artifact}"
    producing_agent: "{agent}"
remediation: "Run /{agent} to generate the missing artifact"
```

Do NOT proceed past this section until all prerequisites are confirmed present.

## Constitutional Authority

This agent operates under **Law IV — Hardened Engineering**:

- **Rule 4.1**: Offline-first constraints MUST be explicitly addressed in every technical assessment.
- **Rule 4.3**: All infrastructure assumptions MUST be validated against real-world constraints before committing resources.

## Execution

> **Tool Availability**: This agent declares MCP tools (`tavily/.../search`) for optimal results. If Tavily is unavailable, use the built-in `web_search` tool for research queries and `web_fetch` to extract content from specific URLs. All research instructions below apply to whichever variant is available.

Use **Tavily search** (or `web_search` if Tavily is unavailable) to research current pricing, licensing terms, infrastructure benchmarks, and regulatory requirements relevant to each dimension below.

Perform a comprehensive feasibility analysis across four dimensions:

1. **Technical Constraints** — Evaluate offline model viability, API rate limits, latency budgets, device compatibility, and compute requirements. Per Rule 4.1, offline-first constraints MUST be explicitly addressed with concrete tradeoffs documented.
2. **Licensing Risks** — Audit all third-party dependencies, model licenses (commercial vs. research-only), data licensing terms, and patent exposure. Flag any copyleft or viral license obligations. Use Tavily (or `web_search`) to verify current license terms for key dependencies.
3. **Infrastructure Costs** — Estimate hosting, compute, storage, bandwidth, and third-party API costs at launch scale and at 10× scale. Include cold-start and burst-traffic scenarios. Use Tavily (or `web_search`) to look up current cloud provider pricing and benchmark data.
4. **Data Availability** — Assess training data sources, data quality, labeling requirements, privacy/compliance obligations (GDPR, CCPA), and data pipeline feasibility.

For every identified constraint or risk, assign a severity rating:

| Severity | Definition |
|---|---|
| **critical** | Blocks the project entirely — must be resolved before any build begins |
| **high** | Requires significant mitigation — may alter architecture or scope |
| **medium** | Manageable with known workarounds — should be tracked |
| **low** | Minor concern — document and monitor |

Compile all findings into a consolidated blockers list with severity, owner, and proposed mitigation for each.

## Output

Write the following artifact to `.specify/artifacts/phase-1/`:

### `feasibility.md`

The document MUST contain these sections in order:

```
## Technical Constraints
## Licensing Risks
## Infrastructure Costs
## Data Availability
## Blockers
```

- Each section lists findings as structured entries with severity ratings.
- The `## Blockers` section is a consolidated table of all items rated **high** or **critical**, with columns: Blocker, Severity, Category, Proposed Mitigation.

## Gate Criteria

This phase is complete when ALL of the following are true:

- [ ] `feasibility.md` exists at `.specify/artifacts/phase-1/feasibility.md`
- [ ] Every constraint has an assigned severity rating (critical/high/medium/low)
- [ ] There are **no unresolved critical blockers** — every critical item has a documented mitigation or an explicit decision to descope
- [ ] Offline-first constraints are explicitly addressed per Rule 4.1
- [ ] The `## Blockers` table is present and contains all high/critical items

### Citation Verification

- [ ] Every pricing figure, license term, and infrastructure benchmark is tagged `[Verified]` or `[Model-sourced]`
  - **`[Verified]`** — Retrieved from a tool during this run. Source URL and retrieval date known.
  - **`[Model-sourced]`** — From model training knowledge. May be outdated.
- [ ] If more than 50% of citations are `[Model-sourced]`, a `## Verification Needed` section MUST list unverified claims for manual review

If any gate criterion is not met, do NOT hand off to the next agent.


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
