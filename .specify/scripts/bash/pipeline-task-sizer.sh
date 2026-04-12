#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

JSON_MODE=false
TASKS_FILE=""

show_help() {
    cat <<'EOF'
Usage: pipeline-task-sizer.sh [OPTIONS] [TASKS_FILE]

Validate that tasks in a tasks.md file are properly sized for
high-context AI reasoning models (target: <4000 tokens per task).

Arguments:
  TASKS_FILE    Path to tasks.md (auto-detected if omitted)

Options:
  --json        Machine-readable JSON output
  --help, -h    Show this help message
EOF
}

for arg in "$@"; do
    case "$arg" in
        --json)   JSON_MODE=true ;;
        --help|-h) show_help; exit 0 ;;
        -*)       echo "Unknown option: $arg" >&2; exit 1 ;;
        *)        TASKS_FILE="$arg" ;;
    esac
done

# Auto-detect tasks file via check-prerequisites if not provided
if [[ -z "$TASKS_FILE" ]]; then
    prereq_output=$(bash "$SCRIPT_DIR/check-prerequisites.sh" --json --paths-only 2>/dev/null) || {
        echo "Error: could not auto-detect tasks.md — pass the path as an argument." >&2
        exit 1
    }
    if has_jq; then
        feature_dir=$(printf '%s' "$prereq_output" | jq -r '.FEATURE_DIR')
    else
        feature_dir=$(printf '%s' "$prereq_output" | grep -o '"FEATURE_DIR":"[^"]*"' | cut -d'"' -f4)
    fi
    TASKS_FILE="$feature_dir/tasks.md"
fi

if [[ ! -f "$TASKS_FILE" ]]; then
    echo "Error: tasks file not found: $TASKS_FILE" >&2
    exit 1
fi

TOKEN_LIMIT=4000
ACCEPT_RE='must|verify|test|assert|expect|confirm|validate|check|ensure|gate|checkpoint'
PATH_RE='[^ ]*\/[^ ]*|\.[a-zA-Z]'

total=0
pass_count=0
fail_count=0
violations_json=""
human_lines=""

while IFS= read -r line; do
    desc="${line#*] }"
    task_id="${desc%% *}"
    desc="${desc#"$task_id"}"
    desc="${desc# }"

    total=$((total + 1))

    word_count=$(printf '%s' "$desc" | wc -w | tr -d ' ')
    est_tokens=$(awk "BEGIN {printf \"%d\", $word_count * 1.33}")
    issues=()

    if (( est_tokens > TOKEN_LIMIT )); then
        issues+=("exceeds_4000_tokens (est: $est_tokens)")
    fi

    if ! printf '%s' "$desc" | grep -qE "$PATH_RE"; then
        issues+=("missing_file_path")
    fi

    if ! printf '%s' "$desc" | grep -iqE "$ACCEPT_RE"; then
        issues+=("missing_acceptance_criteria")
    fi

    if (( ${#issues[@]} == 0 )); then
        pass_count=$((pass_count + 1))
        if ! $JSON_MODE; then
            human_lines+="  ✅ $task_id (est: $est_tokens tokens)"$'\n'
        fi
    else
        fail_count=$((fail_count + 1))
        if $JSON_MODE; then
            issue_array=""
            for iss in "${issues[@]}"; do
                [[ -n "$issue_array" ]] && issue_array+=","
                issue_array+="\"$(json_escape "$iss")\""
            done
            entry="{\"task_id\":\"$(json_escape "$task_id")\",\"issues\":[$issue_array]}"
            [[ -n "$violations_json" ]] && violations_json+=","
            violations_json+="$entry"
        else
            detail=""
            for iss in "${issues[@]}"; do
                [[ -n "$detail" ]] && detail+=", "
                detail+="$iss"
            done
            human_lines+="  ❌ $task_id (est: $est_tokens tokens — $detail)"$'\n'
        fi
    fi
done < <(grep -E '^\s*- \[(x|X| )\] ' "$TASKS_FILE")

status="pass"
(( fail_count > 0 )) && status="fail"

if $JSON_MODE; then
    tasks_path_escaped="$(json_escape "$TASKS_FILE")"
    if has_jq; then
        jq -n \
            --arg status "$status" \
            --arg tasks_file "$TASKS_FILE" \
            --argjson total "$total" \
            --argjson pass "$pass_count" \
            --argjson fail "$fail_count" \
            --argjson violations "[$violations_json]" \
            '{status:$status,tasks_file:$tasks_file,total_tasks:$total,violations:$violations,summary:{total:$total,pass:$pass,fail:$fail}}'
    else
        printf '{"status":"%s","tasks_file":"%s","total_tasks":%d,"violations":[%s],"summary":{"total":%d,"pass":%d,"fail":%d}}\n' \
            "$status" "$tasks_path_escaped" "$total" "$violations_json" "$total" "$pass_count" "$fail_count"
    fi
else
    echo "Task Sizing Validation: $TASKS_FILE"
    printf '%s' "$human_lines"
    echo ""
    echo "Summary: $pass_count/$total tasks pass, $fail_count violations"
fi

(( fail_count > 0 )) && exit 1
exit 0
