#!/bin/sh
set -eu

if root=$(git rev-parse --show-toplevel 2>/dev/null); then
    commit=$(git rev-parse HEAD)
else
    root=${PX8_V2_SOURCE_ROOT:-$PWD}
    commit=${PX8_V2_COMMIT:-}
    if [ -z "$commit" ]; then
        echo "archive audit requires PX8_V2_COMMIT" >&2
        exit 1
    fi
fi
cd "$root"

sha() {
    if command -v sha256sum >/dev/null 2>&1; then
        sha256sum "$1" | awk '{print $1}'
    else
        shasum -a 256 "$1" | awk '{print $1}'
    fi
}

require_hash() {
    expected=$1
    file=$2
    actual=$(sha "$file")
    if [ "$actual" != "$expected" ]; then
        echo "frozen input changed: $file expected=$expected actual=$actual" >&2
        exit 1
    fi
}

require_hash 7226a0e4af0ff484c6fd61c46c9073ce8363692100c2a090b0ce64483f3cfc10 crates/lr1-modulatory-physical-return/src/lib.rs
require_hash a201674f9d558b5bda20aef71e9857b632f8a6565f372aee88994a280e0fea71 arms/px4-lrc-lifetime/src/lib.rs
require_hash d248a8af479872d8148115a405ae7332f7d24ca229378d3fde898ffd3d19e63e crates/px7-lrc-arrival/src/lib.rs
require_hash 8623cae7dc1b14e666140a192c49e59ae72df681bc14f0359f0c4465e0d11e8f arms/px8-lrc-physical-closure/src/lib.rs
require_hash c07f16515a5d4244242130c0eba82374a28a24d0564f21721c6c523943c5ec60 results/px8_lrc_closure_negative_v1_diagnostic.csv
require_hash 9f8a6abdccc1e97a07555d232bc56507e7056c67c9d1d231eec0d0ff3be7f8a5 results/px8_lrc_closure_negative_v1_diagnostic.md
require_hash a47866460ecc4504ee713e0b425d049e7816f48e4aa18bceeb0a1705dcbc5328 experiments/px8_lrc_cumulative_closure_authority_v2_protocol_v1.md
require_hash 8f20eebe4b9e27fbeddc44014bc5ca8af120f19bcdb499a790056d9191b6e81a experiments/px8_lrc_cumulative_closure_authority_v2_coverage_audit_v1.md
require_hash e1a830e15c898b113f295d74e22f6dee1d144bd43ee1aa177d4a7c0ef075043c arms/px8-lrc-closure-authority-v2/src/main.rs
require_hash 38a00f32ccfef870b7e128d7413e60a378bef832f425377ac8324b9465a4e650 arms/px8-lrc-closure-authority-v2/Cargo.toml

source=arms/px8-lrc-closure-authority-v2/src/main.rs
dependencies=$(
    awk '
        /^\[dependencies\]$/ { inside=1; next }
        /^\[/ { inside=0 }
        inside && /^[A-Za-z0-9_-]+[[:space:]]*=/ { print $1 }
    ' arms/px8-lrc-closure-authority-v2/Cargo.toml | LC_ALL=C sort
)
expected=$(printf '%s\n' px7-lrc-arrival px8-lrc-physical-closure | LC_ALL=C sort)
if [ "$dependencies" != "$expected" ]; then
    echo "authority-v2 dependency surface changed" >&2
    exit 1
fi

if grep -Fq 'PX8_LRC_CLOSURE_AUTHORITY_V1_EVIDENCE_SPENT' "$source" || \
   grep -Fq 'PX8_LRC_CLOSURE_NEGATIVE_V1_DIAGNOSTIC_SPENT' "$source" || \
   grep -En -- '--authority-v1|--diagnostic-v1' "$source"; then
    echo "v1 or diagnostic execution surface leaked into authority v2" >&2
    exit 1
fi
if [ "$(grep -c 'PX8_LRC_CLOSURE_AUTHORITY_V2_EVIDENCE_SPENT' "$source")" -ne 1 ]; then
    echo "authority-v2 marker count changed" >&2
    exit 1
fi
if [ "$(grep -c 'map(replay)' "$source")" -ne 1 ]; then
    echo "authority-v2 row runner surface changed" >&2
    exit 1
fi

for root_value in \
    863_001 863_002 863_003 863_004 863_005 863_006 863_007 863_008 \
    863_009 863_010 863_011 863_012 863_013 863_014 863_015 863_016
do
    grep -q "$root_value" "$source" || {
        echo "registered authority-v2 root missing: $root_value" >&2
        exit 1
    }
done
if grep -En '861_00|861_01|862_00|862_01|1_208_' "$source"; then
    echo "prior execution identity leaked into authority v2" >&2
    exit 1
fi

for field in \
    primary_before primary_after uninterrupted_before uninterrupted_after \
    incomplete_before incomplete_after duplicate_before duplicate_after \
    blocked_before blocked_after stale_before stale_after \
    cumulative_before cumulative_after
do
    grep -q "$field" "$source" || {
        echo "registered same-body memory field missing: $field" >&2
        exit 1
    }
done

preflight=$(mktemp)
trap 'rm -f "$preflight"' EXIT HUP INT TERM
awk '
    /^fn preflight\(\)/ { inside=1 }
    /^fn authority\(/ { inside=0 }
    inside { print }
' "$source" > "$preflight"
if grep -En '\b(run|replay|RecursiveBody::new|CompactBody::new|Body::new)[[:space:]]*\(' "$preflight"; then
    echo "authority-v2 preflight can construct or run a body" >&2
    exit 1
fi

mode=${PX8_V2_AUDIT_MODE:-preflight}
case "$mode" in
    preflight)
        for artifact in \
            results/px8_lrc_closure_authority_v2.csv \
            results/px8_lrc_closure_authority_v2.md \
            results/px8_lrc_closure_authority_v2.csv.staging \
            results/px8_lrc_closure_authority_v2.md.staging
        do
            test ! -e "$artifact" || {
                echo "authority-v2 artifact exists during preflight: $artifact" >&2
                exit 1
            }
        done
        ;;
    result)
        test -f results/px8_lrc_closure_authority_v2.csv
        test -f results/px8_lrc_closure_authority_v2.md
        test ! -e results/px8_lrc_closure_authority_v2.csv.staging
        test ! -e results/px8_lrc_closure_authority_v2.md.staging
        awk -F, '
            NR == 1 { if (NF != 44) exit 1; next }
            {
                rows++
                if ($26 != $27 || $28 != $29 || $30 != $31 || $32 != $33 ||
                    $34 != $35 || $36 != $37 || $38 != $39) exit 1
                if ($40 != "true" || $41 != "true" || $42 != "true" || $44 != "true") exit 1
                count = split($43, claims, "|")
                if (count != 14) exit 1
                for (index = 1; index <= count; index++) if (claims[index] != "true") exit 1
            }
            END { if (rows != 16) exit 1 }
        ' results/px8_lrc_closure_authority_v2.csv
        grep -Fq 'Outcome: **DEFINITIVE POSITIVE**.' results/px8_lrc_closure_authority_v2.md
        grep -Fq -- '- total clauses: `230/230`;' results/px8_lrc_closure_authority_v2.md
        ;;
    *)
        echo "unknown PX8_V2_AUDIT_MODE: $mode" >&2
        exit 1
        ;;
esac

printf 'PX8_LRC_CLOSURE_AUTHORITY_V2_AUDIT_OK commit=%s mode=%s active_sources=4 active_changes=0 evaluator_sources=1 unclassified=0\n' \
    "$commit" "$mode"
