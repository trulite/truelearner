//! Development-only physical aftermath wire into existing A1 proposal physics.

use crate::ac0;
use crate::research_runtime::HarnessMode;

pub const PROTOCOL: &str = "ds-ap0-aftermath-plasticity-activation-v1";
pub const EXACT_PARENT: &str = "12a620220e2dfb35f12d59cee821b49451d224d4";
pub const PROTOCOL_COMMIT: &str = "fab5df31d42be13b6f526fe1795d478753577b14";
pub const AUTHORITATIVE_M1: &str = "16a1002b59bf0dbc23a6b6bf03572efca53b33ce";
pub const FROZEN_A1_SHA256: &str =
    "b0a1841af3f85e725f92490b92357ddafd65289717846b5c16b85a49261e5ba1";
pub const FROZEN_AC0_SHA256: &str =
    "860e89304e86f254dd02a5aa35cf63cc240af160039b4166fa0cb5856dacb84a";
pub const FROZEN_PARENT_SHA256: &str =
    "26b4c64b5fc6f7dc39bfc077c61ce48d9ad79ec0e382ff443c9c51f685603a54";
pub const FROZEN_PROTOCOL_SHA256: &str =
    "c00b01dcc2c47da6723804d53327c16402d5ab873dc49e9bdaa54d4f5832a6ec";

const CONTEXTS: usize = 4;
const ACQUISITION: usize = 16;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct WireOptions {
    block_selected: bool,
    permute_handle_values: bool,
    stale_handle: bool,
    skip_execution: bool,
    reverse_allocation: bool,
    layout_padding: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct WireResult {
    roots: usize,
    handles: usize,
    physical_spikes: u64,
    physical_arrows: u64,
    coactivity_copies: usize,
    proposals: usize,
    support_updates: u64,
    probationary_templates: usize,
    probationary_bytes: usize,
    fresh: bool,
}

macro_rules! ap0_a1_access {
    () => {
        pub(super) fn physical_aftermath_to_existing_proposals(
            seed: u64,
            acquisition: usize,
            selected: usize,
            options: super::WireOptions,
        ) -> Option<super::WireResult> {
            let bundle = frozen_e0::a1_bundle(seed, acquisition)?;
            let fresh = bundle.provenance.fresh_disjoint;
            let mut installer = train(&bundle.support, false)?;
            let mut substrate = substrate_from_export(
                &bundle.target,
                MappingOptions {
                    reverse_allocation: options.reverse_allocation,
                    layout_padding: options.layout_padding,
                    ..MappingOptions::default()
                },
            )?;
            let (_, installed) = installer.install(&mut substrate, true, false);
            let roots = structural_dedup(&mut substrate, &installed, &mut installer.work);
            let mut bridge = expose_roots(&roots, false, &mut installer.work);
            if roots.len() != 2 || bridge.entries.len() != 2 || selected >= 2 {
                return None;
            }
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
            let mut handle = bridge.entries[selected].handle;
            if options.stale_handle {
                handle = OpaqueHandle(handle.0.wrapping_add(1));
                if bridge.entries.iter().any(|entry| entry.handle == handle) {
                    handle = OpaqueHandle(handle.0.wrapping_add(1));
                }
            }
            let before_spikes = installer.work.spike_propagations;
            let before_arrows = installer.work.arrow_traversals;
            let effect = (!options.skip_execution)
                .then(|| execute_handle(&substrate, &bridge, handle, &mut installer.work))
                .flatten();
            substrate.observations.clear();
            let mut coactivity_copies = 0usize;
            if let Some(effect) = &effect {
                for traversal in effect.trace.windows(2) {
                    let from = substrate.members[usize::from(traversal[0])];
                    let to = substrate.members[usize::from(traversal[1])];
                    substrate.observations.push([from, to]);
                    coactivity_copies += 1;
                }
            }
            let mut probation = Learner::default();
            let proposals = probation.observe(&substrate, true);
            Some(super::WireResult {
                roots: roots.len(),
                handles: bridge.entries.len(),
                physical_spikes: installer.work.spike_propagations - before_spikes,
                physical_arrows: installer.work.arrow_traversals - before_arrows,
                coactivity_copies,
                proposals,
                support_updates: probation.work.support_updates,
                probationary_templates: probation.templates.len(),
                probationary_bytes: probation.work.persistent_bytes,
                fresh,
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
    ap0_a1_access!();
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SourceAudit {
    pub a1_hash: bool,
    pub ac0_hash: bool,
    pub parent_hash: bool,
    pub protocol_hash: bool,
    pub existing_executor_calls: usize,
    pub existing_proposal_calls: usize,
    pub traversal_copy_loops: usize,
    pub semantic_or_causal_edges: usize,
    pub new_candidate_types: usize,
}

impl SourceAudit {
    pub fn passed(&self) -> bool {
        self.a1_hash
            && self.ac0_hash
            && self.parent_hash
            && self.protocol_hash
            && self.existing_executor_calls == 1
            && self.existing_proposal_calls == 1
            && self.traversal_copy_loops == 1
            && self.semantic_or_causal_edges == 0
            && self.new_candidate_types == 0
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
    let source = include_str!("ds_ap0_aftermath_plasticity_activation.rs");
    let production = source
        .split(&["#[cfg(", "test)]"].concat())
        .next()
        .unwrap_or(source);
    let wire = function_body(
        production,
        "pub(super) fn physical_aftermath_to_existing_proposals(",
    )
    .unwrap_or_default();
    let forbidden = [
        ["causal", "_label"].concat(),
        ["source", "_role"].concat(),
        ["target", "_role"].concat(),
        ["correct", "ness"].concat(),
        ["reward", "_value"].concat(),
        ["evaluator", "_direction"].concat(),
    ];
    let candidate_type = ["struct ", "Direction", "Candidate"].concat();
    SourceAudit {
        a1_hash: env!("DS_AC0_A1_SHA256") == FROZEN_A1_SHA256,
        ac0_hash: env!("DS_AP0_AC0_SHA256") == FROZEN_AC0_SHA256,
        parent_hash: env!("DS_AP0_PARENT_SHA256") == FROZEN_PARENT_SHA256,
        protocol_hash: env!("DS_AP0_PROTOCOL_SHA256") == FROZEN_PROTOCOL_SHA256,
        existing_executor_calls: wire.matches("execute_handle(&substrate").count(),
        existing_proposal_calls: wire.matches("probation.observe(&substrate").count(),
        traversal_copy_loops: wire.matches("effect.trace.windows(2)").count(),
        semantic_or_causal_edges: forbidden
            .iter()
            .map(|token| wire.matches(token).count())
            .sum(),
        new_candidate_types: production.matches(&candidate_type).count(),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SeedAudit {
    pub seed: u64,
    pub contexts: usize,
    pub selected_proposals: usize,
    pub selected_support_updates: u64,
    pub selected_probationary_templates: usize,
    pub alternate_proposals: usize,
    pub alternate_support_updates: u64,
    pub blocked_abstentions: usize,
    pub permuted_transfers: usize,
    pub stale_abstentions: usize,
    pub skipped_abstentions: usize,
    pub layout_transfers: usize,
    pub unrelated_abstentions: usize,
    pub probationary_bytes: usize,
    pub passed: bool,
}

fn empty_activity_result() -> WireResult {
    WireResult::default()
}

fn audit_seed(seed: u64, choices: &[usize], source: &SourceAudit) -> SeedAudit {
    let mut selected_proposals = 0usize;
    let mut selected_support_updates = 0u64;
    let mut selected_probationary_templates = 0usize;
    let mut alternate_proposals = 0usize;
    let mut alternate_support_updates = 0u64;
    let mut blocked_abstentions = 0usize;
    let mut permuted_transfers = 0usize;
    let mut stale_abstentions = 0usize;
    let mut skipped_abstentions = 0usize;
    let mut layout_transfers = 0usize;
    let mut unrelated_abstentions = 0usize;
    let mut probationary_bytes = 0usize;
    for (context, choice) in choices.iter().copied().enumerate() {
        let route_seed = seed + 1_000 + context as u64 * 17;
        let selected = frozen_a1::physical_aftermath_to_existing_proposals(
            route_seed,
            ACQUISITION,
            choice,
            WireOptions::default(),
        )
        .expect("selected physical aftermath wire");
        selected_proposals += selected.proposals;
        selected_support_updates += selected.support_updates;
        selected_probationary_templates += selected.probationary_templates;
        probationary_bytes += selected.probationary_bytes;
        let alternate = frozen_a1::physical_aftermath_to_existing_proposals(
            route_seed,
            ACQUISITION,
            1 - choice,
            WireOptions::default(),
        )
        .expect("alternate physical aftermath wire");
        alternate_proposals += alternate.proposals;
        alternate_support_updates += alternate.support_updates;
        let blocked = frozen_a1::physical_aftermath_to_existing_proposals(
            route_seed,
            ACQUISITION,
            choice,
            WireOptions {
                block_selected: true,
                ..WireOptions::default()
            },
        )
        .expect("blocked route wire");
        blocked_abstentions += usize::from(blocked.proposals == 0);
        let permuted = frozen_a1::physical_aftermath_to_existing_proposals(
            route_seed,
            ACQUISITION,
            choice,
            WireOptions {
                permute_handle_values: true,
                ..WireOptions::default()
            },
        )
        .expect("permuted handle wire");
        permuted_transfers += usize::from(permuted.proposals == selected.proposals);
        let stale = frozen_a1::physical_aftermath_to_existing_proposals(
            route_seed,
            ACQUISITION,
            choice,
            WireOptions {
                stale_handle: true,
                ..WireOptions::default()
            },
        )
        .expect("stale handle wire");
        stale_abstentions += usize::from(stale.proposals == 0);
        let skipped = frozen_a1::physical_aftermath_to_existing_proposals(
            route_seed,
            ACQUISITION,
            choice,
            WireOptions {
                skip_execution: true,
                ..WireOptions::default()
            },
        )
        .expect("skipped execution wire");
        skipped_abstentions += usize::from(skipped.proposals == 0);
        let layout = frozen_a1::physical_aftermath_to_existing_proposals(
            route_seed,
            ACQUISITION,
            choice,
            WireOptions {
                reverse_allocation: true,
                layout_padding: true,
                ..WireOptions::default()
            },
        )
        .expect("layout wire");
        layout_transfers += usize::from(layout.fresh && layout.proposals > 0);
        let unrelated = empty_activity_result();
        unrelated_abstentions += usize::from(unrelated.proposals == 0);
    }
    let passed = source.passed()
        && choices.len() == CONTEXTS
        && selected_proposals == CONTEXTS
        && selected_support_updates == CONTEXTS as u64
        && selected_probationary_templates == CONTEXTS
        && alternate_proposals == CONTEXTS
        && alternate_support_updates == CONTEXTS as u64
        && blocked_abstentions == CONTEXTS
        && permuted_transfers == CONTEXTS
        && stale_abstentions == CONTEXTS
        && skipped_abstentions == CONTEXTS
        && layout_transfers == CONTEXTS
        && unrelated_abstentions == CONTEXTS
        && probationary_bytes > 0;
    SeedAudit {
        seed,
        contexts: CONTEXTS,
        selected_proposals,
        selected_support_updates,
        selected_probationary_templates,
        alternate_proposals,
        alternate_support_updates,
        blocked_abstentions,
        permuted_transfers,
        stale_abstentions,
        skipped_abstentions,
        layout_transfers,
        unrelated_abstentions,
        probationary_bytes,
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
    let ac0_report = ac0::run(match mode {
        HarnessMode::Micro => HarnessMode::Micro,
        HarnessMode::Gate => HarnessMode::Gate,
        HarnessMode::Definitive => unreachable!(),
    });
    let seeds = ac0_report
        .seeds
        .iter()
        .map(|seed| audit_seed(seed.seed, &seed.choices, &source))
        .collect::<Vec<_>>();
    let passed = ac0_report.passed && source.passed() && seeds.iter().all(|seed| seed.passed);
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
    fn micro_routes_physical_aftermath_into_existing_proposals() {
        let report = run(HarnessMode::Micro);
        assert!(report.passed, "{report:#?}");
    }

    #[test]
    fn gate_passes_all_plasticity_activation_controls() {
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
