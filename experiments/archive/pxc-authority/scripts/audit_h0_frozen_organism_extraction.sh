#!/bin/sh
set -eu

root=$(git rev-parse --show-toplevel)
cd "$root"

sha256_file() {
    shasum -a 256 "$1" | awk '{print $1}'
}

require_hash() {
    expected=$1
    path=$2
    actual=$(sha256_file "$path")
    if [ "$actual" != "$expected" ]; then
        echo "frozen source mismatch: $path" >&2
        echo "expected $expected" >&2
        echo "actual   $actual" >&2
        exit 1
    fi
}

require_literal() {
    path=$1
    literal=$2
    if ! rg -F -q "$literal" "$path"; then
        echo "required audit evidence absent: $path: $literal" >&2
        exit 1
    fi
}

forbid_literal() {
    path=$1
    literal=$2
    if rg -F -q "$literal" "$path"; then
        echo "unexpected runtime surface present: $path: $literal" >&2
        exit 1
    fi
}

require_hash \
    6a8590b904403dfa880198f7acf8daf843864dee1a2ea0230d964d928f4076d1 \
    src/post_m7_ds5_closure_emission.rs
require_hash \
    67e170f12d7b7649a0a291ddfc16cd80e4b5c15564b65cd09c884f3e52b9ac5b \
    src/post_m6_ds4_arrival_initiation.rs
require_hash \
    b65b28256d58c184b41bf2ff8d383c99593e6d812480751684209dce1d82f99a \
    src/ds4_cumulative_request_start_port.rs
require_hash \
    8a17e7a5fda9519ad0d4a9d29d04d2434dd5b5ee857e74c1296c5f8b3b06f897 \
    src/iteration.rs
require_hash \
    6aa28a76e1362ac8dfb1d33fb68807da40e7604dfdc8cca9efa1e314e3ce4263 \
    crates/frozen-organism-v1-physics/src/substrate.rs

m8=$(git rev-parse m8-cumulative-closure-emission-authoritative^{commit})
if [ "$m8" != "4ba88b6ed03b8e012231363fa6e3c29ea41308bb" ]; then
    echo "M8 authority moved: $m8" >&2
    exit 1
fi

require_literal src/ds4_cumulative_request_start_port.rs \
    'let stream = standard_stream(seed + episode as u64, RenderOptions::default())?;'
require_literal src/ds4_cumulative_request_start_port.rs \
    'let (roles, program) = fixed_roles_and_program(seed);'
require_literal src/post_m7_ds5_closure_emission.rs \
    'let mut operation = frozen_iterable_operation();'
require_literal src/post_m7_ds5_closure_emission.rs \
    'operation.lookup.begin_episode();'
require_literal src/post_m7_ds5_closure_emission.rs \
    'operation.lookup.erase_temporary();'
require_literal src/post_m7_ds5_closure_emission.rs \
    'closure_fixture: Option<ClosureFixture>,'
require_literal src/post_m6_ds4_arrival_initiation.rs \
    'let active_encounter = productive(gate.seed, ordinal, ordinal.is_multiple_of(2));'
require_literal src/post_m6_ds4_arrival_initiation.rs \
    'let other = contrast(gate.seed, 0, false).snapshot();'
require_literal src/post_m6_ds4_arrival_initiation.rs \
    'raw(gate.seed + 100_000, gate.episode, 0, false),'

forbid_literal crates/frozen-organism-v1-physics/src/lib.rs 'struct FrozenOrganismV1'
forbid_literal crates/frozen-organism-v1-physics/src/lib.rs 'fn arrive('
forbid_literal crates/frozen-organism-v1-physics/src/lib.rs 'fn advance('

printf '%s\n' \
    'surface,status,evidence' \
    'blank_construction,FIXTURE-BOUND,"event gate acquires generated standard_stream histories; request path installs fixed roles/program"' \
    'anonymous_external_arrival,NEW-REPRESENTATION,"M8 consumes generated Episode relations/query and has no raw-arrival adapter"' \
    'continuous_advancement,SEMANTIC-BOUNDARY,"execution calls begin_episode and erase_temporary"' \
    'outward_crossing,EXTRACTABLE,"anonymous role emits carried opaque identity into PhysicalRun crossings"' \
    'learning_update,FIXTURE-BOUND,"M6 update synthesizes productive/contrast encounters and raw consequence from seed"' \
    'ordinary_transient_decay,SEMANTIC-BOUNDARY,"temporary lookup state is explicitly erased after evaluator-delimited execution"' \
    'snapshot_restore,ABSENT,"report Snapshot is evaluator aggregation; no cumulative organism state snapshot/restore exists"' \
    'complete_permanent_fingerprints,ABSENT,"component fingerprints exist; no complete cumulative runtime fingerprints exist"' \
    'evaluator_only_observation,ABSENT,"no separate reusable cumulative runtime observer surface exists"' \
    'first_missing,blank_construction,"mechanical extraction stops before inventing raw-arrival representation or routing"' \
    'classification,H0-PROBE-NEGATIVE,"authoritative cumulative mechanism is not mechanically extractable under frozen H0 rules"'
