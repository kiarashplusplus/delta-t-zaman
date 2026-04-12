---
description: "Define the UX strategy and interaction model before any UI is designed"
handoffs:
  - label: "Next: UI/UX Design"
    agent: "os.design"
    prompt: "Create design constraints and tokens from the UX strategy"
---

## User Input

$ARGUMENTS

## Prerequisites

| # | Artifact | Path | Producing Agent |
|---|----------|------|-----------------|
| 1 | Market Map | `.specify/artifacts/phase-1/market-map.md` | os.market |
| 2 | ICP | `.specify/artifacts/phase-1/icp.md` | os.market |
| 3 | Why Now | `.specify/artifacts/phase-1/why-now.md` | os.market |
| 4 | Competitor Matrix | `.specify/artifacts/phase-1/competitor-matrix.md` | os.competitions |
| 5 | UX Teardown | `.specify/artifacts/phase-1/ux-teardown.md` | os.competition |
| 6 | Steal/Differentiate/Ignore | `.specify/artifacts/phase-1/steal-differentiate-ignore.md` | os.steal |
| 7 | USP | `.specify/artifacts/phase-1/usp.md` | os.usp |
| 8 | Feasibility | `.specify/artifacts/phase-1/feasibility.md` | os.feasibility |
| 9 | Growth Plan | `.specify/artifacts/phase-1/growth-plan.md` | os.validate |
| 10 | Brutal Critique | `.specify/artifacts/phase-1/brutal-critique.md` | os.critique |
| 11 | Tradeoff Matrix | `.specify/artifacts/phase-1/tradeoff-matrix.md` | os.tradeoffs |
| 12 | Moat | `.specify/artifacts/phase-1/moat.md` | os.moat |
| 13 | Persona Simulation | `.specify/artifacts/phase-2/persona-sim.md` | os.persona |

**Gate Check**: Run `bash .specify/scripts/bash/pipeline-audit.sh --phase 1 --json` and verify ALL 12 Phase 1 artifacts are present. Then verify `persona-sim.md` exists in `.specify/artifacts/phase-2/`. If any artifact is missing, emit a gate violation and halt:

```yaml
violation_type: "missing_artifact"
blocking_gate: "Phase 1 → Phase 2"
missing_artifacts:
  - name: "{artifact}"
    expected_path: "{path}"
    producing_agent: "{agent}"
remediation: "Run /{agent} to generate the missing artifact"
```

## Constitutional Authority

- **Law V** (UX as Product Strategy) — Rules 5.1, 5.2, 5.3, 5.4

UX strategy defines the behavioral architecture of the product. NO visual design decisions are made at this stage — only interaction patterns, flows, and cognitive constraints.

## Cognitive Load Reference Framework

Use these research-backed principles to **derive** the cognitive load numbers in step 3 of Execution. Do not pick numbers arbitrarily — start from the baseline, then adjust with a documented justification tied to the ICP and product context.

### 1. Miller's Law (7±2)

Working memory holds 5–9 chunks of information simultaneously.

- **Application**: Max menu items, max options in a picker, max visible list items before pagination.
- **Mobile adjustment**: Use 5±1 — smaller screens and divided attention reduce effective working memory.
- **Default ceiling**: 7 items for desktop, 5 items for mobile.

### 2. Hick's Law

Decision time = a + b × log₂(n), where n = number of equally probable choices.

- **Application**: Every additional choice increases decision time logarithmically.
- **Threshold**: >4 equally-weighted options → use progressive disclosure or smart defaults.
- **Design rule**: Always pre-select the most common choice; make the default path require zero decisions.

### 3. Nielsen's 10-Second Rule

Users form a lasting impression within 10 seconds of first encountering an interface.

- **Application**: Core value proposition must be communicated within 10 seconds of first interaction.
- **Measurement**: Count the steps (and elapsed time) from app launch to first meaningful action.

### 4. Cognitive Load Types (Sweller's Theory)

| Type | Definition | Design Responsibility |
|------|------------|-----------------------|
| **Intrinsic** | Complexity inherent to the task | Cannot reduce, but CAN scaffold (e.g., break into sub-steps) |
| **Extraneous** | Complexity from poor design | MUST minimize — this is the UX designer's primary job |
| **Germane** | Effort spent learning and building mental models | SHOULD support (e.g., consistent patterns, progressive onboarding) |

### 5. Derivation Table

Populate this table for the specific product. Start from the baseline, then adjust and justify.

| Constraint | Baseline | Adjusted Value | Justification |
|-----------|----------|---------------|---------------|
| Max choices per screen | 7 (Miller) | ? | Adjust for ICP tech literacy |
| Max steps to core action | 3 (Nielsen) | ? | Adjust for task complexity |
| Max info density (items/viewport) | 5 mobile / 7 desktop | ? | Adjust for content type |
| Max form fields per step | 5 (Miller mobile) | ? | Adjust for data requirements |
| Decision timeout (seconds) | 10 (Nielsen) | ? | Adjust for user context |

The agent MUST fill the "Adjusted Value" and "Justification" columns based on the specific product and ICP, citing the baseline law and the reason for any adjustment.

## Execution

1. **Define core interaction model**: Based on persona simulations and ICP, determine the fundamental interaction paradigm (e.g., feed-based, search-first, guided flow, canvas). Justify with persona data.

2. **Map primary user flows**: For each core task identified in persona walkthroughs:
   - **Happy path** — optimal sequence from intent to completion
   - **Error paths** — what happens at each step when things go wrong (invalid input, server error, permission denied, timeout)
   - **Edge cases** — empty states, first-time use, power-user shortcuts

3. **Set cognitive load constraints**: Define explicit, measurable limits:
   - Maximum choices per screen (number)
   - Maximum steps per core task (number)
   - Maximum simultaneous information density (items visible)
   - Reading level target (grade level)

   **Reference baselines** — Adapt these research-backed defaults to the specific product and audience. Cite the baseline and justify any deviation:
   - **Miller's Law (7±2)**: Working memory holds 5–9 items. Default max choices per screen: **7**. For mobile, reduce to **5** (smaller viewport, divided attention).
   - **Hick's Law**: Decision time increases logarithmically with choices. For time-critical tasks (e.g., search results, navigation), aim for **≤4 options** to keep decision time under 1 second.
   - **Nielsen's 3-click rule**: Core task completion should require **≤3 steps** from entry point. For complex workflows, allow up to **5 steps** with clear progress indicators.
   - **Flesch-Kincaid readability**: Target **grade 6–8** for consumer apps, **grade 10–12** for professional tools. Measure with standard readability tools.
   - **Information density (Tufte)**: Maximum **3 information groups** visible simultaneously. Each group should contain ≤5 items. For data-heavy screens, use progressive disclosure.
   - **Touch target sizing (WCAG 2.2)**: Minimum **44×44 CSS pixels** for touch targets. Minimum **24×24 CSS pixels** for pointer-only targets.

4. **Define information hierarchy**: Establish what information is primary, secondary, and tertiary on each key screen. Specify how hierarchy shifts based on user context and task.

5. **Specify offline-first UX patterns**:
   - Stale data indicators (visual treatment for data of unknown freshness)
   - Sync status communication (how users know what's synced vs. pending)
   - Offline affordances (what users CAN do offline, made explicit)
   - Conflict resolution UX (how sync conflicts are surfaced to users)

6. **Cross-reference**: Every flow decision MUST cite persona simulation findings from `persona-sim.md`. No flow can be defined without persona-grounded justification.

7. **Monetization cross-check**: Read `growth-plan.md` **§ Monetization Decision** section.
   - If the UX strategy implies a monetization model (e.g., gating features, premium content, trial periods), it MUST be consistent with the frozen monetization decision in growth-plan.md.
   - If a conflict exists, **do not silently override**. Instead, emit a HALT block:
     ```yaml
     conflict_type: "monetization_inconsistency"
     ux_strategy_implies: "<what the UX flow assumes>"
     growth_plan_states: "<what growth-plan.md § Monetization Decision says>"
     recommendation: "<which should be authoritative and why>"
     ```
   - If no conflict exists, add a one-line confirmation: `Monetization model consistent with growth-plan.md § Monetization Decision.`

8. **Content-scope constraints**: If any upstream artifact defines content boundaries (e.g., "50 translated poems", "200 verified texts", "English translations available for N items"), document a `## Content Scope Constraints` section in the output:
   - List every content boundary found in upstream artifacts with its source
   - For each user flow that surfaces content to the user, verify the flow is achievable within the stated content constraints
   - If a flow promises content that exceeds available scope (e.g., "browse all English translations" when only 50 exist), flag it:
     ```
     ⚠️ CONTENT SCOPE WARNING: Flow "<flow name>" assumes <N> items but <artifact> constrains to <M>.
     Recommendation: <constrain the flow | expand content scope | make flow conditional>
     ```

## Output

Write the following artifact to `.specify/artifacts/phase-2/`:

### `ux-strategy.md`

Required sections:

- `## Core Interaction Model` — The fundamental interaction paradigm with justification from persona data.
- `## User Flows` — All primary flows with happy paths, error paths, and edge cases. Each flow cites the persona scenario it addresses.
- `## Cognitive Load Constraints` — Explicit numerical limits for choices, steps, information density, and reading level.
- `## Offline UX Patterns` — Stale data indicators, sync status, offline affordances, and conflict resolution patterns.
- `## Information Hierarchy` — Primary/secondary/tertiary information mapping for key screens.

## Gate Criteria

- [ ] Cognitive load constraints are explicit numbers (not vague terms like "minimal" or "simple")
- [ ] All primary user flows include both happy path and error paths
- [ ] Offline UX patterns are fully defined (stale data, sync status, affordances, conflict resolution)
- [ ] Every flow decision cites a persona simulation finding
- [ ] No visual design decisions are present (colors, fonts, layouts) — behavioral only
- [ ] Monetization assumptions in UX flows are consistent with growth-plan.md § Monetization Decision (or conflict is explicitly HALTed)


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
