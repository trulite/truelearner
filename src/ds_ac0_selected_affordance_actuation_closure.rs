//! Development-only closure of frozen DS1 choice onto an existing A1 route.

use crate::research_runtime::HarnessMode;

pub const PROTOCOL: &str = "ds-ac0-selected-affordance-actuation-closure-v1";
pub const EXACT_PARENT: &str = "dbf630b85ef5b01f42d734c6195077ce5bbe5604";
pub const PROTOCOL_COMMIT: &str = "17c6a66da1476f0e3c7875b7d9c220d8f0aec87a";
pub const AUTHORITATIVE_M1: &str = "16a1002b59bf0dbc23a6b6bf03572efca53b33ce";
pub const FROZEN_A1_SHA256: &str =
    "b0a1841af3f85e725f92490b92357ddafd65289717846b5c16b85a49261e5ba1";
pub const FROZEN_M1_PARENT_SHA256: &str =
    "2b35d8b181b1b477390a2f84a4ad01993d7ca2b2aec6291d16ffd4fc0faf50b0";
pub const FROZEN_COLLAPSE_SHA256: &str =
    "5272b657010e635bb7e02ece804e7e4f2efad2954f222928fa5b083ac0eb12a9";
pub const FROZEN_PROTOCOL_SHA256: &str =
    "b8303120f6647204419cf3f98e8b3963ba91104d2dc01daffbeb7597870eea18";

const ACQUISITION: usize = 16;
const CONTEXTS: usize = 4;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct ActuationOptions {
    block_selected: bool,
    permute_handle_values: bool,
    change_target_binding: bool,
    stale_handle: bool,
    skip_execution: bool,
    reverse_allocation: bool,
    layout_padding: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct PhysicalDelta {
    trace: Vec<u8>,
    activation: [u16; 3],
    spike_propagations: u64,
    arrow_traversals: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Actuation {
    roots_before_choice: usize,
    handles_before_choice: usize,
    bridge_one_to_one: bool,
    fresh: bool,
    delta: Option<PhysicalDelta>,
    incremental_persistent_bytes: usize,
}

macro_rules! ac0_e0_access {
    () => {
        pub(super) fn mature_choice(
            seed: u64,
            acquisition: usize,
            context: usize,
        ) -> Option<usize> {
            let (mut formation, _) = acquire(seed, acquisition);
            let mut learner = Learner::default();
            for ordinal in 0..acquisition {
                let episode = fixture(
                    seed + 20_000,
                    acquisition + 100 + ordinal,
                    ordinal % 4,
                    Perturbation::None,
                );
                let event = formation.form(&episode.raw)?;
                let view = serialize_once(&event, &mut formation.work);
                let signature = Learner::signature(&view);
                let physical = usize::from(signature.gap.saturating_sub(1))
                    ^ usize::from(signature.witness_attachment);
                let (choice, _) = learner.choose(&view, ordinal + seed as usize);
                learner.apply_consequence(&view, choice, choice == physical);
            }
            let episode = fixture(
                seed + 30_000,
                acquisition + 10_000 + context,
                context,
                Perturbation::None,
            );
            let event = formation.form(&episode.raw)?;
            let view = serialize_once(&event, &mut formation.work);
            learner.frozen_choice(&view)
        }

        pub(super) fn frozen_ds1_matches(value: &str) -> bool {
            FROZEN_DS1_LEARNER_SHA256 == value
        }
    };
}

#[allow(dead_code)]
mod frozen_e0 {
    include!(concat!(
        env!("OUT_DIR"),
        "/ds_e0_anonymous_event_formation.rs"
    ));
    ac0_e0_access!();
}

macro_rules! ac0_a1_access {
    () => {
        pub(super) fn actuate_existing(
            seed: u64,
            acquisition: usize,
            selected: usize,
            options: super::ActuationOptions,
        ) -> Option<super::Actuation> {
            let bundle = frozen_e0::a1_bundle(seed, acquisition)?;
            let fresh = bundle.provenance.fresh_disjoint;
            let mut learner = train(&bundle.support, false)?;
            let mut substrate = substrate_from_export(
                &bundle.target,
                MappingOptions {
                    reverse_allocation: options.reverse_allocation,
                    layout_padding: options.layout_padding,
                    ..MappingOptions::default()
                },
            )?;
            let (_, installed) = learner.install(&mut substrate, true, false);
            let roots = structural_dedup(&mut substrate, &installed, &mut learner.work);
            let mut bridge = expose_roots(&roots, false, &mut learner.work);
            if roots.len() != 2 || bridge.entries.len() != 2 || selected >= 2 {
                return None;
            }
            let roots_before_choice = roots.len();
            let handles_before_choice = bridge.entries.len();
            let bridge_one_to_one = bridge
                .entries
                .iter()
                .zip(roots.iter())
                .all(|(entry, root)| entry.root == *root);
            if options.permute_handle_values {
                let first = bridge.entries[0].handle;
                bridge.entries[0].handle = bridge.entries[1].handle;
                bridge.entries[1].handle = first;
            }
            let selected_root = bridge.entries[selected].root;
            if options.block_selected {
                for arrow in &mut substrate.arrows {
                    if arrow.endpoints[0] == selected_root.cell {
                        arrow.live = false;
                    }
                }
            }
            if options.change_target_binding {
                let target = substrate
                    .arrows
                    .iter()
                    .find(|arrow| arrow.endpoints[0] == selected_root.cell)?
                    .endpoints[1];
                let current = substrate.cells[usize::from(target.0)].binding?;
                let member = member_index(&substrate, current)?;
                substrate.cells[usize::from(target.0)].binding =
                    Some(substrate.members[(member + 1) % 3]);
            }
            let mut handle = bridge.entries[selected].handle;
            if options.stale_handle {
                handle = OpaqueHandle(handle.0.wrapping_add(1));
                if bridge.entries.iter().any(|entry| entry.handle == handle) {
                    handle = OpaqueHandle(handle.0.wrapping_add(1));
                }
            }
            let before_spikes = learner.work.spike_propagations;
            let before_arrows = learner.work.arrow_traversals;
            let effect = (!options.skip_execution)
                .then(|| execute_handle(&substrate, &bridge, handle, &mut learner.work))
                .flatten();
            let delta = effect.map(|effect| super::PhysicalDelta {
                trace: effect.trace,
                activation: effect.activation,
                spike_propagations: learner.work.spike_propagations - before_spikes,
                arrow_traversals: learner.work.arrow_traversals - before_arrows,
            });
            Some(super::Actuation {
                roots_before_choice,
                handles_before_choice,
                bridge_one_to_one,
                fresh,
                delta,
                incremental_persistent_bytes: 0,
            })
        }
    };
}

#[allow(dead_code)]
mod frozen_a1 {
    include!(concat!(
        env!("OUT_DIR"),
        "/ds_a1_affordance_multiplicity.rs"
    ));
    ac0_a1_access!();
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct AnonymousAftermath {
    trace: Vec<u8>,
    activation: [u16; 3],
}

fn aftermath_from_physical_delta(delta: &PhysicalDelta) -> Option<AnonymousAftermath> {
    (delta.spike_propagations > 0 && delta.arrow_traversals > 0).then(|| AnonymousAftermath {
        trace: delta.trace.clone(),
        activation: delta.activation,
    })
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SourceAudit {
    pub a1_hash: bool,
    pub m1_parent_hash: bool,
    pub collapse_hash: bool,
    pub protocol_hash: bool,
    pub frozen_ds1_hash: bool,
    pub aftermath_formers: usize,
    pub aftermath_choice_edges: usize,
    pub existing_executors: usize,
    pub bridge_constructors: usize,
    pub new_action_opcode_fields: usize,
}

impl SourceAudit {
    pub fn passed(&self) -> bool {
        self.a1_hash
            && self.m1_parent_hash
            && self.collapse_hash
            && self.protocol_hash
            && self.frozen_ds1_hash
            && self.aftermath_formers == 1
            && self.aftermath_choice_edges == 0
            && self.existing_executors == 1
            && self.bridge_constructors == 1
            && self.new_action_opcode_fields == 0
    }
}

fn function_body<'a>(source: &'a str, marker: &str) -> Option<&'a str> {
    let start = source.find(marker)?;
    let tail = &source[start..];
    let open = tail.find('{')?;
    let mut depth = 0usize;
    for (offset, byte) in tail[open..].bytes().enumerate() {
        match byte {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&tail[..=open + offset]);
                }
            }
            _ => {}
        }
    }
    None
}

fn source_audit() -> SourceAudit {
    let source = include_str!("ds_ac0_selected_affordance_actuation_closure.rs");
    let production = source
        .split(&["#[cfg(", "test)]"].concat())
        .next()
        .unwrap_or(source);
    let aftermath =
        function_body(production, "fn aftermath_from_physical_delta(").unwrap_or_default();
    let actuation =
        function_body(production, "pub(super) fn actuate_existing(").unwrap_or_default();
    let forbidden_action_types = [
        ["Action", "Opcode"].concat(),
        ["Swap", "Action"].concat(),
        ["Keep", "Action"].concat(),
        ["causal", "_label"].concat(),
    ];
    SourceAudit {
        a1_hash: env!("DS_AC0_A1_SHA256") == FROZEN_A1_SHA256,
        m1_parent_hash: env!("DS_AC0_M1_PARENT_SHA256") == FROZEN_M1_PARENT_SHA256,
        collapse_hash: env!("DS_AC0_COLLAPSE_SHA256") == FROZEN_COLLAPSE_SHA256,
        protocol_hash: env!("DS_AC0_PROTOCOL_SHA256") == FROZEN_PROTOCOL_SHA256,
        frozen_ds1_hash: frozen_e0::frozen_ds1_matches(
            "adec6a422e69e7f90bff6482776ea9aa91ae89e5e8d59183f6228165f9f7ff0e",
        ),
        aftermath_formers: production
            .lines()
            .filter(|line| line.starts_with("fn aftermath_from_physical_delta("))
            .count(),
        aftermath_choice_edges: ["choice", "handle", "root", "expected", "evaluator"]
            .iter()
            .map(|token| aftermath.matches(token).count())
            .sum(),
        existing_executors: actuation.matches("execute_handle(&substrate").count(),
        bridge_constructors: actuation.matches("expose_roots(&roots").count(),
        new_action_opcode_fields: forbidden_action_types
            .iter()
            .map(|token| production.matches(token).count())
            .sum(),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SeedAudit {
    pub seed: u64,
    pub contexts: usize,
    pub choices: Vec<usize>,
    pub roots_before_choice: usize,
    pub handles_before_choice: usize,
    pub executions: usize,
    pub arrow_traversals: u64,
    pub distinct_aftermaths: usize,
    pub blocked_route_abstentions: usize,
    pub permuted_handle_transfers: usize,
    pub changed_binding_changes: usize,
    pub skipped_execution_abstentions: usize,
    pub stale_handle_abstentions: usize,
    pub layout_transfers: usize,
    pub incremental_persistent_bytes: usize,
    pub passed: bool,
}

fn audit_seed(seed: u64, source: &SourceAudit) -> SeedAudit {
    let mut choices = Vec::new();
    let mut roots_before_choice = 0usize;
    let mut handles_before_choice = 0usize;
    let mut executions = 0usize;
    let mut arrow_traversals = 0u64;
    let mut distinct_aftermaths = 0usize;
    let mut blocked_route_abstentions = 0usize;
    let mut permuted_handle_transfers = 0usize;
    let mut changed_binding_changes = 0usize;
    let mut skipped_execution_abstentions = 0usize;
    let mut stale_handle_abstentions = 0usize;
    let mut layout_transfers = 0usize;
    let mut incremental_persistent_bytes = 0usize;
    for context in 0..CONTEXTS {
        let choice =
            frozen_e0::mature_choice(seed, ACQUISITION, context).expect("frozen DS1 mature choice");
        choices.push(choice);
        let route_seed = seed + 1_000 + context as u64 * 17;
        let baseline = frozen_a1::actuate_existing(
            route_seed,
            ACQUISITION,
            choice,
            ActuationOptions::default(),
        )
        .expect("existing selected A1 route");
        let alternate = frozen_a1::actuate_existing(
            route_seed,
            ACQUISITION,
            1 - choice,
            ActuationOptions::default(),
        )
        .expect("existing alternate A1 route");
        let baseline_aftermath = baseline
            .delta
            .as_ref()
            .and_then(aftermath_from_physical_delta);
        let alternate_aftermath = alternate
            .delta
            .as_ref()
            .and_then(aftermath_from_physical_delta);
        roots_before_choice += baseline.roots_before_choice;
        handles_before_choice += baseline.handles_before_choice;
        executions += usize::from(baseline_aftermath.is_some());
        arrow_traversals += baseline
            .delta
            .as_ref()
            .map_or(0, |delta| delta.arrow_traversals);
        distinct_aftermaths += usize::from(
            baseline_aftermath.is_some()
                && alternate_aftermath.is_some()
                && baseline_aftermath != alternate_aftermath,
        );
        let blocked = frozen_a1::actuate_existing(
            route_seed,
            ACQUISITION,
            choice,
            ActuationOptions {
                block_selected: true,
                ..ActuationOptions::default()
            },
        )
        .expect("blocked selected route fixture");
        blocked_route_abstentions += usize::from(blocked.delta.is_none());
        let permuted = frozen_a1::actuate_existing(
            route_seed,
            ACQUISITION,
            choice,
            ActuationOptions {
                permute_handle_values: true,
                ..ActuationOptions::default()
            },
        )
        .expect("permuted opaque handle fixture");
        permuted_handle_transfers += usize::from(
            permuted
                .delta
                .as_ref()
                .and_then(aftermath_from_physical_delta)
                == baseline_aftermath,
        );
        let changed = frozen_a1::actuate_existing(
            route_seed,
            ACQUISITION,
            1 - choice,
            ActuationOptions {
                change_target_binding: true,
                ..ActuationOptions::default()
            },
        )
        .expect("changed temporary binding fixture");
        let changed_aftermath = changed
            .delta
            .as_ref()
            .and_then(aftermath_from_physical_delta);
        changed_binding_changes +=
            usize::from(changed_aftermath.is_some() && changed_aftermath != alternate_aftermath);
        let skipped = frozen_a1::actuate_existing(
            route_seed,
            ACQUISITION,
            choice,
            ActuationOptions {
                skip_execution: true,
                ..ActuationOptions::default()
            },
        )
        .expect("no-execution fixture");
        skipped_execution_abstentions += usize::from(skipped.delta.is_none());
        let stale = frozen_a1::actuate_existing(
            route_seed,
            ACQUISITION,
            choice,
            ActuationOptions {
                stale_handle: true,
                ..ActuationOptions::default()
            },
        )
        .expect("stale-handle fixture");
        stale_handle_abstentions += usize::from(stale.delta.is_none());
        let layout = frozen_a1::actuate_existing(
            route_seed,
            ACQUISITION,
            choice,
            ActuationOptions {
                reverse_allocation: true,
                layout_padding: true,
                ..ActuationOptions::default()
            },
        )
        .expect("fresh layout fixture");
        layout_transfers += usize::from(
            layout.fresh
                && layout
                    .delta
                    .as_ref()
                    .and_then(aftermath_from_physical_delta)
                    .is_some(),
        );
        incremental_persistent_bytes += baseline.incremental_persistent_bytes;
        if !baseline.bridge_one_to_one {
            continue;
        }
    }
    let passed = source.passed()
        && choices.contains(&0)
        && choices.contains(&1)
        && roots_before_choice == CONTEXTS * 2
        && handles_before_choice == CONTEXTS * 2
        && executions == CONTEXTS
        && arrow_traversals >= CONTEXTS as u64
        && distinct_aftermaths == CONTEXTS
        && blocked_route_abstentions == CONTEXTS
        && permuted_handle_transfers == CONTEXTS
        && changed_binding_changes == CONTEXTS
        && skipped_execution_abstentions == CONTEXTS
        && stale_handle_abstentions == CONTEXTS
        && layout_transfers == CONTEXTS
        && incremental_persistent_bytes == 0;
    SeedAudit {
        seed,
        contexts: CONTEXTS,
        choices,
        roots_before_choice,
        handles_before_choice,
        executions,
        arrow_traversals,
        distinct_aftermaths,
        blocked_route_abstentions,
        permuted_handle_transfers,
        changed_binding_changes,
        skipped_execution_abstentions,
        stale_handle_abstentions,
        layout_transfers,
        incremental_persistent_bytes,
        passed,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Report {
    pub mode: String,
    pub claim_eligible: bool,
    pub enabling_only: bool,
    pub m1_authoritative: bool,
    pub m2_exists: bool,
    pub source: SourceAudit,
    pub seeds: Vec<SeedAudit>,
    pub passed: bool,
}

pub fn run(mode: HarnessMode) -> Report {
    if mode == HarnessMode::Definitive {
        return Report {
            mode: "DEFINITIVE-FORBIDDEN".to_string(),
            claim_eligible: false,
            enabling_only: true,
            m1_authoritative: true,
            m2_exists: false,
            source: source_audit(),
            seeds: Vec::new(),
            passed: false,
        };
    }
    let source = source_audit();
    let seed_values: &[u64] = match mode {
        HarnessMode::Micro => &[100],
        HarnessMode::Gate => &[100, 101, 102, 103, 104],
        HarnessMode::Definitive => unreachable!(),
    };
    let seeds = seed_values
        .iter()
        .map(|seed| audit_seed(*seed, &source))
        .collect::<Vec<_>>();
    let passed = source.passed() && seeds.iter().all(|seed| seed.passed);
    Report {
        mode: match mode {
            HarnessMode::Micro => "MICRO",
            HarnessMode::Gate => "GATE",
            HarnessMode::Definitive => unreachable!(),
        }
        .to_string(),
        claim_eligible: false,
        enabling_only: true,
        m1_authoritative: true,
        m2_exists: false,
        source,
        seeds,
        passed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn micro_closes_choice_onto_physical_route() {
        let report = run(HarnessMode::Micro);
        assert!(report.passed, "{report:#?}");
        assert!(report.seeds[0].choices.contains(&0));
        assert!(report.seeds[0].choices.contains(&1));
    }

    #[test]
    fn gate_passes_all_physical_controls() {
        let report = run(HarnessMode::Gate);
        assert!(report.passed, "{report:#?}");
        assert_eq!(report.seeds.len(), 5);
    }

    #[test]
    fn definitive_is_inert() {
        let report = run(HarnessMode::Definitive);
        assert!(!report.passed && report.seeds.is_empty());
    }
}
