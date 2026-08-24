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

require_hash 50cf169bb293177a35270adde656f28f98e68c83a4d39d2876399261b7ee697c \
    src/ffs_same0.rs
require_hash 2d220a77a7992771c84cf455d66138ba5d3ffdaa90b2f8bdb452a8630c38e66e \
    src/ds1_boundary_role_cumulative_definitive.rs
require_hash ae90b8e1a72cb8f3e0c64bc0e92d4653a8408f0f93ed9a16675eb044957745f6 \
    src/ds2_cumulative_causal_direction_definitive.rs
require_hash c4fc7aca11a5925effeb5a84b90184a70da0f66da7c063d0f87ba46ca36addf3 \
    src/ds3_cumulative_event_boundary_port.rs
require_hash 3d5659fb26ae804dee6122408f9d703ea1f226349772883075a42686ac3fd110 \
    src/ds6_cumulative_lifetime_probe.rs
require_hash abaedd16717543270c5ed0ef2c8a16e3a4c0fed0215764443948c36d4adfa297 \
    src/ds7_cumulative_plasticity_allocation_gate.rs
require_hash 19c9051d15023c5b88559cba4ee3b3eb55686d1a68e083ca260a4a65629e8f30 \
    src/ds8_cumulative_semantic_credit_gate.rs
require_hash 67e170f12d7b7649a0a291ddfc16cd80e4b5c15564b65cd09c884f3e52b9ac5b \
    src/post_m6_ds4_arrival_initiation.rs
require_hash 6a8590b904403dfa880198f7acf8daf843864dee1a2ea0230d964d928f4076d1 \
    src/post_m7_ds5_closure_emission.rs
require_hash 6aa28a76e1362ac8dfb1d33fb68807da40e7604dfdc8cca9efa1e314e3ce4263 \
    crates/frozen-organism-v1-physics/src/substrate.rs

h0=$(git rev-parse h0-frozen-organism-extraction-probe-v1-negative^{commit})
if [ "$h0" != "cd9d5d2fea3489bded1ec6866a3ab47b6cb7610b" ]; then
    echo "H0 negative moved: $h0" >&2
    exit 1
fi

require_literal src/ffs_same0.rs 'struct RelationMotif {'
require_literal src/ffs_same0.rs 'source_position: u8,'
require_literal src/ffs_same0.rs 'target_position: u8,'
require_literal src/ffs_same0.rs \
    'fn observe(&mut self, motif: RelationMotif, successful: bool, work: &mut Same0Work) {'
require_literal src/ds3_event_boundary.rs 'pub boundary_role: BoundaryRole,'
require_literal src/ds3_event_boundary.rs 'pub causal_link: CausalLink,'
require_literal src/ds3_event_boundary.rs 'pub functional_relation: u8,'
require_literal src/ds3_event_boundary.rs 'pub ordinary_consequence: u8,'
require_literal src/ds3_cumulative_event_boundary_port.rs \
    'let bundle = frozen_e0::a1_bundle(seed, acquisition)?;'
require_literal src/ds3_cumulative_event_boundary_port.rs \
    'let link = if role == BoundaryRole::Open {'
require_literal src/ds3_cumulative_event_boundary_port.rs \
    'functional_relation: relation,'
require_literal src/ds3_cumulative_event_boundary_port.rs \
    'ordinary_consequence,'
require_literal src/post_m7_ds5_closure_emission.rs \
    'operation.lookup.begin_episode();'
require_literal src/post_m7_ds5_closure_emission.rs \
    'operation.lookup.erase_temporary();'
require_literal src/post_m6_ds4_arrival_initiation.rs \
    'let active_encounter = productive(gate.seed, ordinal, ordinal.is_multiple_of(2));'

physics_users=$(
    rg -l 'SpikeInput|frozen_organism_v1_physics|organism::substrate' src | sort
)
expected_physics_users=$(printf '%s\n' src/organism/conformance.rs src/organism/mod.rs)
if [ "$physics_users" != "$expected_physics_users" ]; then
    echo "unexpected retained-physics dependency surface:" >&2
    echo "$physics_users" >&2
    exit 1
fi

printf '%s\n' \
    'edge,status,evidence' \
    'retained_physics_bus_to_M0,NEW-REPRESENTATION,"no M0-M8 mechanism imports the retained Substrate or consumes SpikeInput; M0 consumes constructed RelationMotif plus successful"' \
    'M0_to_M1,FIXTURE-BOUND,"cumulative mechanisms exchange experiment-specific structs rather than common physical activity"' \
    'M2_to_M3,NEW-REPRESENTATION,"M3 Observation requires BoundaryRole CausalLink functional_relation and ordinary_consequence assembled by cumulative fixture glue"' \
    'M3_to_M7,FIXTURE-BOUND,"event/request ports call seeded stream and request encoders rather than consuming bus activity"' \
    'M7_to_M8,SEMANTIC-BOUNDARY,"closure execution consumes evaluator-built Episode and calls begin_episode/erase_temporary"' \
    'boundary_return_to_M6,FIXTURE-BOUND,"M6 synthesizes productive/contrast/raw histories from seed instead of receiving returned bus activity"' \
    'anonymous_bus_only,INSUFFICIENT,"a bus can route spikes but no existing one-to-one physical port connects those spikes to mechanism inputs"' \
    'continuous_scheduler_only,INSUFFICIENT,"a scheduler can order work but cannot derive the structured inputs required by frozen mechanisms"' \
    'first_collapse,retained_physics_bus_to_M0,"connecting raw activity requires choosing and implementing a new encoding/adapter"' \
    'classification,I0-C,"at least one mechanism input is not physically derivable from anonymous activity without a new cognitive representation"'
