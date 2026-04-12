---
description: "Validate pipeline artifact completeness and structural integrity across all phases"
handoffs:
  - label: "Fix findings with revision agent"
    agent: "os.revise"
    prompt: "Apply structural and quality fixes from the audit report"
    send: true
  - label: "Cross-examine full corpus"
    agent: "os.cross-examine"
    prompt: "Run full-corpus cross-examination to detect broken decision chains after audit"
  - label: "Re-run the loop"
    agent: "os.loop"
    prompt: "Continue plan-execute-evaluate cycle after audit findings"
---

## User Input

```text
$ARGUMENTS
```

You **MUST** consider the user input before proceeding (if not empty).

Arguments accept:
- A phase filter (e.g., `phase 2`, `phase 1-3`, `all`). If omitted, audit ALL phases.
- `task sizing` — triggers the task-sizer hook after the standard audit.
- `--mode loop` — use soft gates with confidence scores instead of strict pass/fail (see Loop Mode below).

## Prerequisites

This agent HAS no prerequisites — it IS the prerequisite checker for every other agent in the pipeline. Do not invoke any audit script against itself.

## Constitutional Authority

**Law I — Pipeline Gating** (Rules 1.1–1.6) as defined in `.specify/memory/constitution.md`.

- **Rule 1.1**: No agent may produce Phase N artifacts until all Phase N-1 artifacts pass structural validation.
- **Rule 1.2**: Structural validation requires file existence, required-heading presence, and non-empty content under each heading.
- **Rule 1.3**: Staleness checks must compare artifact mtime against their upstream dependencies.
- **Rule 1.4**: Gate violations must block downstream execution and emit a machine-readable violation block.
- **Rule 1.5**: The audit agent is the sole authority for gate pass/fail decisions.
- **Rule 1.6**: Audit results must be reproducible — identical inputs must yield identical verdicts.

**Law VIII — Loop-First Execution** (Rules 8.1–8.2) — applies when `--mode loop` is specified or the invocation comes from `os.loop`.

- **Rule 8.1**: Soft gates replace strict pass/fail. Every artifact receives a **confidence score** (0.0–1.0) instead of a binary verdict.
- **Rule 8.2**: Confidence grades: hypothesis (<0.5), draft (0.5–0.8), validated (≥0.8). Loop Mode allows progression with draft-grade artifacts while flagging them for future improvement.

**Rule 8.5 — Brownfield Entry**: When `.specify/artifacts/codebase-profile.md` exists and fewer than 3 Phase 1 artifacts are present, the audit treats this as a brownfield workflow. Phase 1 gates are relaxed — missing Phase 1 artifacts are reported as ⚠️ Optional (brownfield) rather than ❌ Missing.

## Schema Reference

All artifact definitions, required headings, gate criteria, minimum content lines, and producing agents are defined in:

```
.specify/artifacts/schema-registry.yml
```

The audit scripts parse this registry as the single source of truth for structural validation.

## Execution

### Step 1 — Run artifact completeness audit

Determine the scope from `$ARGUMENTS`:

**Bash** (Linux / macOS):
- If a phase filter is provided: `bash .specify/scripts/bash/pipeline-audit.sh --phase N --json`
- If no filter is provided: `bash .specify/scripts/bash/pipeline-audit.sh --all --json`

**PowerShell** (Windows):
- If a phase filter is provided: `pwsh .specify/scripts/powershell/pipeline-audit.ps1 -Phase N -Json`
- If no filter is provided: `pwsh .specify/scripts/powershell/pipeline-audit.ps1 -All -Json`

Capture the JSON output as `audit_result`.

### Step 2 — Run staleness check

**Bash**:
```bash
bash .specify/scripts/bash/pipeline-staleness.sh --json
```

**PowerShell**:
```powershell
pwsh .specify/scripts/powershell/pipeline-staleness.ps1 -Json
```

Capture the JSON output as `staleness_result`.

### Step 3 — Conditional task-sizer hook

If the user input contains "task sizing":

**Bash**:
```bash
bash .specify/scripts/bash/pipeline-task-sizer.sh --json
```

**PowerShell**:
```powershell
pwsh .specify/scripts/powershell/pipeline-task-sizer.ps1 -Json
```

Capture the output as `sizer_result` and include it in the final report under a `## Task Sizing` section.

### Step 4 — Brownfield detection

Check for brownfield workflow indicators:

1. Does `.specify/artifacts/codebase-profile.md` exist?
2. Count existing Phase 1 artifacts (from schema-registry.yml).
3. If codebase-profile exists AND fewer than 3 Phase 1 artifacts are present → **brownfield mode**.

In brownfield mode:
- Missing Phase 1 artifacts are reported as `⚠️ Optional (brownfield)` instead of `❌ Missing`.
- The gate verdict is NOT blocked by missing Phase 1 artifacts.
- Phase 2+ artifacts are still strictly validated.

### Step 5 — Merge into gate report

Combine `audit_result` and `staleness_result` into a single structured report. For each artifact, determine its **structural status**:

| Status | Meaning |
|--------|---------|
| ✅ Pass | File exists, all required headings present, content non-empty |
| ❌ Missing | File does not exist at expected path |
| ⚠️ Stale | File exists but is older than its upstream dependency |
| ⚠️ Optional (brownfield) | Phase 1 artifact missing in brownfield mode — not blocking |
| ❌ Invalid | File exists but fails structural validation (missing headings, empty sections) |

### Step 6 — Content quality analysis

For every artifact that passes structural validation (status ✅ Pass or ⚠️ Stale), perform a deeper quality analysis. Read the artifact content and evaluate against the quality dimensions below. Skip this step for artifacts with status ❌ Missing or ❌ Invalid — those already have clear remediation paths.

#### 6a. Section depth analysis

For each required heading in the artifact, count the word count of content under that heading (up to the next heading of equal or higher level). Flag sections as:

| Section depth | Word count | Rating |
|---------------|-----------|--------|
| Robust | ≥ 100 words | No flag |
| Adequate | 30–99 words | No flag |
| Thin | 10–29 words | 🟡 Suggestion |
| Skeletal | < 10 words | 🟠 Important |

#### 6b. Gate criteria evaluation

Each artifact in `.specify/artifacts/schema-registry.yml` has a `gate_criteria` field describing the semantic quality bar (e.g., "All claims cite data sources with dates", ">=5 failure scenarios, each with severity + mitigation"). For each passing artifact:

1. Read the `gate_criteria` string from the schema registry.
2. Evaluate whether the artifact content plausibly satisfies the criteria by scanning for the described evidence patterns.
3. Assign a rating:
   - **Met** — the artifact clearly satisfies the gate criteria.
   - **Partially met** — some evidence is present but incomplete (e.g., 3 of 5 required failure scenarios).
   - **Not met** — no discernible evidence that the criteria are satisfied.
4. Produce a one-sentence explanation of the finding.

#### 6c. Cross-reference completeness

Check whether the artifact references its expected upstream dependencies (as listed in the staleness dependency graph). For each upstream artifact:

- If the artifact text mentions the upstream artifact's key concepts or explicitly cites it → **Linked**.
- If no reference is found → 🔵 Suggestion to cross-reference the upstream artifact for stronger traceability.

#### 6d. Stub detection

If the artifact content begins with `STUB:` or consists predominantly of placeholder text (e.g., "TODO", "TBD", "placeholder"), flag it as 🟠 Important with a recommendation to complete the content.

#### 6e. Confidence scoring (Loop Mode only)

When operating in Loop Mode (`--mode loop`), assign a confidence score (0.0–1.0) to each artifact based on:

| Factor | Weight | Scoring |
|--------|--------|---------|
| Structural validation | 0.3 | 1.0 if pass, 0.0 if fail |
| Gate criteria | 0.3 | 1.0 met, 0.5 partially met, 0.0 not met |
| Section depth | 0.2 | Proportion of sections rated Adequate or Robust |
| Cross-references | 0.1 | Proportion of upstream deps linked |
| Freshness | 0.1 | 1.0 if not stale, 0.0 if stale |

Map the weighted score to a grade: hypothesis (<0.5), draft (0.5–0.8), validated (≥0.8).

### Step 7 — Generate improvement recommendations

Aggregate all findings from Step 6 into a prioritized recommendation list for each artifact. Each recommendation MUST include:

1. **Severity** — one of:
   - 🔴 **Critical**: Gate criteria not met; the artifact structurally passes but provides insufficient semantic value for downstream agents.
   - 🟠 **Important**: Skeletal sections or stub content; artifact exists but provides insufficient value.
   - 🟡 **Suggestion**: Thin sections that could be expanded for better downstream consumption.
   - 🔵 **Enhancement**: Cross-reference gaps or minor traceability improvements.
2. **Target** — the specific artifact and section affected.
3. **Finding** — a concrete, evidence-based description of the gap.
4. **Recommendation** — a specific, actionable improvement step (not generic advice).
5. **Producing agent** — which agent to re-run to address the gap.

## Output

Emit a Markdown gate report to stdout with the following structure:

### Per-phase artifact table

For each phase in scope, produce an expanded table:

```markdown
### Phase {N}: {Phase Name}

| Artifact | Path | Structure | Gate Criteria | Depth | Recommendations |
|----------|------|-----------|---------------|-------|-----------------|
| {name} | {path} | ✅ Pass | ✅ Met | Robust | — |
| {name} | {path} | ✅ Pass | 🟡 Partially met | 2 thin sections | 2 suggestions |
| {name} | {path} | ❌ Missing | — | — | 1 critical |
| {name} | {path} | ⚠️ Stale | ✅ Met | Adequate | 1 enhancement |
| {name} | {path} | ❌ Invalid | — | — | 1 critical |
```

### Improvement Recommendations

Group recommendations by severity, then by artifact:

```markdown
## Improvement Recommendations

### 🔴 Critical

1. **brutal-critique.md** § Failure Scenarios — Gate criteria require ≥5 failure scenarios; only 2 found. Re-run `/os.critic` with explicit instruction to enumerate at least 5 distinct failure modes.

2. **growth-plan.md** — File not found. Run `/os.validate` to generate the growth plan artifact.

### 🟠 Important

3. **icp.md** § Behavioral Patterns — Section contains only 8 words ("Users tend to search before buying products online"). Expand with specific behavioral data points. Re-run `/os.market` focusing on behavioral research.

### 🟡 Suggestions

4. **market-map.md** § Adjacent Markets — Section has 22 words; consider expanding with TAM/SAM estimates for each adjacent market. Re-run `/os.market` if additional market data is available.

### 🔵 Enhancements

5. **usp.md** — Does not reference competitor-matrix.md or steal-differentiate-ignore.md. Add explicit cross-references to strengthen positioning rationale. Re-run `/os.usp` with upstream artifacts attached.
```

If there are no recommendations at a given severity level, omit that subsection.

### Summary

```markdown
## Gate Summary

- **Phases audited**: {list}
- **Total artifacts**: {count}
- **Structure passed**: {count}
- **Structure failed**: {count}
- **Stale**: {count}
- **Brownfield skips**: {count} (only in brownfield mode)
- **Gate verdict**: PASS | FAIL

## Quality Summary

- **Gate criteria met**: {count}/{evaluated count}
- **Gate criteria partially met**: {count}
- **Gate criteria not met**: {count}
- **Thin sections**: {count}
- **Skeletal sections**: {count}
- **Cross-reference gaps**: {count}
- **Total recommendations**: {count} (🔴 {n} · 🟠 {n} · 🟡 {n} · 🔵 {n})
```

### Loop Mode summary (when `--mode loop`)

```markdown
## Confidence Summary

| Artifact | Confidence | Grade | Blocking? |
|----------|-----------|-------|-----------|
| {name} | 0.85 | validated | No |
| {name} | 0.62 | draft | No |
| {name} | 0.30 | hypothesis | ⚠️ Low confidence |

- **Mean confidence**: {value}
- **Lowest confidence artifact**: {name} ({score})
- **Loop recommendation**: {proceed | iterate on lowest-confidence artifacts | halt — critical structural failures}
```

### On failure — violation block

When any artifact fails structural validation, emit a YAML-fenced violation block for EACH failure:

```yaml
violation_type: "missing_artifact"
blocking_gate: "Phase {N-1} → Phase {N}"
missing_artifacts:
  - name: "{artifact}"
    expected_path: "{path}"
    producing_agent: "{agent}"
remediation: "Run /{agent} to generate the missing artifact"
```

For staleness violations:

```yaml
violation_type: "stale_artifact"
blocking_gate: "Phase {N}"
stale_artifacts:
  - name: "{artifact}"
    artifact_path: "{path}"
    upstream_dependency: "{dependency_path}"
    artifact_mtime: "{timestamp}"
    dependency_mtime: "{timestamp}"
remediation: "Re-run /{agent} to regenerate the stale artifact"
```

For structural violations:

```yaml
violation_type: "invalid_structure"
blocking_gate: "Phase {N}"
invalid_artifacts:
  - name: "{artifact}"
    artifact_path: "{path}"
    missing_headings:
      - "{heading}"
    empty_sections:
      - "{heading}"
remediation: "Re-run /{agent} ensuring all required sections are populated"
```

For gate criteria violations (quality):

```yaml
violation_type: "gate_criteria_not_met"
blocking_gate: "none"
quality_issues:
  - name: "{artifact}"
    artifact_path: "{path}"
    gate_criteria: "{expected criteria from schema-registry}"
    assessment: "not_met | partially_met"
    evidence: "{what was found or missing}"
    producing_agent: "{agent}"
remediation: "Re-run /{agent} focusing on: {specific gap}"
```

## Gate Criteria

All of the following must be true for a **PASS** structural verdict:

1. Every artifact in every audited phase exists at its expected path.
2. Every artifact passes structural validation (all required headings present, no empty sections).
3. No artifact is stale relative to its upstream dependencies.
4. The audit script exits with code 0.
5. The staleness script exits with code 0.

If ANY condition fails, the structural verdict is **FAIL** and downstream agents MUST NOT proceed.

**Brownfield exception**: In brownfield mode (codebase-profile.md exists, <3 Phase 1 artifacts), missing Phase 1 artifacts do not trigger FAIL. All other conditions apply.

**Loop Mode exception**: In Loop Mode, the gate verdict is replaced by the confidence summary. Progression is allowed with draft-grade (≥0.5) artifacts, but hypothesis-grade (<0.5) artifacts are flagged for immediate iteration.

**Quality recommendations do NOT block the gate.** They are advisory and appear alongside the structural verdict to help teams improve artifact quality iteratively. However, 🔴 Critical quality issues (gate criteria not met) SHOULD be addressed before implementation phases to avoid rework.

## Available Scripts

| Script | Platform | Purpose |
|--------|----------|---------|
| `.specify/scripts/bash/pipeline-audit.sh` | Bash | Artifact completeness & structural validation |
| `.specify/scripts/bash/pipeline-staleness.sh` | Bash | Timestamp-based staleness detection |
| `.specify/scripts/bash/pipeline-task-sizer.sh` | Bash | Task sizing validation (<4000 tokens) |
| `.specify/scripts/powershell/pipeline-audit.ps1` | PowerShell | Artifact completeness & structural validation |
| `.specify/scripts/powershell/pipeline-staleness.ps1` | PowerShell | Timestamp-based staleness detection |
| `.specify/scripts/powershell/pipeline-task-sizer.ps1` | PowerShell | Task sizing validation (<4000 tokens) |

All scripts support `--json` (Bash) or `-Json` (PowerShell) for machine-readable output.

## Self-Evaluation

Before writing the final artifact, evaluate your output against these criteria:

| Criterion | Score (1-5) | Notes |
|-----------|-------------|-------|
| **Specificity** | _ | Does every recommendation reference a concrete section, word count, or criteria gap? |
| **Actionability** | _ | Does every recommendation name the producing agent and a specific improvement step? |
| **Non-redundancy** | _ | Does this add information not already present in upstream artifacts? |
| **Evidence quality** | _ | Are assertions backed by measured data (word counts, criteria matches) rather than assumptions? |
| **Prioritization** | _ | Are recommendations ordered by severity so teams can triage effectively? |

**Rules**:
- If any criterion scores below 3, identify the weakest section and revise it before producing the final output.
- Include the completed score table in a `## Quality Score` section at the bottom of the output artifact.
- If you cannot score above 3 on Specificity due to missing upstream artifacts, note this explicitly — the loop runner will prioritize filling those gaps.
