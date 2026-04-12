---
description: "Scan an existing codebase and produce an architecture profile with improvement opportunities"
handoffs:
  - label: "Route input to the right agents"
    agent: "speckit.route"
    prompt: "I have an existing codebase profile and improvement opportunities. Route this input to the right speckit agents for feature development."
    send: true
  - label: "Write feature spec with codebase context"
    agent: "speckit.specify"
    prompt: "Write a feature spec using the codebase profile at .specify/artifacts/codebase-profile.md as context."
  - label: "Assess technical feasibility"
    agent: "os.feasibility"
    prompt: "Assess technical feasibility of a major architectural change using the codebase profile."
  - label: "Run adversarial review"
    agent: "os.critic"
    prompt: "Adversarial review of a proposed change against existing architecture."
  - label: "Start the loop"
    agent: "os.loop"
    prompt: "Run the plan-execute-evaluate cycle using the codebase profile and improvement opportunities."
---

## User Input

$ARGUMENTS — Path to the repository root to scan (e.g., `/path/to/repo` or `.` for current repo). Optionally include focus areas: `$ARGUMENTS architecture`, `$ARGUMENTS api-surface`, `$ARGUMENTS tech-debt`.

## Prerequisites

None. This agent is a **zero-prerequisite entry point for brownfield workflows** — the codebase equivalent of `os.sketch` for greenfield.

> ⚠️ **Draft-grade output.** The profile is derived from static analysis and heuristics. It captures structure accurately but may miss runtime behavior, undocumented conventions, or business context that only the team knows. Always review before feeding downstream.

## Constitutional Authority

**Law VIII — Loop-First Execution** (Rules 8.1–8.5) as defined in `.specify/memory/constitution.md`.

- **Rule 8.5**: Brownfield Entry — when `codebase-profile.md` exists, Loop Mode may skip Phase 1 research agents and route directly to speckit or risk agents.

## Outputs

| Artifact | Path | Confidence | Grade |
|----------|------|------------|-------|
| Codebase Profile | `.specify/artifacts/codebase-profile.md` | 0.7 | draft |
| Improvement Opportunities | `.specify/artifacts/improvement-opportunities.md` | 0.5 | hypothesis |

## Execution

### Phase A — Structural Scan

Scan the target repository to build a factual map. Execute these steps in order:

#### A1. Project Root Analysis

```bash
# Repository metadata
ls -la $REPO_PATH
cat $REPO_PATH/README.md 2>/dev/null | head -100
cat $REPO_PATH/.gitignore 2>/dev/null

# Package manifests (detect tech stacks)
find $REPO_PATH -maxdepth 3 -name "package.json" -o -name "requirements.txt" -o -name "Cargo.toml" -o -name "go.mod" -o -name "pyproject.toml" -o -name "Gemfile" -o -name "pom.xml" -o -name "build.gradle" 2>/dev/null

# Container orchestration
cat $REPO_PATH/docker-compose.yml 2>/dev/null || cat $REPO_PATH/docker-compose.yaml 2>/dev/null
find $REPO_PATH -maxdepth 3 -name "Dockerfile*" 2>/dev/null
```

#### A2. Service Inventory

For each service/package discovered:

1. Read its package manifest (dependencies, scripts, version)
2. Identify the tech stack (language, framework, runtime)
3. Determine its purpose from directory name, README, or entry point
4. Note exposed ports (from Dockerfile EXPOSE, docker-compose ports, or config files)
5. Identify database connections (connection strings, ORM configs, migration files)

#### A3. Architecture Topology

Map how services connect:

1. **Inter-service communication**: grep for HTTP client calls, service URLs, environment variables referencing other services
2. **Shared infrastructure**: databases, message queues (Redis, RabbitMQ, Kafka), caches, shared volumes
3. **External integrations**: third-party APIs, SDKs, webhooks (grep for API keys, SDK imports, webhook handlers)
4. **Authentication flow**: identify auth mechanisms (JWT, sessions, OAuth), where tokens are issued vs verified

#### A4. API Surface

For each backend service:

1. Grep for route definitions (Express: `app.get/post/put/delete`, `router.*`; FastAPI: `@app.*`; etc.)
2. List endpoints with HTTP method, path, and brief purpose
3. Identify middleware (auth guards, rate limiting, CORS, logging)
4. Note API versioning patterns if present

#### A5. Data Model

1. Find migration files (SQL migrations, ORM migrations like Prisma/TypeORM/Alembic)
2. Find schema definitions (Prisma schema, TypeORM entities, SQLAlchemy models, raw SQL)
3. List tables/collections with key fields and relationships
4. Note which service owns which data

#### A6. Conventions & Patterns

1. **Code organization**: monorepo structure, package boundaries, shared code patterns
2. **Testing**: test framework, test location conventions, approximate coverage (count test files vs source files)
3. **CI/CD**: GitHub Actions, Dockerfiles, deployment scripts
4. **Config management**: environment variables, config files, secrets handling
5. **Code style**: linter configs (ESLint, Prettier, Black, Ruff), TypeScript strictness

### Phase B — Improvement Analysis

Using the factual profile from Phase A, identify improvement opportunities across these categories:

#### B1. Architecture

- **Redundancy**: services that could be merged (e.g., separate auth backend that could be a module in the main backend)
- **Missing boundaries**: monolith sections that should be extracted
- **Coupling**: services with tight coupling that creates deployment dependencies
- **Scaling bottlenecks**: single points of failure, stateful services that can't scale horizontally

#### B2. Technical Debt

- **Dependency health**: outdated major versions, known CVEs, abandoned packages
- **Type safety**: missing TypeScript strict mode, `any` usage, untyped APIs
- **Error handling**: inconsistent patterns, swallowed errors, missing error boundaries
- **Dead code**: unused exports, unreachable routes, commented-out blocks

#### B3. Developer Experience

- **Local development**: Docker Compose completeness, hot reload support, seed data
- **Documentation**: missing READMEs, undocumented APIs, stale docs
- **Onboarding**: setup complexity (how many steps to go from clone to running?)
- **Testing gaps**: untested critical paths, missing integration tests

#### B4. Security & Operations

- **Secrets management**: hardcoded values, exposed env vars, missing .env.example
- **Auth patterns**: token expiry, refresh flow, CORS configuration
- **Logging & monitoring**: structured logging, health checks, error tracking
- **Backup & recovery**: database backup strategy, disaster recovery

### Phase C — Write Artifacts

#### C1. Write `codebase-profile.md`

Use the template at `.specify/templates/codebase-profile-template.md`. Fill every section with findings from Phase A. Include:

- YAML frontmatter with artifact metadata (produced_by, confidence, revision, grade)
- Service inventory table
- Text-based architecture diagram (use ASCII or Mermaid)
- API surface summary per service
- Data model overview
- Conventions and constraints

#### C2. Write `improvement-opportunities.md`

Structure as a prioritized table followed by detailed analysis:

```markdown
---
produced_by: os.discover
produced_at: <ISO-8601>
confidence: 0.5
revision: 1
grade: hypothesis
inputs_used:
  - path: ".specify/artifacts/codebase-profile.md"
    existed: true
---

# Improvement Opportunities

## Summary

| # | Category | Title | Impact | Effort | Priority |
|---|----------|-------|--------|--------|----------|
| 1 | Architecture | ... | High | Medium | P1 |
| 2 | Tech Debt | ... | Medium | Low | P2 |

## Detailed Analysis

### 1. [Title]

**Category**: Architecture | Tech Debt | DX | Security
**Impact**: High | Medium | Low
**Effort**: High | Medium | Low

**Current state**: What exists today and why it's suboptimal.

**Proposed change**: Specific, actionable recommendation.

**Risks**: What could go wrong.

**Feeds into**: Which speckit workflow this would use (e.g., `speckit.specify` for a new feature, `speckit.plan` for a refactor).
```

### Phase D — Handoff

After writing both artifacts, present findings to the user and suggest next steps:

**For applying a bug/feature list** (Flow A):
> "The codebase profile is ready. For each bug or feature, invoke `speckit.specify` — the profile provides architectural context for spec generation."

**For improvement opportunities** (Flow B):
> "I've identified N improvement opportunities. Review them and pick which to pursue. For each, invoke `speckit.specify` to generate a spec, or `os.feasibility` first if the change is architecturally risky."

**For full pipeline engagement** (optional):
> "If you want adversarial review of a major change, invoke `os.critic` or `os.feasibility` with the codebase profile as context — they can assess risk even without Phase 1 market research artifacts."

## Handoffs

- **@speckit.specify** — Generate a feature spec with codebase context (Flow A: bugs/features)
- **@speckit.plan** — Plan implementation for a selected improvement (Flow B: opportunities)
- **@os.feasibility** — Assess technical feasibility of a major architectural change
- **@os.critic** — Adversarial review of a proposed change against existing architecture
- **@os.loop** — Let the orchestrator determine the next best action given current artifact state

## Constraints

1. **Static analysis only.** This agent reads files and directory structures. It does NOT run the application, execute tests, or make network requests to the scanned codebase. Runtime behavior must be inferred from code.
2. **No code modification.** This agent produces artifacts in `.specify/artifacts/`. It MUST NOT modify any files in the scanned repository.
3. **Privacy-aware.** Do not include actual secrets, credentials, or API keys in the output artifacts. Reference their existence (e.g., "JWT_SECRET is set via environment variable") without exposing values.
4. **Honest confidence.** The profile (0.7) reflects high structural accuracy but possible gaps in runtime behavior understanding. The opportunities (0.5) are hypotheses — they need human validation before acting on them.

## Self-Evaluation

Before writing the final artifacts, evaluate your output against these criteria:

| Criterion | Score (1-5) | Notes |
|-----------|-------------|-------|
| **Specificity** | _ | Does every claim reference a specific file, directory, or code pattern? |
| **Actionability** | _ | Could a developer act on each improvement opportunity without further research? |
| **Non-redundancy** | _ | Does this add information not already obvious from reading the repo's README? |
| **Evidence quality** | _ | Are assertions backed by actual file contents rather than assumptions? |

**Rules**:
- If any criterion scores below 3, identify the weakest section and revise it before producing the final output.
- Include the completed score table in a `## Quality Score` section at the bottom of each output artifact.
- If you cannot score above 3 on Specificity due to repository access limitations, note this explicitly.
