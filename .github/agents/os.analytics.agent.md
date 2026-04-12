---
description: "Consume real-world logs and behavioral data to diagnose product health"
handoffs:
  - label: "Next: Simplification"
    agent: "os.simplify"
    prompt: "Identify features to cut based on analytics data"
---

## User Input

$ARGUMENTS

## Prerequisites

| Artifact | Path | Producing Agent |
|---|---|---|
| Launch Kit | `.specify/artifacts/phase-4/launch-kit.md` | `os.launch` |

Ideally, real-world logs exist (Cloudflare, GitHub API, app analytics). However, this agent MUST handle the no-data case gracefully by defining the analytics framework even when no logs are available yet.

Run the pipeline audit to validate all prerequisite artifacts exist:

```bash
bash .specify/scripts/bash/pipeline-audit.sh --phase 1 --json
```

```bash
bash .specify/scripts/bash/pipeline-audit.sh --phase 2 --json
```

```bash
bash .specify/scripts/bash/pipeline-audit.sh --phase 3 --json
```

Verify that `launch-kit.md` exists in `.specify/artifacts/phase-4/`. If ANY prerequisite artifact is missing, HALT immediately and emit a YAML gate violation:

```yaml
violation_type: "missing_artifact"
blocking_gate: "Phase 3 → Phase 4"
missing_artifacts:
  - name: "{artifact}"
    expected_path: "{path}"
    producing_agent: "{agent}"
remediation: "Run /{agent} to generate the missing artifact"
```

Do **not** proceed past this section until all prerequisites are confirmed present.

## Constitutional Authority

**Law II — Anti-Lazy Analysis** (Rules 2.1, 2.4)

- **Rule 2.1**: Every metric must answer a specific business question. No vanity metrics, no dashboards without purpose.
- **Rule 2.4**: Data without interpretation is noise. Every data point must be accompanied by a hypothesis about what it means and what action it implies.

## Execution

> **Tool Availability**: This agent declares GitHub MCP server tools (`issue_read`, `pull_request_read`, `search_issues`). These are built into the Copilot CLI environment. If running in an environment without GitHub MCP tools, fall back to the `gh` CLI tool via bash (e.g., `gh issue list`, `gh pr list`, `gh api`). The quality expectation is the same regardless of tool used.

1. **CHECK** for real-world data sources:
   - Cloudflare analytics logs
   - GitHub API usage data — use the **GitHub MCP server** tools (`issue_read`, `pull_request_read`, `search_issues`) to query issue activity, PR merge rates, and contributor patterns directly from the repository. If GitHub MCP tools are unavailable, use the `gh` CLI (e.g., `gh issue list --json`, `gh pr list --json`, `gh api`) to retrieve the same data.
   - Application-level analytics (events, page views, user flows)
   - Error tracking logs
2. **IF logs exist**, consume them and produce:
   - Key metrics with trends (daily/weekly)
   - Funnel analysis from acquisition to retention
   - Drop-off points with magnitude (what percentage of users leave at each step?)
   - Anomaly detection (unexpected spikes or drops)
3. **IF no logs exist yet**, define the analytics framework:
   - What events to track (with exact event names and payloads)
   - What tools to use (analytics platform, error tracking, performance monitoring)
   - What dashboards to build (with specific metrics per dashboard)
   - What alerting thresholds to set

   **Event Schema Template** — Every tracked event MUST follow this structure:

   | Field | Description | Example |
   |-------|-------------|---------|
   | `event_name` | Snake_case action identifier | `user_signup_completed` |
   | `trigger` | Exact user action or system condition | "User clicks 'Create Account' and backend returns 201" |
   | `payload` | Structured data fields captured | `{ user_id: string, method: "email" \| "oauth", referral_source: string, timestamp: ISO8601 }` |
   | `business_question` | The question this event answers | "How many users complete signup, and through which channel?" |
   | `dashboard` | Which dashboard this feeds | "Acquisition Funnel" |

   **Example events** (adapt to your project):
   - `app_opened` → payload: `{ session_id, platform, is_offline, app_version }` → answers: "What is daily active usage?"
   - `feature_engaged` → payload: `{ feature_id, duration_ms, completion: bool }` → answers: "Which features retain users?"
   - `error_occurred` → payload: `{ error_code, stack_trace_hash, user_action_before }` → answers: "Where are users hitting errors?"
   - `search_performed` → payload: `{ query, results_count, result_clicked: bool }` → answers: "Is search effective?"
4. **MAP every metric to a business question**:
   - For each KPI, state the business question it answers (e.g., "Are users completing onboarding?" → onboarding completion rate)
   - If a metric doesn't answer a business question, remove it
5. **IDENTIFY drop-off points** and hypothesize reasons:
   - For each significant drop-off, propose at least one hypothesis
   - Rank hypotheses by testability (can we run an experiment to validate this?)
   - Reference ICP expectations from `icp.md` where relevant

## Output

Write the following artifact to `.specify/artifacts/phase-4/`:

- **Path**: `.specify/artifacts/phase-4/analytics-report.md`
- **Required sections**:
  - `## Metrics Framework` — either real metrics from consumed logs or a defined framework of what to track, with exact event names and payloads
  - `## Key Performance Indicators` — each KPI explicitly mapped to a business question it answers
  - `## Drop-off Analysis` — identified drop-off points with magnitude, hypotheses for causes, and testability ranking
  - `## Hypotheses` — ranked list of hypotheses about user behavior, each with a proposed experiment to validate or invalidate it

## Gate Criteria

- [ ] Every KPI maps to a specific, stated business question
- [ ] Drop-off analysis is present (either from real data or as a framework for future analysis)
- [ ] No vanity metrics are included without a business question mapping
- [ ] If real logs were consumed, metrics include trends and anomaly observations
- [ ] If no logs exist, the framework defines exact event names, tools, and dashboards
- [ ] All four required sections are present in the output artifact

### Citation Verification

- [ ] Every data-backed claim is tagged `[Verified]` or `[Model-sourced]`
  - **`[Verified]`** — Retrieved from a tool (GitHub MCP, `gh` CLI, analytics API) during this run. Data source and query date known.
  - **`[Model-sourced]`** — From model training knowledge (e.g., industry benchmarks not retrieved live). May be outdated.
- [ ] If no live data sources were available and the output is a framework definition, tag the entire artifact as `[Framework — no live data]` and skip the 50% threshold rule
- [ ] Otherwise, if more than 50% of claims are `[Model-sourced]`, a `## Verification Needed` section MUST list unverified claims for manual review


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
