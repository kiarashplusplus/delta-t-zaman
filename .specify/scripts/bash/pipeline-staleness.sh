#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

show_help() {
    cat <<'EOF'
Usage: pipeline-staleness.sh [OPTIONS]

Detect stale artifacts by comparing modification timestamps.

Options:  --json  Output JSON    --help  Show help
Exit codes:  0 = clean   2 = stale artifacts found
EOF
}

JSON_MODE=false
for arg in "$@"; do
    case "$arg" in
        --json) JSON_MODE=true ;;
        --help|-h) show_help; exit 0 ;;
    esac
done

# OS-specific modification-time function (epoch seconds)
case "$(uname -s)" in
    Darwin*) get_mtime() { stat -f %m "$1"; } ;;
    *)       get_mtime() { stat -c %Y "$1"; } ;;
esac

REPO_ROOT="$(get_repo_root)"
ARTIFACT_DIR="$REPO_ROOT/.specify/artifacts"

# Resolve feature directory for Phase 3 artifacts (qa-report.md)
FEATURE_DIR=""
if _paths_output=$(get_feature_paths 2>/dev/null); then
    eval "$_paths_output"
    unset _paths_output
fi

# Map artifact name → phase number
get_phase() {
    case "$1" in
        market-map.md|icp.md|why-now.md)                          echo 1 ;;
        competitor-matrix.md|ux-teardown.md|steal-differentiate-ignore.md) echo 1 ;;
        usp.md|feasibility.md|growth-plan.md)                     echo 1 ;;
        brutal-critique.md|tradeoff-matrix.md|moat.md)            echo 1 ;;
        persona-sim.md|ux-strategy.md|design-tokens.md|design-constraints.md) echo 2 ;;
        qa-report.md|trust-safety-report.md)                          echo 3 ;;
        launch-kit.md|analytics-report.md|simplification-plan.md|iteration-brief.md) echo 4 ;;
        *) return 1 ;;
    esac
}

# Resolve absolute path; returns 1 if unresolvable (e.g. no feature dir for Phase 3)
artifact_path() {
    local phase
    phase=$(get_phase "$1") || return 1
    if [ "$phase" = "3" ]; then
        [ -n "$FEATURE_DIR" ] && echo "$FEATURE_DIR/$1" || return 1
    else
        echo "$ARTIFACT_DIR/phase-$phase/$1"
    fi
}

# ── Dependency graph ────────────────────────────────────────────────
P1_PRE="market-map.md icp.md why-now.md competitor-matrix.md ux-teardown.md steal-differentiate-ignore.md usp.md feasibility.md growth-plan.md"
P1_ALL="$P1_PRE brutal-critique.md tradeoff-matrix.md moat.md"

get_deps() {
    case "$1" in
        competitor-matrix.md|ux-teardown.md|steal-differentiate-ignore.md)
            echo "market-map.md icp.md why-now.md" ;;
        usp.md)             echo "competitor-matrix.md steal-differentiate-ignore.md icp.md" ;;
        feasibility.md)     echo "usp.md market-map.md" ;;
        growth-plan.md)     echo "icp.md usp.md feasibility.md" ;;
        brutal-critique.md) echo "$P1_PRE" ;;
        tradeoff-matrix.md) echo "brutal-critique.md feasibility.md growth-plan.md" ;;
        moat.md)            echo "competitor-matrix.md usp.md growth-plan.md" ;;
        persona-sim.md)     echo "$P1_ALL" ;;
        ux-strategy.md)     echo "$P1_ALL persona-sim.md" ;;
        design-tokens.md|design-constraints.md) echo "ux-strategy.md usp.md" ;;
        qa-report.md)               echo "brutal-critique.md feasibility.md ux-strategy.md" ;;
        trust-safety-report.md)     echo "qa-report.md" ;;
        launch-kit.md)          echo "growth-plan.md moat.md ux-strategy.md" ;;
        analytics-report.md)    echo "launch-kit.md" ;;
        simplification-plan.md) echo "analytics-report.md qa-report.md" ;;
        iteration-brief.md)     echo "analytics-report.md simplification-plan.md brutal-critique.md" ;;
    esac
}

# Every downstream artifact (those that have at least one upstream)
DOWNSTREAMS="competitor-matrix.md ux-teardown.md steal-differentiate-ignore.md usp.md feasibility.md growth-plan.md brutal-critique.md tradeoff-matrix.md moat.md persona-sim.md ux-strategy.md design-tokens.md design-constraints.md qa-report.md trust-safety-report.md launch-kit.md analytics-report.md simplification-plan.md iteration-brief.md"

# ── Collect stale artifacts ─────────────────────────────────────────
stale_count=0; stale_names=""; stale_phases=""; stale_paths=""; stale_reasons=""
SEP="$(printf '\x1e')"; RSEP="|"

for downstream in $DOWNSTREAMS; do
    ds_path=$(artifact_path "$downstream") || continue
    [ -f "$ds_path" ] || continue
    ds_mtime=$(get_mtime "$ds_path")

    reasons=""
    for upstream in $(get_deps "$downstream"); do
        us_path=$(artifact_path "$upstream") || continue
        [ -f "$us_path" ] || continue
        us_mtime=$(get_mtime "$us_path")
        if [ "$us_mtime" -gt "$ds_mtime" ]; then
            reasons="${reasons:+${reasons}${RSEP}}upstream $upstream modified after $downstream"
        fi
    done

    if [ -n "$reasons" ]; then
        phase=$(get_phase "$downstream")
        rel="${ds_path#"$REPO_ROOT/"}"
        stale_names="${stale_names:+${stale_names}${SEP}}${downstream}"
        stale_phases="${stale_phases:+${stale_phases}${SEP}}${phase}"
        stale_paths="${stale_paths:+${stale_paths}${SEP}}${rel}"
        stale_reasons="${stale_reasons:+${stale_reasons}${SEP}}${reasons}"
        stale_count=$((stale_count + 1))
    fi
done

# ── Parse collected data ─────────────────────────────────────────────
if [ "$stale_count" -gt 0 ]; then
    IFS="$SEP" read -ra s_names  <<< "$stale_names"
    IFS="$SEP" read -ra s_phases <<< "$stale_phases"
    IFS="$SEP" read -ra s_paths  <<< "$stale_paths"
    IFS="$SEP" read -ra s_rsns   <<< "$stale_reasons"
fi

# ── Output ──────────────────────────────────────────────────────────
if $JSON_MODE; then
    status="clean"; [ "$stale_count" -gt 0 ] && status="stale"
    if has_jq; then
        items="[]"
        i=0; while [ "$i" -lt "$stale_count" ]; do
            IFS="$RSEP" read -ra r_arr <<< "${s_rsns[$i]}"
            reasons_json=$(printf '%s\n' "${r_arr[@]}" | jq -R . | jq -sc .)
            items=$(echo "$items" | jq \
                --arg name "${s_names[$i]}" --argjson phase "${s_phases[$i]}" \
                --arg path "${s_paths[$i]}" --argjson reasons "$reasons_json" \
                '. + [{name:$name, phase:$phase, path:$path, stale_because:$reasons}]')
            i=$((i + 1))
        done
        jq -cn --arg s "$status" --argjson a "$items" '{status:$s,stale_artifacts:$a}'
    else
        printf '{"status":"%s","stale_artifacts":[' "$status"
        i=0; while [ "$i" -lt "$stale_count" ]; do
            [ "$i" -gt 0 ] && printf ','
            IFS="$RSEP" read -ra r_arr <<< "${s_rsns[$i]}"
            printf '{"name":"%s","phase":%s,"path":"%s","stale_because":[' \
                "$(json_escape "${s_names[$i]}")" "${s_phases[$i]}" "$(json_escape "${s_paths[$i]}")"
            j=0; for reason in "${r_arr[@]}"; do
                [ "$j" -gt 0 ] && printf ','
                printf '"%s"' "$(json_escape "$reason")"
                j=$((j + 1))
            done
            printf ']}'
            i=$((i + 1))
        done
        printf ']}\n'
    fi
else
    echo "Staleness Check"
    if [ "$stale_count" -eq 0 ]; then
        echo "  ✅ No stale artifacts detected"
    else
        i=0; while [ "$i" -lt "$stale_count" ]; do
            echo "  ⚠️  ${s_names[$i]} is STALE"
            IFS="$RSEP" read -ra r_arr <<< "${s_rsns[$i]}"
            for reason in "${r_arr[@]}"; do echo "    - $reason"; done
            i=$((i + 1))
        done
    fi
fi

[ "$stale_count" -gt 0 ] && exit 2 || exit 0
