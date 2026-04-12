---
description: "Map the competitive landscape and identify strategic positioning opportunities"
handoffs:
  - label: "Next: USP & Positioning"
    agent: "os.usp"
    prompt: "Distill a falsifiable USP from the competitive analysis"
---

## User Input

$ARGUMENTS

## Prerequisites

| Artifact | Path | Producing Agent |
|---|---|---|
| `market-map.md` | `.specify/artifacts/phase-1/market-map.md` | `os.market` |
| `icp.md` | `.specify/artifacts/phase-1/icp.md` | `os.market` |
| `why-now.md` | `.specify/artifacts/phase-1/why-now.md` | `os.market` |

Run the pipeline audit to validate all prerequisite artifacts exist:

```bash
bash .specify/scripts/bash/pipeline-audit.sh --phase 1 --json
```

Verify that `market-map.md`, `icp.md`, and `why-now.md` are all present in the audit output. If **any** artifact is missing, **halt immediately** and emit the following YAML violation block (substituting the actual missing artifact names):

```yaml
violation_type: "missing_artifact"
blocking_gate: "Phase 0 → Phase 1"
missing_artifacts:
  - name: "market-map.md"
    expected_path: ".specify/artifacts/phase-1/market-map.md"
    producing_agent: "os.market"
  - name: "icp.md"
    expected_path: ".specify/artifacts/phase-1/icp.md"
    producing_agent: "os.market"
  - name: "why-now.md"
    expected_path: ".specify/artifacts/phase-1/why-now.md"
    producing_agent: "os.market"
remediation: "Run /os.market to generate the missing artifact"
```

Do **not** proceed past this section until all three artifacts are confirmed present.

## Constitutional Authority

- **Law II (Anti-Lazy Analysis)** — All competitive claims must be grounded in evidence from the prerequisite artifacts; no hand-waving or generic statements.
- **Law III (Brutal Critic) Rule 3.3** — Challenge every assumption. If a competitor's weakness looks too convenient, verify it. If a strength looks overstated, question it.

## Execution

> **Tool Availability**: This agent declares MCP tools (`tavily/.../search`, `firecrawl/.../scrape`, `firecrawl/.../crawl`) for optimal results. If these MCP tools are unavailable, use the built-in `web_search` tool for research queries and `web_fetch` to extract content from specific URLs. All research instructions below apply to whichever variant is available — the quality expectation is the same regardless of tool used.

1. Read all three prerequisite artifacts in full: `market-map.md`, `icp.md`, `why-now.md`.
2. Identify the **top 5 competitors** from the market map. Use **Tavily search** (or `web_search` if Tavily is unavailable) to find additional competitor details, feature lists, and recent product updates. If fewer than 5 exist, use all listed competitors and note the gap.
3. Build a **feature-by-feature comparison matrix** across all 5 competitors and the target product, covering every feature dimension mentioned in the market map and ICP.
4. For each competitor, perform a **UX teardown**: use **Firecrawl scrape** (or `web_fetch` if Firecrawl is unavailable) to crawl competitor product pages, documentation, and public-facing UX. Use **Firecrawl crawl** (or iterate `web_fetch` across multiple URLs) to navigate multi-page competitor sites. Analyze strengths (what they do well from the user's perspective) and weaknesses (friction points, gaps, poor design decisions).
5. Classify every competitor feature into exactly one of three buckets:
   - **Steal** — Feature is table-stakes or clearly superior; adopt it.
   - **Differentiate** — Feature exists but can be done meaningfully better; this is a positioning opportunity.
   - **Ignore** — Feature is irrelevant to the ICP or a distraction; skip it.
6. Cross-reference all classifications against the ICP to ensure the steal/differentiate/ignore decisions align with target user needs.

## Output

Write the following artifacts to `.specify/artifacts/phase-1/`:

### `competitor-matrix.md`

```markdown
## Competitors
<!-- Top 5 competitors with one-line descriptions -->

## Feature Comparison
<!-- Table: rows = features, columns = competitors + our product, cells = capability level -->

## Analysis
<!-- Key takeaways from the matrix: where we lead, where we lag, gaps in the market -->
```

### `ux-teardown.md`

```markdown
## Competitor UX Analysis
<!-- Per-competitor UX evaluation methodology and scope -->

## Strengths
<!-- Per-competitor list of UX strengths with evidence -->

## Weaknesses
<!-- Per-competitor list of UX weaknesses with evidence -->
```

### `steal-differentiate-ignore.md`

```markdown
## Steal
<!-- Features to adopt as-is, with rationale tied to ICP needs -->

## Differentiate
<!-- Features to do better, with specific differentiation angle -->

## Ignore
<!-- Features to skip, with rationale for why they don't serve the ICP -->
```

## Gate Criteria

- [ ] Top 5 competitors are explicitly named (or all available competitors if fewer than 5, with justification)
- [ ] Every feature in the comparison matrix has a value for every competitor
- [ ] Every competitor feature is classified as exactly one of: steal, differentiate, or ignore
- [ ] All steal/differentiate/ignore decisions reference the ICP for justification
- [ ] UX teardown covers both strengths and weaknesses for each competitor with concrete evidence
- [ ] No feature is left unclassified

### Citation Verification

- [ ] Every competitor claim (features, pricing, UX observations) is tagged `[Verified]` or `[Model-sourced]`
  - **`[Verified]`** — Retrieved from a tool during this run (Tavily, Firecrawl, `web_search`, `web_fetch`). Source URL and retrieval date known.
  - **`[Model-sourced]`** — From model training knowledge. May be outdated.
- [ ] If more than 50% of citations in any artifact are `[Model-sourced]`, that artifact MUST include a `## Verification Needed` section listing unverified claims for manual review


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
