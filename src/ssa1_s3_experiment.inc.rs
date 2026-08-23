
pub const S3_PROTOCOL: &str = "ssa1-s3-structural-commitment-causality-v1";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct S3Arm {
    pub name: &'static str,
    pub intervention_episode: usize,
    pub live_before: [usize; 2],
    pub live_after: [usize; 2],
    pub recurrences_delivered: usize,
    pub target_transition_changed: bool,
    pub final_live: [usize; 2],
    pub final_class: &'static str,
    pub duplicate_exact: bool,
    pub schedule_exact: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct S3Cell {
    pub seed: u64,
    pub descriptor: ScheduleDescriptor,
    pub incumbent_side: usize,
    pub route_at_side: [usize; 2],
    pub reference_threshold_episode: usize,
    pub reference_deallocation_episode: usize,
    pub prefix_exact: bool,
    pub reference: S3Arm,
    pub threshold_block: S3Arm,
    pub deallocation_protection: S3Arm,
    pub post_threshold_block: S3Arm,
    pub post_deallocation_recurrence: S3Arm,
    pub threshold_causal: bool,
    pub deallocation_causal: bool,
    pub postcommit_inert: bool,
    pub controls_passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct S3Report {
    pub protocol: &'static str,
    pub stage: &'static str,
    pub cells: Vec<S3Cell>,
    pub threshold_causal_cells: usize,
    pub deallocation_causal_cells: usize,
    pub postcommit_inert_cells: usize,
    pub classification: &'static str,
    pub frozen_parent_exact: bool,
    pub claim_eligible: bool,
    pub passed: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum S3Stage {
    Probe,
    Micro,
    Gate,
}

impl S3Stage {
    fn name(self) -> &'static str {
        match self {
            Self::Probe => "PROBE",
            Self::Micro => "MICRO",
            Self::Gate => "GATE",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct S3Config {
    seed: u64,
    descriptor: ScheduleDescriptor,
    incumbent_side: usize,
    route_at_side: [usize; 2],
    reverse: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum S3Intervention {
    None,
    BlockRoute(usize),
    RecurRoute(usize),
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct S3Step {
    live_before: [usize; 2],
    live_after: [usize; 2],
    recurrences: usize,
    realized_route: Option<usize>,
    returned: usize,
}

#[derive(Clone, Debug)]
struct S3Run {
    session: frozen_ssa1::Adapter,
    scheduled: [usize; 2],
    executions: [usize; 2],
    consequences: [usize; 2],
    threshold_episode: Option<usize>,
    deallocation_episode: Option<usize>,
    duplicate_exact: bool,
}

impl S3Run {
    fn new(config: S3Config) -> Self {
        let mut duplicate_exact = true;
        let (session, initial_executions) = mature_session(
            config.seed,
            MAIN_MATURITY,
            config.incumbent_side,
            config.route_at_side,
            config.reverse,
            &mut duplicate_exact,
        );
        assert_eq!(initial_executions, MAIN_MATURITY);
        Self {
            session,
            scheduled: [0; 2],
            executions: [0; 2],
            consequences: [0; 2],
            threshold_episode: None,
            deallocation_episode: None,
            duplicate_exact,
        }
    }

    fn role_routes(&self, config: S3Config) -> [usize; 2] {
        [
            config.route_at_side[config.incumbent_side],
            config.route_at_side[1 - config.incumbent_side],
        ]
    }

    fn step(
        &mut self,
        config: S3Config,
        episode: usize,
        intervention: S3Intervention,
    ) -> S3Step {
        let role_routes = self.role_routes(config);
        let alternative = schedule_is_alternative(episode, config.descriptor);
        let role = usize::from(alternative);
        let side = if alternative {
            1 - config.incumbent_side
        } else {
            config.incumbent_side
        };
        self.scheduled[role] += 1;
        let live_before_landscape = self.session.inspect();
        let live_before = role_routes.map(|route| live_before_landscape.live_supporters[route]);
        let recurrences = match intervention {
            S3Intervention::RecurRoute(route) => {
                self.session.recur_live_before_event(route)
            }
            S3Intervention::None | S3Intervention::BlockRoute(_) => 0,
        };
        let mut present = [true; 2];
        if let S3Intervention::BlockRoute(route) = intervention {
            present[route] = false;
        }
        let live = self.session.offer_masked(present);
        let physical = resolve_exact(
            live,
            &PhysicalWorld {
                opportunity_side: Some(side),
                postclosure: false,
                favor_side: None,
                route_at_side: config.route_at_side,
                stale_route: [false; 2],
                reverse_allocation: config.reverse ^ episode.is_multiple_of(2),
            },
            config
                .seed
                .wrapping_add(100_000_000)
                .wrapping_add(episode as u64),
            &mut self.duplicate_exact,
        );
        let mut returned = 0;
        if let (Some(route), Some(realized_side)) =
            (physical.realized_route, physical.realized_side)
        {
            let realized_role = usize::from(realized_side != config.incumbent_side);
            self.executions[realized_role] += 1;
            let variant = if realized_role == 1 {
                0
            } else {
                variable_variant(episode + 17)
            };
            let before = self.session.audit().routes[route].evidence_observations;
            let _ = self.session.return_consequence(route, variant);
            let after = self.session.audit().routes[route].evidence_observations;
            returned = usize::from(after > before);
            self.consequences[realized_role] += returned;
        }
        let live_after_landscape = self.session.inspect();
        let live_after = role_routes.map(|route| live_after_landscape.live_supporters[route]);
        if self.threshold_episode.is_none()
            && live_before[1] < FIRING_THRESHOLD as usize
            && live_after[1] >= FIRING_THRESHOLD as usize
        {
            self.threshold_episode = Some(episode + 1);
        }
        if self.deallocation_episode.is_none()
            && live_before[0] >= FIRING_THRESHOLD as usize
            && live_after[0] < FIRING_THRESHOLD as usize
        {
            self.deallocation_episode = Some(episode + 1);
        }
        S3Step {
            live_before,
            live_after,
            recurrences,
            realized_route: physical.realized_route,
            returned,
        }
    }

    fn continue_normal(&mut self, config: S3Config, start: usize) {
        for episode in start..EPISODES {
            let _ = self.step(config, episode, S3Intervention::None);
        }
    }

    fn role_live(&self, config: S3Config) -> [usize; 2] {
        let landscape = self.session.inspect();
        self.role_routes(config)
            .map(|route| landscape.live_supporters[route])
    }

    fn class_name(&self, config: S3Config) -> &'static str {
        class(&self.session.inspect(), self.role_routes(config)).name()
    }

    fn schedule_exact(&self, descriptor: ScheduleDescriptor) -> bool {
        let expected_alternative = EPISODES * descriptor.ratio.alternative
            / (descriptor.ratio.alternative + descriptor.ratio.incumbent);
        self.scheduled == [EPISODES - expected_alternative, expected_alternative]
    }
}

fn s3_reference(config: S3Config) -> S3Run {
    let mut run = S3Run::new(config);
    run.continue_normal(config, 0);
    run
}

fn s3_prefix(config: S3Config, event: usize) -> S3Run {
    assert!((1..=EPISODES).contains(&event));
    let mut run = S3Run::new(config);
    for episode in 0..event - 1 {
        let _ = run.step(config, episode, S3Intervention::None);
    }
    run
}

fn s3_arm(
    name: &'static str,
    config: S3Config,
    event: usize,
    intervention: S3Intervention,
    expected_before: S3Run,
) -> (S3Arm, S3Run, S3Step) {
    let mut run = expected_before;
    let step = run.step(config, event - 1, intervention);
    run.continue_normal(config, event);
    let target_transition_changed = match intervention {
        S3Intervention::BlockRoute(_) => {
            step.live_before[1] < FIRING_THRESHOLD as usize
                && step.live_after[1] < FIRING_THRESHOLD as usize
        }
        S3Intervention::RecurRoute(_) => {
            step.live_before[0] >= FIRING_THRESHOLD as usize
                && step.live_after[0] >= FIRING_THRESHOLD as usize
        }
        S3Intervention::None => true,
    };
    let arm = S3Arm {
        name,
        intervention_episode: event,
        live_before: step.live_before,
        live_after: step.live_after,
        recurrences_delivered: step.recurrences,
        target_transition_changed,
        final_live: run.role_live(config),
        final_class: run.class_name(config),
        duplicate_exact: run.duplicate_exact,
        schedule_exact: run.schedule_exact(config.descriptor),
    };
    (arm, run, step)
}

fn s3_reference_arm(config: S3Config, reference: &S3Run) -> S3Arm {
    S3Arm {
        name: "T0-reference",
        intervention_episode: 0,
        live_before: [0; 2],
        live_after: [0; 2],
        recurrences_delivered: 0,
        target_transition_changed: true,
        final_live: reference.role_live(config),
        final_class: reference.class_name(config),
        duplicate_exact: reference.duplicate_exact,
        schedule_exact: reference.schedule_exact(config.descriptor),
    }
}

fn s3_run_cell(config: S3Config) -> S3Cell {
    let reference = s3_reference(config);
    let reference_duplicate = s3_reference(config);
    let reference_exact = reference.session.state_exact(&reference_duplicate.session)
        && reference.scheduled == reference_duplicate.scheduled
        && reference.executions == reference_duplicate.executions
        && reference.consequences == reference_duplicate.consequences;
    let threshold = reference.threshold_episode.unwrap_or(0);
    let deallocation = reference.deallocation_episode.unwrap_or(0);
    let reference_valid = threshold > 0
        && deallocation > threshold
        && reference.class_name(config) == "ALTERNATIVE"
        && reference.role_live(config) == [1, 4]
        && reference_exact;

    let empty_arm = |name| S3Arm {
        name,
        intervention_episode: 0,
        live_before: [0; 2],
        live_after: [0; 2],
        recurrences_delivered: 0,
        target_transition_changed: false,
        final_live: reference.role_live(config),
        final_class: reference.class_name(config),
        duplicate_exact: false,
        schedule_exact: false,
    };

    if !reference_valid {
        return S3Cell {
            seed: config.seed,
            descriptor: config.descriptor,
            incumbent_side: config.incumbent_side,
            route_at_side: config.route_at_side,
            reference_threshold_episode: threshold,
            reference_deallocation_episode: deallocation,
            prefix_exact: false,
            reference: s3_reference_arm(config, &reference),
            threshold_block: empty_arm("T1-threshold-block"),
            deallocation_protection: empty_arm("T2-deallocation-protection"),
            post_threshold_block: empty_arm("T3-post-threshold-block"),
            post_deallocation_recurrence: empty_arm("T3-post-deallocation-recurrence"),
            threshold_causal: false,
            deallocation_causal: false,
            postcommit_inert: false,
            controls_passed: false,
        };
    }

    let alternative_route = config.route_at_side[1 - config.incumbent_side];
    let incumbent_route = config.route_at_side[config.incumbent_side];

    let threshold_prefix = s3_prefix(config, threshold);
    let threshold_control = threshold_prefix.clone();
    let prefix_threshold_exact = threshold_prefix.session.state_exact(&threshold_control.session);
    let (threshold_block, _, threshold_step) = s3_arm(
        "T1-threshold-block",
        config,
        threshold,
        S3Intervention::BlockRoute(alternative_route),
        threshold_prefix,
    );
    let (threshold_normal, threshold_normal_run, _) = s3_arm(
        "T1-matched-reference",
        config,
        threshold,
        S3Intervention::None,
        threshold_control,
    );
    let threshold_reference_exact = threshold_normal_run.session.state_exact(&reference.session);

    let deallocation_prefix = s3_prefix(config, deallocation);
    let deallocation_control = deallocation_prefix.clone();
    let prefix_deallocation_exact = deallocation_prefix
        .session
        .state_exact(&deallocation_control.session);
    let (deallocation_protection, _, deallocation_step) = s3_arm(
        "T2-deallocation-protection",
        config,
        deallocation,
        S3Intervention::RecurRoute(incumbent_route),
        deallocation_prefix,
    );
    let (deallocation_normal, deallocation_normal_run, _) = s3_arm(
        "T2-matched-reference",
        config,
        deallocation,
        S3Intervention::None,
        deallocation_control,
    );
    let deallocation_reference_exact =
        deallocation_normal_run.session.state_exact(&reference.session);

    let post_threshold_event = threshold + 1;
    let (post_threshold_block, _, _) = s3_arm(
        "T3-post-threshold-block",
        config,
        post_threshold_event,
        S3Intervention::BlockRoute(alternative_route),
        s3_prefix(config, post_threshold_event),
    );
    let post_deallocation_event = deallocation + 1;
    let (post_deallocation_recurrence, _, _) = s3_arm(
        "T3-post-deallocation-recurrence",
        config,
        post_deallocation_event,
        S3Intervention::RecurRoute(incumbent_route),
        s3_prefix(config, post_deallocation_event),
    );

    let threshold_causal = threshold_block.target_transition_changed
        && threshold_step.realized_route != Some(alternative_route)
        && threshold_block.final_class == "INCUMBENT_LOCK";
    let deallocation_causal = deallocation_protection.target_transition_changed
        && deallocation_step.recurrences == FIRING_THRESHOLD as usize
        && deallocation_protection.final_class == "MIXED";
    let postcommit_inert = post_threshold_block.final_class == "ALTERNATIVE"
        && post_deallocation_recurrence.final_class == "ALTERNATIVE";
    let prefix_exact = prefix_threshold_exact
        && prefix_deallocation_exact
        && threshold_reference_exact
        && deallocation_reference_exact;
    let all_arms = [
        &threshold_block,
        &deallocation_protection,
        &post_threshold_block,
        &post_deallocation_recurrence,
        &threshold_normal,
        &deallocation_normal,
    ];
    let controls_passed = reference_valid
        && prefix_exact
        && all_arms
            .iter()
            .all(|arm| arm.duplicate_exact && arm.schedule_exact)
        && threshold_step.live_before[1] < FIRING_THRESHOLD as usize
        && deallocation_step.live_before[0] == 4
        && reference.scheduled == threshold_normal_run.scheduled
        && reference.scheduled == deallocation_normal_run.scheduled;

    S3Cell {
        seed: config.seed,
        descriptor: config.descriptor,
        incumbent_side: config.incumbent_side,
        route_at_side: config.route_at_side,
        reference_threshold_episode: threshold,
        reference_deallocation_episode: deallocation,
        prefix_exact,
        reference: s3_reference_arm(config, &reference),
        threshold_block,
        deallocation_protection,
        post_threshold_block,
        post_deallocation_recurrence,
        threshold_causal,
        deallocation_causal,
        postcommit_inert,
        controls_passed,
    }
}

fn s3_descriptor(stride: usize, offset: usize) -> ScheduleDescriptor {
    ScheduleDescriptor {
        ratio: OpportunityRatio {
            alternative: 1,
            incumbent: 2,
        },
        stride,
        offset,
        discovery: false,
    }
}

fn s3_configs(stage: S3Stage) -> Vec<S3Config> {
    let descriptors = match stage {
        S3Stage::Probe | S3Stage::Micro => vec![s3_descriptor(7, 1)],
        S3Stage::Gate => vec![
            s3_descriptor(7, 1),
            s3_descriptor(13, 43),
            s3_descriptor(17, 1),
        ],
    };
    let seeds: &[u64] = match stage {
        S3Stage::Probe => &[2_030_000_000, 2_030_000_001],
        S3Stage::Micro => &[2_130_000_000, 2_130_000_001, 2_130_000_002, 2_130_000_003],
        S3Stage::Gate => &[
            2_140_000_000,
            2_140_000_001,
            2_140_000_002,
            2_140_000_003,
            2_140_000_004,
            2_140_000_005,
        ],
    };
    seeds
        .iter()
        .enumerate()
        .flat_map(|(index, seed)| {
            descriptors.iter().copied().map(move |descriptor| S3Config {
                seed: *seed,
                descriptor,
                incumbent_side: index % 2,
                route_at_side: if index.is_multiple_of(2) {
                    [0, 1]
                } else {
                    [1, 0]
                },
                reverse: index % 3 == 0,
            })
        })
        .collect()
}

fn s3_frozen_parent_exact() -> bool {
    let source = include_str!("ssa1_s2_application_history_predictor.rs");
    let frozen = include_str!("ssa1_learned_variation_control.rs");
    let lifetime = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/ds7_cumulative_plasticity_targeting_probe.rs"
    ));
    source.contains("pub const PROTOCOL: &str = \"ssa1-s2-application-history-predictor-v1\"")
        && source.contains("P8-structural-commitment")
        && frozen.contains("const FIRING_THRESHOLD: i32 = 4;")
        && lifetime.contains("fn pressure_if_due(&mut self)")
        && !include_str!("ssa1_s3_physical_adapter.inc.rs").contains("m5_score")
        && !include_str!("ssa1_s3_physical_adapter.inc.rs").contains("evidence")
}

fn s3_report(stage: S3Stage) -> S3Report {
    let cells: Vec<_> = s3_configs(stage).into_iter().map(s3_run_cell).collect();
    let threshold_causal_cells = cells.iter().filter(|cell| cell.threshold_causal).count();
    let deallocation_causal_cells = cells.iter().filter(|cell| cell.deallocation_causal).count();
    let postcommit_inert_cells = cells.iter().filter(|cell| cell.postcommit_inert).count();
    let controls = cells.iter().all(|cell| cell.controls_passed);
    let all_threshold = threshold_causal_cells == cells.len();
    let all_deallocation = deallocation_causal_cells == cells.len();
    let all_postcommit = postcommit_inert_cells == cells.len();
    let frozen_parent_exact = s3_frozen_parent_exact();
    let classification = if !controls {
        "E — scientific ambiguity"
    } else if all_threshold && all_deallocation && all_postcommit {
        "A — structural commitment causal"
    } else if all_threshold && !all_deallocation {
        "B — threshold causal only"
    } else if !all_threshold && all_deallocation {
        "C — deallocation causal only"
    } else {
        "D — P8 readout, not established causal boundary"
    };
    S3Report {
        protocol: S3_PROTOCOL,
        stage: stage.name(),
        cells,
        threshold_causal_cells,
        deallocation_causal_cells,
        postcommit_inert_cells,
        classification,
        frozen_parent_exact,
        claim_eligible: false,
        passed: controls && frozen_parent_exact,
    }
}

pub fn run_s3_probe() -> S3Report {
    s3_report(S3Stage::Probe)
}

pub fn run_s3_micro() -> S3Report {
    s3_report(S3Stage::Micro)
}

pub fn run_s3_gate() -> S3Report {
    s3_report(S3Stage::Gate)
}

#[cfg(test)]
mod s3_tests {
    use super::*;

    #[test]
    #[ignore = "explicit SSA1-S3 PROBE"]
    fn probe_reaches_both_structural_nodes() {
        assert!(run_s3_probe().passed);
    }

    #[test]
    fn definitive_surface_is_absent() {
        assert!(!S3_PROTOCOL.contains("definitive"));
    }
}
