---
description: "Plan-execute-evaluate orchestrator with temporal undo — determines which agent to run next, checkpoints every cycle, supports rewind/replay/branch"
handoffs:
  - label: "Phase 0: Sketch an idea"
    agent: "os.sketch"
    prompt: "Generate a lightweight hypothesis sketch. Context: $ARGUMENTS"
  - label: "Phase 0: Discover existing codebase"
    agent: "os.discover"
    prompt: "Scan the codebase and produce architecture profile + improvement opportunities. Target: $ARGUMENTS"
  - label: "Phase 0: Route complex input"
    agent: "speckit.route"
    prompt: "Route this raw input to the right speckit agents with tailored prompts for each"
  - label: "Phase 1: Market Analysis"
    agent: "os.market"
    prompt: "Run market analysis. Context: $ARGUMENTS"
  - label: "Phase 1: Competitive Intelligence"
    agent: "os.competition"
    prompt: "Run competitive analysis using available Phase 1 artifacts"
  - label: "Phase 1: USP & Positioning"
    agent: "os.usp"
    prompt: "Distill USP using available Phase 1 artifacts"
  - label: "Phase 1: Feasibility"
    agent: "os.feasibility"
    prompt: "Assess technical, legal, and financial feasibility"
  - label: "Phase 1: Growth Validation"
    agent: "os.validate"
    prompt: "Define MVP and growth validation strategy"
  - label: "Phase 1: Brutal Critic"
    agent: "os.critic"
    prompt: "Run adversarial critique of all available Phase 1 artifacts"
  - label: "Phase 1: Tradeoff Prioritization"
    agent: "os.tradeoff"
    prompt: "Prioritize features and determine what NOT to build"
  - label: "Phase 1: Moat & Defensibility"
    agent: "os.moat"
    prompt: "Define long-term competitive moat"
  - label: "Phase 2: Persona Simulation"
    agent: "os.persona"
    prompt: "Simulate target user interactions and stress-test UX assumptions"
  - label: "Phase 2: UX Strategy"
    agent: "os.ux-strategy"
    prompt: "Define core interaction model and user flows"
  - label: "Phase 2: Design Tokens"
    agent: "os.design"
    prompt: "Establish design tokens and constraints"
  - label: "Phase 3: Audit"
    agent: "os.audit"
    prompt: "Validate pipeline artifact completeness and structural integrity"
  - label: "Phase 3: QA"
    agent: "os.qa"
    prompt: "Run paranoia-level QA on implemented code"
  - label: "Phase 3: Safety"
    agent: "os.safety"
    prompt: "Generate hallucination mitigation scoring and trust report"
  - label: "Phase 3: Cross-Examine"
    agent: "os.cross-examine"
    prompt: "Run full-corpus cross-examination to detect broken decision chains"
  - label: "Phase 3: Revise"
    agent: "os.revise"
    prompt: "Apply revision findings to fix broken decision chains"
  - label: "Phase 3: Specify"
    agent: "speckit.specify"
    prompt: "Write feature spec using all available artifacts"
  - label: "Phase 3: Plan"
    agent: "speckit.plan"
    prompt: "Create implementation plan"
  - label: "Phase 3: Tasks"
    agent: "speckit.tasks"
    prompt: "Decompose plan into actionable tasks"
  - label: "Phase 3: Implement"
    agent: "speckit.implement"
    prompt: "Execute tasks and build the feature"
  - label: "Phase 4: Launch"
    agent: "os.launch"
    prompt: "Create launch kit and go-to-market assets"
  - label: "Phase 4: Analytics"
    agent: "os.analytics"
    prompt: "Consume real-world logs and diagnose product health"
  - label: "Phase 4: Simplify"
    agent: "os.simplify"
    prompt: "Identify what to cut based on usage data"
  - label: "Phase 4: Iterate"
    agent: "os.iterate"
    prompt: "Synthesize learnings into next iteration brief"
---

## User Input

$ARGUMENTS — a goal string describing what you want to achieve.

### Forward mode (default)
- `"validate this idea"` — run Phase 1 agents until the idea is validated or killed
- `"get to working MVP"` — run the full pipeline from current state to shipped code
- `"ship iteration 2"` — run from current analytics through next implementation cycle
- `"fill gaps in Phase 1"` — identify and run whichever Phase 1 agents are missing or stale

### Undo mode (triggered by undo/rewind/replay keywords)
- `"undo last"` — revert to the checkpoint before the most recent cycle
- `"undo to cycle N"` — revert to the state at the end of cycle N
- `"undo os.agent_name"` — revert to the checkpoint before that agent ran
- `"replay os.agent_name"` — undo that agent's cycle, then immediately re-run it
- `"replay os.agent_name with twist: ..."` — undo + re-run with a modified directive injected as additional context
- `"branch from cycle N as branch_name"` — fork a new pipeline branch from a checkpoint without destroying the current branch

### Mode detection

Parse the first token of $ARGUMENTS:
- Starts with `undo` or `rewind` → **Undo mode** (Phase U)
- Starts with `replay` → **Undo mode + immediate re-execution** (Phase U → Phase B → Phase C)
- Starts with `branch` → **Branch mode** (Phase B₀)
- Anything else → **Forward mode** (Phase A → Phase B → Phase C)

## Prerequisites

None. The orchestrator reads current artifact state and determines what to run.

## Purpose

`os.loop` is the **Loop Mode orchestrator** for my-kit. Instead of manually invoking agents one by one in sequence, you provide a goal and the orchestrator determines which agent to run next based on the current state of artifacts.

This implements a **Plan → Checkpoint → Execute → Evaluate** cycle with **temporal undo**. Every cycle creates a checkpoint before execution, enabling precise rewind to any prior state. Undo cascades through the dependency graph — reverting cycle 2 automatically invalidates cycles 3, 4, ... that depended on its outputs.

> **Honest framing:** The current execution model is one-shot-per-invocation.
> Each time you invoke `os.loop`, it plans → selects → checkpoints → hands off to one agent.
> You review the result and invoke `os.loop` again for the next step.
> Undo is also one-shot: invoke with `"undo ..."` to rewind, then invoke again to resume forward.
> This is deliberate — human-in-the-loop review between steps is a feature, not a limitation.

---

## Forward Mode: Plan → Checkpoint → Execute → Evaluate

### Phase A — Plan (assess current state)

1. **Read artifact state.** Check every artifact path registered in `.specify/extensions.yml`:
   - Does the artifact exist?
   - If it has `artifact_meta` frontmatter, read its `confidence`, `grade`, and `revision`
   - If it exists but has no metadata, treat as `confidence: 0.7, grade: "draft"`

2. **Read the goal.** Parse `$ARGUMENTS` to understand what the user wants to achieve.

3. **Read loop state.** If `.specify/artifacts/loop-state.md` exists, read it to understand:
   - What has been executed in previous cycles
   - Any failed attempts or escalations
   - The current phase of progress toward the goal
   - **Checkpoint history** — what snapshots exist for potential undo
   - **Active branch** — which branch is currently being extended

4. **Build the gap map.** For each agent in the pipeline, determine:
   - Are its output artifacts present? At what confidence/grade?
   - Are its input artifacts present? (Affects expected output quality)
   - Is any output artifact stale? (Input revision > output revision)

### Phase B — Select (choose highest-value next action)

Apply this priority ranking to select the single most valuable agent to invoke next:

#### Brownfield Detection

Before applying the standard priority ranking, check for brownfield context:

**Brownfield mode is active when:**
- `.specify/artifacts/codebase-profile.md` exists (produced by `os.discover`)

The presence of a codebase profile is the definitive brownfield signal — it means `os.discover` has been run against an existing codebase. Phase 1 research artifacts may or may not exist alongside it.

**When brownfield mode is active, modify the selection rules:**
- **Skip Phase 1 research agents** (`os.market`, `os.competition`, `os.usp`, `os.moat`, `os.validate`) — these answer "should we build this?" which is already answered for existing products.
- **Keep Phase 1 risk agents** (`os.feasibility`, `os.critic`, `os.tradeoff`) — these are valuable for brownfield because they assess technical risk and prioritization.
- **Route to speckit for feature work** — if the goal is a bug fix or feature request, route directly to `speckit.specify` with the codebase profile as context.
- **Route to risk agents for architecture changes** — if the goal involves refactoring, merging services, or major changes, route to `os.feasibility` first.
- **Use improvement-opportunities.md as backlog** — if no specific goal is provided, read `improvement-opportunities.md` and recommend the highest-priority opportunity.

#### Standard Priority Ranking

1. **Goal-critical gaps first.** If the goal is "validate this idea" and no Phase 1 artifacts exist, Phase 1 agents outrank everything else. In brownfield mode, the goal maps to speckit or risk agents instead.

2. **Stale high-confidence artifacts.** An artifact that was `validated` but whose inputs have changed is higher priority than a missing low-value artifact.

3. **Dependency chain order.** Among equally valuable gaps, prefer agents whose inputs are already available. Don't invoke `os.usp` if `competitor-matrix.md` doesn't exist yet — invoke `os.competition` first.

4. **Hard-gate prerequisites.** `os.critic` has `gate: hard`. Before invoking it, check that all of its required input artifacts exist and that their **minimum confidence ≥ 0.7** (i.e., the lowest `confidence` value among its inputs must be at least 0.7). If any input is missing or below 0.7, fill that input first.

5. **Diminishing returns.** If an agent has been retried 3+ times without quality improvement (tracked in loop state), escalate to human rather than retrying.

6. **Revision cycle trigger.** If `os.cross-examine` has been run and produced findings with severity `critical` or `high`, recommend `os.revise` before proceeding to implementation. After `os.revise` completes, recommend re-running `os.cross-examine` to verify. This creates a convergent loop: cross-examine → revise → cross-examine until the corpus is clean.

7. **Pre-implementation gate.** Before recommending `speckit.implement`, check whether `cross-examination-report.md` exists and is clean (no critical/high findings). If it doesn't exist, recommend `os.cross-examine` first. If it exists but has unresolved critical/high findings, recommend `os.revise`.

### Phase C₀ — Checkpoint (snapshot before execution)

Before handing off to the selected agent, create a checkpoint. This is the temporal anchor that enables undo.

1. **Snapshot artifact state.** For every artifact that currently exists, record:
   - File path
   - `confidence`, `grade`, `revision` from frontmatter
   - Content hash (first 8 chars of SHA-256) for tamper detection

2. **Write checkpoint file.** Create `.specify/artifacts/checkpoints/cycle-{N}-pre.md`:

   ```markdown
   ---
   checkpoint_meta:
     cycle: {N}
     created_at: "<ISO 8601>"
     agent_about_to_run: "{agent_name}"
     branch: "{branch_name or 'main'}"
     parent_checkpoint: "cycle-{N-1}-pre"
   ---

   # Checkpoint: Cycle {N} (pre-execution)

   ## Artifact Snapshot

   | Artifact | Confidence | Grade | Revision | Hash |
   |----------|-----------|-------|----------|------|
   | sketch.md | 0.3 | hypothesis | 1 | a1b2c3d4 |
   | market-map.md | 0.55 | draft | 1 | e5f6g7h8 |
   {... one row per existing artifact}

   ## About to Execute
   - **Agent:** {agent_name}
   - **Expected outputs:** {artifact_list}
   - **Selection rationale:** {brief reason}
   ```

3. **Update loop-state.md** — add checkpoint to the Checkpoints table.

### Phase C — Execute & Evaluate

After checkpointing, hand off to the selected agent. On the next invocation, evaluate:

1. **Did the agent produce output?** Check if the expected artifact file exists.
2. **Quality scores.** If the output includes a `## Quality Score` section, read the self-evaluation scores. Any score < 3 indicates the agent recognized a weakness.
3. **Confidence.** Read the `artifact_meta.confidence` value. If < 0.5, flag as hypothesis-grade.
4. **Update loop state.** Record what was run, what was produced, and the quality assessment.
5. **Record checkpoint completion.** Note the post-execution state for this cycle.

---

## Undo Mode: Rewind → Cascade Invalidate → (optional) Replay

Triggered when `$ARGUMENTS` starts with `undo`, `rewind`, or `replay`.

### Phase U₁ — Parse Undo Target

Determine what to undo:

| Command | Target Cycle | Action |
|---------|-------------|--------|
| `undo last` | Current − 1 | Revert to previous checkpoint |
| `undo to cycle N` | N | Revert to end-of-cycle-N state |
| `undo os.{agent}` | Cycle where agent ran | Revert to pre-{agent} checkpoint |
| `replay os.{agent}` | Cycle where agent ran | Revert + immediately re-run |
| `replay os.{agent} with twist: X` | Cycle where agent ran | Revert + re-run with X injected |

If the target checkpoint doesn't exist, **escalate** — don't guess.

### Phase U₂ — Cascade Invalidation

Walking forward from the undo target, identify all artifacts that must be invalidated:

1. **Direct outputs.** Every artifact produced by cycles AFTER the target cycle.
2. **Dependency cascade.** Read the dependency graph from `.specify/extensions.yml`. Any artifact whose prerequisite artifacts include an invalidated artifact is also invalidated, even if produced by an earlier cycle that isn't being undone.
3. **Archive, don't delete.** Move invalidated artifacts to `.specify/artifacts/.archive/undo-{undo_number}/`:
   - Preserves history for comparison or re-adoption
   - Archive directory includes a manifest: `manifest.md` listing what was archived and why

   ```markdown
   # Undo Archive: U{N}

   ## Reason
   {parsed from $ARGUMENTS}

   ## Rewound To
   Cycle {target_cycle} — checkpoint `cycle-{target_cycle}-pre.md`

   ## Archived Artifacts
   | Artifact | Was At | Produced By Cycle | Reason Invalidated |
   |----------|--------|------------------|-------------------|
   | usp.md | confidence 0.75, revision 1 | 4 (os.usp) | downstream of target |
   | competitor-matrix.md | confidence 0.60, revision 1 | 3 (os.competition) | downstream of target |
   ```

4. **Update loop-state.md.**
   - Remove invalidated cycles from Execution History
   - Add entry to Undo History table
   - Recalculate Current Artifact State table from surviving artifacts
   - Set `current_cycle` back to target cycle
   - Decrement loop-state `revision`

### Phase U₃ — Replay (if requested)

If the command was `replay` (not just `undo`):

1. **Re-enter Forward Mode** at the target cycle.
2. **If `with twist: X`**, inject X as additional context in the agent's handoff prompt:
   - Append to the standard prompt: `"\n\nADDITIONAL DIRECTIVE (from loop-undo replay): {X}"`
   - Record the twist in the execution history for audit trail
3. **Execute Phase B → C₀ → C** as normal — the replayed agent gets a fresh checkpoint and clean evaluation.

---

## Branch Mode: Fork pipeline from checkpoint

Triggered when `$ARGUMENTS` starts with `branch`.

### Phase B₀ — Fork

1. **Parse:** `branch from cycle N as {branch_name}`
2. **Validate:** Checkpoint for cycle N must exist.
3. **Create branch state.** Copy the checkpoint's artifact snapshot as the branch's starting state. Unlike undo, branching does **not** archive or remove any artifacts from the current branch.
4. **Update loop-state.md.** Add branch to Active Branches table. Set `active_branch` to the new branch name.
5. **Forward mode resumes** on the new branch.

To switch branches: `"switch to branch {name}"`. To merge insights from a branch back to main: `"merge branch {name}"` (escalates to human for conflict resolution on any artifact modified by both branches).

---

## Output

After every invocation (forward, undo, or branch), update `.specify/artifacts/loop-state.md`:

```markdown
---
artifact_meta:
  produced_by: "os.loop"
  produced_at: "<ISO 8601 timestamp>"
  confidence: <float based on overall progress toward goal>
  inputs_used: []
  stale_after: "on_input_change"
  revision: <increment on every update>
  quality_scores:
    specificity: null
    actionability: null
    non_redundancy: null
    evidence_quality: null
  grade: "draft"
---

# Loop State

## Goal
{The user's stated goal from $ARGUMENTS}

## Active Branch
{branch_name or "main"}

## Brownfield Detection
- `codebase-profile.md` exists: **✅/❌**
- **Mode:** Greenfield / Brownfield

## Current Artifact State

| Artifact | Exists | Confidence | Grade | Stale | Revision |
|----------|--------|------------|-------|-------|----------|
| sketch.md | ✅/❌ | 0.0–1.0 | hypothesis/draft/validated | yes/no | N |
| codebase-profile.md | ✅/❌ | 0.0–1.0 | hypothesis/draft/validated | yes/no | N |
| improvement-opportunities.md | ✅/❌ | 0.0–1.0 | hypothesis/draft/validated | yes/no | N |
| market-map.md | ... | ... | ... | ... | ... |
| cross-examination-report.md | ✅/❌ | 0.0–1.0 | hypothesis/draft/validated | yes/no | N |
| revision-plan.md | ✅/❌ | 0.0–1.0 | hypothesis/draft/validated | yes/no | N |
| revision-changelog.md | ✅/❌ | 0.0–1.0 | hypothesis/draft/validated | yes/no | N |
{... one row per registered artifact}

## Execution History

| Cycle | Agent Invoked | Artifact Produced | Confidence | Quality Issues |
|-------|---------------|-------------------|------------|----------------|
| 1 | os.sketch | sketch.md | 0.3 | — |
| 2 | os.market | market-map.md, icp.md, why-now.md | 0.7 | — |
{... one row per completed cycle}

## Checkpoints

| Cycle | Agent | Timestamp | Branch | Artifacts at Checkpoint |
|-------|-------|-----------|--------|------------------------|
| 1 | os.sketch | 2025-07-26T12:00:00Z | main | sketch.md (0.3 r1) |
| 2 | os.market | 2025-07-27T02:30:00Z | main | +market-map.md, +icp.md, +why-now.md |
{... one row per checkpoint}

## Undo History

| Undo | From Cycle | To Cycle | Reason | Artifacts Archived | Replay |
|------|-----------|----------|--------|-------------------|--------|
{empty if no undos have been performed}

## Active Branches

| Branch | Forked From | Current Cycle | Status |
|--------|------------|---------------|--------|
| main | — | {N} | active |
{... additional branches if any}

## Next Recommended Action

**Agent:** {selected agent name}
**Reason:** {why this agent is highest priority given the current state and goal}
**Expected output:** {artifact names}
**Risk:** {what might go wrong — e.g., missing inputs that will lower confidence}

## Escalations

{Any agents that have failed 3+ times, or situations requiring human judgment}
```

Then **hand off to the selected agent** via the appropriate handoff label.

## Escalation Rules

Surface to the human operator when:
1. An agent has been retried 3 times on the same cycle without quality improvement
2. The goal requires a decision the orchestrator cannot make (e.g., "should we pivot?")
3. `os.critic` produces a critical finding with no mitigation path
4. Overall progress toward the goal has stalled for 3+ cycles
5. **Undo target checkpoint not found** — the requested cycle has no checkpoint file
6. **Branch merge conflict** — two branches modified the same artifact differently
7. **Large cascade** — undo would invalidate 5+ artifacts; confirm with human before proceeding

When escalating, state clearly:
- What was attempted
- Why it failed or stalled
- What human input is needed to proceed

## Validation Checklist

- [ ] Loop state file is updated with current artifact state
- [ ] Gap map covers all registered agents
- [ ] Selection rationale is documented in "Next Recommended Action"
- [ ] Execution history captures all previous cycles
- [ ] Escalation conditions are checked before proceeding
- [ ] Hard-gate agents (os.critic) are not invoked without sufficient inputs
- [ ] Brownfield detection is checked (codebase-profile.md existence)
- [ ] **Checkpoint created before every agent execution** (Phase C₀)
- [ ] **Undo history is append-only** — never delete undo records
- [ ] **Archived artifacts preserved in `.archive/` directory** — never hard-delete
- [ ] **Cascade invalidation follows dependency graph from extensions.yml**
