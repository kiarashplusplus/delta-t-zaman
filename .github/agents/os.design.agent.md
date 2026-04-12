---
description: "Establish design tokens and constraints that enforce the UX strategy"
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
| 14 | UX Strategy | `.specify/artifacts/phase-2/ux-strategy.md` | os.ux-strategy |

**Gate Check**: Run `bash .specify/scripts/bash/pipeline-audit.sh --phase 1 --json` and verify ALL 12 Phase 1 artifacts are present. Then verify both `persona-sim.md` and `ux-strategy.md` exist in `.specify/artifacts/phase-2/`. If any artifact is missing, emit a gate violation and halt:

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

- **Law V** (UX as Product Strategy) — Rules 5.3, 5.4

Design tokens and constraints MUST be derived from the UX strategy, not invented independently. Every token must trace back to a strategic requirement.

## Execution

1. **Translate UX strategy into design constraints**: Read `ux-strategy.md` and extract every behavioral requirement that implies a visual or interactive constraint. Map each requirement to a concrete design decision.

2. **Define design tokens**:
   - **Color system** — Primary, secondary, accent, semantic colors (success, warning, error, info). Include dark mode variants. Define color roles, not just values.
   - **Typography** — Type scale (sizes, weights, line heights). Font family selections with fallback stacks. Reading-optimized settings for body text. RTL-compatible font selections.
   - **Spacing** — Base unit and scale (e.g., 4px base with multipliers). Component spacing rules. Page margin/padding standards.

3. **Set accessibility requirements**:
   - Contrast ratios (WCAG AA minimum: 4.5:1 for normal text, 3:1 for large text)
   - Touch target minimums (48×48dp per Material guidelines)
   - Minimum text sizes for body and UI elements
   - Focus indicator specifications
   - Screen reader annotation patterns

4. **Define responsive breakpoints**: Specify breakpoints with layout behavior at each tier. Map breakpoints to cognitive load constraints from UX strategy (e.g., fewer choices on smaller screens).

5. **Specify animation/transition timing**: Define duration scales (micro-interactions, page transitions, loading states). Specify easing curves. Set reduced-motion alternatives.

6. **RTL language considerations**: Critical for Persian poetry app context:
   - Bidirectional text rendering rules
   - Mirrored layout specifications
   - RTL-specific spacing and alignment
   - Mixed-direction content handling (Persian + English)
   - Nastaliq script rendering requirements (if applicable)

7. **Traceability**: Map EVERY design decision back to a specific UX strategy requirement. No orphan tokens — every value must have a justification chain: persona → UX strategy → design token.

## Output

Write the following artifacts to `.specify/artifacts/phase-2/`:

### `design-tokens.md`

Required sections:

- `## Color System` — Full color palette with roles, values, dark mode variants, and semantic meanings.
- `## Typography` — Type scale, font selections, line heights, RTL-compatible choices.
- `## Spacing` — Base unit, scale, component spacing, page-level spacing.
- `## Accessibility` — Contrast ratios, touch targets, text sizes, focus indicators, screen reader patterns.
- `## RTL Support` — Bidirectional rendering, mirrored layouts, mixed-direction content, script-specific requirements.

### `design-constraints.md`

Required sections:

- `## Layout Constraints` — Grid system, responsive behavior, content width limits, breakpoint-specific rules.
- `## Interaction Constraints` — Animation timing, transition specs, reduced-motion alternatives, gesture patterns.
- `## Performance Constraints` — Asset size budgets, rendering performance targets, animation frame rate minimums.
- `## Responsive Breakpoints` — Breakpoint values, layout behavior per tier, cognitive load adjustments per breakpoint.

## Gate Criteria

- [ ] Every design token traces to a UX strategy requirement (no orphan tokens)
- [ ] Accessibility thresholds are explicit (contrast ratios as numbers, touch targets in dp/px)
- [ ] RTL support is fully specified (not deferred or marked as TODO)
- [ ] Responsive breakpoints map to cognitive load constraints from UX strategy
- [ ] Both `design-tokens.md` and `design-constraints.md` are produced


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
