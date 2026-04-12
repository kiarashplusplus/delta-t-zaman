---
description: "Evaluate a raw product idea against market reality"
handoffs:
  - label: "Next: Competitive Intelligence"
    agent: "os.competition"
    prompt: "Run competitive analysis based on the market research just completed"
---

## User Input

$ARGUMENTS — a raw product idea in natural language. This is the seed input for the entire pipeline.

## Prerequisites

This is the pipeline entry point. No prerequisite artifacts are required.

| Phase | Artifact | Status |
|-------|----------|--------|
| — | — | No prerequisites. This agent starts the pipeline. |

No audit script invocation is needed. This agent produces the first artifacts that all downstream agents depend on.

## Constitutional Authority

**Law II — Anti-Lazy Analysis** (Rules 2.1–2.3) as defined in `.specify/memory/constitution.md`.

- **Rule 2.1**: Every market size claim MUST cite a named source with a publication date. Unsourced numbers are constitutional violations.
- **Rule 2.2**: Every ICP trait MUST cite behavioral evidence — observed patterns, survey data, or usage analytics. Personas invented from assumptions are prohibited.
- **Rule 2.3**: Every timing claim MUST cite a specific market catalyst — a regulation change, technology inflection, funding trend, or behavioral shift with a verifiable date. "The time is right" without evidence is a constitutional violation.

## Execution

> **Tool Availability**: This agent declares MCP tools (`tavily/.../search`, `firecrawl/.../scrape`, `firecrawl/.../crawl`) for optimal results. If these MCP tools are unavailable, use the built-in `web_search` tool for research queries and `web_fetch` to extract content from specific URLs. All research instructions below apply to whichever variant is available — the quality expectation is the same regardless of tool used.

### Step 1 — Parse and understand the idea

Extract the core value proposition, target user, and implied market from `$ARGUMENTS`. If the idea is vague, make reasonable inferences but flag assumptions explicitly.

### Step 2 — Market research

Use **Tavily search** (or `web_search` if Tavily is unavailable) to find current market data, industry reports, and analyst publications. When search results return promising sources, use **Firecrawl scrape** (or `web_fetch` if Firecrawl is unavailable) to extract detailed data from those pages. For comprehensive industry portals, use **Firecrawl crawl** (or iterate `web_fetch` across multiple URLs) to gather data across multiple pages.

Investigate the market landscape with DATA-BACKED claims:

- **Market sizing**: Find TAM/SAM/SOM figures from credible sources (analyst reports, government data, industry publications). Every number must have a citation with source name and date.
- **Adjacent markets**: Identify related markets that could expand the opportunity or represent competitive threats.
- **Market trends**: Identify 3-5 macro trends affecting this market, each backed by evidence.

### Step 3 — Ideal Customer Profile

Build a data-backed ICP:

- **Demographics**: Age, role, company size, geography — cite surveys or census data.
- **Psychographics**: Values, priorities, decision-making patterns — cite behavioral research.
- **Pain points**: Specific, measurable problems — cite support forums, review sites, or survey data.
- **Behavioral patterns**: How they currently solve the problem — cite usage data or ethnographic research.

### Step 4 — Timing analysis

Evaluate why NOW is the right moment:

- Identify 2-4 specific market catalysts (regulation changes, technology shifts, funding trends, behavioral changes).
- Each catalyst must have a verifiable date and source.
- Assess whether the window is opening, peak, or closing.

### Step 5 — Kill-or-proceed decision

Apply rigorous judgment. This agent must KILL weak ideas:

- If TAM is below a viable threshold, say so and explain why.
- If timing is wrong (too early, too late), say so with evidence.
- If the ICP is too narrow or too diffuse, say so.
- If the idea survives scrutiny, state the conviction level (high / medium / cautious) with reasoning.

## Output

Write the following three artifacts to `.specify/artifacts/phase-1/`:

### 1. `market-map.md`

```markdown
## Market Overview
{High-level market description and boundaries}

## Total Addressable Market
{TAM/SAM/SOM with cited figures — every number must include [Source, Date]}

## Adjacent Markets
{Related markets with size estimates and overlap analysis}

## Market Trends
{3-5 macro trends, each with evidence and citation}

## Data Sources
{Complete bibliography of all sources cited in this document}
```

### 2. `icp.md`

```markdown
## Demographics
{Target user demographics with cited data — surveys, census, industry reports}

## Psychographics
{Values, motivations, decision-making patterns with behavioral citations}

## Pain Points
{Specific, measurable problems with evidence from forums, reviews, surveys}

## Behavioral Patterns
{Current solution behaviors with usage data or ethnographic citations}
```

### 3. `why-now.md`

```markdown
## Timing Thesis
{One-paragraph summary of why the timing is right — or why it isn't}

## Market Catalysts
{2-4 specific catalysts, each with:
- What changed
- When it changed (specific date or timeframe)
- Source citation
- Impact on the opportunity}

## Evidence
{Supporting data points, trend lines, and cross-references to market-map.md and icp.md}
```

## Gate Criteria

All of the following must be true before handing off to the next agent:

1. All three files exist: `.specify/artifacts/phase-1/market-map.md`, `.specify/artifacts/phase-1/icp.md`, `.specify/artifacts/phase-1/why-now.md`.
2. `market-map.md` contains all required headings: `## Market Overview`, `## Total Addressable Market`, `## Adjacent Markets`, `## Market Trends`, `## Data Sources`.
3. `icp.md` contains all required headings: `## Demographics`, `## Psychographics`, `## Pain Points`, `## Behavioral Patterns`.
4. `why-now.md` contains all required headings: `## Timing Thesis`, `## Market Catalysts`, `## Evidence`.
5. Every market size claim in `market-map.md` cites a source with a date (Rule 2.1).
6. Every ICP trait in `icp.md` cites behavioral evidence (Rule 2.2).
7. Every timing claim in `why-now.md` cites a market catalyst with a date (Rule 2.3).
8. No section under any required heading is empty.

### Citation Verification

Every externally-sourced claim must be tagged with its verification tier:

- **`[Verified]`** — Data retrieved from a tool during this run (Tavily, Firecrawl, `web_search`, `web_fetch`). Source URL and retrieval date are known.
- **`[Model-sourced]`** — Data from model training knowledge. May be outdated or imprecise. Flag with approximate date if known.

9. Every citation in all three artifacts is tagged `[Verified]` or `[Model-sourced]`.
10. If more than 50% of citations in any artifact are `[Model-sourced]`, that artifact MUST include a `## Verification Needed` section listing unverified claims for manual review.


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
