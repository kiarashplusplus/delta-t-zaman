<!--
  Sync Impact Report
  ==================
  Version change: [unpopulated template] → 1.0.0
  Modified principles: N/A (initial population)
  Added sections:
    - 7 Core Principles (I–VII):
        I. Offline-First Architecture
        II. Cross-Platform Parity
        III. Performance Budget
        IV. Data Locality and Privacy
        V. Test Discipline
        VI. Accessibility by Default
        VII. Simplicity and YAGNI
    - Technical Constraints (locked stack decisions)
    - Quality Gates (PR merge requirements)
    - Governance (amendment procedure + compliance rules)
  Removed sections: None
  Templates requiring updates:
    - specs/001-tauri-world-clock/plan.md → ✅ updated
      (Constitution Check populated with 9 rule rows)
    - .specify/templates/plan-template.md → ✅ no changes needed
    - .specify/templates/spec-template.md → ✅ no changes needed
    - .specify/templates/tasks-template.md → ✅ no changes needed
  Follow-up TODOs: None
-->
# Delta-T Zaman Constitution

## Core Principles

### I. Offline-First Architecture

All core functionality MUST operate without network connectivity.
Timezone data, search indices, and user preferences MUST be bundled
or stored locally. No feature MAY depend on a remote service for
its primary operation.

- Timezone metadata (~250 KB IANA JSON) MUST be bundled in the
  app binary.
- `Intl.DateTimeFormat` MUST be the sole runtime timezone rendering
  engine; no server-side lookups.
- Alarm scheduling MUST use the OS-native notification system, not
  a cloud push service.
- The app MUST pass all acceptance scenarios with airplane mode
  enabled on every target platform.

**Rationale**: A clock is infrastructure. Infrastructure that fails
when the network fails is not infrastructure — it is a liability.

### II. Cross-Platform Parity

The app MUST ship on macOS, Windows, Linux, iOS, and Android from
a single shared codebase. Platform-specific behavior MUST be
isolated behind Tauri's plugin abstraction; the frontend MUST NOT
contain platform-conditional branches except for documented,
unavoidable divergences.

- Every Functional Requirement (FR) that applies to "all platforms"
  MUST be verified on at least macOS + one non-Apple platform
  before merge.
- Platform-specific features (system tray, autostart) MUST degrade
  gracefully: the app MUST remain fully functional without them
  and MUST log a warning when a capability is unavailable.
- Mobile-specific constraints (1/min update rate, no tray) MUST be
  documented in-code, not discovered at runtime.

**Rationale**: A "cross-platform" app that works on two platforms
is a lie. Every FR claim implies all five targets unless explicitly
scoped.

### III. Performance Budget

The app MUST meet quantified performance targets. Violations MUST
be treated as bugs, not optimization opportunities for later.

| Metric | Desktop Target | Mobile Target |
|--------|---------------|---------------|
| Cold start | < 2 s | < 3 s |
| Clock tick DOM cost | ≤ 1 ms | ≤ 1 ms |
| Timezone search | < 300 ms | < 300 ms |
| Memory (10 zones) | < 50 MB | < 30 MB |
| Install size | < 15 MB | < 15 MB |
| Background battery | N/A | < 2%/hr |

- Every PR that adds a dependency MUST include its gzipped size
  and justify the cost against the install-size budget.
- Animations MUST NOT cause frame drops below 60 fps on a
  mid-range 2023 device.

**Rationale**: Utility apps compete with the OS clock. If we are
slower or heavier, we lose.

### IV. Data Locality and Privacy

All user data MUST remain on-device. The app MUST NOT transmit,
phone-home, or exfiltrate any user data.

- Persistence MUST use `@tauri-apps/plugin-store` with JSON
  key-value files (`zones.dat`, `alarms.dat`, `preferences.dat`).
- No analytics, telemetry, or crash-reporting SDK MAY be included
  without an explicit constitution amendment.
- No user authentication or account system MAY be introduced
  without an explicit constitution amendment.

**Rationale**: A world clock knows where you are, when you wake up,
and who you talk to across time zones. That data stays local.

### V. Test Discipline

Every merged change MUST be covered by tests at the appropriate
layer. Untested code MUST NOT ship.

- **Unit tests** (Vitest): Pure logic — timezone search, day/night
  calculation, alarm state transitions, formatting.
- **DOM tests** (Vitest + jsdom/happy-dom):
  Component create/update/destroy lifecycle, event handler wiring,
  ARIA attribute correctness.
- **Integration tests** (Vitest): Store plugin interaction, IPC
  command round-trips.
- **E2E tests** (Playwright): Full user flows across at least one
  platform.
- **Rust tests** (cargo test): All Tauri command handlers with
  mock runtime.

Bug fixes MUST include a regression test that fails without the
fix.

**Rationale**: Five platforms × six user stories × nine edge cases
= 270 potential failure points. Tests are the only scalable safety
net.

### VI. Accessibility by Default

Accessibility MUST be a first-class implementation concern, not a
retrofit.

- Every interactive element MUST have a keyboard-accessible
  equivalent (FR-027).
- Every interactive element and time display MUST have a
  screen-reader-compatible label (FR-028).
- Focus order MUST follow logical reading order.
- Color MUST NOT be the sole indicator of state (day/night, alarm
  status).
- Custom drag-and-drop keyboard mode (Space to grab, arrows to
  move, Enter to drop, Escape to cancel) MUST be implemented and
  tested.

**Rationale**: Accessibility is not a feature — it is a quality
attribute. Retrofitting it is 3–5x more expensive than building
it in.

### VII. Simplicity and YAGNI

The app MUST remain a single-user, local-only utility. Complexity
MUST be justified against a concrete Functional Requirement.

- No feature MAY be added without tracing to an existing FR or a
  new FR added through the spec amendment process.
- No abstraction layer MAY be introduced speculatively ("in case
  we need it later").
- Dependencies MUST be minimal: prefer platform APIs and Tauri
  plugins over third-party libraries.
- The frontend MUST NOT exceed ~15 TypeScript component modules and the
  backend MUST NOT exceed ~10 Rust commands (as scoped in
  plan.md).

**Rationale**: Scope creep is the leading cause of death for
utility apps. Every added feature is a maintenance liability
across five platforms.

## Technical Constraints

The following technology decisions are locked for the duration of
this feature branch. Changes require a constitution amendment.

| Layer | Decision | Reference |
|-------|----------|-----------|
| Runtime | Tauri 2.10.3 | R6, R7, R10 |
| Frontend framework | Vanilla TypeScript (no framework) | R1 |
| Styling | Style Dictionary 4.x (design tokens → CSS custom properties) | R2 |
| Timezone data | Bundled IANA JSON + Intl.DateTimeFormat | R3 |
| Timezone search | Built-in substring matching | R4 |
| Day/night | SunCalc (~3 KB) | R5 |
| Notifications | @tauri-apps/plugin-notification 2.3.3 | R6 |
| Persistence | @tauri-apps/plugin-store (JSON KV) | R7 |
| Drag-and-drop | Custom vanilla DnD (HTML5 API + keyboard) | R8 |
| Testing | Vitest + Playwright + cargo test | R9 |

Adding a new dependency MUST be documented in research.md with a
decision record following the R-number format.

## Quality Gates

Every pull request MUST pass the following gates before merge:

1. **Lint**: No warnings from `eslint` and
   `clippy` (Rust).
2. **Type check**: `tsc --noEmit` and `cargo check` MUST succeed
   with zero errors.
3. **Unit tests**: `vitest run` MUST pass with no failures.
4. **Rust tests**: `cargo test` MUST pass with no failures.
5. **Build**: `tauri build` (or `tauri dev` smoke test) MUST
   succeed on at least the contributor's platform.
6. **Constitution compliance**: The PR description MUST confirm no
   principle violations, or document justified exceptions in
   plan.md's Complexity Tracking section.
7. **Performance spot-check**: If the PR adds a dependency or
   modifies the render loop, cold-start and tick-cost MUST be
   re-measured.

## Governance

This constitution is the highest-authority document for the
Delta-T Zaman project. It supersedes all other practices,
conventions, and ad-hoc decisions.

### Amendment Procedure

1. Propose the change as a diff to `constitution.md` in a
   dedicated PR or as part of a feature PR that necessitates it.
2. Document the rationale: which principle is being added,
   modified, or removed, and why.
3. Update the version number following semantic versioning:
   - **MAJOR**: Principle removal or backward-incompatible
     redefinition.
   - **MINOR**: New principle or materially expanded guidance.
   - **PATCH**: Clarification, wording, or non-semantic
     refinement.
4. Update `LAST_AMENDED_DATE` to the merge date.
5. Propagate changes to dependent artifacts: plan.md Constitution
   Check, spec.md scope, tasks.md task types.

### Compliance

- All PRs and code reviews MUST verify compliance with these
  principles.
- Violations MUST be documented in plan.md's Complexity Tracking
  table with a justification.
- Unjustified violations are grounds for blocking a PR.

**Version**: 1.0.0 | **Ratified**: 2026-04-12 | **Last Amended**: 2026-04-12
