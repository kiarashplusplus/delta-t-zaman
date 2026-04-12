# Artifact Metadata Schema

> Standard frontmatter block for every artifact produced by pipeline agents.
> Defines confidence, staleness, quality tracking, and provenance.

## Schema Definition

Every artifact written by an `os.*` or `speckit.*` agent SHOULD include
the following YAML frontmatter block at the top of the file:

```yaml
---
artifact_meta:
  produced_by: "<agent name>"         # e.g., "os.market", "os.sketch"
  produced_at: "<ISO 8601 timestamp>" # e.g., "2026-03-26T12:00:00Z"
  confidence: <0.0–1.0 float>        # How confident the agent is in this output
  inputs_used:
    - path: "<artifact path>"         # e.g., ".specify/artifacts/phase-1/icp.md"
      existed: <true|false>           # Whether the input existed when this artifact was produced
  stale_after: "on_input_change"      # Staleness trigger
  revision: <integer>                 # Starts at 1, increments on each re-generation
  quality_scores:
    specificity: <1–5 or null>        # From self-evaluation (null if not scored)
    actionability: <1–5 or null>
    non_redundancy: <1–5 or null>
    evidence_quality: <1–5 or null>
  grade: "<hypothesis|draft|validated>"
---
```

## Field Semantics

### `confidence` (0.0–1.0)

Reflects the agent's assessment of output reliability given available inputs.

| Range | Grade | Meaning |
|-------|-------|---------|
| 0.0–0.49 | `hypothesis` | Unvalidated assumption. Treat as a research target, not a fact. |
| 0.5–0.79 | `draft` | Reasonable output but based on incomplete inputs. Usable with caveats. |
| 0.8–1.0 | `validated` | Based on complete, verified inputs. Ready for downstream consumption. |

**How to set confidence:**
- Start at the base confidence for the agent (e.g., `os.sketch` starts at 0.3)
- Reduce by 0.1 for each prerequisite artifact that was missing (`existed: false`)
- Increase toward 1.0 as prerequisite artifacts are produced and incorporated
- Never set above 0.9 unless all inputs are `validated` grade

### `inputs_used`

Lists every artifact the agent consulted during execution.

- `path`: The file path relative to the repository root
- `existed`: Whether the file existed at execution time

This enables **staleness detection**: if an input artifact is later regenerated
(its `revision` increments), any downstream artifact that consumed the previous
version is considered stale.

### `stale_after`

Defines when this artifact should be considered stale:
- `"on_input_change"` (default) — Stale when any input artifact's revision has
  increased since this artifact was produced.

### `revision`

Integer starting at 1. Increments each time the artifact is regenerated.
The orchestrator (`os.loop`) uses revision numbers to detect staleness across
the artifact graph.

### `quality_scores`

Self-evaluation scores from the agent's `## Self-Evaluation` section.
Each criterion is scored 1–5:
- **Specificity**: Does every claim reference a source, data point, or upstream artifact?
- **Actionability**: Could the next downstream agent consume this without requesting clarification?
- **Non-redundancy**: Does this add information not already present in upstream artifacts?
- **Evidence quality**: Are assertions backed by data rather than assumptions?

Set to `null` for agents that don't perform self-evaluation (e.g., `os.sketch`).

### `grade`

Derived from `confidence`:
- `"hypothesis"` — confidence < 0.5
- `"draft"` — confidence ≥ 0.5 and < 0.8
- `"validated"` — confidence ≥ 0.8

## Example: Hypothesis-Grade Artifact (os.sketch)

```yaml
---
artifact_meta:
  produced_by: "os.sketch"
  produced_at: "2026-03-26T12:00:00Z"
  confidence: 0.3
  inputs_used: []
  stale_after: "on_input_change"
  revision: 1
  quality_scores:
    specificity: null
    actionability: null
    non_redundancy: null
    evidence_quality: null
  grade: "hypothesis"
---
```

## Example: Draft-Grade Artifact (os.market with partial inputs)

```yaml
---
artifact_meta:
  produced_by: "os.market"
  produced_at: "2026-03-26T13:00:00Z"
  confidence: 0.7
  inputs_used: []
  stale_after: "on_input_change"
  revision: 1
  quality_scores:
    specificity: 4
    actionability: 3
    non_redundancy: 4
    evidence_quality: 3
  grade: "draft"
---
```

## Example: Validated-Grade Artifact (os.usp with all inputs present)

```yaml
---
artifact_meta:
  produced_by: "os.usp"
  produced_at: "2026-03-26T15:00:00Z"
  confidence: 0.85
  inputs_used:
    - path: ".specify/artifacts/phase-1/competitor-matrix.md"
      existed: true
    - path: ".specify/artifacts/phase-1/steal-differentiate-ignore.md"
      existed: true
    - path: ".specify/artifacts/phase-1/icp.md"
      existed: true
  stale_after: "on_input_change"
  revision: 2
  quality_scores:
    specificity: 5
    actionability: 4
    non_redundancy: 4
    evidence_quality: 5
  grade: "validated"
---
```

## Adoption

- **New artifacts**: MUST include the full metadata block
- **Existing artifacts**: SHOULD add metadata when regenerated
- **os.loop**: Reads metadata to build the artifact state table and detect staleness
- **os.audit**: MAY validate metadata completeness as part of structural audits
