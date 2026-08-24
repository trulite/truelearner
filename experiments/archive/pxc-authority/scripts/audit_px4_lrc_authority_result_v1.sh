#!/bin/sh
set -eu

csv=results/px4_lrc_lifetime_authority_v1.csv
report=results/px4_lrc_lifetime_authority_v1.md

printf '%s  %s\n' \
    050a2b489e41d13e8d8a3d55dd7d69c6e06894b85b2c172f7dc24614af09aeaa "$csv" \
    445c465ba61cc12c0ece84a8ebb9a83bea1e67c1a4d640964cc7d93c0dbe4390 "$report" \
    | sha256sum -c -

awk -F, '
NR == 1 {
    if (NF != 44) exit 1
    next
}
{
    rows += 1
    marks[$5] = 1
    origins[$8] += 1
    replicates[$9] += 1
    clauses += $40
    if ($1 != "authority") exit 1
    if ($2 != "px4-lrc-cumulative-lifetime-authority-v1") exit 1
    if ($3 != "f9057fe78a86db9111b0b69310d03accef3bc970") exit 1
    if ($10 != "4|7|12|22" || $11 != "4|7|12|22") exit 1
    if ($12 != "true" || $13 != 4 || $14 != 2) exit 1
    if ($15 != "true" || $16 != "true" || $17 != "true") exit 1
    if ($18 != "true" || $19 != 2 || $20 != "true" || $21 != "true") exit 1
    if ($24 != "true" || $25 != "true" || $27 != "true") exit 1
    if ($28 != "true" || $30 != 1 || $31 != 1 || $32 != "true") exit 1
    if ($33 != "true" || $34 != "true" || $35 != "true") exit 1
    if ($36 != "true" || $37 != "true") exit 1
    if ($38 != "true" || $39 != "true") exit 1
    if ($40 != 41 || $41 != 41 || $42 != "true") exit 1
    if ($43 != "true" || $44 != "PASS") exit 1
}
END {
    for (mark = 461001; mark <= 461016; mark += 1) {
        if (!(mark in marks)) exit 1
    }
    if (rows != 16 || length(marks) != 16) exit 1
    if (origins[200] != 8 || origins[400] != 8) exit 1
    if (replicates[1] != 8 || replicates[2] != 8) exit 1
    if (clauses + 1 != 657) exit 1
    print "PX4 result audit: rows=16/16 clauses=657/657 hashes=PASS controls=PASS replay=PASS quiescence=PASS conformance=PASS"
}' "$csv"
