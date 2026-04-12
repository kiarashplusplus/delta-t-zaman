---
description: "Lightweight idea-to-build sketch — bypasses full Phase 1 for 0→1 speed"
handoffs:
  - label: "Start the loop: plan-execute-evaluate"
    agent: "os.loop"
    prompt: "Run the plan-execute-evaluate cycle with goal: 'get to working MVP' starting from the sketch just produced"
  - label: "Jump straight to spec"
    agent: "speckit.specify"
    prompt: "Write a feature spec using the sketch artifact at .specify/artifacts/sketch.md as primary input. Phase 1 artifacts may not exist yet — use the sketch as the source of truth and flag assumptions."
  - label: "Route complex input to agents"
    agent: "speckit.route"
    prompt: "Route this input to the right speckit agents. The sketch has been produced — determine the best next steps."
    send: true
---

## User Input

$ARGUMENTS — a raw idea in 1–3 sentences. This is the only required input.

## Prerequisites

None. This agent is the zero-prerequisite entry point for 0→1 workflows.

> ⚠️ **Hypothesis-grade output.** The sketch is explicitly marked as unvalidated.
> Phase 1 agents (`os.market`, `os.competition`, `os.usp`, etc.) should be run
> afterward — either manually or via `os.loop` — to validate and replace sketch
> assumptions with evidence.

## Execution

### Step 1 — Extract the core idea

Parse `$ARGUMENTS` to extract:
- The core value proposition (what problem does this solve, for whom?)
- Any implied constraints (platform, technology, audience, geography)
- Any stated features or requirements

If the idea is very vague (< 1 sentence of signal), ask one clarifying question before proceeding.

### Step 2 — Generate the USP hypothesis

Write one falsifiable sentence that describes the unique value in the format:

> "For [target user], [product name] is the only [category] that [differentiating claim] because [evidence or reasoning]."

Mark this explicitly as **hypothesis-grade** — it has not been validated by `os.usp` or competitive analysis.

### Step 3 — Identify core features (3–5)

List 3–5 features that are strictly necessary for the USP to be true. Apply ruthless scope reduction:
- Exclude anything that does not directly enable the core value proposition
- For each feature, state: what it does, why it's required for the USP, and what the simplest possible implementation looks like

### Step 4 — Surface known risks

Without doing full research, surface the most obvious risks in these categories:
- **Market risk**: Is there evidence this market exists? Are there obvious competitors?
- **Technical risk**: Any hard technical constraints or blockers you can identify?
- **Legal/licensing risk**: Any obvious legal, IP, or regulatory concerns?
- **Distribution risk**: How will the first users find this?

Flag each risk as `[KNOWN]` (you have evidence) or `[ASSUMED]` (you're inferring). Assumed risks are Phase 1 research targets.

### Step 5 — Define target user persona (lightweight)

Write a 2–3 sentence sketch of the target user:
- Who they are (role, context, situation)
- What problem they have right now
- How they currently solve it (the status quo you're displacing)

Mark as **hypothesis-grade**.

### Step 6 — Define success criteria

Write 2–3 measurable success criteria that would confirm the USP hypothesis is correct. These become the validation targets for Phase 1 agents and the loop runner.

Example format: "If X% of [target users] do [specific action] within [timeframe], the USP hypothesis is confirmed."

## Output

Write the following artifact to `.specify/artifacts/sketch.md`:

```markdown
---
artifact_meta:
  produced_by: "os.sketch"
  produced_at: "<ISO 8601 timestamp>"
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

# Product Sketch: {Product Name or Working Title}

> **Grade: HYPOTHESIS** — This sketch has not been validated by Phase 1 research agents.
> All claims marked `[ASSUMED]` are research targets, not facts.

## USP Hypothesis

{One falsifiable sentence in "For [user], [product] is the only [category] that [claim] because [reasoning]" format}

## Core Features

| # | Feature | Why Required for USP | Simplest Implementation |
|---|---------|---------------------|------------------------|
| 1 | ... | ... | ... |
| 2 | ... | ... | ... |
| 3 | ... | ... | ... |

## Known Risks

| Category | Risk | Grade | Notes |
|----------|------|-------|-------|
| Market | ... | [KNOWN] or [ASSUMED] | ... |
| Technical | ... | [KNOWN] or [ASSUMED] | ... |
| Legal | ... | [KNOWN] or [ASSUMED] | ... |
| Distribution | ... | [KNOWN] or [ASSUMED] | ... |

## Target User

{2–3 sentence persona sketch}

## Success Criteria

1. {Measurable criterion with target metric and timeframe}
2. {Measurable criterion with target metric and timeframe}

## Phase 1 Research Targets

{List of [ASSUMED] items from the risks and claims above that Phase 1 agents should validate}
```

## Validation Checklist

- [ ] USP hypothesis is a single falsifiable sentence
- [ ] Core features are ≤ 5 and each justifies its existence via the USP
- [ ] Every risk is tagged `[KNOWN]` or `[ASSUMED]`
- [ ] Success criteria are measurable (include numbers and timeframes)
- [ ] Grade is set to `hypothesis` and confidence to 0.3
- [ ] Phase 1 research targets are listed for every `[ASSUMED]` item
