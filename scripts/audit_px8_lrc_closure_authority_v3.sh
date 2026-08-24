#!/bin/sh
set -eu

if root=$(git rev-parse --show-toplevel 2>/dev/null); then
    commit=$(git rev-parse HEAD)
else
    root=${PX8_V3_SOURCE_ROOT:-$PWD}
    commit=${PX8_V3_COMMIT:-}
    if [ -z "$commit" ]; then
        echo "archive audit requires PX8_V3_COMMIT" >&2
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
require_hash 3805623f6b9ad5d138ba1c90c1b99afb9063c74381cb5545e059254996d7a227 experiments/px8_lrc_cumulative_closure_authority_v3_protocol_v1.md
require_hash 8757ea20d2409bdab5741e2d0201b439cc362a4da234a13682e84d859b848076 experiments/px8_lrc_cumulative_closure_authority_v3_coverage_audit_v1.md
require_hash 7b3bb0c01d42fc2f25b945ab49c50c7a9e40885590c24eb4e5b64ba85ec1475a arms/px8-lrc-closure-authority-v3/src/main.rs
require_hash 322a34a80124ef68577baeab325255336c4111a414385fbd425b2caa4129cd7e arms/px8-lrc-closure-authority-v3/Cargo.toml

source=arms/px8-lrc-closure-authority-v3/src/main.rs
dependencies=$(
    awk '
        /^\[dependencies\]$/ { inside=1; next }
        /^\[/ { inside=0 }
        inside && /^[A-Za-z0-9_-]+[[:space:]]*=/ { print $1 }
    ' arms/px8-lrc-closure-authority-v3/Cargo.toml | LC_ALL=C sort
)
expected=$(printf '%s\n' px7-lrc-arrival px8-lrc-physical-closure | LC_ALL=C sort)
if [ "$dependencies" != "$expected" ]; then
    echo "authority-v3 dependency surface changed" >&2
    exit 1
fi

if grep -Eq 'PX8_LRC_CLOSURE_AUTHORITY_V[12]_EVIDENCE_SPENT|PX8_LRC_CLOSURE_.*DIAGNOSTIC.*SPENT|--authority-v[12]|--.*diagnostic' "$source"; then
    echo "prior authority or diagnostic execution surface leaked into authority v3" >&2
    exit 1
fi
if [ "$(grep -c 'PX8_LRC_CLOSURE_AUTHORITY_V3_EVIDENCE_SPENT' "$source")" -ne 1 ]; then
    echo "authority-v3 marker count changed" >&2
    exit 1
fi
if [ "$(grep -c 'map(replay)' "$source")" -ne 1 ]; then
    echo "authority-v3 row runner surface changed" >&2
    exit 1
fi

for root_value in \
    865_001 865_002 865_003 865_004 865_005 865_006 865_007 865_008 \
    865_009 865_010 865_011 865_012 865_013 865_014 865_015 865_016
do
    grep -q "$root_value" "$source" || {
        echo "registered authority-v3 root missing: $root_value" >&2
        exit 1
    }
done
if grep -Eq '861_00|862_00|863_00|864_00|1_208_' "$source"; then
    echo "prior execution identity leaked into authority v3" >&2
    exit 1
fi

for field in \
    primary uninterrupted incomplete duplicate blocked cumulative \
    before after delta capacity outward route_executions fresh_proposals \
    queue_empty quiet replay
do
    grep -q "$field" "$source" || {
        echo "registered v3 observation field missing: $field" >&2
        exit 1
    }
done

publish_line=$(grep -n 'publish(CSV_S' "$source" | cut -d: -f1)
assert_line=$(grep -n 'assert!(rows.iter().all' "$source" | cut -d: -f1)
if [ -z "$publish_line" ] || [ -z "$assert_line" ] || [ "$publish_line" -ge "$assert_line" ]; then
    echo "authority-v3 results are not published before aggregate assertion" >&2
    exit 1
fi

preflight=$(mktemp)
trap 'rm -f "$preflight"' EXIT HUP INT TERM
awk '
    /^fn absent\(\)/ { inside=1 }
    /^fn authority\(\)/ { inside=0 }
    inside { print }
' "$source" > "$preflight"
if grep -Eq '\b(run|replay|RecursiveBody::new|CompactBody::new|Body::new)[[:space:]]*\(' "$preflight"; then
    echo "authority-v3 preflight can construct or run a body" >&2
    exit 1
fi

mode=${PX8_V3_AUDIT_MODE:-preflight}
case "$mode" in
    preflight)
        for artifact in \
            results/px8_lrc_closure_authority_v3.csv \
            results/px8_lrc_closure_authority_v3.md \
            results/px8_lrc_closure_authority_v3.csv.staging \
            results/px8_lrc_closure_authority_v3.md.staging
        do
            test ! -e "$artifact" || {
                echo "authority-v3 artifact exists during preflight: $artifact" >&2
                exit 1
            }
        done
        ;;
    result)
        test -f results/px8_lrc_closure_authority_v3.csv
        test -f results/px8_lrc_closure_authority_v3.md
        test ! -e results/px8_lrc_closure_authority_v3.csv.staging
        test ! -e results/px8_lrc_closure_authority_v3.md.staging
        awk -F, '
            NR == 1 { if (NF != 32) exit 1; next }
            {
                rows++
                if ($1 != 865000 + rows) exit 1
                if ($17 > 20000 || $18 > 8192 || $23 != 8192) exit 1
                count = split($19, pairs, ";")
                if (count != 6) exit 1
                for (index = 1; index <= count; index++) {
                    split(pairs[index], named, "=")
                    split(named[2], values, "\\|")
                    if (values[1] != values[2]) exit 1
                }
                if ($22 != $21 - $20) exit 1
                if ($24 != 0 || $25 != 0 || $26 < 0) exit 1
                if ($27 != "true" || $28 != "true" || $29 != "true" || $32 != "true") exit 1
                count = split($31, claims, "|")
                if (count != 14) exit 1
                for (index = 1; index <= count; index++) if (claims[index] != "true") exit 1
            }
            END { if (rows != 16) exit 1 }
        ' results/px8_lrc_closure_authority_v3.csv
        grep -Fq 'Outcome: **DEFINITIVE POSITIVE**.' results/px8_lrc_closure_authority_v3.md
        grep -Fq -- '- total clauses: `230/230`;' results/px8_lrc_closure_authority_v3.md
        test "$(grep -c 'memory_before=' results/px8_lrc_closure_authority_v3.md)" -eq 16
        ;;
    *)
        echo "unknown PX8_V3_AUDIT_MODE: $mode" >&2
        exit 1
        ;;
esac

printf 'PX8_LRC_CLOSURE_AUTHORITY_V3_AUDIT_OK commit=%s mode=%s active_sources=4 active_changes=0 evaluator_sources=1 unclassified=0\n' \
    "$commit" "$mode"
