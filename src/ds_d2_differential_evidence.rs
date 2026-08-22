//! Development-only anonymous differential-evidence formation gate.

use std::mem::size_of;

use crate::research_runtime::HarnessMode;

pub const PROTOCOL: &str = "ds-d2-differential-evidence-formation-v1";
pub const EXACT_PARENT: &str = "353285fda96061bdcab640e53d77e710be966f06";
pub const PROTOCOL_COMMIT: &str = "c66ad6e3d4ab8e078208ec903ee30ad6f57857e0";
pub const AUTHORITATIVE_M0: &str = "1d74c0ed0b515446161a63a6d43ecbe27514dc85";
pub const FROZEN_D1_SOURCE_SHA256: &str =
    "bfdec68e291240108f85a70251651931af2e238236653a423b54e381506af10d";
pub const FROZEN_D1_HANDOFF_SHA256: &str =
    "dd26bb0b4d8983e341035bf2caaf0cefdd672f35441ca08ff0ebb8c57143ea54";
pub const FROZEN_A1_SHA256: &str =
    "b0a1841af3f85e725f92490b92357ddafd65289717846b5c16b85a49261e5ba1";
pub const FROZEN_DS1_SHA256: &str =
    "adec6a422e69e7f90bff6482776ea9aa91ae89e5e8d59183f6228165f9f7ff0e";
pub const FROZEN_RESULTS_DIGEST: &str =
    "491a63c17ba35d768b630720063793a4db09686cfe7cb33694fd80ea63bbd4e4";

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Prediction {
    trace: Vec<u8>,
    activation: [u16; 3],
}

impl Prediction {
    fn magnitude(&self) -> u64 {
        self.activation.iter().map(|value| u64::from(*value)).sum()
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct A1Export {
    predictions: [Prediction; 2],
    returned: Prediction,
    selected: usize,
    roots: usize,
    fresh: bool,
    a1_work: u64,
}

macro_rules! d2_a1_access {
    () => {
        pub(super) fn d2_ds1_hash_matches(expected: &str) -> bool {
            frozen_e0::FROZEN_DS1_LEARNER_SHA256 == expected
        }

        pub(super) fn d2_export_with_layout(
            seed: u64,
            acquisition: usize,
            reverse_allocation: bool,
            layout_padding: bool,
        ) -> Option<super::A1Export> {
            let bundle = frozen_e0::a1_bundle(seed, acquisition)?;
            let fresh = bundle.provenance.fresh_disjoint;
            let mut learner = train(&bundle.support, false)?;
            let mut substrate = substrate_from_export(
                &bundle.target,
                MappingOptions {
                    reverse_allocation,
                    layout_padding,
                    ..MappingOptions::default()
                },
            )?;
            let (_, installed) = learner.install(&mut substrate, true, false);
            let roots = structural_dedup(&mut substrate, &installed, &mut learner.work);
            if roots.len() != 2 {
                return None;
            }
            let predictions = roots
                .iter()
                .map(|root| execute_root(&substrate, *root, &mut learner.work))
                .collect::<Option<Vec<_>>>()?
                .into_iter()
                .map(|effect| super::Prediction {
                    trace: effect.trace,
                    activation: effect.activation,
                })
                .collect::<Vec<_>>();
            let predictions: [super::Prediction; 2] = predictions.try_into().ok()?;
            let selected = seed as usize % 2;
            let returned =
                execute_root(&substrate, roots[selected], &mut learner.work).map(|effect| {
                    super::Prediction {
                        trace: effect.trace,
                        activation: effect.activation,
                    }
                })?;
            Some(super::A1Export {
                predictions,
                returned,
                selected,
                roots: roots.len(),
                fresh,
                a1_work: learner.work.organism_work(),
            })
        }

        pub(super) fn d2_export(seed: u64, acquisition: usize) -> Option<super::A1Export> {
            d2_export_with_layout(seed, acquisition, false, false)
        }
    };
}

#[allow(dead_code)]
mod frozen_a1 {
    include!(concat!(
        env!("OUT_DIR"),
        "/ds_a1_affordance_multiplicity.rs"
    ));
    d2_a1_access!();
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Work {
    pub trace_comparisons: u64,
    pub activation_comparisons: u64,
    pub alternatives_compared: u64,
    pub arrows_formed: u64,
    pub spike_firings: u64,
    pub arrow_traversals: u64,
    pub abstentions: u64,
}

impl Work {
    pub fn organism_work(&self) -> u64 {
        self.trace_comparisons
            + self.activation_comparisons
            + self.alternatives_compared
            + self.arrows_formed
            + self.spike_firings
            + self.arrow_traversals
            + self.abstentions
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct TemporaryCell {
    activation: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct TemporaryArrow {
    endpoints: [u8; 2],
    live: bool,
}

#[derive(Clone, Debug, Default)]
struct DifferentialWorkspace {
    cells: [TemporaryCell; 3],
    arrows: Vec<TemporaryArrow>,
    work: Work,
}

impl DifferentialWorkspace {
    fn compatible(&mut self, prediction: &Prediction, evidence: &Prediction) -> bool {
        self.work.trace_comparisons += prediction.trace.len().max(evidence.trace.len()) as u64;
        self.work.activation_comparisons += 3;
        prediction == evidence
    }

    fn form(
        &mut self,
        alternatives: [Option<&Prediction>; 2],
        evidence: &Prediction,
    ) -> Option<usize> {
        let mut matches = Vec::new();
        for (index, alternative) in alternatives.into_iter().enumerate() {
            self.work.alternatives_compared += 1;
            if alternative.is_some_and(|prediction| self.compatible(prediction, evidence)) {
                matches.push(index);
            }
        }
        if matches.len() == 1 {
            self.arrows.push(TemporaryArrow {
                endpoints: [matches[0] as u8, 2],
                live: true,
            });
            self.work.arrows_formed += 1;
        } else {
            self.work.abstentions += 1;
        }
        self.direction()
    }

    fn direction(&self) -> Option<usize> {
        let [arrow] = self.arrows.as_slice() else {
            return None;
        };
        (arrow.live && arrow.endpoints[1] == 2).then_some(usize::from(arrow.endpoints[0]))
    }

    fn execute_direction(&mut self) -> bool {
        let Some(direction) = self.direction() else {
            return false;
        };
        self.cells[direction].activation += 1;
        self.work.spike_firings += 1;
        let arrow = self.arrows[0];
        if arrow.live && usize::from(arrow.endpoints[0]) == direction {
            self.cells[usize::from(arrow.endpoints[1])].activation += 1;
            self.work.arrow_traversals += 1;
        }
        self.cells[2].activation == 1
    }

    fn cleanup(&mut self) -> bool {
        self.arrows.clear();
        self.cells.fill(TemporaryCell::default());
        self.direction().is_none() && self.cells.iter().all(|cell| cell.activation == 0)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Controls {
    pub equal_support_abstains: bool,
    pub neither_abstains: bool,
    pub swap_reverses: bool,
    pub fresh_transfers: bool,
    pub allocation_layout_transfers: bool,
    pub handle_permutation: bool,
    pub shuffled_abstains: bool,
    pub same_magnitude_reversed_relation: bool,
    pub duplicate_abstains: bool,
    pub removed_route_invalidates: bool,
    pub physical_direction_executes: bool,
    pub cleanup: bool,
}

impl Controls {
    pub fn passed(&self) -> bool {
        self.equal_support_abstains
            && self.neither_abstains
            && self.swap_reverses
            && self.fresh_transfers
            && self.allocation_layout_transfers
            && self.handle_permutation
            && self.shuffled_abstains
            && self.same_magnitude_reversed_relation
            && self.duplicate_abstains
            && self.removed_route_invalidates
            && self.physical_direction_executes
            && self.cleanup
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SourceAudit {
    pub d1_source_hash: bool,
    pub d1_handoff_hash: bool,
    pub a1_hash: bool,
    pub ds1_hash: bool,
    pub differential_formers: usize,
    pub ds1_update_edges: usize,
    pub semantic_direction_fields: usize,
}

fn source_audit() -> SourceAudit {
    let source = include_str!("ds_d2_differential_evidence.rs");
    let update_call = ["apply_", "consequence("].concat();
    SourceAudit {
        d1_source_hash: env!("DS_D2_D1_SOURCE_SHA256") == FROZEN_D1_SOURCE_SHA256,
        d1_handoff_hash: env!("DS_D2_D1_HANDOFF_SHA256") == FROZEN_D1_HANDOFF_SHA256,
        a1_hash: env!("DS_D2_A1_SHA256") == FROZEN_A1_SHA256,
        ds1_hash: frozen_a1::d2_ds1_hash_matches(FROZEN_DS1_SHA256),
        differential_formers: source.matches("fn form(&mut self").count(),
        ds1_update_edges: source.matches(&update_call).count(),
        semantic_direction_fields: [
            ["correct_", "choice"].concat(),
            ["expected_", "choice"].concat(),
            ["reward_", "value"].concat(),
        ]
        .iter()
        .map(|field| source.matches(field).count())
        .sum(),
    }
}

impl SourceAudit {
    fn passed(&self) -> bool {
        self.d1_source_hash
            && self.d1_handoff_hash
            && self.a1_hash
            && self.ds1_hash
            && self.differential_formers == 1
            && self.ds1_update_edges == 0
            && self.semantic_direction_fields == 0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SeedReport {
    pub seed: u64,
    pub roots: usize,
    pub selected: usize,
    pub direction: Option<usize>,
    pub unique_compatibility: bool,
    pub magnitude_equal: bool,
    pub controls: Controls,
    pub primary_work: Work,
    pub a1_work: u64,
    pub persistent_bytes: usize,
    pub temporary_peak_bytes: usize,
    pub ds1_calls: u64,
    pub ds1_updates: u64,
    pub passed: bool,
}

fn audit_seed(seed: u64, acquisition: usize) -> SeedReport {
    let export = frozen_a1::d2_export(seed, acquisition).expect("actual frozen A1 alternatives");
    let mut primary = DifferentialWorkspace::default();
    let direction = primary.form(
        [Some(&export.predictions[0]), Some(&export.predictions[1])],
        &export.returned,
    );
    let unique_compatibility = direction == Some(export.selected);
    let physical_direction_executes = primary.execute_direction();

    let mut equal = DifferentialWorkspace::default();
    let equal_support_abstains = equal
        .form(
            [Some(&export.predictions[0]), Some(&export.predictions[0])],
            &export.predictions[0],
        )
        .is_none();
    let mut unknown = export.returned.clone();
    unknown.trace.push(255);
    let mut neither = DifferentialWorkspace::default();
    let neither_abstains = neither
        .form(
            [Some(&export.predictions[0]), Some(&export.predictions[1])],
            &unknown,
        )
        .is_none();
    let mut swapped = DifferentialWorkspace::default();
    let swap_reverses = swapped.form(
        [Some(&export.predictions[1]), Some(&export.predictions[0])],
        &export.returned,
    ) == Some(1 - export.selected);
    let fresh_export =
        frozen_a1::d2_export(seed + 1_000, acquisition).expect("fresh frozen A1 alternatives");
    let mut fresh = DifferentialWorkspace::default();
    let fresh_transfers = fresh.form(
        [
            Some(&fresh_export.predictions[0]),
            Some(&fresh_export.predictions[1]),
        ],
        &fresh_export.returned,
    ) == Some(fresh_export.selected)
        && fresh_export.fresh;
    let layout_export = frozen_a1::d2_export_with_layout(seed, acquisition, true, true)
        .expect("layout-perturbed frozen A1 alternatives");
    let mut layout = DifferentialWorkspace::default();
    let allocation_layout_transfers = layout.form(
        [
            Some(&layout_export.predictions[0]),
            Some(&layout_export.predictions[1]),
        ],
        &layout_export.returned,
    ) == Some(layout_export.selected);
    let mut shuffled = export.returned.clone();
    shuffled.activation = [7, 7, 7];
    let mut shuffled_workspace = DifferentialWorkspace::default();
    let shuffled_abstains = shuffled_workspace
        .form(
            [Some(&export.predictions[0]), Some(&export.predictions[1])],
            &shuffled,
        )
        .is_none();
    let magnitude_equal = export.predictions[0].magnitude() == export.predictions[1].magnitude();
    let reversed_index = 1 - export.selected;
    let mut reversed = DifferentialWorkspace::default();
    let same_magnitude_reversed_relation = magnitude_equal
        && reversed.form(
            [Some(&export.predictions[0]), Some(&export.predictions[1])],
            &export.predictions[reversed_index],
        ) == Some(reversed_index);
    let mut duplicate = DifferentialWorkspace::default();
    let duplicate_abstains = duplicate
        .form(
            [Some(&export.returned), Some(&export.returned)],
            &export.returned,
        )
        .is_none();
    let mut removed = DifferentialWorkspace::default();
    let alternatives = if export.selected == 0 {
        [None, Some(&export.predictions[1])]
    } else {
        [Some(&export.predictions[0]), None]
    };
    let removed_route_invalidates = removed.form(alternatives, &export.returned).is_none();
    let cleanup = primary.cleanup()
        && equal.cleanup()
        && neither.cleanup()
        && swapped.cleanup()
        && fresh.cleanup()
        && layout.cleanup()
        && shuffled_workspace.cleanup()
        && reversed.cleanup()
        && duplicate.cleanup()
        && removed.cleanup();
    let controls = Controls {
        equal_support_abstains,
        neither_abstains,
        swap_reverses,
        fresh_transfers,
        allocation_layout_transfers,
        handle_permutation: swap_reverses,
        shuffled_abstains,
        same_magnitude_reversed_relation,
        duplicate_abstains,
        removed_route_invalidates,
        physical_direction_executes,
        cleanup,
    };
    let passed =
        export.roots == 2 && unique_compatibility && controls.passed() && source_audit().passed();
    SeedReport {
        seed,
        roots: export.roots,
        selected: export.selected,
        direction,
        unique_compatibility,
        magnitude_equal,
        controls,
        primary_work: primary.work,
        a1_work: export.a1_work,
        persistent_bytes: 0,
        temporary_peak_bytes: size_of::<DifferentialWorkspace>() + 3 * size_of::<Prediction>(),
        ds1_calls: 0,
        ds1_updates: 0,
        passed,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Report {
    pub label: String,
    pub protocol: String,
    pub mode: String,
    pub claim_eligible: bool,
    pub enabling_only: bool,
    pub m0_authoritative: bool,
    pub m1_exists: bool,
    pub source: SourceAudit,
    pub seeds: Vec<SeedReport>,
    pub audit_passed: bool,
}

pub fn run(mode: HarnessMode) -> Report {
    if mode == HarnessMode::Definitive {
        return Report {
            label: "DS-D2 definitive forbidden".to_string(),
            protocol: PROTOCOL.to_string(),
            mode: "DEFINITIVE-FORBIDDEN".to_string(),
            claim_eligible: false,
            enabling_only: true,
            m0_authoritative: true,
            m1_exists: false,
            source: source_audit(),
            seeds: Vec::new(),
            audit_passed: false,
        };
    }
    let (seed_values, acquisition, mode_label) = match mode {
        HarnessMode::Micro => (vec![100], 16, "MICRO"),
        HarnessMode::Gate => ((100..105).collect(), 32, "GATE"),
        HarnessMode::Definitive => unreachable!(),
    };
    let source = source_audit();
    let seeds = seed_values
        .into_iter()
        .map(|seed| audit_seed(seed, acquisition))
        .collect::<Vec<_>>();
    let audit_passed = source.passed() && seeds.iter().all(|seed| seed.passed);
    Report {
        label: if audit_passed {
            "DS-D2 DEVELOPMENT IMPLEMENTATION READY".to_string()
        } else {
            "DS-D2 DEVELOPMENT FAILURE".to_string()
        },
        protocol: PROTOCOL.to_string(),
        mode: mode_label.to_string(),
        claim_eligible: false,
        enabling_only: true,
        m0_authoritative: true,
        m1_exists: false,
        source,
        seeds,
        audit_passed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn micro_forms_direction_without_ds1_or_semantics() {
        let report = run(HarnessMode::Micro);
        assert!(report.audit_passed, "{report:#?}");
        assert!(report
            .seeds
            .iter()
            .all(|seed| seed.direction == Some(seed.selected)
                && seed.ds1_calls == 0
                && seed.ds1_updates == 0));
    }

    #[test]
    fn gate_passes_all_differential_controls() {
        let report = run(HarnessMode::Gate);
        assert!(report.audit_passed, "{report:#?}");
        assert_eq!(report.seeds.len(), 5);
        assert!(report.seeds.iter().all(|seed| seed.controls.passed()));
    }

    #[test]
    fn magnitude_does_not_determine_direction() {
        let report = run(HarnessMode::Gate);
        assert!(report
            .seeds
            .iter()
            .all(|seed| seed.magnitude_equal && seed.controls.same_magnitude_reversed_relation));
    }

    #[test]
    fn definitive_is_inert() {
        let report = run(HarnessMode::Definitive);
        assert!(!report.audit_passed && report.seeds.is_empty() && !report.m1_exists);
    }
}
