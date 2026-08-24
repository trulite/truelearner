#!/bin/sh
set -eu

if root=$(git rev-parse --show-toplevel 2>/dev/null); then
    commit=$(git rev-parse HEAD)
else
    root=${PX8_V3_RESULT_SOURCE_ROOT:-$PWD}
    commit=${PX8_V3_RESULT_COMMIT:-}
    test -n "$commit" || {
        echo "archive audit requires PX8_V3_RESULT_COMMIT" >&2
        exit 1
    }
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
    test "$actual" = "$expected" || {
        echo "frozen result changed: $file expected=$expected actual=$actual" >&2
        exit 1
    }
}

require_hash cb119201de493a80a3c58756fbafcfec474b5cf7c86b43587360c58e9f088804 scripts/audit_px8_lrc_closure_authority_v3.sh
require_hash 7b3bb0c01d42fc2f25b945ab49c50c7a9e40885590c24eb4e5b64ba85ec1475a arms/px8-lrc-closure-authority-v3/src/main.rs
require_hash 3ed00e8d71392b5ac39f38ce4804cc71337ea86702283fdbe13425ea3240b1fa results/px8_lrc_closure_authority_v3.csv
require_hash f8357997574e875872c42cab361073f08b6cb39b638b587553226bf8e940ed26 results/px8_lrc_closure_authority_v3.md

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
        for (idx = 1; idx <= count; idx++) {
            split(pairs[idx], named, "=")
            split(named[2], values, "\\|")
            if (values[1] != values[2]) exit 1
        }
        if ($22 != $21 - $20) exit 1
        if ($24 != 0 || $25 != 0 || $26 < 0) exit 1
        if ($27 != "true" || $28 != "true" || $29 != "true" || $32 != "true") exit 1
        count = split($31, claims, "|")
        if (count != 14) exit 1
        for (idx = 1; idx <= count; idx++) if (claims[idx] != "true") exit 1
    }
    END { if (rows != 16) exit 1 }
' results/px8_lrc_closure_authority_v3.csv

grep -Fq 'Outcome: **DEFINITIVE POSITIVE**.' results/px8_lrc_closure_authority_v3.md
grep -Fq -- '- total clauses: `230/230`;' results/px8_lrc_closure_authority_v3.md
test "$(grep -c 'memory_before=' results/px8_lrc_closure_authority_v3.md)" -eq 16

printf 'PX8_LRC_CLOSURE_AUTHORITY_V3_RESULT_AUDIT_OK commit=%s rows=16 clauses=230 stale_outward=0 stale_route_executions=0\n' "$commit"
