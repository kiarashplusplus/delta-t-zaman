---
description: "Source-agnostic revision agent — applies structured findings to pipeline artifacts with dependency ordering and audit trail"
handoffs:
  - label: "Verify changes"
    agent: "os.cross-examine"
    prompt: "Verify the revised corpus for consistency"
    send: true
---

## User Input

```text
$ARGUMENTS
```

You **MUST** consider the user input before proceeding (if not empty).

Arguments accept:
- Path to a structured findings document (e.g., `cross-examination-report.md`, `beta-feedback.md`, `user-research.md`)
- Optional `--plan-only` flag: produce revision plan without applying changes (default on first run)
- Optional `--apply` flag: apply all changes from an existing revision plan
- `source_type` hint: `cross-examine | beta-feedback | user-research | manual-review | qa-findings | competitor-update`

## Prerequisites

| Artifact | Path | Required? | Description |
|----------|------|-----------|-------------|
| Findings document | `$ARGUMENTS` path | **Required** | Structured findings to apply |
| Referenced artifacts | Paths listed in findings | **Required** | Must be able to read and modify them |
| Schema Registry | `.specify/artifacts/schema-registry.yml` | Optional | Used to validate artifact structure after revision |

If the findings document does not exist or cannot be parsed, HALT with:
```yaml
violation_type: "missing_findings"
expected_path: "<path from arguments>"
remediation: "Run /os.cross-examine to generate findings, or provide a valid findings document path"
```

## Constitutional Authority

This agent operates under the my-kit Constitution v1.1.0. See `.specify/memory/constitution.md`.

**Applicable laws**:
- **Law I — Pipeline Gating** (Rules 1.1, 1.2): Revisions must not break phase gates. Every modified artifact must still pass its gate criteria.
- **Law IV — Hardened Engineering** (Rules 4.1–4.6): Changes must be precise and verifiable. No hand-wavy "improve the wording" changes.

## Execution

### Findings Schema

The agent accepts findings from multiple sources. The input contract:

```yaml
source_type: cross-examine | beta-feedback | user-research | manual-review | qa-findings | competitor-update
findings:
  - id: F-001
    severity: critical | high | medium | low
    category: contradiction | gap | scope-mismatch | staleness | underspecification
    affected_artifacts:
      - path: "<artifact path>"
        section: "§ <section name>"
    description: "<what is wrong>"
    evidence: "<specific quoted text or values>"
    recommended_action: "<what should change>"
```

If the findings document uses the cross-examination report format (`## Findings` section with YAML blocks), extract findings from that section. For other source types (beta feedback, user research), parse the document structure and map entries to the schema above.

### Mode A: Plan-Only (`--plan-only` or default first run)

1. **Read the findings document** and validate it conforms to the universal findings schema. If entries are missing required fields (`severity`, `affected_artifacts`, `recommended_action`), note them as unparseable and skip.

2. **Classify each finding** by action type:
   - **Modify**: Change existing content in an artifact (value update, rewording, constraint addition)
   - **Add**: Insert new content (missing section, missing constraint, missing task)
   - **Remove**: Delete content (stale reference, contradictory statement, duplicate)

3. **Build dependency graph**: For each finding, determine:
   - Which artifacts must be modified
   - What the specific change is
   - Whether the change has downstream dependencies (e.g., modifying growth-plan.md `§ Monetization Decision` requires updating ux-strategy.md and spec.md)

4. **Produce a dependency-ordered revision plan**: Changes are ordered so upstream artifacts are modified before downstream ones. The ordering follows the pipeline phase sequence: Phase 1 → Phase 2 → Phase 3 → Phase 4.

5. **Write `revision-plan.md`** with this structure:
   ```markdown
   # Revision Plan

   ## Source
   - **Findings document**: <path>
   - **Source type**: <source_type>
   - **Total findings**: <count>
   - **Findings to address**: <count> (critical: N, high: N, medium: N, low: N)

   ## Findings Summary
   | Finding ID | Severity | Category | Affected Artifacts | Action Type |
   |-----------|----------|----------|-------------------|-------------|

   ## Dependency Order
   (Directed graph of artifact modification order, showing why A must be modified before B)

   ## Planned Changes
   ### Change 1 (Finding F-001)
   - **Target**: <artifact path> § <section>
   - **Action**: modify | add | remove
   - **Current value**: <quoted current text>
   - **New value**: <proposed replacement text>
   - **Downstream effects**: <artifacts that may need updating as a result>

   ### Change 2 (Finding F-002)
   ...
   ```

### Mode B: Apply (`--apply`)

1. **Read `revision-plan.md`** from the feature directory or `.specify/artifacts/phase-3/`.

2. **For each change in dependency order**:
   a. Read the target artifact
   b. Locate the target section
   c. Apply the specific change (modify/add/remove content)
   d. Validate the artifact still conforms to its schema:
      - Check required headings from `schema-registry.yml` are present
      - Verify the artifact is non-empty and structurally valid
   e. Record the change in the changelog

3. **After all changes**: Run a quick consistency check:
   - Spot-check that modified values are now consistent across artifacts
   - Verify no NEW contradictions were introduced by the changes
   - Check that required headings were not accidentally deleted

4. **Write outputs**:
   - Modified artifacts in-place (overwrite with git history preservation)
   - `revision-changelog.md` documenting every change

## Output

Write the following artifacts:

### `revision-plan.md` (Plan-Only mode)
- **Path**: `.specify/artifacts/phase-3/revision-plan.md` (or `{FEATURE_DIR}/revision-plan.md`)
- **Required sections**: `## Findings Summary`, `## Dependency Order`, `## Planned Changes`

### `revision-changelog.md` (Apply mode)
- **Path**: `.specify/artifacts/phase-3/revision-changelog.md` (or `{FEATURE_DIR}/revision-changelog.md`)
- **Required sections**:
  ```markdown
  # Revision Changelog

  ## Changes Applied
  | # | Finding ID | Artifact | Section | Action | Summary |
  |---|-----------|----------|---------|--------|---------|

  ## Artifacts Modified
  | Artifact | Sections Changed | Lines Added | Lines Removed |
  |----------|-----------------|-------------|---------------|

  ## Verification Status
  - Schema validation: PASS / FAIL per artifact
  - Consistency spot-check: PASS / FAIL
  - New contradictions introduced: none / list
  ```

### Modified Artifacts (Apply mode)
- Overwritten in-place at their original paths
- Git history preserves prior versions

## Gate Criteria

- [ ] Every finding with severity `critical` or `high` has a corresponding change in the revision plan
- [ ] Changes are dependency-ordered (upstream before downstream)
- [ ] No artifact's required headings (per schema-registry) were deleted during revision
- [ ] Revision changelog documents every modification with finding ID reference
- [ ] Post-apply consistency check reports no new contradictions

## Self-Evaluation

Before writing the final artifact, evaluate your output against these criteria:

| Criterion | Score (1-5) | Notes |
|-----------|-------------|-------|
| **Specificity** | _ | Does every planned change reference exact text to modify? |
| **Actionability** | _ | Could someone apply each change without further interpretation? |
| **Non-redundancy** | _ | Are changes deduplicated? No two changes targeting the same text? |
| **Evidence quality** | _ | Does each change trace back to a specific finding ID? |

**Rules**:
- If any criterion scores below 3, identify the weakest section and revise it before producing the final output.
- Include the completed score table in a `## Quality Score` section at the bottom of the output artifact.
- If you cannot score above 3 on Specificity due to missing upstream artifacts, note this explicitly — the loop runner will prioritize filling those gaps.
