#!/usr/bin/env bash
# Pipeline audit — validates Founder OS artifacts against schema-registry.yml
# Usage: pipeline-audit.sh --phase <N> | --all [--json] [--help]
set -euo pipefail
SCRIPT_DIR="$(CDPATH="" cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"
REPO_ROOT=$(get_repo_root)
REGISTRY="$REPO_ROOT/.specify/artifacts/schema-registry.yml"

phase_label() {
    case "$1" in
        1) echo "Phase 1: Reality Check";; 2) echo "Phase 2: Design & UX";;
        3) echo "Phase 3: Orchestration & Build";; 4) echo "Phase 4: Launch & Iterate";;
        *) echo "Phase $1";;
    esac
}

# --- Argument parsing ---
TARGET_PHASE=""; ALL_PHASES=false; JSON_MODE=false
while [[ $# -gt 0 ]]; do
    case "$1" in
        --phase) [[ -z "${2:-}" ]] && { echo "ERROR: --phase requires a value" >&2; exit 1; }
                 TARGET_PHASE="$2"; shift 2;;
        --all)   ALL_PHASES=true; shift;;
        --json)  JSON_MODE=true; shift;;
        --help|-h) cat <<'EOF'
Usage: pipeline-audit.sh [OPTIONS]
  --phase <N>   Validate a single phase (1, 2, 3, or 4)
  --all         Validate all phases
  --json        Output machine-readable JSON
  --help, -h    Show this help
Exit codes: 0=pass, 1=fail/missing, 2=stubs only
EOF
                 exit 0;;
        *) echo "ERROR: Unknown option '$1'. Use --help for usage." >&2; exit 1;;
    esac
done
[[ -z "$TARGET_PHASE" ]] && ! $ALL_PHASES && { echo "ERROR: Specify --phase <N> or --all." >&2; exit 1; }
[[ ! -f "$REGISTRY" ]] && { echo "ERROR: Schema registry not found: $REGISTRY" >&2; exit 1; }

# --- YAML parser (pure bash, 2-space indentation) ---
# Populates parallel arrays: PHASE_NUMS, FILENAMES, HEADINGS (;-delimited), MIN_LINES,
#   GATE_CRITERIA, PRODUCING_AGENTS
PHASE_NUMS=(); FILENAMES=(); HEADINGS=(); MIN_LINES=()
GATE_CRITERIA=(); PRODUCING_AGENTS=()
parse_registry() {
    local cur_phase="" cur_file="" hdg_buf="" min_l="" gate_c="" prod_a=""
    flush() {
        if [[ -n "$cur_file" ]]; then
            PHASE_NUMS+=("$cur_phase"); FILENAMES+=("$cur_file")
            HEADINGS+=("$hdg_buf"); MIN_LINES+=("${min_l:-0}")
            GATE_CRITERIA+=("$gate_c"); PRODUCING_AGENTS+=("$prod_a")
        fi
        hdg_buf=""; min_l=""; gate_c=""; prod_a=""
    }
    while IFS= read -r line || [[ -n "$line" ]]; do
        [[ -z "$line" || "$line" =~ ^[[:space:]]*# ]] && continue
        [[ "$line" =~ ^artifacts: ]] && continue
        if [[ "$line" =~ ^[[:space:]]{2}phase-([0-9]+): ]]; then
            flush; cur_file=""; cur_phase="${BASH_REMATCH[1]}"; continue; fi
        if [[ "$line" =~ ^[[:space:]]{4}([a-zA-Z0-9._-]+\.md): ]]; then
            flush; cur_file="${BASH_REMATCH[1]}"; continue; fi
        if [[ "$line" =~ ^[[:space:]]{8,}-[[:space:]]\"(.+)\" ]]; then
            local h="${BASH_REMATCH[1]}"
            [[ -n "$hdg_buf" ]] && hdg_buf="$hdg_buf;$h" || hdg_buf="$h"; continue; fi
        [[ "$line" =~ min_content_lines:[[:space:]]*([0-9]+) ]] && { min_l="${BASH_REMATCH[1]}"; continue; }
        if [[ "$line" =~ gate_criteria:[[:space:]]*\"(.+)\" ]]; then
            gate_c="${BASH_REMATCH[1]}"; continue; fi
        if [[ "$line" =~ producing_agent:[[:space:]]*\"(.+)\" ]]; then
            prod_a="${BASH_REMATCH[1]}"; continue; fi
    done < "$REGISTRY"
    flush
}
parse_registry

# --- Determine phases to validate ---
PHASES=()
if $ALL_PHASES; then
    for p in "${PHASE_NUMS[@]}"; do
        _dup=false
        for existing in "${PHASES[@]+"${PHASES[@]}"}"; do
            [[ "$existing" == "$p" ]] && { _dup=true; break; }
        done
        $_dup || PHASES+=("$p")
    done
else PHASES=("$TARGET_PHASE"); fi

# --- Section word-count analysis ---
# Given a file and ;-delimited headings, produce ;-delimited "heading:wordcount" pairs
# and a depth rating (robust|adequate|thin|skeletal) for each section.
analyze_sections() {
    local filepath="$1" headings_str="$2"
    [[ ! -f "$filepath" || -z "$headings_str" ]] && return
    IFS=';' read -ra hdgs <<< "$headings_str"
    local result="" thin=0 skeletal=0
    for h in "${hdgs[@]}"; do
        [[ -z "$h" ]] && continue
        # Extract content between this heading and the next heading of same or higher level
        local level hpat wc depth
        # Count leading '#' characters to determine heading level.
        # grep -o outputs the matched hashes plus a trailing newline; wc -c counts
        # that total byte length, so we subtract 1 to remove the newline byte.
        level=$(echo "$h" | grep -o '^#*' | wc -c)
        level=$((level - 1))
        # Skip entries that are not valid Markdown headings (no leading '#')
        [[ "$level" -lt 1 ]] && continue
        # Build a regex matching headings of the same or higher (fewer '#') level
        hpat=""
        for ((ll=1; ll<=level; ll++)); do hpat="${hpat}#"; done
        # Extract section content: from the heading line to the next heading of same/higher level.
        # Heading level is compared by counting leading '#' characters, not by line length.
        wc=$(awk -v hdr="$h" -v max_level="$level" '
            BEGIN { found=0; total=0 }
            found && /^#{1,}[[:space:]]/ {
                # Count leading # characters in this line
                match($0, /^#+/)
                cur_level = RLENGTH
                if (cur_level <= max_level) exit
            }
            found { gsub(/^[[:space:]]+|[[:space:]]+$/, ""); if (NF>0) total+=NF }
            index($0, hdr) == 1 { found=1 }
            END { print total+0 }
        ' "$filepath" 2>/dev/null || echo 0)
        if [[ "$wc" -ge 100 ]]; then depth="robust"
        elif [[ "$wc" -ge 30 ]]; then depth="adequate"
        elif [[ "$wc" -ge 10 ]]; then depth="thin"; thin=$((thin+1))
        else depth="skeletal"; skeletal=$((skeletal+1))
        fi
        local entry="${h}:${wc}:${depth}"
        [[ -n "$result" ]] && result="$result|$entry" || result="$entry"
    done
    echo "${result}|thin=${thin}|skeletal=${skeletal}"
}

# --- Validate artifacts ---
R_ST=(); R_ISS=(); R_STUB=(); R_SECTIONS=()  # parallel result arrays
TOTAL=0; PASS=0; FAIL=0; MISSING=0; STUBS=0
TOTAL_THIN=0; TOTAL_SKELETAL=0
for idx in "${!PHASE_NUMS[@]}"; do
    phase="${PHASE_NUMS[$idx]}"; match=false
    for p in "${PHASES[@]}"; do [[ "$p" == "$phase" ]] && { match=true; break; }; done
    $match || { R_ST+=("skip"); R_ISS+=(""); R_STUB+=(0); R_SECTIONS+=(""); continue; }
    file="${FILENAMES[$idx]}"
    path="$REPO_ROOT/.specify/artifacts/phase-${phase}/${file}"
    issues=""; is_stub=0; TOTAL=$((TOTAL + 1))
    section_data=""
    # Existence & emptiness
    if [[ ! -f "$path" ]]; then
        R_ST+=("missing"); R_ISS+=(""); R_STUB+=(0); R_SECTIONS+=(""); MISSING=$((MISSING+1)); continue; fi
    if [[ ! -s "$path" ]]; then
        R_ST+=("fail"); R_ISS+=("file is empty"); R_STUB+=(0); R_SECTIONS+=(""); FAIL=$((FAIL+1)); continue; fi
    # Stub check
    grep -q "^STUB:" "$path" 2>/dev/null && { is_stub=1; STUBS=$((STUBS+1)); }
    # Min content lines
    lc=$(grep -c '.' "$path" 2>/dev/null || echo 0); min="${MIN_LINES[$idx]}"
    [[ "$lc" -lt "$min" ]] && issues="below min lines (${lc}/${min})"
    # Required headings
    IFS=';' read -ra hdgs <<< "${HEADINGS[$idx]}"
    for h in "${hdgs[@]}"; do
        [[ -z "$h" ]] && continue
        if ! grep -qF "$h" "$path" 2>/dev/null; then
            _msg="missing heading: $h"
            [[ -n "$issues" ]] && issues="$issues;$_msg" || issues="$_msg"
        fi
    done
    # Section depth analysis (only for files that exist and have headings)
    if [[ -n "${HEADINGS[$idx]}" ]]; then
        section_data=$(analyze_sections "$path" "${HEADINGS[$idx]}" || true)
        # Extract thin/skeletal counts from section_data
        if [[ "$section_data" =~ thin=([0-9]+) ]]; then TOTAL_THIN=$((TOTAL_THIN + BASH_REMATCH[1])); fi
        if [[ "$section_data" =~ skeletal=([0-9]+) ]]; then TOTAL_SKELETAL=$((TOTAL_SKELETAL + BASH_REMATCH[1])); fi
    fi
    if [[ -n "$issues" ]]; then R_ST+=("fail"); FAIL=$((FAIL+1))
    else R_ST+=("pass"); PASS=$((PASS+1)); fi
    R_ISS+=("$issues"); R_STUB+=("$is_stub"); R_SECTIONS+=("$section_data")
done

# Overall status
if [[ "$FAIL" -gt 0 || "$MISSING" -gt 0 ]]; then OVERALL="fail"
elif [[ "$STUBS" -gt 0 ]]; then OVERALL="warn"
else OVERALL="pass"; fi

# --- JSON output ---
if $JSON_MODE; then
    phases_json=""
    for phase in "${PHASES[@]}"; do
        arts=""; pst="pass"
        for idx in "${!PHASE_NUMS[@]}"; do
            [[ "${PHASE_NUMS[$idx]}" != "$phase" || "${R_ST[$idx]}" == "skip" ]] && continue
            st="${R_ST[$idx]}"; [[ "$st" == "fail" || "$st" == "missing" ]] && pst="fail"
            iss_arr=""
            if [[ -n "${R_ISS[$idx]}" ]]; then
                IFS=';' read -ra parts <<< "${R_ISS[$idx]}"
                for part in "${parts[@]}"; do
                    [[ -z "$part" ]] && continue
                    [[ -n "$iss_arr" ]] && iss_arr="$iss_arr,"
                    iss_arr="${iss_arr}\"$(json_escape "$part")\""
                done
            fi
            # Build section_details JSON array from R_SECTIONS
            sec_arr=""
            if [[ -n "${R_SECTIONS[$idx]}" ]]; then
                IFS='|' read -ra sec_parts <<< "${R_SECTIONS[$idx]}"
                for sp in "${sec_parts[@]}"; do
                    # Skip the summary entries (thin=N, skeletal=N)
                    [[ "$sp" =~ ^(thin|skeletal)= ]] && continue
                    [[ -z "$sp" ]] && continue
                    # Format: heading:wordcount:depth
                    s_heading="${sp%%:*}"; rest="${sp#*:}"
                    s_wc="${rest%%:*}"; s_depth="${rest#*:}"
                    [[ -n "$sec_arr" ]] && sec_arr="$sec_arr,"
                    sec_arr="${sec_arr}{\"heading\":\"$(json_escape "$s_heading")\",\"word_count\":${s_wc},\"depth\":\"$s_depth\"}"
                done
            fi
            # Include gate_criteria and producing_agent
            gc_json="\"$(json_escape "${GATE_CRITERIA[$idx]}")\""
            pa_json="\"$(json_escape "${PRODUCING_AGENTS[$idx]}")\""
            [[ -n "$arts" ]] && arts="$arts,"
            arts="${arts}{\"name\":\"$(json_escape "${FILENAMES[$idx]}")\",\"status\":\"$st\",\"issues\":[$iss_arr],\"gate_criteria\":$gc_json,\"producing_agent\":$pa_json,\"sections\":[$sec_arr]}"
        done
        [[ -n "$phases_json" ]] && phases_json="$phases_json,"
        phases_json="${phases_json}\"$phase\":{\"status\":\"$pst\",\"artifacts\":[$arts]}"
    done
    printf '{"status":"%s","phases":{%s},"summary":{"total":%d,"pass":%d,"fail":%d,"missing":%d,"stubs":%d,"thin_sections":%d,"skeletal_sections":%d}}\n' \
        "$OVERALL" "$phases_json" "$TOTAL" "$PASS" "$FAIL" "$MISSING" "$STUBS" "$TOTAL_THIN" "$TOTAL_SKELETAL"
# --- Human-readable output ---
else
    for phase in "${PHASES[@]}"; do
        echo "$(phase_label "$phase")"
        for idx in "${!PHASE_NUMS[@]}"; do
            [[ "${PHASE_NUMS[$idx]}" != "$phase" || "${R_ST[$idx]}" == "skip" ]] && continue
            name="${FILENAMES[$idx]}"; st="${R_ST[$idx]}"
            stub=""; [[ "${R_STUB[$idx]}" == "1" ]] && stub=" [STUB]"
            case "$st" in
                pass)    echo "  ✅ ${name} (PASS)${stub}";;
                missing) echo "  ❌ ${name} (MISSING)";;
                fail)    IFS=';' read -ra parts <<< "${R_ISS[$idx]}"
                         echo "  ❌ ${name} (FAIL: ${parts[0]})${stub}"
                         for ((i=1; i<${#parts[@]}; i++)); do echo "     ↳ ${parts[$i]}"; done;;
            esac
            # Show section depth details for passing/failing artifacts that have sections
            if [[ "$st" == "pass" || "$st" == "fail" ]] && [[ -n "${R_SECTIONS[$idx]}" ]]; then
                IFS='|' read -ra sec_parts <<< "${R_SECTIONS[$idx]}"
                has_weak=false
                for sp in "${sec_parts[@]}"; do
                    [[ "$sp" =~ ^(thin|skeletal)= ]] && continue
                    [[ -z "$sp" ]] && continue
                    s_depth="${sp##*:}"
                    if [[ "$s_depth" == "thin" || "$s_depth" == "skeletal" ]]; then
                        has_weak=true; break
                    fi
                done
                if $has_weak; then
                    for sp in "${sec_parts[@]}"; do
                        [[ "$sp" =~ ^(thin|skeletal)= ]] && continue
                        [[ -z "$sp" ]] && continue
                        s_heading="${sp%%:*}"; rest="${sp#*:}"
                        s_wc="${rest%%:*}"; s_depth="${rest#*:}"
                        if [[ "$s_depth" == "thin" ]]; then
                            echo "     🟡 ${s_heading} — thin (${s_wc} words)"
                        elif [[ "$s_depth" == "skeletal" ]]; then
                            echo "     🟠 ${s_heading} — skeletal (${s_wc} words)"
                        fi
                    done
                fi
            fi
            # Show gate criteria for passing artifacts
            if [[ "$st" == "pass" && -n "${GATE_CRITERIA[$idx]}" ]]; then
                echo "     📋 Gate criteria: ${GATE_CRITERIA[$idx]}"
            fi
        done
        echo ""
    done
    echo "Summary: ${TOTAL} artifacts — ${PASS} pass, ${FAIL} fail, ${MISSING} missing"
    [[ "$STUBS" -gt 0 ]] && echo "⚠️  ${STUBS} stub(s) detected"
    [[ "$TOTAL_THIN" -gt 0 || "$TOTAL_SKELETAL" -gt 0 ]] && echo "📊 Section depth: ${TOTAL_THIN} thin, ${TOTAL_SKELETAL} skeletal sections across all artifacts"
fi

# Exit code: 0=pass, 1=fail/missing, 2=stubs only
[[ "$FAIL" -gt 0 || "$MISSING" -gt 0 ]] && exit 1
[[ "$STUBS" -gt 0 ]] && exit 2
exit 0
