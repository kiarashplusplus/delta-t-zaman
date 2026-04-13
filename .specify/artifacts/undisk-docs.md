# UNDISK MCP — Documentation

> The undo-first, versioned file workspace for AI agents.
> Live at [mcp.undisk.app](https://mcp.undisk.app)

---

## What is UNDISK?

UNDISK MCP is a remote file workspace for AI agents. Every file mutation an agent makes over [MCP](https://spec.modelcontextprotocol.io) creates an immutable version, so any single write can be surgically undone in under 50ms without affecting other files.

No VM rollbacks. No "restore everything." Just precise, per-file undo with a tamper-evident audit trail.

---

## Quick Start

1. **Sign up** at [mcp.undisk.app/signup](https://mcp.undisk.app/signup)
2. **Get your API key** (shown once after signup at [/keys](https://mcp.undisk.app/keys))
3. **Connect your MCP client:**

### GitHub Copilot / CLI

```http
Server Name: undisk
Server Type: http
URL: https://mcp.undisk.app/v1/mcp
HTTP Headers: {"x-api-key":"sk_live_YOUR_KEY_HERE","Authorization":"Bearer sk_live_YOUR_KEY_HERE"}
Tools: *
```

Both `x-api-key` and `Authorization` included for maximum client compatibility.

> **How state works under the hood:** Undisk keeps workspace state as the latest version pointer for each file path. `write_file`, `create_file`, `append_log`, `delete_file`, `move_file`, and `restore_version` all append new history entries instead of mutating old ones in place. Deletes become tombstones, moves keep both paths in the audit trail, and undo promotes an older version back to the head by creating a fresh version entry.

### GitHub Copilot Cloud Agent (Coding Agent)

For repository-level MCP in GitHub Copilot cloud agent, add `.github/copilot/mcp.json`:

```json
{
  "mcpServers": {
    "undisk": {
      "type": "http",
      "url": "https://mcp.undisk.app/v1/mcp",
      "headers": {
        "x-api-key": "$COPILOT_MCP_UNDISK_API_KEY"
      },
      "tools": ["*"]
    }
  }
}
```

**Important:** Secrets must use the `COPILOT_MCP_` prefix. Go to your repo **Settings → Environments → copilot → Environment secrets** and add `COPILOT_MCP_UNDISK_API_KEY` with your `sk_live_...` key. Only secrets with this prefix are available to MCP configurations.

Optionally add `.github/workflows/copilot-setup-steps.yml` to preinstall dependencies in the agent environment.

### Claude Code

```bash
claude mcp add --transport http undisk https://mcp.undisk.app/v1/mcp \
  --header "Authorization: Bearer sk_live_YOUR_KEY_HERE"
```

### Cursor / Windsurf / VS Code

```json
{
  "mcpServers": {
    "undisk": {
      "url": "https://mcp.undisk.app/v1/mcp",
      "headers": {
        "Authorization": "Bearer sk_live_YOUR_KEY_HERE"
      }
    }
  }
}
```

### Gemini CLI

Add to `~/.gemini/settings.json`:

```json
{
  "mcpServers": {
    "undisk": {
      "httpUrl": "https://mcp.undisk.app/v1/mcp",
      "headers": {
        "Authorization": "Bearer sk_live_YOUR_KEY_HERE"
      },
      "timeout": 30000,
      "trust": true
    }
  }
}
```

### WebSocket (fastest — p50 4ms reads)

```text
wss://mcp.undisk.app/ws?token=sk_live_YOUR_KEY_HERE
```

> **Security note:** Token in query string is necessary because WebSocket handshake does not support custom headers in all clients. The token is used only during the TLS-encrypted upgrade handshake and is not logged server-side. Prefer header-based auth (Streamable HTTP) when your client supports it.

### Direct HTTP

```http
URL:   https://mcp.undisk.app/v1/mcp
Auth:  Authorization: Bearer sk_live_YOUR_KEY_HERE
```

### Google ADK (Python)

Use [Google Agent Development Kit](https://google.github.io/adk-docs/) with `MCPToolset` to consume Undisk tools inside a Gemini-powered agent:

```python
from google.adk.agents import LlmAgent
from google.adk.tools.mcp_tool import McpToolset
from google.adk.tools.mcp_tool.mcp_session_manager import StreamableHTTPConnectionParams

root_agent = LlmAgent(
    model="gemini-2.0-flash",
    name="undisk_agent",
    instruction="You have a versioned file workspace via Undisk.",
    tools=[McpToolset(connection_params=StreamableHTTPConnectionParams(
        url="https://mcp.undisk.app/v1/mcp",
        headers={"Authorization": "Bearer sk_live_YOUR_KEY_HERE"},
    ))],
)
```

---

## MCP Tools

25 tools exposed via the Model Context Protocol. Every write is versioned.

### File Operations

| Tool | Description |
|------|-------------|
| `read_file` | Read file contents by path. Returns UTF-8 text or base64 for binary with a mimeType hint, plus metadata. Large text files (over ~10 KB) are automatically truncated — use line_start/line_end to paginate through remaining content. Supports negative indices for tailing (e.g., line_start: -50). Use version_id to read a specific historical version without modifying state. |
| `write_file` | Write UTF-8 text or base64-encoded binary to a file. Creates an immutable version automatically. |
| `create_file` | Create a new file (fails if path already exists). Use write_file for upsert behavior. |
| `append_log` | Append log entries to a file without overwriting. Ideal for shared logs across agents, sessions, and machines. |
| `start_upload` | Start a staged upload for a large binary file. Then send chunks with append_upload_chunk, finalize with complete_upload. |
| `append_upload_chunk` | Append one ordered base64 chunk to a staged upload session. Keep each chunk under 1 MB (~1.3 MB base64). |
| `complete_upload` | Assemble uploaded chunks and commit them as a normal immutable file version. |
| `cancel_upload` | Discard a staged upload session and delete all buffered chunks. |
| `delete_file` | Soft-delete a file or directory. Set recursive: true to delete all files under a path. Each file is individually tombstoned and restorable. |
| `move_file` | Move or rename a file or directory. Set recursive: true to move all files under a path. Full audit trail on every path. |
| `list_files` | List files in a directory. Supports recursive listing. Returns up to 1,000 entries per call. |
| `search_files` | Search file contents by plain text pattern. Returns matching file metadata (path, name, size). Supports regex, case-insensitive, and content snippets. |

### Version Control

| Tool | Description |
|------|-------------|
| `list_versions` | Show version history for a file: timestamp, agent, content hash, size. |
| `list_changes` | Query all changes across the workspace by agent and/or time range. At least one filter required. Returns version records from all files — ideal for session review, audit, and compliance evidence. |
| `restore_version` | Restore a file to any prior version. Creates a new version with restored content. |
| `get_diff` | Structured diff between two versions: changes with type (added/removed/modified), line ranges, and before/after content. |
| `workspace_checkpoint` | Create, list, restore, and delete named workspace snapshots. A checkpoint captures every live file's current version, enabling atomic multi-file rollback without duplicating content. Use before risky operations. |

### Workspace Admin

| Tool | Description |
|------|-------------|
| `get_policy` | Read workspace access rules and limits. |
| `set_policy` | Configure path ACLs, size limits, extension rules, rate caps. Use mode 'replace' (default) for full replacement or 'merge' for incremental add/remove. |
| `vault_secret` | Securely store, retrieve, list, rotate, and delete workspace secrets. Encrypted at rest, never in file listings or audit plaintext. |
| `share_with_public` | Share a file via an unlisted public link (experimental). Requires enabling at /keys. Shared links need Undisk auth to view. Flagged content blocked. |
| `audit_trail` | Query the workspace's tamper-evident audit trail. Use 'list' to browse entries with filters, 'export' for JSON/NDJSON evidence dumps, and 'verify' to validate hash-chain integrity. Supports filtering by agent, time, operation, and path. |
| `workspace_collaborate` | Multi-agent coordination: claim/release file locks, leave handoff notes, discover active agents. Locks auto-expire after a configurable TTL. Use for collaborative workspace scenarios. |

---

## Tool Reference

Detailed parameter reference for all 25 tools. Parameters marked ✓ are required.

### `read_file`

Read a file's current content and metadata from the Undisk workspace. Text files return UTF-8 content directly. Binary files return base64 content with encoding='base64' and a mimeType field. Size in bytes, SHA-256 content hash, last-modified timestamp, and current version number are always included. Each result includes a `browser_url` — a permanent link the human user can open in their browser to view the file. Large text files (over ~10 KB) are automatically truncated with pagination metadata — use `line_start`/`line_end` to read remaining content. Use list_files to discover available paths first.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `path` | string | ✓ | File path within the workspace (e.g., 'docs/readme.md') |
| `version_id` | string |  | Optional version ID to read a specific historical version instead of the current one. Use list_versions to find version IDs. This is read-only — it does not modify the file's current state (unlike restore_version). |
| `line_start` | number |  | First line to return (1-based, inclusive). Negative values count from the end (e.g., -10 = 10th line from the end). Only applies to text files. Omit to start from line 1. Example: line_start: 1, line_end: 10 returns the first 10 lines. |
| `line_end` | number |  | Last line to return (1-based, inclusive). Negative values count from the end (e.g., -1 = last line). Only applies to text files. Omit to read through the last line. |

---

### `write_file`

Write content to a file in the Undisk workspace. Automatically creates an immutable version, so the previous content is preserved and can be restored anytime via restore_version. Creates the file if it doesn't exist, or updates it if it does (upsert). Use create_file instead when you need a strict 'file must not exist' guard. Supports UTF-8 text by default, or raw binary bytes when content is base64-encoded with encoding='base64'. Returns the new version metadata including version ID and content hash.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `path` | string | ✓ | File path within the workspace (e.g., 'src/config.json') |
| `content` | string | ✓ | File content. Use plain UTF-8 text by default, or base64-encoded bytes when encoding='base64'. |
| `encoding` | string |  | Content encoding. Defaults to 'utf-8'. Use 'base64' for binary files. Values: `utf-8`, `base64`. |
| `if_version` | number |  | Optional optimistic lock. If provided, the write only succeeds when the file's current version number matches this value. Returns a 409 version_conflict error on mismatch. Use to prevent concurrent agents from overwriting each other. Pass 0 to require the file does not yet exist. |

---

### `create_file`

Create a new file in the Undisk workspace. Fails if a file already exists at the given path. Use create_file when you need a strict 'file must not exist' guard; use write_file for upsert behavior. Supports UTF-8 text by default, or raw binary bytes when content is base64-encoded with encoding='base64'. The creation is recorded as a new version with a full audit trail. Returns version metadata including version ID and content hash.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `path` | string | ✓ | File path for the new file (e.g., 'reports/q1-summary.md') |
| `content` | string | ✓ | Initial file content. Use plain UTF-8 text by default, or base64-encoded bytes when encoding='base64'. |
| `encoding` | string |  | Content encoding. Defaults to 'utf-8'. Use 'base64' for binary files. Values: `utf-8`, `base64`. |

---

### `append_log`

Append a log entry to a file in the Undisk workspace. Designed for structured logging where multiple agents, sessions, or machines write to a shared log. Creates the file if it doesn't exist. Each call appends the provided content as new lines at the end of the file — existing content is never overwritten. Automatically creates an immutable version on each append. Use `read_file` with `line_start`/`line_end` (e.g., `line_start: -50`) to efficiently tail recent entries without downloading the entire log.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `path` | string | ✓ | Log file path within the workspace (e.g., 'logs/agent-activity.log') |
| `content` | string | ✓ | Log content to append. Each call appends this text as new lines at the end of the file. |

---

### `start_upload`

Start a staged binary upload for a local file that is too large to fit comfortably in one MCP tool call. Workflow: call start_upload first to get an upload ID, then send ordered chunks via append_upload_chunk, and finalize with complete_upload (or discard with cancel_upload). Returns an upload ID plus recommended chunk sizes.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `path` | string | ✓ | Destination file path within the workspace (for example 'music/track.wav'). |
| `mode` | string |  | Use 'write' to create-or-update, or 'create' to fail if the path already exists. Values: `write`, `create`. |
| `expected_bytes` | number |  | Optional final file size in raw bytes. If provided, complete_upload verifies it exactly. |
| `mime_type` | string |  | Optional MIME type hint for the upload session (for example 'audio/wav'). |

---

### `append_upload_chunk`

Append one ordered base64 chunk to an upload session started with start_upload. Use small chunks rather than one huge base64 string so local binary files do not overflow the model context window.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `upload_id` | string | ✓ | Upload ID returned by start_upload. |
| `part_number` | number | ✓ | Zero-based chunk index. Send chunks in strict order: 0, 1, 2, ... |
| `content` | string | ✓ | Base64-encoded chunk bytes. Keep each chunk under 1 MB (approx. 1.3 MB base64-encoded) to fit within standard MCP message limits. Use the size guidance from start_upload. |

---

### `complete_upload`

Finalize a staged upload after all chunks have been sent. Undisk assembles the uploaded bytes and commits them as a normal immutable file version via write or create mode.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `upload_id` | string | ✓ | Upload ID returned by start_upload. |

---

### `cancel_upload`

Cancel a staged upload and delete all buffered chunks without creating a workspace version. Use this if the local file changed or the upload should be abandoned.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `upload_id` | string | ✓ | Upload ID returned by start_upload. |

---

### `delete_file`

Soft-delete a file or directory. The target disappears from list_files but all prior versions are preserved. Use restore_version to bring it back. Set recursive: true to delete all files under a directory path. Each file gets its own tombstone version for individual undo.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `path` | string | ✓ | Path of the file or directory to delete (e.g., 'temp/draft.txt' or 'old-project') |
| `recursive` | boolean |  | If true, deletes all files under the given directory path. Each file is individually tombstoned and restorable. Default: false (single file delete). |

---

### `move_file`

Move or rename a file or directory. Both source and destination paths are tracked in version history, preserving the full audit trail. Set recursive: true to move all files under a directory path to a new location.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `from_path` | string | ✓ | Current file or directory path (e.g., 'drafts/proposal.md' or 'drafts/v1') |
| `to_path` | string | ✓ | New file or directory path (e.g., 'final/proposal.md' or 'final/v1') |
| `recursive` | boolean |  | If true, moves all files under from_path to to_path, preserving relative structure. Each file gets its own move version record. Default: false (single file move). |

---

### `list_files`

List files and directories in the Undisk workspace. Returns metadata for each entry including path, size, and last-modified timestamp. Each result includes a `browser_url` — a permanent link the human user can open in their browser to view the file. Use recursive mode to get the full workspace tree. Start here to discover what files are available before reading or writing. Returns up to 1,000 entries per call. For larger directories, use search_files or navigate subdirectories explicitly. Note: results are consistent within a sequential request chain but may not reflect writes from concurrent parallel requests.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `path` | string |  | Directory path to list (default: '/' for workspace root) |
| `recursive` | boolean |  | If true, includes all files in subdirectories recursively |

---

### `search_files`

Search across all file contents in the Undisk workspace. By default uses case-sensitive substring matching. Supports regex mode and case-insensitive search. Returns matching file metadata (path, name, size, lastModified). Each result includes a `browser_url` — a permanent link the human user can open in their browser to view the file. Use this to find files when you don't know the exact path. Results are capped at 100 matches. Set `context_lines` to include matching line snippets in results.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `pattern` | string | ✓ | Text pattern to search for across file contents. Interpreted as regex when `regex` is true. |
| `path` | string |  | Directory scope to search within (omit to search entire workspace) |
| `regex` | boolean |  | When true, treat `pattern` as a regular expression. Default: false. |
| `case_sensitive` | boolean |  | When false, match case-insensitively. Default: true. |
| `context_lines` | number |  | Number of lines of context to include around each match (0–10). When > 0, results include a `matches` array where each entry has `line` (1-based line number of the match itself) and `content` (the matched line plus surrounding context lines). Default: 0 (no snippets). |

---

### `list_versions`

Get the complete version history for a file. Every write, delete, move, and restore is recorded as an immutable version. Each entry includes: version ID, timestamp, acting agent, content hash, and size. Use this to find a version ID before calling restore_version or get_diff. Supports pagination via limit/offset. Filter by agentId and/or since timestamp to narrow results.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `path` | string | ✓ | File path to retrieve version history for |
| `limit` | number |  | Maximum number of versions to return (default: 50) |
| `offset` | number |  | Number of versions to skip for pagination |
| `agentId` | string |  | Filter versions by the agent that created them |
| `since` | string |  | Filter versions created after this ISO 8601 timestamp (e.g., '2025-01-15T00:00:00Z') |

---

### `list_changes`

Query all file changes across the workspace, filtered by agent and/or time range. At least one of 'agentId' or 'since' is required to scope the query. Returns version records from all files in a single call — useful for reviewing what an agent changed in a session, auditing recent activity, or building PR-style change summaries. Results are ordered by time (newest first), capped at 500.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `agentId` | string |  | Filter to changes made by a specific agent (e.g., 'key:abc123:summarizer-agent'). Matches the agentId recorded in version history. |
| `since` | string |  | Only return changes after this ISO 8601 timestamp (e.g., '2025-01-15T10:00:00Z'). For recent changes, use today's date at midnight. |
| `limit` | number |  | Maximum number of results to return (default: 100, max: 500). |

---

### `restore_version`

Undo any file change by restoring to a prior version. This is the core undo operation. Call list_versions first to find the version ID, then restore to instantly revert. A new version is created with the restored content, so the restore itself is versioned and reversible.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `path` | string | ✓ | Path of the file to restore |
| `version_id` | string | ✓ | Target version ID to restore to (from list_versions output) |

---

### `get_diff`

Compare two versions of a file and get a structured line-by-line diff. Use list_versions to find version IDs, then diff any two points in the file's history. Returns an array of changes, each with type ('added', 'removed', or 'modified'), lineRange (start/end), and before/after content strings. Useful for reviewing what changed between writes or before deciding whether to restore.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `path` | string | ✓ | Path of the file to diff |
| `from_version` | string | ✓ | Starting version ID (the 'before' state) |
| `to_version` | string | ✓ | Ending version ID (the 'after' state) |

---

### `get_policy`

Get the current workspace policy. Returns path ACLs, size limits, extension rules, and rate limit rules configured for this workspace.

No parameters.

---

### `set_policy`

Set or update the workspace policy. Use mode 'replace' (default) to replace the entire policy, or mode 'merge' to incrementally add/remove individual rules without affecting the rest. Requires workspace owner access. Important: when adding path ACLs, include a catch-all rule (e.g., { pattern: '/**', permission: 'read' }) to allow workspace navigation. Without it, list_files and search_files will return 'Access denied by default'.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `mode` | string |  | Operation mode. 'replace' (default): replaces entire policy with provided config. 'merge': incrementally adds/removes rules via add and remove fields. In merge mode, removals are applied before additions — so you can atomically replace a rule by removing the old and adding the new in a single call. Values: `replace`, `merge`. |
| `pathAcls` | object[] |  | Path-based access control rules (used in replace mode, or within add/remove for merge mode). Each item: `pattern` (string, required), `permission` (string: read \| read-write \| append \| none, required), `agentId` (string, optional). |
| `sizeLimits` | object[] |  | File size limit rules. Each item: `maxBytes` (number, required). |
| `extensionRules` | object[] |  | File extension rules. Each rule has allowed and/or denied arrays of extensions (include the dot, e.g. '.txt'). Each item: `allowed` (string[], optional), `denied` (string[], optional). |
| `rateLimits` | object[] |  | Rate limiting rules. Each item: `maxOps` (number, required), `windowSeconds` (number, required), `scope` (string: agent \| workspace, required). |
| `secretScanning` | object |  | Secret scanning config. In merge mode, fields provided here overwrite existing values. Fields: `enabled` (boolean, required), `block` (boolean, optional), `allowPatterns` (string[], optional). |
| `add` | object |  | Rules to add (merge mode only). Fields: `pathAcls` (object[], optional), `sizeLimits` (object[], optional), `extensionRules` (object[], optional), `rateLimits` (object[], optional). |
| `remove` | object |  | Rules to remove (merge mode only). Match by fields: pathAcls on pattern (+agentId), sizeLimits on maxBytes, rateLimits on scope+maxOps+windowSeconds. Fields: `pathAcls` (object[], optional), `sizeLimits` (object[], optional), `extensionRules` (object[], optional), `rateLimits` (object[], optional). |

---

### `vault_secret`

Securely store, retrieve, list, rotate, and delete secrets scoped to a workspace. Secrets are encrypted at rest and never appear in file listings, search results, browser URLs, diffs, or audit log plaintext. Use this instead of write_file for API keys, tokens, PEM files, and other sensitive values.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `action` | string | ✓ | Operation to perform Values: `put`, `get`, `list`, `rotate`, `delete`. |
| `name` | string |  | Logical secret name, e.g. 'openai/default_api_key'. Required for all actions except 'list'. |
| `value` | string |  | Secret value. Required for 'put' and 'rotate' actions. |
| `encoding` | string |  | Content encoding: 'utf-8' (default) or 'base64' for binary secret blobs. Values: `utf-8`, `base64`. |
| `mode` | string |  | For 'put' action: 'create' (fail if exists) or 'upsert' (default). Values: `create`, `upsert`. |
| `allowedAgentIds` | string[] |  | Agent IDs allowed to access this secret. If omitted, all agents in the workspace can access it. |
| `description` | string |  | Human-readable note about the secret's purpose. |
| `tags` | string[] |  | Optional classification labels. |
| `expiresAt` | string |  | Optional expiration timestamp (RFC 3339). Secret becomes inaccessible after this time. |
| `reveal` | boolean |  | For 'get' action: if true, return the plaintext secret value. Default false (returns masked value). |
| `purpose` | string |  | Required when reveal=true. Audit reason for plaintext retrieval. |

---

### `share_with_public`

Share a workspace file via a public link accessible to any authenticated Undisk user. This is an experimental feature — enable it at https://mcp.undisk.app/keys before use. Shared links are unlisted (like GitHub gists): not indexed, not crawlable, but viewable by anyone with the URL who has an Undisk account. Shared content shows the latest file version. Flagged content cannot be shared.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `action` | string | ✓ | Operation to perform: 'create' generates a share link, 'revoke' disables a share link, 'list' enumerates active shares. Values: `create`, `revoke`, `list`. |
| `path` | string |  | File path to share or revoke. Required for 'create' and 'revoke' actions. Not used for 'list'. |

---

### `audit_trail`

Query the workspace's tamper-evident audit trail. Every file mutation is recorded with hash-chain integrity. Use 'list' to browse recent entries, 'export' for full JSON/NDJSON dumps (ideal for compliance evidence packages), and 'verify' to validate hash-chain integrity. Supports filtering by agent, time range, operation type, and file path. Read-only — does not modify the workspace.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `action` | string | ✓ | Operation to perform Values: `list`, `export`, `verify`. |
| `startTime` | string |  | Filter entries after this ISO 8601 timestamp (e.g., '2025-01-15T00:00:00Z'). |
| `endTime` | string |  | Filter entries before this ISO 8601 timestamp. |
| `agentId` | string |  | Filter to entries from a specific agent. |
| `operationType` | string |  | Filter to a specific operation (e.g., 'write', 'delete', 'move', 'restore', 'create', 'append'). |
| `filePath` | string |  | Filter to entries affecting a specific file path. |
| `limit` | number |  | Maximum entries to return for 'list' action (default: 100, max: 1000). Not used for 'export' or 'verify'. |
| `format` | string |  | Export format: 'json' (default) or 'ndjson' (newline-delimited JSON). Only used with 'export' action. Values: `json`, `ndjson`. |
| `fromSeq` | number |  | Starting sequence number for 'verify' action. Omit to verify from the beginning. |
| `toSeq` | number |  | Ending sequence number for 'verify' action. Omit to verify to the end. |

---

### `workspace_checkpoint`

Create, list, restore, and delete named workspace snapshots. A checkpoint captures every live file's current version, enabling atomic multi-file rollback. No content is duplicated — snapshots reference immutable versions already stored. Use 'create' before risky operations, 'restore' to roll back, 'list' to browse, and 'delete' to clean up.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `action` | string | ✓ | Operation to perform. Values: `create`, `list`, `restore`, `delete`. |
| `name` | string |  | Human-readable checkpoint name. Required for 'create'. Example: 'before-refactor', 'pre-migration-v2'. |
| `description` | string |  | Optional description for the checkpoint. Only used with 'create'. |
| `checkpoint_id` | string |  | Checkpoint ID to restore or delete. Required for 'restore' and 'delete' actions. |
| `limit` | number |  | Maximum number of checkpoints to return for 'list' action (default: 20, max: 100). |

---

### `workspace_collaborate`

Multi-agent coordination: claim/release file locks, leave handoff notes for other agents, and discover active agents. Locks auto-expire after a configurable TTL (default 5 minutes). Handoff notes persist until read.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `action` | string | ✓ | Operation to perform. Values: `claim_lock`, `release_lock`, `list_locks`, `handoff_note`, `list_notes`, `list_agents`. |
| `path` | string |  | File path to lock or unlock. Required for 'claim_lock' and 'release_lock'. Glob patterns supported for 'claim_lock' (e.g., 'src/**/*.ts'). |
| `ttl_seconds` | number |  | Lock time-to-live in seconds (default: 300 = 5 minutes, max: 3600). Only used with 'claim_lock'. |
| `message` | string |  | Handoff message text. Required for 'handoff_note'. |
| `to_agent` | string |  | Optional target agent ID for 'handoff_note'. Omit to broadcast to all agents. |
| `limit` | number |  | Maximum results to return for 'list_notes' or 'list_agents' (default: 50, max: 200). |
| `include_read` | boolean |  | Include already-read notes in 'list_notes' results (default: false). |

---

### `discover_tools_and_capabilities`

Call NOW after tools/list to learn what this server can do. Returns available tools, server capabilities, protocol versions, provider feature flags, health status, and caller identity. Useful for startup discovery, diagnostics, and enterprise attribution. Includes an SSO-friendly stable sub claim derived from the API key.

No parameters.

---

### `run_in_sandbox`

Execute code in a secure, ephemeral E2B cloud sandbox and return stdout, stderr, and exit code. Supports Python, JavaScript, TypeScript, Bash, and C. Each call creates a fresh sandbox — no state persists between calls. Requires the workspace owner to have enabled Undisk Compute (E2B_API_KEY configured).

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `language` | string | ✓ | Programming language to execute. Values: `python`, `javascript`, `typescript`, `bash`, `c`. |
| `code` | string | ✓ | Source code to execute in the sandbox. |
| `timeout_ms` | number |  | Maximum execution time in milliseconds (1–60000). Default: 30000. |

---

## Authentication

All MCP connections require an API key. Get your key at [/keys](https://mcp.undisk.app/keys) after signing up.

```http
Authorization: Bearer sk_live_...   ← canonical header
x-api-key: sk_live_...              ← also accepted (equivalent)
```

Both headers are first-class. If both are present, `Authorization` takes precedence. WebSocket connections use query-string auth (`?token=...`).

API keys are shown once at creation and stored as SHA-256 hashes. If you lose your key, regenerate at [/keys](https://mcp.undisk.app/keys) — this revokes all previous keys.

> ⚠️ **Key rotation warning:** API key validations are cached for up to 5 minutes (KV TTL). After revoking or regenerating a key, the old key may remain valid for up to 5 minutes. Plan key rotations accordingly — do not assume instant revocation.

---

## How Versioning Works

Every file mutation (write, delete, move) creates an **immutable version**. Versions are content-addressed via SHA-256, so identical content deduplicates automatically.

1. Agent calls `write_file` with new content
2. UNDISK hashes the content (SHA-256), stores it, and creates a version entry
3. Version entry records: timestamp, agent identity, content hash, file path, size
4. Deletes create tombstone versions, so files disappear from listings without erasing their history
5. Moves record both the old path and the new path, preserving rename history
6. Previous versions are **never modified or deleted** (within retention period)
7. Call `restore_version` with any version ID to undo — creates a *new* head version with old content instead of rewinding history

---

## Binary File Support

UNDISK handles binary files natively over MCP. Write binary content as base64 with `encoding: "base64"`; read responses return base64 plus a `mimeType` hint.

```javascript
write_file({ path: "music/track.wav", content: "<base64>", encoding: "base64" })
read_file({ path: "music/track.wav" })
→ { content: "<base64>", encoding: "base64", mimeType: "audio/wav", size: 4200000, ... }
```

MCP request-body cap is 16 MiB. For larger binaries, use staged uploads:

```javascript
start_upload({ path: "music/track.wav", mode: "create", expected_bytes: 4200000 })
append_upload_chunk({ upload_id: "upl_...", part_number: 0, content: "<base64 chunk>" })  # keep each chunk under 1 MB (~1.3 MB base64)
append_upload_chunk({ upload_id: "upl_...", part_number: 1, content: "<base64 chunk>" })
complete_upload({ upload_id: "upl_..." })
```

Maximum file size via staged upload: **256 MB**.

---

## The Undo Moment

```text
# Agent writes a bad file
→ write_file({ path: "config.yml", content: bad_content })
  version: ver_a1b2c3

# See what happened
→ list_versions({ path: "config.yml" })
  ver_a1b2c3  2026-04-06 11:20  agent_claude  4.1 KB
  ver_x9y8z7  2026-04-06 11:15  agent_claude  3.8 KB  ← good

# Restore the good version
→ restore_version({ path: "config.yml", version_id: "ver_x9y8z7" })
  restored in 8ms. new version: ver_d4e5f6
```

Restore is non-destructive: it creates a new version with old content. The bad write stays in the audit trail, so you can inspect it later with `list_versions` or `get_diff`. No data is ever lost within the retention window.

---

## Policy Engine

Control what agents can do with path-based ACLs, file size limits, rate caps, and extension rules. Available on Pro plans and above.

**Replace mode** — set the entire policy at once:

```json
set_policy({
  "mode": "replace",
  "pathAcls": [
    { "pattern": "production/**", "permission": "read" },
    { "pattern": "drafts/**", "permission": "read-write" },
    { "pattern": "secrets/**", "permission": "none" }
  ],
  "sizeLimits": [{ "maxBytes": 10485760 }],
  "rateLimits": [{ "maxOps": 1000, "windowSeconds": 60, "scope": "workspace" }],
  "extensionRules": [{ "denied": [".exe", ".sh"] }],
  "secretScanning": { "enabled": true, "block": true }
})
```

**Merge mode** — add or remove individual rules without replacing the rest:

```json
set_policy({
  "mode": "merge",
  "add": {
    "pathAcls": [{ "pattern": "logs/**", "permission": "read" }]
  },
  "remove": {
    "pathAcls": [{ "pattern": "drafts/**" }]
  }
})
```

All top-level fields (`pathAcls`, `sizeLimits`, `rateLimits`, `extensionRules`, `secretScanning`, `add`, `remove`) are optional — include only what you need.

Permission denials return explanatory errors: the agent is told *which policy* blocked the action and why.

> **ACL default behavior:** When no `pathAcls` rules exist, all paths are accessible (default-allow). Once **any** ACL rule is added, behavior switches to **default-deny**: paths not matched by any rule are blocked. Agent-scoped rules (with `agentId`) take priority over global rules. Among matching rules, more specific patterns (longer glob) win. Permissions: `"read"` allows reads/lists/searches only, `"read-write"` allows all operations, `"none"` blocks everything.
> **Path convention:** MCP tools use paths without a leading slash (e.g., `docs/readme.md`). Both file paths and ACL patterns are normalized — `production/**` and `/production/**` are equivalent. The web file browser uses URL paths with a leading slash; this is handled automatically.

---

## Secret Detection

`write_file`, `create_file`, `append_log`, and staged uploads (`complete_upload`) are scanned for 20+ secret patterns before content reaches storage. Matched secrets are blocked by default; the full secret never persists.

```json
{ "secretScanning": { "enabled": true, "block": true, "allowPatterns": [] } }
```

Built-in patterns: AWS keys, GitHub PATs, Stripe keys, OpenAI/Anthropic keys, Slack tokens, GCP/Azure credentials, private keys, JWTs, database URLs, and generic secret assignments.

> **Note:** `restore_version` does **not** re-run secret scanning. If an old version contains a secret that was previously allowed, restoring it will succeed. Secret scanning is also subject to policy — if you need to store secrets for agent use, see the `vault_secret` tool below.

---

## Content Moderation

File content written via `write_file`, `create_file`, and staged uploads (`complete_upload`) is asynchronously scanned for harmful content using Cloudflare Workers AI (`@cf/meta/llama-guard-3-8b`).

**How it works:** After a successful write, content is enqueued for background moderation. The scan runs asynchronously — writes are never blocked or delayed by moderation.

**What agents see:** Moderation is transparent — `read_file` only includes moderation metadata when content is **flagged**. Pending and approved statuses are internal bookkeeping and are never surfaced. When flagged, the response includes `moderationStatus: "flagged"` and a `moderationCategory` indicating the hazard type.

**What happens when content is flagged:** Flagged content is **not blocked from reads** — it remains accessible. Flagged files **cannot** be shared via `share_with_public`. To dispute a flag, contact support@undisk.app.

**Hazard categories:** `violent_crimes`, `non_violent_crimes`, `sex_related_crimes`, `child_sexual_exploitation`, `defamation`, `specialized_advice`, `privacy`, `intellectual_property`, `indiscriminate_weapons`, `hate`, `suicide_self_harm`, `sexual_content`.

**Deduplication:** Moderation is keyed by content SHA-256 hash. Identical content is only scanned once regardless of how many files reference it.

> **Note:** `append_log` and `restore_version` do **not** trigger moderation. Content moderation is optional — it degrades gracefully if the Workers AI binding is not configured.

---

## Workspace Secrets (vault_secret)

The `vault_secret` tool provides a first-class primitive for storing API keys, tokens, PEM files, and other sensitive values — separate from the file namespace. Secrets are encrypted at rest (AES-256-GCM, per-workspace key) and never appear in file listings, search results, browser URLs, diffs, or audit log plaintext.

> **Note:** The vault is only accessible programmatically via the `vault_secret` MCP tool (used by AI agents), not through the web UI.

### Actions

| Action | Description |
|--------|-------------|
| `put` | Store or update a secret. Use `mode: "create"` to fail if exists, or `"upsert"` (default). |
| `get` | Retrieve a secret. Returns masked value by default. Set `reveal: true` and provide `purpose` for plaintext. |
| `list` | List all accessible secrets (metadata only, no values). |
| `rotate` | Update the value of an existing secret, incrementing its version. |
| `delete` | Permanently delete a secret (not soft-delete — old values are irrecoverable). |

### Example: Store an OpenAI key

```json
vault_secret({
  "action": "put",
  "name": "openai/default_api_key",
  "value": "sk-proj-...",
  "mode": "create",
  "allowedAgentIds": ["agent_research", "agent_builder"],
  "description": "Default OpenAI API key for workspace agents"
})
```

### Example: Retrieve it later

```json
vault_secret({
  "action": "get",
  "name": "openai/default_api_key",
  "reveal": true,
  "purpose": "Call OpenAI API for summarization task"
})
```

### Security properties

- **Encrypted at rest** — per-workspace AES-256-GCM key derived from workspace identity
- **Not part of file namespace** — never appears in `list_files`, `search_files`, or `browser_url`
- **Plaintext never in audit logs** — audit entries record only metadata and SHA-256 fingerprint
- **Access control** — optional `allowedAgentIds` restricts which agents can read the secret
- **Expiration** — optional `expiresAt` (RFC 3339) makes the secret inaccessible after that time
- **Reveal requires purpose** — `get` with `reveal: true` requires a `purpose` string recorded in the audit trail
- **⚠️ Reveal exposes plaintext to LLM context** — when `reveal: true` is used, the decrypted secret value is returned in the MCP tool response, which means it enters the agent's context window. This is by design (agents need the secret to make API calls), but be aware that the value may appear in LLM provider logs depending on your provider's data retention policy. Use `allowedAgentIds` to limit which agents can request reveal.
- **No version history** — old secret values are overwritten, not retained (unlike files)
- **64 KiB size cap** — designed for keys and tokens, not arbitrary blobs

### Parameters

| Parameter | Type | Required | Notes |
|-----------|------|----------|-------|
| `action` | `"put" \| "get" \| "list" \| "rotate" \| "delete"` | yes | Operation to perform |
| `name` | string | except `list` | Logical secret name, e.g. `openai/default_api_key` |
| `value` | string | for `put`/`rotate` | Secret value |
| `encoding` | `"utf-8" \| "base64"` | no | Default `utf-8`; use `base64` for binary blobs |
| `mode` | `"create" \| "upsert"` | no | For `put`; default `upsert` |
| `allowedAgentIds` | string[] | no | If omitted, all agents can access |
| `description` | string | no | Human-readable note |
| `tags` | string[] | no | Optional labels |
| `expiresAt` | string (RFC 3339) | no | Auto-expire |
| `reveal` | boolean | no | For `get`; default `false` |
| `purpose` | string | when `reveal=true` | Required audit reason for plaintext retrieval |

---

## Public Sharing (experimental)

The `share_with_public` tool lets agents create unlisted share links for workspace files — similar to GitHub unlisted gists. **This is an experimental feature** that must be enabled per-workspace at [/keys](/keys).

### How it works

1. Enable the experiment toggle in your workspace settings at `/keys`
2. Use `share_with_public` with `action: "create"` to generate a share link
3. Anyone with an Undisk account who has the link can view the file content
4. Shares always serve the **latest file version**
5. Revoke any share at any time with `action: "revoke"`

### Parameters

| Parameter | Type | Required | Notes |
|---|---|---|---|
| `action` | `"create" \| "revoke" \| "list"` | yes | Operation to perform |
| `path` | string | for create/revoke | File path to share or revoke |

### Example: share a file

```json
share_with_public({
  "action": "create",
  "path": "reports/q1-summary.md"
})
```

### Security and behavior

- **Authentication required**: viewers must have an Undisk account — links are not truly "public"
- **No metadata exposed**: shared content pages show only file content, no paths, workspace info, or owner details
- **Not indexed**: all share pages include `noindex, nofollow` and `X-Robots-Tag: noindex` headers
- **Content moderation**: files flagged by content moderation cannot be shared; existing shares become inaccessible if the file is later flagged
- **Limit**: up to 100 active shares per workspace
- **Shares follow moves**: if you move a shared file, the share link continues to work
- **Immediate revocation**: revoking a share takes effect instantly; the share ID cannot be reused
- **Cloudflare only**: this feature is currently available on the Cloudflare provider only

---

## Audit Trail

Every operation produces an immutable version record with content hashes for integrity verification:

```json
{
  "id": "ver_a1b2c3",
  "path": "regulatory/q2-report.md",
  "versionNum": 3,
  "contentHash": "sha256:e3b0c44298fc1c...",
  "size": 4096,
  "operation": "write",
  "agentId": "key:78bd0ae5:kyc-agent",
  "principalId": "user_abc123",
  "createdAt": "2026-04-06T10:00:00.000Z"
}
```

### Agent labeling

By default, `agentId` is derived from the API key (`key:{keyId}`). To distinguish multiple agents sharing the same key, there are two mechanisms:

**Automatic** — Undisk captures the `clientInfo.name` from the MCP `initialize` handshake. If your client sends `clientInfo: { name: "cursor" }`, the agentId automatically becomes `key:{keyId}:cursor` — zero configuration needed. All major MCP clients (Claude Desktop, Cursor, Copilot, Windsurf) send this.

**Explicit** — Send an `X-Agent-Name` header with each request to set a custom label:

```http
X-Agent-Name: kyc-agent
```

**Priority**: `X-Agent-Name` header > `clientInfo.name` > bare `key:{keyId}`

The agentId becomes `key:{keyId}:{name}` — giving each agent a unique, human-readable identity in version history and activity context. Names must be 1–64 characters, alphanumeric with hyphens, underscores, and dots.

For full EU AI Act Article 12 compliance, we recommend one API key per agent **or** consistent agent labeling so the audit trail can attribute each operation to a specific AI system.

Export audit logs as NDJSON via the dashboard. Integrity hashes allow offline verification that no entries were modified after creation. Audit export is available on all plans; extended retention and compliance features are available on Pro+ plans.

### Audit Export API

Export and verify audit logs programmatically for SIEM integration (Splunk, Datadog, etc.):

```http
GET /api/workspaces/:id/audit/export?format=ndjson&start=2026-01-01T00:00:00Z&end=2026-04-01T00:00:00Z&agentId=key:78bd:kyc-agent
```

**Query parameters:**

- `format` — `json` (default) or `ndjson` (recommended for streaming/SIEM)
- `start` / `end` — ISO 8601 time range filter
- `agentId` — filter by agent identity
- `operationType` — filter by operation (e.g. `write`, `delete`)
- `filePath` — filter by file path

**Chain verification** — cryptographically verify no audit entries were tampered with:

```http
GET /api/workspaces/:id/audit/verify?fromSeq=1&toSeq=100
```

Returns `{ "valid": true, "checked": 100 }` or `{ "valid": false, "brokenAt": 42 }`.

---

## SSO (Single Sign-On)

SSO is available on **Team** and **Enterprise** plans. Undisk supports both OIDC (OpenID Connect) and SAML 2.0 identity providers.

### Setup

1. **Create an organization** — `POST /api/orgs` with `{ name, slug }`
2. **Add a domain** — `POST /api/orgs/:orgId/sso/domains` with `{ domain: "example.com" }`
3. **Verify the domain** — Add the returned TXT record to your DNS, then `POST /api/orgs/:orgId/sso/domains/:domain/verify`
4. **Register a provider** — `POST /api/orgs/:orgId/sso/providers` with type (oidc/saml), issuer URL, client ID/secret
5. **Activate** — `POST /api/orgs/:orgId/sso/providers/:id/activate`

### SSO Login

Users with email domains matching a verified SSO domain are redirected to the IdP:

```http
GET /api/auth/sso/login?email=user@example.com
```

The flow: domain lookup → IdP redirect → callback → JIT user provisioning → session creation → redirect to dashboard.

### Session Enforcement

SSO sessions have an expiry set by the IdP. When an SSO session expires, the user must re-authenticate via their IdP. The `X-SSO-Reauth-Required` header is sent 5 minutes before expiry.

Organizations can enforce SSO by setting `enforceSSO: true` in org metadata. When enforced, password login is blocked for users with matching email domains — they must use SSO.

### Credential Revocation

Org owners can revoke all SSO sessions and OAuth tokens for a user:

```http
POST /api/orgs/:orgId/sso/revoke-user
{ "userId": "user_abc123" }
```

### SAML Metadata

SP metadata for SAML IdP configuration:

```http
GET /api/auth/sso/saml/metadata?orgId=org_123
```

### SSO Audit Trail

SSO admin events (domain verification, provider changes, credential revocation) are logged in a separate `sso_admin_log` table at the organization level. File-level audit entries include `ssoIdentity` when the operation was performed via an SSO session.

---

## Transports

| Transport | p50 Read | p50 Write | Best For |
|-----------|----------|-----------|----------|
| **WebSocket** ⚡ | 4 ms | 19 ms | Persistent agent sessions, latency-sensitive |
| **Streamable HTTP** | 37 ms | 91 ms | Universal — all MCP clients |
| **Edge RPC** | <1 ms | <5 ms | Internal — direct DO binding |
| **stdio Proxy** | 37 ms | 91 ms | Local CLI tools via stdin/stdout |

> **Benchmark methodology:** p50 latencies measured over 10K requests, 1 KB payloads, warm connection, same-region (US-East). Includes auth. Real-world latency varies by region, payload size, and network conditions.

**Why WebSocket is faster:** persistent connection eliminates per-request TLS handshake, auth parsing, and HTTP framing. Batch mode: N requests in one frame at cost of one round-trip.

---

## Architecture

- **Smart Placement** — Worker colocated next to your LLM, not at user edge
- **One Durable Object per workspace** — SQLite for metadata, R2 for content
- **Content-addressable storage** — SHA-256 deduplication; files ≤128 KB inline in SQLite
- **WebSocket Hibernation** — p50 4ms reads (same-region, warm connection), zero idle billing, JSON-RPC batch support

---

## Pricing

| Plan | Price | Workspaces | Storage | Ops/Day | Retention |
|------|-------|-----------|---------|---------|-----------|
| Free | $0 | 1 | 100 MB | 1,000 | 30 days |
| Pro | $29/mo | 5 | 10 GB | 50,000 | 180 days |
| Team | $99/mo | 25 | 100 GB | 500,000 | 365 days |
| Enterprise | Custom | Unlimited | Custom | Unlimited | Up to 10 years |

Free tier retention (30 days) does **not** meet EU AI Act Article 26(6) minimum of 6 months. Upgrade to Pro or above for extended retention that supports compliance requirements.

---

## Error Responses

| Code | Meaning |
|------|---------|
| `PERMISSION_DENIED` | Write/delete blocked by a policy ACL. Includes the policy name and rule. |
| `STORAGE_LIMIT` | Workspace storage cap reached. Includes current usage and upgrade URL. |
| `RATE_LIMITED` | Daily or per-minute op cap exceeded. Includes `retry_after_seconds`. **You MUST wait the specified seconds before retrying — do not instantly retry.** |
| `FILE_TOO_LARGE` | File exceeds max size (10 MB Free/Pro, 100 MB Team; platform limit 256 MB). |
| `VERSION_EXPIRED` | Requested version purged per retention policy. |
| `VERSION_NOT_FOUND` | Invalid version ID for the specified file. |
| `WORKSPACE_NOT_FOUND` | Workspace ID doesn't match any active workspace. |
| `INTERNAL_ERROR` | Server error. All committed writes are safe. |

---

## Limits

| Limit | Free | Pro | Team | Enterprise |
|-------|------|-----|------|------------|
| Workspaces | 1 | 5 | 25 | Unlimited |
| Total storage | 100 MB | 10 GB | 100 GB | Custom |
| Operations/day | 1,000 | 50,000 | 500,000 | Unlimited |
| Max file size | 10 MB | 10 MB | 100 MB | Custom (256 MB platform limit) |
| Version retention | 30 days | 180 days | 365 days | Up to 10 years |

---

## Endpoints

| Endpoint | Protocol | Description |
|----------|----------|-------------|
| `https://mcp.undisk.app/v1/mcp` | Streamable HTTP | Primary MCP endpoint |
| `https://mcp.undisk.app/mcp` | Streamable HTTP | Alias |
| `wss://mcp.undisk.app/ws` | WebSocket | Fastest (4ms reads) |
| `wss://mcp.undisk.app/v1/ws` | WebSocket | Alias |
| `GET https://mcp.undisk.app/health` | HTTP | Health check |

---

## OAuth Scopes

OAuth 2.1 scopes restrict what each access token can do. Request specific scopes when issuing tokens via `POST /api/oauth/token`. Legacy API keys (`sk_live_*`) bypass scope enforcement and have full access.

| Scope | Grants Access To |
|-------|-----------------|
| `files:read` | `read_file`, `list_files`, `search_files` |
| `files:write` | `write_file`, `create_file`, `append_log`, `delete_file`, `move_file`, `start_upload`, `append_upload_chunk`, `complete_upload`, `cancel_upload` |
| `versions:read` | `list_versions`, `get_diff` |
| `versions:write` | `restore_version` |
| `policy:read` | `get_policy` |
| `policy:write` | `set_policy` |
| `secrets:read` | `vault_secret` (get, list) |
| `secrets:write` | `vault_secret` (put, rotate, delete) |
| `shares:write` | `share_with_public` (create, revoke, list) |
| `audit:read` | `audit_trail` (list, export, verify) |

---

## FAQ & Behavior Details

**What counts as an operation?** Every MCP tool call counts as one operation. `append_upload_chunk` counts as one op per chunk. `list_files`, `search_files`, and `read_file` each count as one op. WebSocket batch requests count as N ops (one per sub-request).

**Do deleted versions count toward storage?** Yes. All retained versions count toward your storage quota. Content is SHA-256 deduplicated within each workspace, so identical content only counts once regardless of how many versions reference it.

**How do versions work across moves and deletes?**

- **After a move:** A new version is created at the **new path** (operation: `"move"`, metadata: `{ movedFrom: "old/path" }`). A tombstone version is created at the **old path** (metadata: `{ movedTo: "new/path" }`). Call `list_versions` on the new path to see the full history including the move event.
- **After a delete:** A tombstone version is created at the file's path (operation: `"delete"`). Call `list_versions` on the original path — the delete event and all prior versions remain visible.
- **Restoring a deleted file:** Call `restore_version` with the original path and a version ID from before the delete. This creates a **new** version (operation: `"restore"`) — it does not revert or remove the delete record. Version history is append-only.
- **Restoring a moved file:** Call `restore_version` with the **new** path and the version ID you want to restore. You cannot restore across paths — `restore_version` requires the version ID to belong to the specified path's history.
- **Version operations are always INSERT-only** — no version record is ever modified or deleted. Every restore, move, or delete creates a new version entry, preserving the full audit trail.

**Does `restore_version` re-run policy checks?** Path ACLs are enforced (the restoring agent must have write access to the target path). Content moderation is re-run on restored content. Secret scanning is **not** re-run — previously stored content is trusted. Extension rules and size limits are enforced against the restored content.

**What does `browser_url` point to?** An authenticated URL that requires an active session (login via the web dashboard). It points to the latest live version of the file and updates automatically after moves. After deletion, the URL shows a "file deleted" state with restore option.

**Pagination limits:** `list_files` returns up to 1,000 entries per call. `search_files` returns up to 100 matches. For larger workspaces, use path-scoped calls (e.g., `list_files({ path: "subdir/" })`) to stay within limits.

---

*[Compliance Documentation](https://mcp.undisk.app/docs/compliance) · [Privacy Policy](https://mcp.undisk.app/privacy) · [Terms](https://mcp.undisk.app/terms)*
