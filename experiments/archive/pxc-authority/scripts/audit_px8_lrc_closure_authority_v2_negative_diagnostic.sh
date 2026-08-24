#!/bin/sh
set -eu

if root=$(git rev-parse --show-toplevel 2>/dev/null); then
    commit=$(git rev-parse HEAD)
else
    root=${PX8_V2_DIAGNOSTIC_SOURCE_ROOT:-$PWD}
    commit=${PX8_V2_DIAGNOSTIC_COMMIT:-}
    test -n "$commit" || { echo "archive audit requires PX8_V2_DIAGNOSTIC_COMMIT" >&2; exit 1; }
fi
cd "$root"

sha() {
    if command -v sha256sum >/dev/null 2>&1; then sha256sum "$1" | awk '{print $1}';
    else shasum -a 256 "$1" | awk '{print $1}'; fi
}
require_hash() {
    expected=$1; file=$2; actual=$(sha "$file")
    test "$actual" = "$expected" || { echo "frozen input changed: $file expected=$expected actual=$actual" >&2; exit 1; }
}

require_hash 7226a0e4af0ff484c6fd61c46c9073ce8363692100c2a090b0ce64483f3cfc10 crates/lr1-modulatory-physical-return/src/lib.rs
require_hash a201674f9d558b5bda20aef71e9857b632f8a6565f372aee88994a280e0fea71 arms/px4-lrc-lifetime/src/lib.rs
require_hash d248a8af479872d8148115a405ae7332f7d24ca229378d3fde898ffd3d19e63e crates/px7-lrc-arrival/src/lib.rs
require_hash 8623cae7dc1b14e666140a192c49e59ae72df681bc14f0359f0c4465e0d11e8f arms/px8-lrc-physical-closure/src/lib.rs
require_hash e1a830e15c898b113f295d74e22f6dee1d144bd43ee1aa177d4a7c0ef075043c arms/px8-lrc-closure-authority-v2/src/main.rs
require_hash a47866460ecc4504ee713e0b425d049e7816f48e4aa18bceeb0a1705dcbc5328 experiments/px8_lrc_cumulative_closure_authority_v2_protocol_v1.md
require_hash 2a527fecb9906e4bdf4bce703a760e646439f205ec2eee6a8288a38de6cc1620 experiments/px8_lrc_cumulative_closure_authority_v2_negative_v1.md
require_hash 40bafda8f6caa2cf3bce08fbdd34dfe9802aa4c392b6ebb85ab646fab752fa2a experiments/px8_lrc_closure_authority_v2_negative_diagnostic_protocol_v1.md
require_hash 5ebeaa6e684a800407e078ba0e63d213d31b08aa336dbf2722f4b441721f6635 arms/px8-lrc-closure-authority-v2-diagnostic/src/main.rs
require_hash 2f5dac8b1a21c9e57618ebded05e274ecc13d65742c9a7ad0f875d5c78e9286b arms/px8-lrc-closure-authority-v2-diagnostic/Cargo.toml

source=arms/px8-lrc-closure-authority-v2-diagnostic/src/main.rs
dependencies=$(awk '/^\[dependencies\]$/ {on=1; next} /^\[/ {on=0} on && /^[A-Za-z0-9_-]+[[:space:]]*=/ {print $1}' arms/px8-lrc-closure-authority-v2-diagnostic/Cargo.toml | LC_ALL=C sort)
expected=$(printf '%s\n' px7-lrc-arrival px8-lrc-physical-closure | LC_ALL=C sort)
test "$dependencies" = "$expected" || { echo "diagnostic dependency surface changed" >&2; exit 1; }

for forbidden in PX8_LRC_CLOSURE_AUTHORITY_V1_EVIDENCE_SPENT PX8_LRC_CLOSURE_AUTHORITY_V2_EVIDENCE_SPENT; do
    ! grep -Fq "$forbidden" "$source" || { echo "authority marker leaked into diagnostic" >&2; exit 1; }
done
! grep -En -- '--authority-v1|--authority-v2|--diagnostic-v1' "$source" || { echo "prior execution mode leaked" >&2; exit 1; }
test "$(grep -c 'PX8_LRC_CLOSURE_AUTHORITY_V2_NEGATIVE_DIAGNOSTIC_SPENT' "$source")" -eq 1 || { echo "diagnostic marker count changed" >&2; exit 1; }
test "$(grep -c 'map(reconstruct)' "$source")" -eq 1 || { echo "diagnostic runner surface changed" >&2; exit 1; }

for root_value in 864_001 864_002 864_003 864_004 864_005 864_006 864_007 864_008 864_009 864_010 864_011 864_012 864_013 864_014 864_015 864_016; do
    grep -q "$root_value" "$source" || { echo "diagnostic root missing: $root_value" >&2; exit 1; }
done
! grep -En '861_00|862_00|863_00|1_208_' "$source" || { echo "prior identity leaked" >&2; exit 1; }

mode=${PX8_V2_DIAGNOSTIC_AUDIT_MODE:-preflight}
case "$mode" in
preflight)
    for artifact in results/px8_lrc_closure_authority_v2_negative_diagnostic.csv results/px8_lrc_closure_authority_v2_negative_diagnostic.md results/px8_lrc_closure_authority_v2_negative_diagnostic.csv.staging results/px8_lrc_closure_authority_v2_negative_diagnostic.md.staging; do
        test ! -e "$artifact" || { echo "diagnostic artifact exists during preflight: $artifact" >&2; exit 1; }
    done
    ;;
result)
    test -f results/px8_lrc_closure_authority_v2_negative_diagnostic.csv
    test -f results/px8_lrc_closure_authority_v2_negative_diagnostic.md
    test ! -e results/px8_lrc_closure_authority_v2_negative_diagnostic.csv.staging
    test ! -e results/px8_lrc_closure_authority_v2_negative_diagnostic.md.staging
    awk -F, 'NR==1 {if(NF!=21) exit 1; next} {records++; roots[$1]++; clauses[$1 SUBSEP $7]++; if($19!="true") replay_failures++} END {if(records!=224) exit 1; n=0; for(r in roots){n++; if(roots[r]!=14) exit 1} if(n!=16) exit 1; for(c in clauses) if(clauses[c]!=1) exit 1; if(replay_failures!=0) exit 1}' results/px8_lrc_closure_authority_v2_negative_diagnostic.csv
    grep -Fq 'Outcome: **DIAGNOSTIC COMPLETE; NOT AUTHORITY**.' results/px8_lrc_closure_authority_v2_negative_diagnostic.md
    grep -Fq -- '- roots serialized: `16/16`;' results/px8_lrc_closure_authority_v2_negative_diagnostic.md
    grep -Fq -- '- clause records serialized: `224/224`;' results/px8_lrc_closure_authority_v2_negative_diagnostic.md
    ;;
*) echo "unknown audit mode: $mode" >&2; exit 1;;
esac

printf 'PX8_LRC_CLOSURE_AUTHORITY_V2_NEGATIVE_DIAGNOSTIC_AUDIT_OK commit=%s mode=%s active_changes=0 evaluator_sources=1 unclassified=0\n' "$commit" "$mode"
