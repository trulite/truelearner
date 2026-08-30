#[cfg(target_os = "macos")]
mod macos {
    use academy_workstation::{WorkstationPresentation, WorkstationWorld};
    use serde::Serialize;
    use std::ffi::{c_int, c_void};
    use std::sync::OnceLock;
    use truelearner_core::ExecutionCost;
    use truelearner_workstation::{
        Protocol, ResearchHarnessConfig, ResearchOpportunityIncidence, ResearchTransitionBoundary,
        ResearchTransitionEvent, ResearchTransitionOpportunity, ResearchTransitionPhase,
        ResearchVisualComposition, WorkstationHarness, WorkstationStepObservation,
    };

    const SEED: u64 = 0x5eed_c0de;
    const RUSAGE_INFO_V4: c_int = 4;
    const RUSAGE_V4_FIELD_COUNT: usize = 35;
    const USER_TIME_FIELD: usize = 0;
    const SYSTEM_TIME_FIELD: usize = 1;
    const INSTRUCTIONS_FIELD: usize = 29;
    const CYCLES_FIELD: usize = 30;
    const PHASE_COUNT: usize = 5;

    #[repr(C)]
    struct RusageInfoV4 {
        uuid: [u8; 16],
        fields: [u64; RUSAGE_V4_FIELD_COUNT],
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct MachTimebaseInfo {
        numer: u32,
        denom: u32,
    }

    static TIMEBASE: OnceLock<MachTimebaseInfo> = OnceLock::new();

    unsafe extern "C" {
        fn proc_pid_rusage(pid: c_int, flavor: c_int, buffer: *mut c_void) -> c_int;
        fn mach_timebase_info(info: *mut MachTimebaseInfo) -> c_int;
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
    struct HostCost {
        cycles: u64,
        instructions: u64,
        cpu_nanoseconds: u64,
    }

    impl HostCost {
        fn delta(self, before: Self) -> Self {
            Self {
                cycles: self.cycles.saturating_sub(before.cycles),
                instructions: self.instructions.saturating_sub(before.instructions),
                cpu_nanoseconds: self.cpu_nanoseconds.saturating_sub(before.cpu_nanoseconds),
            }
        }

        fn minus(self, overhead: Self) -> Self {
            Self {
                cycles: self.cycles.saturating_sub(overhead.cycles),
                instructions: self.instructions.saturating_sub(overhead.instructions),
                cpu_nanoseconds: self
                    .cpu_nanoseconds
                    .saturating_sub(overhead.cpu_nanoseconds),
            }
        }

        fn plus(self, other: Self) -> Self {
            Self {
                cycles: self.cycles.saturating_add(other.cycles),
                instructions: self.instructions.saturating_add(other.instructions),
                cpu_nanoseconds: self.cpu_nanoseconds.saturating_add(other.cpu_nanoseconds),
            }
        }

        fn times(self, count: u64) -> Self {
            Self {
                cycles: self.cycles.saturating_mul(count),
                instructions: self.instructions.saturating_mul(count),
                cpu_nanoseconds: self.cpu_nanoseconds.saturating_mul(count),
            }
        }
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
    struct InputCounts {
        admitted: u64,
        focused_transitions: u64,
        runtime_attachment: u64,
    }

    impl InputCounts {
        fn observe(&mut self, observation: &WorkstationStepObservation) {
            self.admitted = self
                .admitted
                .saturating_add(u64::try_from(observation.admitted_inputs).unwrap_or(u64::MAX));
            self.focused_transitions = self.focused_transitions.saturating_add(
                u64::try_from(observation.focused_vision.admitted_transitions).unwrap_or(u64::MAX),
            );
            self.runtime_attachment = self.runtime_attachment.saturating_add(
                u64::try_from(observation.runtime_attachment.admitted_inputs).unwrap_or(u64::MAX),
            );
        }
    }

    #[derive(Clone, Copy, Debug, Default, Serialize)]
    struct LayerCost {
        environment_sense: HostCost,
        current_transition: HostCost,
        world_effect: HostCost,
    }

    impl LayerCost {
        fn primary_total(self) -> HostCost {
            self.environment_sense
                .plus(self.current_transition)
                .plus(self.world_effect)
        }
    }

    #[derive(Debug, Default)]
    struct PhaseCollector {
        starts: [Option<HostCost>; PHASE_COUNT],
        totals: [HostCost; PHASE_COUNT],
        counter_read_cost: HostCost,
        event_count: u64,
    }

    impl PhaseCollector {
        fn new(counter_read_cost: HostCost) -> Self {
            Self {
                counter_read_cost,
                ..Self::default()
            }
        }

        fn observe(&mut self, event: ResearchTransitionEvent) {
            let now = process_cost();
            self.event_count = self.event_count.saturating_add(1);
            let index = phase_index(event.phase);
            match event.boundary {
                ResearchTransitionBoundary::Begin => {
                    assert!(self.starts[index].replace(now).is_none());
                }
                ResearchTransitionBoundary::End => {
                    let start = self.starts[index].take().expect("phase begin precedes end");
                    let cost = now.delta(start).minus(self.counter_read_cost);
                    self.totals[index] = self.totals[index].plus(cost);
                }
            }
        }

        fn total(&self, phase: ResearchTransitionPhase) -> HostCost {
            self.totals[phase_index(phase)]
        }

        fn assert_balanced(&self) {
            assert!(self.starts.iter().all(Option::is_none));
        }
    }

    #[derive(Serialize)]
    struct CycleReport {
        schema: &'static str,
        host: &'static str,
        composition: &'static str,
        primary_steps: usize,
        counter_read_cost: HostCost,
        no_proof_primary_total: HostCost,
        environment_sense: HostCost,
        current_transition_with_observers: HostCost,
        world_effect: HostCost,
        transaction_clone: HostCost,
        body_core: HostCost,
        sensorimotor_adapter: HostCost,
        bare_organism: HostCost,
        choice_projection: HostCost,
        fingerprint: HostCost,
        execution_cost: ExecutionCost,
        input_counts: InputCounts,
        physical_trace_events: u64,
        observer_event_count: u64,
        bare_organism_cycles_per_step: u64,
        bare_organism_cpu_nanoseconds_per_step: u64,
        excluded: [&'static str; 7],
    }

    pub fn run() {
        let one_step = std::env::args().any(|argument| argument == "--one-step");
        let (composition_name, composition) = if std::env::args().any(|arg| arg == "--baseline") {
            (
                "binocular-stable-fixation",
                ResearchVisualComposition::binocular_stable_fixation(),
            )
        } else if std::env::args().any(|arg| arg == "--runtime-only") {
            (
                "binocular-stable-fixation+runtime-attachment",
                ResearchVisualComposition::binocular_stable_fixation()
                    .with_runtime_attachment(true),
            )
        } else {
            (
                "complete-runtime-attached-workstation",
                ResearchVisualComposition::complete_runtime_attached_workstation(),
            )
        };
        let counter_read_cost = counter_read_cost();

        let warm = initial_pair(composition);
        let _ = run_case(warm, counter_read_cost, false, one_step);

        let unobserved = run_case(
            initial_pair(composition),
            counter_read_cost,
            false,
            one_step,
        );
        let observed = run_case(initial_pair(composition), counter_read_cost, true, one_step);
        assert_eq!(unobserved.final_states, observed.final_states);
        observed.phases.assert_balanced();

        let body_core = observed
            .phases
            .total(ResearchTransitionPhase::ReturnedBody)
            .plus(observed.phases.total(ResearchTransitionPhase::MainBody));
        let transaction_clone = observed
            .phases
            .total(ResearchTransitionPhase::TransactionClone);
        let choice_projection = observed
            .phases
            .total(ResearchTransitionPhase::ChoiceProjection);
        let fingerprint = observed.phases.total(ResearchTransitionPhase::Fingerprint);
        let observed_counter_cost = counter_read_cost.times(observed.phases.event_count);
        let clean_transition = observed
            .layers
            .current_transition
            .minus(observed_counter_cost);
        let named_nonphysical = transaction_clone.plus(choice_projection).plus(fingerprint);
        let bare_organism = clean_transition.minus(named_nonphysical);
        let sensorimotor_adapter = bare_organism.minus(body_core);
        let primary_steps = if one_step { 1 } else { 6 };
        let report = CycleReport {
            schema: "truelearner-host-cycle-lens/v1",
            host: "macos proc_pid_rusage RUSAGE_INFO_V4",
            composition: composition_name,
            primary_steps,
            counter_read_cost,
            no_proof_primary_total: unobserved.layers.primary_total(),
            environment_sense: unobserved.layers.environment_sense,
            current_transition_with_observers: observed.layers.current_transition,
            world_effect: unobserved.layers.world_effect,
            transaction_clone,
            body_core,
            sensorimotor_adapter,
            bare_organism,
            choice_projection,
            fingerprint,
            execution_cost: observed.execution_cost,
            input_counts: observed.input_counts,
            physical_trace_events: observed.physical_trace_events,
            observer_event_count: observed.phases.event_count,
            bare_organism_cycles_per_step: bare_organism
                .cycles
                .checked_div(u64::try_from(primary_steps).unwrap())
                .unwrap_or(0),
            bare_organism_cpu_nanoseconds_per_step: bare_organism
                .cpu_nanoseconds
                .checked_div(u64::try_from(primary_steps).unwrap())
                .unwrap_or(0),
            excluded: [
                "process and Cargo startup",
                "world and body construction",
                "counterfactual branch cloning",
                "checkpoint serialization and restoration",
                "exact replay steps",
                "compact trace hashing and JSON",
                "test harness",
            ],
        };
        println!("{}", serde_json::to_string_pretty(&report).unwrap());
    }

    fn run_case(
        pair: (WorkstationHarness, WorkstationWorld),
        counter_read_cost: HostCost,
        observed: bool,
        one_step: bool,
    ) -> PrimaryRun {
        if one_step {
            run_one_primary(pair, counter_read_cost, observed)
        } else {
            run_primary(pair, counter_read_cost, observed)
        }
    }

    fn run_one_primary(
        (mut body, mut world): (WorkstationHarness, WorkstationWorld),
        counter_read_cost: HostCost,
        observed: bool,
    ) -> PrimaryRun {
        let mut ignored_layers = LayerCost::default();
        let mut ignored_phases = PhaseCollector::new(counter_read_cost);
        let mut ignored_execution_cost = ExecutionCost::default();
        let mut ignored_input_counts = InputCounts::default();
        let mut ignored_trace_events = 0;
        step(
            &mut body,
            &mut world,
            false,
            counter_read_cost,
            &mut ignored_layers,
            &mut ignored_phases,
            &mut ignored_execution_cost,
            &mut ignored_input_counts,
            &mut ignored_trace_events,
        );

        let mut layers = LayerCost::default();
        let mut phases = PhaseCollector::new(counter_read_cost);
        let mut execution_cost = ExecutionCost::default();
        let mut input_counts = InputCounts::default();
        let mut physical_trace_events = 0;
        step(
            &mut body,
            &mut world,
            observed,
            counter_read_cost,
            &mut layers,
            &mut phases,
            &mut execution_cost,
            &mut input_counts,
            &mut physical_trace_events,
        );
        PrimaryRun {
            layers,
            phases,
            execution_cost,
            input_counts,
            physical_trace_events,
            final_states: vec![(body, world)],
        }
    }

    struct PrimaryRun {
        layers: LayerCost,
        phases: PhaseCollector,
        execution_cost: ExecutionCost,
        input_counts: InputCounts,
        physical_trace_events: u64,
        final_states: Vec<(WorkstationHarness, WorkstationWorld)>,
    }

    fn run_primary(
        (mut body, mut world): (WorkstationHarness, WorkstationWorld),
        counter_read_cost: HostCost,
        observed: bool,
    ) -> PrimaryRun {
        let mut layers = LayerCost::default();
        let mut phases = PhaseCollector::new(counter_read_cost);
        let mut execution_cost = ExecutionCost::default();
        let mut input_counts = InputCounts::default();
        let mut physical_trace_events = 0;
        for _ in 0..2 {
            step(
                &mut body,
                &mut world,
                observed,
                counter_read_cost,
                &mut layers,
                &mut phases,
                &mut execution_cost,
                &mut input_counts,
                &mut physical_trace_events,
            );
        }

        let presentations = [
            WorkstationPresentation::with_monitor_glyph('?'),
            WorkstationPresentation::with_monitor_glyph('!'),
            WorkstationPresentation::default(),
            WorkstationPresentation::with_monitor_glyph('?'),
        ];
        let mut final_states = Vec::with_capacity(presentations.len());
        for presentation in presentations {
            let mut branch_body = body.clone();
            let mut branch_world = world.clone();
            branch_world.set_presentation(presentation).unwrap();
            step(
                &mut branch_body,
                &mut branch_world,
                observed,
                counter_read_cost,
                &mut layers,
                &mut phases,
                &mut execution_cost,
                &mut input_counts,
                &mut physical_trace_events,
            );
            final_states.push((branch_body, branch_world));
        }
        PrimaryRun {
            layers,
            phases,
            execution_cost,
            input_counts,
            physical_trace_events,
            final_states,
        }
    }

    fn step(
        body: &mut WorkstationHarness,
        world: &mut WorkstationWorld,
        observed: bool,
        counter_read_cost: HostCost,
        layers: &mut LayerCost,
        phases: &mut PhaseCollector,
        execution_cost: &mut ExecutionCost,
        input_counts: &mut InputCounts,
        physical_trace_events: &mut u64,
    ) {
        let (sample, sense) = measured(counter_read_cost, || world.sense(body.state()).unwrap());
        layers.environment_sense = layers.environment_sense.plus(sense);

        let before = process_cost();
        let (next, observation) = if observed {
            body.transition_observed(sample, |event| phases.observe(event))
                .unwrap()
        } else {
            body.transition(sample).unwrap()
        };
        let transition = process_cost().delta(before).minus(counter_read_cost);
        layers.current_transition = layers.current_transition.plus(transition);
        assert!(observation.naturally_quiescent);
        execution_cost.accumulate(observation.metrics.execution_cost);
        input_counts.observe(&observation);
        *physical_trace_events =
            physical_trace_events.saturating_add(observation.metrics.physical_trace_events);

        let (_, effect) = measured(counter_read_cost, || {
            world.advance(&observation.state_before, &observation.state_after)
        });
        layers.world_effect = layers.world_effect.plus(effect);
        *body = next;
    }

    fn initial_pair(
        composition: ResearchVisualComposition,
    ) -> (WorkstationHarness, WorkstationWorld) {
        let config = ResearchHarnessConfig {
            protocol: Protocol::RecursiveLearnerCausalTopologyProductCompositionOutcomeLifetime,
            opportunity_incidence: ResearchOpportunityIncidence::SharedWave,
            transition_opportunity: ResearchTransitionOpportunity::OutputSpecificProprioceptiveSequentialAlignedCausalDeltaPalmComponentWideRetinaVisualTransitionRetinotopicThresholdsFovealVisualReach,
        };
        (
            WorkstationHarness::new_research_composed(SEED, config, composition).unwrap(),
            WorkstationWorld::new().unwrap(),
        )
    }

    fn measured<T>(counter_read_cost: HostCost, operation: impl FnOnce() -> T) -> (T, HostCost) {
        let before = process_cost();
        let value = operation();
        let cost = process_cost().delta(before).minus(counter_read_cost);
        (value, cost)
    }

    fn counter_read_cost() -> HostCost {
        let mut costs = (0..257)
            .map(|_| {
                let before = process_cost();
                process_cost().delta(before)
            })
            .collect::<Vec<_>>();
        costs.sort_unstable_by_key(|cost| cost.cycles);
        let cycles = costs[costs.len() / 2].cycles;
        costs.sort_unstable_by_key(|cost| cost.instructions);
        let instructions = costs[costs.len() / 2].instructions;
        costs.sort_unstable_by_key(|cost| cost.cpu_nanoseconds);
        let cpu_nanoseconds = costs[costs.len() / 2].cpu_nanoseconds;
        HostCost {
            cycles,
            instructions,
            cpu_nanoseconds,
        }
    }

    fn process_cost() -> HostCost {
        let mut usage = RusageInfoV4 {
            uuid: [0; 16],
            fields: [0; RUSAGE_V4_FIELD_COUNT],
        };
        let result = unsafe {
            proc_pid_rusage(
                c_int::try_from(std::process::id()).unwrap(),
                RUSAGE_INFO_V4,
                (&mut usage as *mut RusageInfoV4).cast::<c_void>(),
            )
        };
        assert_eq!(result, 0, "proc_pid_rusage failed");
        let cpu_ticks =
            usage.fields[USER_TIME_FIELD].saturating_add(usage.fields[SYSTEM_TIME_FIELD]);
        HostCost {
            cycles: usage.fields[CYCLES_FIELD],
            instructions: usage.fields[INSTRUCTIONS_FIELD],
            cpu_nanoseconds: ticks_to_nanoseconds(cpu_ticks),
        }
    }

    fn ticks_to_nanoseconds(ticks: u64) -> u64 {
        let timebase = TIMEBASE.get_or_init(|| {
            let mut info = MachTimebaseInfo { numer: 0, denom: 0 };
            let result = unsafe { mach_timebase_info(&mut info) };
            assert_eq!(result, 0, "mach_timebase_info failed");
            assert_ne!(info.denom, 0, "mach timebase denominator is zero");
            info
        });
        u64::try_from(
            u128::from(ticks).saturating_mul(u128::from(timebase.numer))
                / u128::from(timebase.denom),
        )
        .unwrap_or(u64::MAX)
    }

    const fn phase_index(phase: ResearchTransitionPhase) -> usize {
        match phase {
            ResearchTransitionPhase::TransactionClone => 0,
            ResearchTransitionPhase::ReturnedBody => 1,
            ResearchTransitionPhase::MainBody => 2,
            ResearchTransitionPhase::ChoiceProjection => 3,
            ResearchTransitionPhase::Fingerprint => 4,
        }
    }
}

#[cfg(target_os = "macos")]
fn main() {
    macos::run();
}

#[cfg(not(target_os = "macos"))]
fn main() {
    eprintln!("runtime_attached_physical_cycle_lens requires macOS process cycle counters");
    std::process::exit(2);
}
