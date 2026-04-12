# Codebase Profile Template

> This template defines the required structure for `codebase-profile.md`, produced by `os.discover`. Every section is mandatory — use "N/A" or "Not detected" for sections where the scan found nothing relevant, but do not omit them.

## Required YAML Frontmatter

```yaml
---
produced_by: os.discover
produced_at: <ISO-8601 timestamp>
confidence: 0.7
revision: 1
grade: draft
repo_path: <absolute path to scanned repository>
repo_name: <repository name>
inputs_used: []
stale_after: "Major dependency update, service addition/removal, or architecture change"
---
```

## Required Sections

### 1. Overview

Brief description of the project: what it does, who it's for, and its current state (active development, maintenance mode, etc.).

### 2. Service Inventory

Table of all services, packages, or major modules:

```markdown
| Service | Path | Tech Stack | Purpose | Port | Database |
|---------|------|------------|---------|------|----------|
| api | ./services/api | Express + TypeScript | Main REST API | 3001 | PostgreSQL |
| auth | ./services/auth | Express + TypeScript | Authentication service | 3002 | PostgreSQL |
| web | ./apps/web | Expo (React Native Web) | Web client | 8081 | — |
| mobile | ./apps/mobile | Expo (React Native) | iOS + Android client | — | — |
```

### 3. Architecture Diagram

Text-based diagram showing service topology and connections. Use ASCII art or Mermaid syntax:

```
┌─────────┐     ┌──────────┐     ┌──────────┐
│  Client  │────▶│   API    │────▶│ Postgres │
│  (Expo)  │     │ (Express)│     │          │
└─────────┘     └──────────┘     └──────────┘
                     │
                     ▼
               ┌──────────┐
               │   Auth   │
               │ Backend  │
               └──────────┘
```

### 4. Shared Infrastructure

List all shared infrastructure components:

- **Databases**: engine, version, which services connect, schema ownership
- **Caches**: Redis, Memcached — purpose and which services use them
- **Message queues**: RabbitMQ, Kafka, Redis pub/sub — topics and consumers
- **Shared volumes**: Docker volumes shared between services
- **External services**: LiveKit, S3, CDN, email providers, etc.

### 5. API Surface

Per-service endpoint summary:

```markdown
#### Service: api

| Method | Path | Purpose | Auth Required |
|--------|------|---------|---------------|
| GET | /api/v1/users | List users | Yes |
| POST | /api/v1/auth/login | User login | No |
```

Include middleware stack (auth, CORS, rate limiting, logging).

### 6. Data Model

Key entities and their relationships:

```markdown
| Entity | Table/Collection | Service Owner | Key Fields |
|--------|-----------------|---------------|------------|
| User | users | auth | id, email, password_hash, created_at |
| Session | sessions | auth | id, user_id, token, expires_at |
```

Note: Include migration file locations and ORM used.

### 7. Authentication & Authorization

- Token type (JWT, session, OAuth2)
- Where tokens are issued (which service)
- Where tokens are verified (which services)
- Refresh flow
- Role/permission model (if any)

### 8. Conventions & Patterns

- **Code organization**: monorepo tool (Turborepo, Nx, Lerna, none), shared packages
- **Naming**: file naming, variable naming, API naming patterns
- **Error handling**: error classes, error response format, logging patterns
- **Testing**: framework, location conventions, coverage level
- **CI/CD**: pipeline description, deployment targets
- **Config**: environment variable patterns, config file locations

### 9. Constraints & Known Limitations

- Technical debt items visible from code (TODO/FIXME/HACK comments, deprecated patterns)
- Performance constraints (known bottlenecks, N+1 queries, missing indexes)
- Platform constraints (minimum OS versions, browser support, device requirements)
- Dependency constraints (pinned versions with reasons, known incompatibilities)

### 10. Quality Score

Self-evaluation from os.discover:

```markdown
| Criterion | Score (1-5) | Notes |
|-----------|-------------|-------|
| Specificity | _ | Every claim references a specific file or pattern |
| Actionability | _ | Sufficient detail for downstream agents |
| Non-redundancy | _ | Adds value beyond the repo's own README |
| Evidence quality | _ | Based on actual file contents |
```
