use crate::{AcademyError, CrossingRecord, PhysicalInput, SpikeRecord, VisualSurface};
use serde::{Deserialize, Serialize};
use truelearner_core::{
    ArenaId, ArrowId, ArrowSpec, BoundaryLiveCheckpoint, BoundaryRuntime, CellId, CellSpec,
    ContentHash, MechanicalConfig, ResidentArenaId, SpikeInput, TransmissionMode, Work,
};

const OUTWARD_REGION: i16 = 1;
const INPUT_CAPACITY: usize = 128;
const OUTPUT_CAPACITY: usize = 128;
const SCAFFOLD_RESISTANCE: u32 = 100;
const CANDIDATE_RESISTANCE: u32 = 1;
const TEACHING_HORIZON: i64 = 50;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum A1ProbeFamily {
    LearnedRelation,
    Echo,
    Distractor,
    WrongContext,
    UnsupportedReturn,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum A1ExperienceKind {
    Teach,
    Probe(A1ProbeFamily),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeachingCase {
    pub id: String,
    pub capability_id: String,
    pub seed: u64,
    pub left: PhysicalInput,
    pub right: PhysicalInput,
    pub distractor: PhysicalInput,
    pub reflected: bool,
    pub reverse_allocation: bool,
}

impl TeachingCase {
    pub fn generated_text(seed: u64) -> Self {
        let left = token_for_port(seed, 0);
        let right = token_for_port(seed.rotate_left(11), 1);
        let distractor = token_for_port(seed.rotate_left(23), 2);
        Self {
            id: format!("text-relation-{seed:016x}"),
            capability_id: "novel-binding".to_string(),
            seed,
            left: PhysicalInput::Text(left),
            right: PhysicalInput::Text(right),
            distractor: PhysicalInput::Text(distractor),
            reflected: seed & 1 != 0,
            reverse_allocation: seed & 2 != 0,
        }
    }

    pub fn generated_raster(seed: u64) -> Self {
        Self {
            id: format!("raster-relation-{seed:016x}"),
            capability_id: "visual-symbol".to_string(),
            seed,
            left: PhysicalInput::Raster(surface_for_port(seed, 0)),
            right: PhysicalInput::Text(token_for_port(seed.rotate_left(7), 1)),
            distractor: PhysicalInput::Raster(surface_for_port(seed.rotate_left(19), 2)),
            reflected: seed & 1 != 0,
            reverse_allocation: seed & 2 != 0,
        }
    }

    pub fn ports_are_distinct(&self) -> bool {
        let ports = [
            physical_port(&self.left),
            physical_port(&self.right),
            physical_port(&self.distractor),
        ];
        ports == [0, 1, 2]
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct A1WorldObservation {
    pub kind: A1ExperienceKind,
    pub outward_relation_crossings: usize,
    pub outward_distractor_crossings: usize,
    pub plasticity_updates: u64,
    pub modulatory_deliveries: u64,
    pub physical_work: u64,
    pub naturally_quiescent: bool,
    pub body_before: String,
    pub body_after: String,
    pub clock_start: i64,
    pub clock_end: i64,
    pub candidate_resistance: u32,
    pub candidate_live: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct A1Experience {
    pub id: String,
    pub case_id: String,
    pub capability_id: String,
    pub seed: u64,
    pub kind: A1ExperienceKind,
    pub checkpoint_before: Vec<u8>,
    pub checkpoint_after: Vec<u8>,
    pub admitted_inputs: Vec<SpikeRecord>,
    pub crossings: Vec<CrossingRecord>,
    pub organism_surface: VisualSurface,
    pub shared_world_surface: VisualSurface,
    pub observation: A1WorldObservation,
    pub replay_exact: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct A1ReplayOutcome {
    pub experience_id: String,
    pub exact: bool,
    pub crossings_exact: bool,
    pub body_exact: bool,
    pub clock_exact: bool,
    pub work_exact: bool,
    pub quiescence_exact: bool,
}

#[derive(Clone, Copy)]
struct Sites {
    sources: [CellId; 4],
    downstream: [CellId; 2],
    returning: CellId,
    relay: [CellId; 2],
    candidates: [ArrowId; 2],
    outward_from: [u64; 2],
}

pub struct GenuineTeachingLab {
    case: TeachingCase,
    boundary: BoundaryRuntime,
    placements: Vec<ResidentArenaId>,
    sites: Sites,
    sequence: u64,
    experiences: Vec<A1Experience>,
}

impl GenuineTeachingLab {
    pub fn new(case: TeachingCase) -> Result<Self, AcademyError> {
        assert!(
            case.ports_are_distinct(),
            "generated physical ports must be distinct"
        );
        let (boundary, placements, sites) = build_world(&case)?;
        Ok(Self {
            case,
            boundary,
            placements,
            sites,
            sequence: 0,
            experiences: Vec::new(),
        })
    }

    pub fn case(&self) -> &TeachingCase {
        &self.case
    }

    pub fn experiences(&self) -> &[A1Experience] {
        &self.experiences
    }

    pub fn teach_supported(&mut self) -> Result<A1Experience, AcademyError> {
        let start = self.boundary.substrate().clock().tick;
        let a = self.sites.sources[physical_port(&self.case.left)];
        let b = self.sites.sources[physical_port(&self.case.right)];
        let inputs = vec![
            external(a, start, 0, self.case.seed + 10),
            external(b, start, 1, self.case.seed + 11),
            external(self.sites.downstream[0], start + 2, 20, self.case.seed + 20),
            external(self.sites.returning, start + 3, 21, self.case.seed + 21),
            external(a, start + 10, 0, self.case.seed + 30),
            external(b, start + 10, 1, self.case.seed + 31),
            external(
                self.sites.downstream[0],
                start + 12,
                20,
                self.case.seed + 40,
            ),
            external(self.sites.returning, start + 13, 21, self.case.seed + 41),
        ];
        self.execute(A1ExperienceKind::Teach, inputs, start + TEACHING_HORIZON)
    }

    pub fn teach_unsupported(&mut self) -> Result<A1Experience, AcademyError> {
        let start = self.boundary.substrate().clock().tick;
        let a = self.sites.sources[physical_port(&self.case.left)];
        let b = self.sites.sources[physical_port(&self.case.right)];
        let inputs = vec![
            external(a, start, 0, self.case.seed + 50),
            external(b, start, 1, self.case.seed + 51),
            external(a, start + 10, 0, self.case.seed + 60),
            external(b, start + 10, 1, self.case.seed + 61),
        ];
        self.execute(
            A1ExperienceKind::Probe(A1ProbeFamily::UnsupportedReturn),
            inputs,
            start + TEACHING_HORIZON,
        )
    }

    pub fn probe(&mut self, family: A1ProbeFamily) -> Result<A1Experience, AcademyError> {
        let tick = self.boundary.substrate().clock().tick.saturating_add(1);
        let a = self.sites.sources[physical_port(&self.case.left)];
        let b = self.sites.sources[physical_port(&self.case.right)];
        let c = self.sites.sources[physical_port(&self.case.distractor)];
        let inputs = match family {
            A1ProbeFamily::LearnedRelation => vec![
                external(a, tick, 0, self.case.seed + 100 + self.sequence),
                external(b, tick, 1, self.case.seed + 200 + self.sequence),
            ],
            A1ProbeFamily::Echo => {
                vec![external(a, tick, 0, self.case.seed + 300 + self.sequence)]
            }
            A1ProbeFamily::Distractor => vec![
                external(a, tick, 0, self.case.seed + 400 + self.sequence),
                external(c, tick, 1, self.case.seed + 500 + self.sequence),
            ],
            A1ProbeFamily::WrongContext => vec![
                external(b, tick, 0, self.case.seed + 600 + self.sequence),
                external(c, tick, 1, self.case.seed + 700 + self.sequence),
            ],
            A1ProbeFamily::UnsupportedReturn => vec![external(
                self.sites.relay[0],
                tick,
                0,
                self.case.seed + 800 + self.sequence,
            )],
        };
        self.execute(A1ExperienceKind::Probe(family), inputs, tick)
    }

    pub fn replay(&self, experience_id: &str) -> Result<A1ReplayOutcome, AcademyError> {
        let experience = self
            .experiences
            .iter()
            .find(|experience| experience.id == experience_id)
            .ok_or(AcademyError::NoReplay)?;
        let checkpoint = BoundaryLiveCheckpoint::decode(&experience.checkpoint_before)?;
        let mut boundary = BoundaryRuntime::from_live_checkpoint(checkpoint)?;
        boundary.reconfigure_mechanics(MechanicalConfig::PRODUCTION);
        boundary.repartition_resident(&self.placements);
        let inputs = experience
            .admitted_inputs
            .iter()
            .copied()
            .map(SpikeInput::from)
            .collect::<Vec<_>>();
        let result = boundary.arrive(&inputs, OUTWARD_REGION)?;
        let pressure_work = if experience.observation.clock_end > boundary.substrate().clock().tick
        {
            boundary.advance_time(experience.observation.clock_end)
        } else {
            Work::default()
        };
        let crossings = result
            .crossings
            .iter()
            .copied()
            .map(CrossingRecord::from)
            .collect::<Vec<_>>();
        let body = body_fingerprint(&boundary)?;
        let crossings_exact = crossings == experience.crossings;
        let body_exact = body == experience.observation.body_after;
        let clock_exact = boundary.substrate().clock().tick == experience.observation.clock_end;
        let observed_work = result.work.total().saturating_add(pressure_work.total());
        let work_exact = observed_work == experience.observation.physical_work;
        let quiescence_exact =
            result.naturally_quiescent == experience.observation.naturally_quiescent;
        Ok(A1ReplayOutcome {
            experience_id: experience.id.clone(),
            exact: crossings_exact && body_exact && clock_exact && work_exact && quiescence_exact,
            crossings_exact,
            body_exact,
            clock_exact,
            work_exact,
            quiescence_exact,
        })
    }

    fn execute(
        &mut self,
        kind: A1ExperienceKind,
        inputs: Vec<SpikeInput>,
        horizon: i64,
    ) -> Result<A1Experience, AcademyError> {
        let clock_start = self.boundary.substrate().clock().tick;
        let body_before = body_fingerprint(&self.boundary)?;
        let checkpoint_before = self
            .boundary
            .live_checkpoint(self.sequence)?
            .canonical_bytes()?;
        let result = self.boundary.arrive(&inputs, OUTWARD_REGION)?;
        let pressure_work = if horizon > self.boundary.substrate().clock().tick {
            self.boundary.advance_time(horizon)
        } else {
            Work::default()
        };
        let body_after = body_fingerprint(&self.boundary)?;
        let checkpoint_after = self
            .boundary
            .live_checkpoint(self.sequence.saturating_add(1))?
            .canonical_bytes()?;
        let crossings = result
            .crossings
            .iter()
            .copied()
            .map(CrossingRecord::from)
            .collect::<Vec<_>>();
        let outward_relation_crossings = result
            .crossings
            .iter()
            .filter(|crossing| crossing.from_physical == self.sites.outward_from[0])
            .count();
        let outward_distractor_crossings = result
            .crossings
            .iter()
            .filter(|crossing| crossing.from_physical == self.sites.outward_from[1])
            .count();
        let (candidate_resistance, candidate_live) =
            candidate_state(self.boundary.substrate(), self.sites.candidates[0]);
        let physical_work = result.work.total().saturating_add(pressure_work.total());
        let observation = A1WorldObservation {
            kind,
            outward_relation_crossings,
            outward_distractor_crossings,
            plasticity_updates: result.work.local_return_updates,
            modulatory_deliveries: result.work.modulatory_deliveries,
            physical_work,
            naturally_quiescent: result.naturally_quiescent,
            body_before,
            body_after,
            clock_start,
            clock_end: self.boundary.substrate().clock().tick,
            candidate_resistance,
            candidate_live,
        };
        let experience = A1Experience {
            id: format!("{}-{:04}", self.case.id, self.sequence),
            case_id: self.case.id.clone(),
            capability_id: self.case.capability_id.clone(),
            seed: self.case.seed,
            kind,
            checkpoint_before,
            checkpoint_after,
            admitted_inputs: inputs.into_iter().map(SpikeRecord::from).collect(),
            crossings,
            organism_surface: relation_surface(&self.case, kind, false),
            shared_world_surface: relation_surface(&self.case, kind, true),
            observation,
            replay_exact: None,
        };
        self.sequence = self.sequence.saturating_add(1);
        self.experiences.push(experience.clone());
        Ok(experience)
    }
}

fn build_world(
    case: &TeachingCase,
) -> Result<(BoundaryRuntime, Vec<ResidentArenaId>, Sites), AcademyError> {
    let mut space = truelearner_core::PlasticSubstrate::with_mechanics(
        ArenaId(case.seed),
        64,
        128,
        MechanicalConfig::PRODUCTION,
    );
    space.set_physical_tracing(true);
    let namespace = case.seed.wrapping_mul(10_000).wrapping_add(1_000_000);
    let order = if case.reverse_allocation {
        [3, 2, 1, 0]
    } else {
        [0, 1, 2, 3]
    };
    let mut sources = [None; 4];
    let mut traces = [None; 4];
    for port in order {
        sources[port] = Some(space.add_cell(cell(
            namespace + 100 + port as u64,
            position(case, 100 + port as i32 * 10),
            0,
            1,
        )));
        traces[port] = Some(space.add_cell(cell(
            namespace + 200 + port as u64,
            position(case, 500 + port as i32 * 10),
            0,
            1,
        )));
    }
    let sources = sources.map(Option::unwrap);
    let traces = traces.map(Option::unwrap);
    let mut coincidence = [None; 2];
    let mut inner = [None; 2];
    let mut downstream = [None; 2];
    let mut downstream_trace = [None; 2];
    let mut relay = [None; 2];
    let mut outward = [None; 2];
    let pair_order = if case.reverse_allocation {
        [1, 0]
    } else {
        [0, 1]
    };
    for pair in pair_order {
        coincidence[pair] = Some(space.add_cell(cell(
            namespace + 300 + pair as u64,
            position(case, 700 + pair as i32 * 10),
            0,
            2,
        )));
        inner[pair] = Some(space.add_cell(cell(
            namespace + 400 + pair as u64,
            position(case, 900 + pair as i32 * 10),
            0,
            1,
        )));
        downstream[pair] = Some(space.add_cell(cell(
            namespace + 500 + pair as u64,
            position(case, 1_100 + pair as i32 * 10),
            0,
            2,
        )));
        downstream_trace[pair] = Some(space.add_cell(cell(
            namespace + 600 + pair as u64,
            position(case, 1_300 + pair as i32 * 10),
            0,
            1,
        )));
        relay[pair] = Some(space.add_cell(cell(
            namespace + 700 + pair as u64,
            position(case, 1_500 + pair as i32 * 10),
            0,
            2,
        )));
        outward[pair] = Some(space.add_cell(cell(
            namespace + 800 + pair as u64,
            position(case, 1_700 + pair as i32 * 10),
            OUTWARD_REGION,
            1,
        )));
    }
    let coincidence = coincidence.map(Option::unwrap);
    let inner = inner.map(Option::unwrap);
    let downstream = downstream.map(Option::unwrap);
    let downstream_trace = downstream_trace.map(Option::unwrap);
    let relay = relay.map(Option::unwrap);
    let outward = outward.map(Option::unwrap);
    let returning = space.add_cell(cell(namespace + 900, position(case, 1_900), 0, 1));
    for port in order {
        space.add_arrow(drive(
            sources[port],
            traces[port],
            1,
            1,
            SCAFFOLD_RESISTANCE,
        ));
    }
    space.add_arrow(drive(traces[0], coincidence[0], 0, 1, SCAFFOLD_RESISTANCE));
    space.add_arrow(drive(traces[1], coincidence[0], 0, 1, SCAFFOLD_RESISTANCE));
    space.add_arrow(drive(traces[0], coincidence[1], 0, 1, SCAFFOLD_RESISTANCE));
    space.add_arrow(drive(traces[2], coincidence[1], 0, 1, SCAFFOLD_RESISTANCE));
    let mut candidates = [None; 2];
    for pair in pair_order {
        space.add_arrow(drive(
            coincidence[pair],
            inner[pair],
            0,
            1,
            SCAFFOLD_RESISTANCE,
        ));
        candidates[pair] = Some(space.add_arrow(drive(
            inner[pair],
            downstream[pair],
            1,
            1,
            CANDIDATE_RESISTANCE,
        )));
        space.add_arrow(drive(
            downstream[pair],
            downstream_trace[pair],
            1,
            1,
            SCAFFOLD_RESISTANCE,
        ));
        space.add_arrow(drive(
            downstream_trace[pair],
            relay[pair],
            0,
            1,
            SCAFFOLD_RESISTANCE,
        ));
        space.add_arrow(drive(returning, relay[pair], 0, 1, SCAFFOLD_RESISTANCE));
        space.add_arrow(modulatory(
            relay[pair],
            inner[pair],
            1,
            1,
            SCAFFOLD_RESISTANCE,
        ));
        space.add_arrow(drive(
            downstream[pair],
            outward[pair],
            1,
            1,
            SCAFFOLD_RESISTANCE,
        ));
    }
    let candidates = candidates.map(Option::unwrap);
    let placements = (0..space.arena_body(0).cells.len())
        .map(|index| ResidentArenaId(u32::try_from((index + case.seed as usize) % 4).unwrap_or(0)))
        .collect::<Vec<_>>();
    space.repartition_resident(&placements);
    let sites = Sites {
        sources,
        downstream,
        returning,
        relay,
        candidates,
        outward_from: [namespace + 500, namespace + 501],
    };
    Ok((
        BoundaryRuntime::new(space, OUTWARD_REGION, INPUT_CAPACITY, OUTPUT_CAPACITY)?,
        placements,
        sites,
    ))
}

fn cell(physical_id: u64, position: i32, region: i16, threshold: i32) -> CellSpec {
    CellSpec {
        physical_id,
        position,
        region,
        threshold,
        resistance: SCAFFOLD_RESISTANCE,
    }
}

fn drive(from: CellId, to: CellId, delay: i64, coupling: i32, resistance: u32) -> ArrowSpec {
    ArrowSpec {
        from,
        to,
        delay,
        phase: 0,
        coupling,
        resistance,
        mode: TransmissionMode::Drive,
    }
}

fn modulatory(from: CellId, to: CellId, delay: i64, coupling: i32, resistance: u32) -> ArrowSpec {
    ArrowSpec {
        from,
        to,
        delay,
        phase: 0,
        coupling,
        resistance,
        mode: TransmissionMode::Modulatory,
    }
}

fn external(target: CellId, tick: i64, phase: i32, origin: u64) -> SpikeInput {
    SpikeInput {
        arrival_tick: tick,
        phase,
        origin_physical: origin,
        target,
        impulse: 1,
    }
}

fn candidate_state(space: &truelearner_core::PlasticSubstrate, arrow: ArrowId) -> (u32, bool) {
    space
        .arena_body(0)
        .arrows
        .into_iter()
        .find(|candidate| candidate.id == arrow)
        .map_or((0, false), |candidate| {
            (candidate.resistance, candidate.live)
        })
}

fn body_fingerprint(boundary: &BoundaryRuntime) -> Result<String, AcademyError> {
    let bytes = boundary.substrate().canonical_body_bytes(0)?;
    Ok(ContentHash::of(&bytes)
        .as_bytes()
        .iter()
        .take(8)
        .map(|byte| format!("{byte:02x}"))
        .collect())
}

fn position(case: &TeachingCase, value: i32) -> i32 {
    if case.reflected {
        -value
    } else {
        value
    }
}

fn physical_port(input: &PhysicalInput) -> usize {
    let bytes = match input {
        PhysicalInput::Text(text) => text.as_bytes(),
        PhysicalInput::Raster(surface) => surface.rgba_pixels(),
    };
    let hash = bytes.iter().fold(0xcbf29ce484222325_u64, |state, byte| {
        state.wrapping_mul(0x100000001b3) ^ u64::from(*byte)
    });
    usize::try_from(hash % 4).unwrap_or(0)
}

fn token_for_port(seed: u64, port: usize) -> String {
    (0_u64..10_000)
        .map(|index| format!("{}-{:x}", syllable(seed, index), seed.wrapping_add(index)))
        .find(|token| physical_port(&PhysicalInput::Text(token.clone())) == port)
        .expect("a four-way physical text port must be reachable")
}

fn syllable(seed: u64, index: u64) -> &'static str {
    const PARTS: [&str; 8] = ["dax", "wug", "kiv", "zot", "pel", "nib", "ruf", "teg"];
    PARTS[((seed ^ index.rotate_left(7)) as usize) % PARTS.len()]
}

fn surface_for_port(seed: u64, port: usize) -> VisualSurface {
    (0_u64..10_000)
        .map(|index| {
            let mixed = seed.wrapping_add(index).rotate_left(13);
            let color = [
                (mixed & 0xff) as u8,
                ((mixed >> 8) & 0xff) as u8,
                ((mixed >> 16) & 0xff) as u8,
                255,
            ];
            let mut surface = VisualSurface::new(32, 32, color);
            let inset = 4 + (mixed as u32 % 8);
            surface.draw_line(
                (inset, 4),
                (31 - inset, 27),
                [255 - color[0], 255 - color[1], 255 - color[2], 255],
                2,
            );
            surface.set_pixel(
                31,
                31,
                [
                    index as u8,
                    index.rotate_right(8) as u8,
                    index.rotate_right(16) as u8,
                    255,
                ],
            );
            surface
        })
        .find(|surface| physical_port(&PhysicalInput::Raster(surface.clone())) == port)
        .expect("a four-way physical raster port must be reachable")
}

fn relation_surface(case: &TeachingCase, kind: A1ExperienceKind, observer: bool) -> VisualSurface {
    let background = if observer {
        [31, 38, 45, 255]
    } else {
        [246, 244, 238, 255]
    };
    let mut surface = VisualSurface::new(640, 360, background);
    let color = match kind {
        A1ExperienceKind::Teach => [91, 211, 165, 255],
        A1ExperienceKind::Probe(A1ProbeFamily::LearnedRelation) => [104, 187, 255, 255],
        A1ExperienceKind::Probe(_) => [236, 158, 82, 255],
    };
    let offset = u32::try_from(case.seed % 120).unwrap_or(0);
    surface.draw_line((80 + offset, 84), (260 + offset, 276), color, 6);
    surface.draw_line((260 + offset, 84), (80 + offset, 276), color, 6);
    surface
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supported_relation_is_reusable_but_controls_are_silent() {
        for seed in [41, 42, 43, 44] {
            let case = TeachingCase::generated_text(seed);
            let mut lab = GenuineTeachingLab::new(case).unwrap();
            let teaching = lab.teach_supported().unwrap();
            assert!(teaching.observation.plasticity_updates >= 2, "{teaching:?}");
            assert!(teaching.observation.candidate_live, "{teaching:?}");
            let learned = lab.probe(A1ProbeFamily::LearnedRelation).unwrap();
            assert_eq!(
                learned.observation.outward_relation_crossings, 1,
                "{learned:?}"
            );
            assert_eq!(learned.observation.plasticity_updates, 0, "{learned:?}");
            for family in [
                A1ProbeFamily::Echo,
                A1ProbeFamily::Distractor,
                A1ProbeFamily::WrongContext,
                A1ProbeFamily::UnsupportedReturn,
            ] {
                let control = lab.probe(family).unwrap();
                assert_eq!(
                    control.observation.outward_relation_crossings, 0,
                    "{control:?}"
                );
                assert_eq!(
                    control.observation.outward_distractor_crossings, 0,
                    "{control:?}"
                );
                assert_eq!(control.observation.plasticity_updates, 0, "{control:?}");
            }
            let replay = lab.replay(&learned.id).unwrap();
            assert!(replay.exact, "{replay:?}");
        }
    }

    #[test]
    fn adjacent_unsupported_repetitions_do_not_mature() {
        let case = TeachingCase::generated_text(73);
        let mut lab = GenuineTeachingLab::new(case).unwrap();
        let unsupported = lab.teach_unsupported().unwrap();
        assert_eq!(unsupported.observation.plasticity_updates, 0);
        assert!(!unsupported.observation.candidate_live, "{unsupported:?}");
        let probe = lab.probe(A1ProbeFamily::LearnedRelation).unwrap();
        assert_eq!(probe.observation.outward_relation_crossings, 0, "{probe:?}");
    }

    #[test]
    fn raster_bytes_select_physical_ports_without_evaluator_labels() {
        for seed in [101, 102, 103, 104] {
            let case = TeachingCase::generated_raster(seed);
            assert!(case.ports_are_distinct());
            assert!(matches!(case.left, PhysicalInput::Raster(_)));
            let mut lab = GenuineTeachingLab::new(case).unwrap();
            lab.teach_supported().unwrap();
            let learned = lab.probe(A1ProbeFamily::LearnedRelation).unwrap();
            assert_eq!(
                learned.observation.outward_relation_crossings, 1,
                "{learned:?}"
            );
            let distractor = lab.probe(A1ProbeFamily::Distractor).unwrap();
            assert_eq!(
                distractor.observation.outward_relation_crossings, 0,
                "{distractor:?}"
            );
        }
    }
}
