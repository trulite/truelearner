use px0_physical_correspondence::{
    ArrowId, ArrowSpec, CellId, CellSpec, Execution, PlasticSubstrate, SpikeInput, WorkLedger,
};
use std::env;
use std::fs::{read_to_string, rename, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;

const ROUTES: usize = 4;
const SITES: usize = 6;
const REINFORCEMENTS: usize = 5;
const OCCURRENCES: usize = 2 + REINFORCEMENTS;
const SOURCE_THRESHOLD: usize = 2;
const AUTHORITATIVE_COMMIT: &str = "2fbee861a0aeed335d3ffa8f9095ca28f2ac6129";
const AUTHORITY_TAG: &str = "px2-physical-causal-direction-authoritative";
const AUTHORITY_SOURCE_SHA256: &str =
    "3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d";
const V1_PROTOCOL_SHA256: &str = "aab8b7ed8eb8b96b6dd3b8fef95775d77cf797c1b2329284f93997a6c9db6236";
const V1_INVALID_SHA256: &str = "96c0816e6310661243bb1abd04452128d24f0aabe3ca46906297c1d84b2d0f23";
const V1_INVALID_COMMIT: &str = "62e8774466bfeaa214302fe95b61960c1085d0b7";
const V2_PROTOCOL_SHA256: &str = "0f2918c94f79a1f300240fde0b6a2f1c5adb3c0f334aa27dd01e679b8c56a5c1";
const V2_INVALID_SHA256: &str = "1ad71e8064b412c4e6fff28cfd50f7c758466f045e1a184aff969c66dff3474c";
const V2_INVALID_COMMIT: &str = "6690d69493d8673b6ce34d6c2a5f9424ff14606d";
const V3_PROTOCOL_SHA256: &str = "99ca2690e474ce79e0c761389ff95e50075341218f93dabf02c056b6b680331f";
const PX3_NEGATIVE: &str = "873094497ff6eb74363191dc5edc479c7d66de72";
const ARM_A_NEGATIVE: &str = "26aa795377c47ecf6fd28232865d5404408b6df9";
const ARM_B_NEGATIVE: &str = "82c0433329cf85bf3fe261661acd033011000656";
const ARM_C_NEGATIVE: &str = "5feb9b4c4755ed40d58ffc9cb8769d5523ea46f0";
const RESULT_CSV: &str = "results/cj0_a_coincidence_threshold_probe_v3.csv";
const RESULT_MD: &str = "results/cj0_a_coincidence_threshold_probe_v3.md";
const STAGING_CSV: &str = "results/.cj0_a_coincidence_threshold_probe_v3.csv.staging";
const STAGING_MD: &str = "results/.cj0_a_coincidence_threshold_probe_v3.md.staging";
const SOURCE_PATH: &str =
    "crates/px0-physical-correspondence/examples/cj0_a_coincidence_threshold.rs";

// Evaluator-only fixed incidence for the complete symmetric physical field.
const INCIDENCE: [[usize; 2]; SITES] = [[0, 1], [0, 2], [0, 3], [1, 2], [1, 3], [2, 3]];
const OLD: [usize; 2] = [0, 5];
const NEW: [usize; 2] = [2, 3];

// BEGIN ORGANISM-VISIBLE PHYSICAL BLOCK
fn propagate_physics(substrate: &mut PlasticSubstrate) -> Execution {
    substrate.propagate()
}
// END ORGANISM-VISIBLE PHYSICAL BLOCK

#[derive(Clone, Copy, Debug)]
struct Variant {
    name: &'static str,
    namespace: u64,
    reverse_allocation: bool,
    descending_identity: bool,
    mirror: bool,
    reverse_insertion: bool,
}

const VARIANTS: [Variant; 2] = [
    Variant {
        name: "primary",
        namespace: 0xa310_0000_0000,
        reverse_allocation: false,
        descending_identity: false,
        mirror: false,
        reverse_insertion: false,
    },
    Variant {
        name: "mirrored-reversed",
        namespace: 0xa320_0000_0000,
        reverse_allocation: true,
        descending_identity: true,
        mirror: true,
        reverse_insertion: true,
    },
];

#[derive(Clone)]
struct Site {
    ports: [CellId; 2],
    input_initial: [Option<ArrowId>; 2],
    joint_physical: u64,
    effect_physical: u64,
}

#[derive(Clone)]
struct Matter {
    substrate: PlasticSubstrate,
    sites: Vec<Site>,
    route_ports: [Vec<CellId>; ROUTES],
    port_physical: [Vec<u64>; ROUTES],
    distractors: [CellId; 2],
    namespace: u64,
    reverse_insertion: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Activity {
    site_firings: [usize; SITES],
    effects: [usize; SITES],
    port_firings: [usize; ROUTES],
    quiescent: bool,
    work: WorkLedger,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Metrics {
    route_occurrences: [usize; ROUTES],
    training_port_firings: [usize; ROUTES],
    training_site_firings: [usize; SITES],
    training_effects: [usize; SITES],
    trained_support: [[u32; 2]; 2],
    crossed_support: [[u32; 2]; 2],
    heldout_old_effects: [usize; 2],
    heldout_new_effects: [usize; 2],
    singleton_once_effects: [usize; ROUTES],
    repeated_a_effects: [usize; SITES],
    repeated_b_effects: [usize; SITES],
    repeated_a_old_support_before: [u32; 2],
    repeated_a_old_support_after: [u32; 2],
    reversal_port_firings: [usize; ROUTES],
    reversal_old_effects: [usize; 2],
    reversal_new_effects: [usize; 2],
    post_reversal_old_effects: [usize; 2],
    post_reversal_new_effects: [usize; 2],
    post_reversal_old_support: [[u32; 2]; 2],
    post_reversal_new_support: [[u32; 2]; 2],
    too_late_effects: usize,
    correlation_only_effects: usize,
    no_return_heldout_effects: usize,
    absent_opportunity_effects: usize,
    stale_opportunity_effects: usize,
    ambiguity_three_effects: usize,
    ambiguity_four_effects: usize,
    fresh_alternative_old_effects: usize,
    fresh_alternative_new_effects: usize,
    source_refirings: usize,
    naturally_quiescent: bool,
    work: WorkLedger,
    persistent_bytes: usize,
    fingerprint: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Claims {
    p0_frozen: bool,
    p1_marginals: bool,
    p2_training: bool,
    p3_discrimination: bool,
    p4_singleton_self_evidence: bool,
    p5_reversal_bootstrap: bool,
    p6_controls: bool,
    p7_duplicate: bool,
}

impl Claims {
    fn bits(&self) -> String {
        [
            self.p0_frozen,
            self.p1_marginals,
            self.p2_training,
            self.p3_discrimination,
            self.p4_singleton_self_evidence,
            self.p5_reversal_bootstrap,
            self.p6_controls,
            self.p7_duplicate,
        ]
        .into_iter()
        .map(|value| if value { '1' } else { '0' })
        .collect()
    }

    fn first_failure(&self) -> &'static str {
        let ordered = [
            (self.p0_frozen, "P0"),
            (self.p1_marginals, "P1"),
            (self.p2_training, "P2"),
            (self.p3_discrimination, "P3"),
            (self.p4_singleton_self_evidence, "P4"),
            (self.p5_reversal_bootstrap, "P5"),
            (self.p6_controls, "P6"),
            (self.p7_duplicate, "P7"),
        ];
        ordered
            .into_iter()
            .find_map(|(passed, name)| (!passed).then_some(name))
            .unwrap_or("none")
    }

    fn all(&self) -> bool {
        self.first_failure() == "none"
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Row {
    variant: &'static str,
    namespace: u64,
    metrics: Metrics,
    duplicate_exact: bool,
    claims: Claims,
}

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let preflight = args == ["--preflight"];
    let probe = args == ["--probe-v3"];
    if !preflight && !probe {
        eprintln!("CJ0-A requires --preflight or --probe-v3");
        std::process::exit(2);
    }
    assert!(source_audit(), "frozen source/lineage audit failed");
    for path in [RESULT_CSV, RESULT_MD, STAGING_CSV, STAGING_MD] {
        assert!(
            !Path::new(path).exists(),
            "result path already exists: {path}"
        );
    }
    if preflight {
        println!("CJ0_A_COINCIDENCE_THRESHOLD_PROBE_V3_PREFLIGHT_OK");
        return;
    }

    eprintln!("CJ0_A_COINCIDENCE_THRESHOLD_PROBE_V3_EVIDENCE_SPENT");
    let mut rows = Vec::new();
    for variant in VARIANTS {
        let first = run_replica(variant);
        let second = run_replica(variant);
        let duplicate_exact = first == second;
        let claims = claims(&first, duplicate_exact);
        rows.push(Row {
            variant: variant.name,
            namespace: variant.namespace,
            metrics: first,
            duplicate_exact,
            claims,
        });
    }
    let passed = rows.iter().all(|row| row.claims.all());
    publish(&csv(&rows), &markdown(&rows, passed));
    if !passed {
        eprintln!("CJ0_A_COINCIDENCE_THRESHOLD_PROBE_V3_FROZEN_NEGATIVE");
        std::process::exit(1);
    }
}

fn build_matter(
    variant: Variant,
    namespace_offset: u64,
    with_return: bool,
    with_opportunity: bool,
) -> Matter {
    let namespace = variant.namespace + namespace_offset;
    let mut substrate = PlasticSubstrate::new();
    let mut logical_sites: Vec<Option<Site>> = vec![None; SITES];
    let mut route_ports: [Vec<CellId>; ROUTES] = std::array::from_fn(|_| Vec::new());
    let mut port_physical: [Vec<u64>; ROUTES] = std::array::from_fn(|_| Vec::new());
    let allocation = if variant.reverse_allocation {
        (0..SITES).rev().collect::<Vec<_>>()
    } else {
        (0..SITES).collect::<Vec<_>>()
    };

    for (slot, logical) in allocation.into_iter().enumerate() {
        let identity_slot = if variant.descending_identity {
            SITES - 1 - logical
        } else {
            logical
        };
        let base_id = namespace + identity_slot as u64 * 0x100;
        let centre = slot as i32 * 100;
        let distance = if with_opportunity { 2 } else { 3 };
        let offsets = if variant.mirror {
            [distance, -distance]
        } else {
            [-distance, distance]
        };
        let mut ports = Vec::new();
        let mut initial = Vec::new();
        let joint_physical = base_id + 0x30;
        let effect_physical = base_id + 0x40;
        let joint = substrate.add_cell(CellSpec {
            physical_id: joint_physical,
            position: centre,
            region: 0,
            threshold: 3,
            resistance: 1000,
        });
        let effect = substrate.add_cell(CellSpec {
            physical_id: effect_physical,
            position: centre + 20,
            region: 1,
            threshold: 1,
            resistance: 1000,
        });
        for side in 0..2 {
            let route = INCIDENCE[logical][side];
            let physical_id = base_id + 0x10 + side as u64;
            let port = substrate.add_cell(CellSpec {
                physical_id,
                position: centre + offsets[side],
                region: 0,
                threshold: 2,
                resistance: 1000,
            });
            route_ports[route].push(port);
            port_physical[route].push(physical_id);
            ports.push(port);
            initial.push(with_opportunity.then(|| {
                substrate.add_arrow(ArrowSpec {
                    from: port,
                    to: joint,
                    delay: 1,
                    phase: 0,
                    coupling: 1,
                    resistance: 1,
                })
            }));
            if with_return {
                substrate.add_arrow(ArrowSpec {
                    from: joint,
                    to: port,
                    delay: 1,
                    phase: 0,
                    coupling: 1,
                    resistance: 1000,
                });
            }
        }
        substrate.add_arrow(ArrowSpec {
            from: joint,
            to: effect,
            delay: 1,
            phase: 0,
            coupling: 1,
            resistance: 1000,
        });
        logical_sites[logical] = Some(Site {
            ports: [ports[0], ports[1]],
            input_initial: [initial[0], initial[1]],
            joint_physical,
            effect_physical,
        });
    }
    let distractors = [
        substrate.add_cell(CellSpec {
            physical_id: namespace + 0xf000,
            position: 2000,
            region: 0,
            threshold: 2,
            resistance: 1000,
        }),
        substrate.add_cell(CellSpec {
            physical_id: namespace + 0xf100,
            position: 2100,
            region: 0,
            threshold: 2,
            resistance: 1000,
        }),
    ];
    Matter {
        substrate,
        sites: logical_sites.into_iter().map(Option::unwrap).collect(),
        route_ports,
        port_physical,
        distractors,
        namespace,
        reverse_insertion: variant.reverse_insertion,
    }
}

fn enqueue(matter: &mut Matter, active: &[usize], tick: i64, serial_base: u64) {
    let mut entries = Vec::new();
    for route in active.iter().copied() {
        for (port_index, port) in matter.route_ports[route].iter().copied().enumerate() {
            for impulse_index in 0..SOURCE_THRESHOLD {
                entries.push((
                    route,
                    port,
                    serial_base
                        + route as u64 * 0x1000
                        + port_index as u64 * 0x10
                        + impulse_index as u64,
                    impulse_index as i32,
                ));
            }
        }
    }
    if matter.reverse_insertion {
        entries.reverse();
    }
    for (_, target, origin, phase) in entries {
        matter.substrate.enter(SpikeInput {
            arrival_tick: tick,
            phase,
            origin_physical: origin,
            target,
            impulse: 1,
        });
    }
}

fn activate(matter: &mut Matter, active: &[usize], tick: i64, serial_base: u64) -> Activity {
    enqueue(matter, active, tick, serial_base);
    let execution = propagate_physics(&mut matter.substrate);
    classify(matter, execution)
}

fn activate_burst(matter: &mut Matter, active: &[usize], tick: i64, serial_base: u64) -> Activity {
    enqueue(matter, active, tick, serial_base);
    enqueue(matter, active, tick + 1, serial_base + 0x100);
    let execution = propagate_physics(&mut matter.substrate);
    classify(matter, execution)
}

fn fire_distractors(matter: &mut Matter, tick: i64) -> Activity {
    for (index, target) in matter.distractors.into_iter().enumerate() {
        for impulse in 0..2 {
            matter.substrate.enter(SpikeInput {
                arrival_tick: tick,
                phase: impulse,
                origin_physical: matter.namespace + 0xfe00 + index as u64 * 8 + impulse as u64,
                target,
                impulse: 1,
            });
        }
    }
    let execution = propagate_physics(&mut matter.substrate);
    classify(matter, execution)
}

fn classify(matter: &Matter, execution: Execution) -> Activity {
    let mut activity = Activity {
        quiescent: execution.naturally_quiescent,
        work: execution.work,
        ..Activity::default()
    };
    for entry in execution.trace.iter().filter(|entry| entry.fired) {
        for (site_index, site) in matter.sites.iter().enumerate() {
            activity.site_firings[site_index] +=
                usize::from(entry.target_physical == site.joint_physical);
            activity.effects[site_index] +=
                usize::from(entry.target_physical == site.effect_physical);
        }
        for route in 0..ROUTES {
            activity.port_firings[route] +=
                usize::from(matter.port_physical[route].contains(&entry.target_physical));
        }
    }
    activity
}

fn add_activity(total: &mut Activity, value: Activity) {
    for index in 0..SITES {
        total.site_firings[index] += value.site_firings[index];
        total.effects[index] += value.effects[index];
    }
    for route in 0..ROUTES {
        total.port_firings[route] += value.port_firings[route];
    }
    total.quiescent &= value.quiescent;
    add_work(&mut total.work, &value.work);
}

fn train_at(matter: &mut Matter, first: [usize; 2], second: [usize; 2], start: i64) -> Activity {
    let mut total = Activity {
        quiescent: true,
        ..Activity::default()
    };
    let namespace = matter.namespace;
    let first_burst = activate_burst(matter, &first, start, namespace + 0x10_0000);
    add_activity(&mut total, first_burst);
    let second_burst = activate_burst(matter, &second, start + 8, namespace + 0x20_0000);
    add_activity(&mut total, second_burst);
    for reinforcement in 0..REINFORCEMENTS {
        let offset = 20 + reinforcement as i64 * 20;
        let first_serial = matter.namespace + 0x30_0000 + reinforcement as u64 * 0x1000;
        let second_serial = matter.namespace + 0x40_0000 + reinforcement as u64 * 0x1000;
        let first_value = activate(matter, &first, start + offset, first_serial);
        add_activity(&mut total, first_value);
        let second_value = activate(matter, &second, start + offset + 8, second_serial);
        add_activity(&mut total, second_value);
    }
    total
}

fn observe_once(base: &Matter, active: &[usize], tick: i64, serial: u64) -> (Matter, Activity) {
    let mut clone = base.clone();
    clone.substrate.advance_time(tick);
    let activity = activate(&mut clone, active, tick, serial);
    (clone, activity)
}

fn observe_repeated(base: &Matter, active: &[usize], tick: i64, serial: u64) -> (Matter, Activity) {
    let mut clone = base.clone();
    clone.substrate.advance_time(tick);
    let mut total = Activity {
        quiescent: true,
        ..Activity::default()
    };
    let burst = activate_burst(&mut clone, active, tick, serial);
    add_activity(&mut total, burst);
    (clone, total)
}

fn support(matter: &Matter, logical_site: usize) -> [u32; 2] {
    std::array::from_fn(|side| {
        let site = &matter.sites[logical_site];
        matter
            .substrate
            .arrows_between(site.ports[side], joint_id(matter, logical_site))
            .into_iter()
            .filter(|arrow| matter.substrate.arrow_is_live(*arrow))
            .map(|arrow| matter.substrate.arrow_resistance(arrow))
            .max()
            .unwrap_or(0)
    })
}

fn joint_id(matter: &Matter, logical_site: usize) -> CellId {
    let site = &matter.sites[logical_site];
    if let Some(arrow) = site.input_initial.into_iter().flatten().next() {
        let (_, to) = matter.substrate.arrow_endpoints(arrow);
        return to;
    }
    // Absent-opportunity worlds never call support.
    panic!("joint handle unavailable without an initial opportunity")
}

fn run_replica(variant: Variant) -> Metrics {
    let mut work = WorkLedger::default();
    let mut matter = build_matter(variant, 0, true, true);
    let training = train_at(&mut matter, [0, 1], [2, 3], 0);
    add_work(&mut work, &training.work);
    let trained_support = [support(&matter, OLD[0]), support(&matter, OLD[1])];
    let crossed_support = [support(&matter, NEW[0]), support(&matter, NEW[1])];
    let observation_tick = 116;

    let mut heldout_old_effects = [0; 2];
    let mut heldout_new_effects = [0; 2];
    for (index, site) in OLD.into_iter().enumerate() {
        let (_, activity) = observe_once(
            &matter,
            &INCIDENCE[site],
            observation_tick,
            variant.namespace + 0x30_0000 + index as u64 * 0x1000,
        );
        heldout_old_effects[index] = activity.effects[site];
        add_work(&mut work, &activity.work);
    }
    for (index, site) in NEW.into_iter().enumerate() {
        let (_, activity) = observe_once(
            &matter,
            &INCIDENCE[site],
            observation_tick,
            variant.namespace + 0x31_0000 + index as u64 * 0x1000,
        );
        heldout_new_effects[index] = activity.effects[site];
        add_work(&mut work, &activity.work);
    }

    let mut singleton_once_effects = [0; ROUTES];
    for (route, effect_count) in singleton_once_effects.iter_mut().enumerate() {
        let (_, activity) = observe_once(
            &matter,
            &[route],
            observation_tick,
            variant.namespace + 0x32_0000 + route as u64 * 0x1000,
        );
        *effect_count = activity.effects.iter().sum();
        add_work(&mut work, &activity.work);
    }
    let repeated_a_old_support_before = support(&matter, OLD[0]);
    let (after_a, repeated_a) = observe_repeated(
        &matter,
        &[0],
        observation_tick,
        variant.namespace + 0x33_0000,
    );
    let repeated_a_old_support_after = support(&after_a, OLD[0]);
    let (_, repeated_b) = observe_repeated(
        &matter,
        &[1],
        observation_tick,
        variant.namespace + 0x34_0000,
    );
    add_work(&mut work, &repeated_a.work);
    add_work(&mut work, &repeated_b.work);

    matter.substrate.advance_time(120);
    let reversal = train_at(&mut matter, [0, 3], [2, 1], 120);
    add_work(&mut work, &reversal.work);
    let post_tick = 236;
    let mut post_reversal_old_effects = [0; 2];
    let mut post_reversal_new_effects = [0; 2];
    for (index, site) in OLD.into_iter().enumerate() {
        let (_, activity) = observe_once(
            &matter,
            &INCIDENCE[site],
            post_tick,
            variant.namespace + 0x40_0000 + index as u64 * 0x1000,
        );
        post_reversal_old_effects[index] = activity.effects[site];
        add_work(&mut work, &activity.work);
    }
    for (index, site) in NEW.into_iter().enumerate() {
        let (_, activity) = observe_once(
            &matter,
            &INCIDENCE[site],
            post_tick,
            variant.namespace + 0x41_0000 + index as u64 * 0x1000,
        );
        post_reversal_new_effects[index] = activity.effects[site];
        add_work(&mut work, &activity.work);
    }

    let controls = controls(variant);
    add_work(&mut work, &controls.work);
    let expected_port_firings = OCCURRENCES * 3;
    let source_refirings = training
        .port_firings
        .iter()
        .map(|count| count.saturating_sub(expected_port_firings))
        .sum::<usize>()
        + reversal
            .port_firings
            .iter()
            .map(|count| count.saturating_sub(expected_port_firings))
            .sum::<usize>();
    let naturally_quiescent = training.quiescent
        && reversal.quiescent
        && repeated_a.quiescent
        && repeated_b.quiescent
        && controls.quiescent;

    Metrics {
        route_occurrences: [OCCURRENCES; ROUTES],
        training_port_firings: training.port_firings,
        training_site_firings: training.site_firings,
        training_effects: training.effects,
        trained_support,
        crossed_support,
        heldout_old_effects,
        heldout_new_effects,
        singleton_once_effects,
        repeated_a_effects: repeated_a.effects,
        repeated_b_effects: repeated_b.effects,
        repeated_a_old_support_before,
        repeated_a_old_support_after,
        reversal_port_firings: reversal.port_firings,
        reversal_old_effects: [reversal.effects[OLD[0]], reversal.effects[OLD[1]]],
        reversal_new_effects: [reversal.effects[NEW[0]], reversal.effects[NEW[1]]],
        post_reversal_old_effects,
        post_reversal_new_effects,
        post_reversal_old_support: [support(&matter, OLD[0]), support(&matter, OLD[1])],
        post_reversal_new_support: [support(&matter, NEW[0]), support(&matter, NEW[1])],
        too_late_effects: controls.too_late_effects,
        correlation_only_effects: controls.correlation_only_effects,
        no_return_heldout_effects: controls.no_return_heldout_effects,
        absent_opportunity_effects: controls.absent_opportunity_effects,
        stale_opportunity_effects: controls.stale_opportunity_effects,
        ambiguity_three_effects: controls.ambiguity_three_effects,
        ambiguity_four_effects: controls.ambiguity_four_effects,
        fresh_alternative_old_effects: controls.fresh_alternative_old_effects,
        fresh_alternative_new_effects: controls.fresh_alternative_new_effects,
        source_refirings,
        naturally_quiescent,
        work,
        persistent_bytes: matter.substrate.persistent_bytes(),
        fingerprint: matter.substrate.complete_fingerprint(),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ControlMetrics {
    too_late_effects: usize,
    correlation_only_effects: usize,
    no_return_heldout_effects: usize,
    absent_opportunity_effects: usize,
    stale_opportunity_effects: usize,
    ambiguity_three_effects: usize,
    ambiguity_four_effects: usize,
    fresh_alternative_old_effects: usize,
    fresh_alternative_new_effects: usize,
    quiescent: bool,
    work: WorkLedger,
}

fn controls(variant: Variant) -> ControlMetrics {
    let mut work = WorkLedger::default();
    let mut quiescent = true;

    let mut late = build_matter(variant, 0x1000_0000, true, true);
    let late_namespace = late.namespace;
    let first = activate(&mut late, &[0], 0, late_namespace + 0x10000);
    let second = activate(&mut late, &[1], 4, late_namespace + 0x20000);
    let too_late_effects =
        first.effects.iter().sum::<usize>() + second.effects.iter().sum::<usize>();
    quiescent &= first.quiescent && second.quiescent;
    add_work(&mut work, &first.work);
    add_work(&mut work, &second.work);

    let mut correlation = build_matter(variant, 0x2000_0000, true, true);
    let correlated = fire_distractors(&mut correlation, 0);
    let correlation_only_effects = correlated.effects.iter().sum();
    quiescent &= correlated.quiescent;
    add_work(&mut work, &correlated.work);

    let mut no_return = build_matter(variant, 0x3000_0000, false, true);
    let no_return_training = train_at(&mut no_return, [0, 1], [2, 3], 0);
    no_return.substrate.advance_time(180);
    let no_return_namespace = no_return.namespace;
    let no_return_observed = activate(&mut no_return, &[0, 1], 180, no_return_namespace + 0x30000);
    let no_return_heldout_effects = no_return_observed.effects.iter().sum();
    quiescent &= no_return_training.quiescent && no_return_observed.quiescent;
    add_work(&mut work, &no_return_training.work);
    add_work(&mut work, &no_return_observed.work);

    let mut absent = build_matter(variant, 0x4000_0000, true, false);
    let absent_namespace = absent.namespace;
    let absent_burst = activate_burst(&mut absent, &[0, 1], 0, absent_namespace + 0x10000);
    let absent_opportunity_effects = absent_burst.effects.iter().sum::<usize>();
    quiescent &= absent_burst.quiescent;
    add_work(&mut work, &absent_burst.work);

    let mut stale = build_matter(variant, 0x5000_0000, true, true);
    let pressure = stale.substrate.advance_time(30);
    add_work(&mut work, &pressure);
    let stale_namespace = stale.namespace;
    let stale_use = activate(&mut stale, &[0, 1], 30, stale_namespace + 0x10000);
    let stale_opportunity_effects = stale_use.effects.iter().sum();
    quiescent &= stale_use.quiescent;
    add_work(&mut work, &stale_use.work);

    let mut three = build_matter(variant, 0x6000_0000, true, true);
    let three_namespace = three.namespace;
    let three_burst = activate_burst(&mut three, &[0, 1, 3], 0, three_namespace + 0x10000);
    let ambiguity_three_effects = three_burst.effects.iter().sum::<usize>();
    quiescent &= three_burst.quiescent;
    add_work(&mut work, &three_burst.work);

    let mut four = build_matter(variant, 0x7000_0000, true, true);
    let four_namespace = four.namespace;
    let four_burst = activate_burst(&mut four, &[0, 1, 2, 3], 0, four_namespace + 0x10000);
    let ambiguity_four_effects = four_burst.effects.iter().sum::<usize>();
    quiescent &= four_burst.quiescent;
    add_work(&mut work, &four_burst.work);

    let mut alternative = build_matter(variant, 0x8000_0000, true, true);
    let alternative_training = train_at(&mut alternative, [0, 3], [2, 1], 0);
    add_work(&mut work, &alternative_training.work);
    let mut fresh_alternative_old_effects = 0;
    let mut fresh_alternative_new_effects = 0;
    for (index, site) in OLD.into_iter().enumerate() {
        let (_, value) = observe_once(
            &alternative,
            &INCIDENCE[site],
            116,
            alternative.namespace + 0x30000 + index as u64 * 0x1000,
        );
        fresh_alternative_old_effects += value.effects[site];
        quiescent &= value.quiescent;
        add_work(&mut work, &value.work);
    }
    for (index, site) in NEW.into_iter().enumerate() {
        let (_, value) = observe_once(
            &alternative,
            &INCIDENCE[site],
            116,
            alternative.namespace + 0x40000 + index as u64 * 0x1000,
        );
        fresh_alternative_new_effects += value.effects[site];
        quiescent &= value.quiescent;
        add_work(&mut work, &value.work);
    }

    ControlMetrics {
        too_late_effects,
        correlation_only_effects,
        no_return_heldout_effects,
        absent_opportunity_effects,
        stale_opportunity_effects,
        ambiguity_three_effects,
        ambiguity_four_effects,
        fresh_alternative_old_effects,
        fresh_alternative_new_effects,
        quiescent,
        work,
    }
}

fn claims(metrics: &Metrics, duplicate_exact: bool) -> Claims {
    let p0_frozen = source_audit();
    let p1_marginals = metrics
        .route_occurrences
        .windows(2)
        .all(|pair| pair[0] == pair[1])
        && metrics
            .training_port_firings
            .windows(2)
            .all(|pair| pair[0] == pair[1])
        && metrics
            .reversal_port_firings
            .windows(2)
            .all(|pair| pair[0] == pair[1]);
    let p2_training = OLD.iter().all(|site| metrics.training_effects[*site] > 0)
        && [1usize, 2, 3, 4]
            .into_iter()
            .all(|site| metrics.training_effects[site] == 0)
        && metrics
            .trained_support
            .iter()
            .flatten()
            .all(|value| *value > 1)
        && metrics
            .crossed_support
            .iter()
            .flatten()
            .all(|value| *value <= 1);
    let p3_discrimination = metrics.heldout_old_effects == [1, 1]
        && metrics.heldout_new_effects == [0, 0]
        && metrics.singleton_once_effects == [0; ROUTES];
    let p4_singleton_self_evidence = metrics.repeated_a_effects.iter().sum::<usize>() == 0
        && metrics.repeated_b_effects.iter().sum::<usize>() == 0
        && metrics.repeated_a_old_support_after == metrics.repeated_a_old_support_before;
    let p5_reversal_bootstrap = metrics.reversal_old_effects == [0, 0]
        && metrics.reversal_new_effects.iter().all(|value| *value > 0)
        && metrics.post_reversal_old_effects == [0, 0]
        && metrics.post_reversal_new_effects == [1, 1]
        && metrics
            .post_reversal_old_support
            .iter()
            .flatten()
            .all(|value| *value == 0)
        && metrics
            .post_reversal_new_support
            .iter()
            .flatten()
            .all(|value| *value > 1);
    let p6_controls = metrics.too_late_effects == 0
        && metrics.correlation_only_effects == 0
        && metrics.no_return_heldout_effects == 0
        && metrics.absent_opportunity_effects == 0
        && metrics.stale_opportunity_effects == 0
        && metrics.ambiguity_three_effects == 3
        && metrics.ambiguity_four_effects == 6
        && metrics.fresh_alternative_old_effects == 0
        && metrics.fresh_alternative_new_effects == 2
        && metrics.source_refirings == 0
        && metrics.naturally_quiescent
        && metrics.work.total() > 0
        && metrics.persistent_bytes > 0;
    Claims {
        p0_frozen,
        p1_marginals,
        p2_training,
        p3_discrimination,
        p4_singleton_self_evidence,
        p5_reversal_bootstrap,
        p6_controls,
        p7_duplicate: duplicate_exact,
    }
}

fn add_work(total: &mut WorkLedger, value: &WorkLedger) {
    total.queue_comparisons += value.queue_comparisons;
    total.spikes_delivered += value.spikes_delivered;
    total.generation_checks += value.generation_checks;
    total.state_updates += value.state_updates;
    total.threshold_checks += value.threshold_checks;
    total.firings += value.firings;
    total.arrow_checks += value.arrow_checks;
    total.spikes_emitted += value.spikes_emitted;
    total.local_eligibility_writes += value.local_eligibility_writes;
    total.local_return_updates += value.local_return_updates;
    total.ordinary_pressure_updates += value.ordinary_pressure_updates;
    total.local_structural_proposals += value.local_structural_proposals;
    total.physical_deallocations += value.physical_deallocations;
}

fn source_audit() -> bool {
    sha256("crates/px0-physical-correspondence/src/lib.rs") == AUTHORITY_SOURCE_SHA256
        && sha256("experiments/cj0_a_coincidence_threshold_probe_v1_protocol.md")
            == V1_PROTOCOL_SHA256
        && sha256("experiments/cj0_a_coincidence_threshold_probe_v1_invalid_audit.md")
            == V1_INVALID_SHA256
        && rev_parse("cj0-a-coincidence-threshold-probe-v1-invalid^{commit}") == V1_INVALID_COMMIT
        && sha256("experiments/cj0_a_coincidence_threshold_probe_v2_protocol.md")
            == V2_PROTOCOL_SHA256
        && sha256("experiments/cj0_a_coincidence_threshold_probe_v2_invalid_audit.md")
            == V2_INVALID_SHA256
        && rev_parse("cj0-a-coincidence-threshold-probe-v2-invalid^{commit}") == V2_INVALID_COMMIT
        && sha256("experiments/cj0_a_coincidence_threshold_probe_v3_protocol.md")
            == V3_PROTOCOL_SHA256
        && rev_parse("HEAD^{commit}") != AUTHORITATIVE_COMMIT
        && rev_parse(&format!("{AUTHORITY_TAG}^{{commit}}")) == AUTHORITATIVE_COMMIT
        && rev_parse("px3-physical-event-boundaries-frozen-negative-handoff-v1^{commit}")
            == PX3_NEGATIVE
        && rev_parse("px3-r-direct-trace-coupling-frozen-negative-handoff-v1^{commit}")
            == ARM_A_NEGATIVE
        && rev_parse("px3-r-shared-cell-frozen-negative-handoff-v1^{commit}") == ARM_B_NEGATIVE
        && rev_parse("px3-r-c-downstream-convergence-frozen-negative-handoff-v1^{commit}")
            == ARM_C_NEGATIVE
        && organism_visible_audit()
}

fn organism_visible_audit() -> bool {
    let source = read_to_string(SOURCE_PATH).expect("source must be readable");
    let start = source
        .find("// BEGIN ORGANISM-VISIBLE PHYSICAL BLOCK")
        .expect("physical block start");
    let end = source
        .find("// END ORGANISM-VISIBLE PHYSICAL BLOCK")
        .expect("physical block end");
    let block = source[start..end].to_ascii_lowercase();
    [
        "event",
        "episode",
        "history",
        "pair",
        "group",
        "member",
        "semantic",
        "evaluator",
        "trained",
        "crossed",
    ]
    .into_iter()
    .all(|token| !block.contains(token))
}

fn rev_parse(reference: &str) -> String {
    let output = Command::new("git")
        .args(["rev-parse", reference])
        .output()
        .expect("git rev-parse must run");
    assert!(output.status.success(), "git ref must exist: {reference}");
    String::from_utf8(output.stdout)
        .expect("git output must be utf8")
        .trim()
        .to_string()
}

fn sha256(path: &str) -> String {
    let output = Command::new("shasum")
        .args(["-a", "256", path])
        .output()
        .expect("shasum must run");
    assert!(output.status.success(), "hash target must exist: {path}");
    String::from_utf8(output.stdout)
        .expect("hash output must be utf8")
        .split_whitespace()
        .next()
        .expect("hash must be present")
        .to_string()
}

fn csv(rows: &[Row]) -> String {
    let mut out = String::from(
        "variant,namespace,route_occurrences,training_port_firings,training_site_firings,training_effects,trained_support,crossed_support,heldout_old_effects,heldout_new_effects,singleton_once_effects,repeated_a_effects,repeated_b_effects,repeated_a_support_before,repeated_a_support_after,reversal_port_firings,reversal_old_effects,reversal_new_effects,post_old_effects,post_new_effects,post_old_support,post_new_support,too_late,correlation_only,no_return,absent_opportunity,stale_opportunity,ambiguity_three,ambiguity_four,fresh_alternative_old,fresh_alternative_new,source_refirings,quiescent,work,persistent_bytes,fingerprint,duplicate_exact,claims,first_failure,passed\n",
    );
    for row in rows {
        let m = &row.metrics;
        out.push_str(&format!(
            "{},{:#x},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            row.variant,
            row.namespace,
            join(&m.route_occurrences),
            join(&m.training_port_firings),
            join(&m.training_site_firings),
            join(&m.training_effects),
            join_nested(&m.trained_support),
            join_nested(&m.crossed_support),
            join(&m.heldout_old_effects),
            join(&m.heldout_new_effects),
            join(&m.singleton_once_effects),
            join(&m.repeated_a_effects),
            join(&m.repeated_b_effects),
            join(&m.repeated_a_old_support_before),
            join(&m.repeated_a_old_support_after),
            join(&m.reversal_port_firings),
            join(&m.reversal_old_effects),
            join(&m.reversal_new_effects),
            join(&m.post_reversal_old_effects),
            join(&m.post_reversal_new_effects),
            join_nested(&m.post_reversal_old_support),
            join_nested(&m.post_reversal_new_support),
            m.too_late_effects,
            m.correlation_only_effects,
            m.no_return_heldout_effects,
            m.absent_opportunity_effects,
            m.stale_opportunity_effects,
            m.ambiguity_three_effects,
            m.ambiguity_four_effects,
            m.fresh_alternative_old_effects,
            m.fresh_alternative_new_effects,
            m.source_refirings,
            m.naturally_quiescent,
            m.work.total(),
            m.persistent_bytes,
            m.fingerprint,
            row.duplicate_exact,
            row.claims.bits(),
            row.claims.first_failure(),
            row.claims.all(),
        ));
    }
    out
}

fn markdown(rows: &[Row], passed: bool) -> String {
    let classification = if passed {
        "PROBE_POSITIVE_MICRO_ELIGIBLE"
    } else {
        "FROZEN_NEGATIVE_EXISTING_PHYSICS_SELF_EVIDENCE"
    };
    let total_work = rows.iter().map(|row| row.metrics.work.total()).sum::<u64>();
    let total_storage = rows
        .iter()
        .map(|row| row.metrics.persistent_bytes)
        .sum::<usize>();
    let mut out = format!(
        "# CJ0 Arm A coincidence-threshold CELL PROBE v3\n\n- Classification: `{classification}`\n- Candidate law added: `none`\n- Authoritative source changed: `false`\n- Rows passed: `{}/{}`\n- Ledgered work: `{total_work}`\n- Final persistent matter across primary executions: `{total_storage}` bytes\n\n| replica | initial discriminator | repeated singleton | reversal old/new | claims | first failure | duplicate |\n|---|---:|---:|---:|---:|---:|---:|\n",
        rows.iter().filter(|row| row.claims.all()).count(),
        rows.len(),
    );
    for row in rows {
        let m = &row.metrics;
        out.push_str(&format!(
            "| {} | old {} / crossed {} | A {} / B {} | old {} / new {} | `{}` | `{}` | {} |\n",
            row.variant,
            join(&m.heldout_old_effects),
            join(&m.heldout_new_effects),
            m.repeated_a_effects.iter().sum::<usize>(),
            m.repeated_b_effects.iter().sum::<usize>(),
            join(&m.post_reversal_old_effects),
            join(&m.post_reversal_new_effects),
            row.claims.bits(),
            row.claims.first_failure(),
            row.duplicate_exact,
        ));
    }
    out.push_str(
        "\nThe CSV serializes every physical stage and control. A negative is terminal for CJ0-A: MICRO, GATE, recursion, OR, the temporal-expressivity matrix, definitive evidence, and authority are not executed.\n",
    );
    out
}

fn join<T: ToString>(values: &[T]) -> String {
    values
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("|")
}

fn join_nested<T: ToString, const N: usize, const M: usize>(values: &[[T; N]; M]) -> String {
    values
        .iter()
        .map(|inner| join(inner))
        .collect::<Vec<_>>()
        .join(";")
}

fn publish(csv: &str, markdown: &str) {
    write_new(STAGING_CSV, csv);
    write_new(STAGING_MD, markdown);
    rename(STAGING_CSV, RESULT_CSV).expect("CSV publication must be atomic");
    rename(STAGING_MD, RESULT_MD).expect("report publication must be atomic");
}

fn write_new(path: &str, contents: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .unwrap_or_else(|error| panic!("fresh staging path required: {path}: {error}"));
    file.write_all(contents.as_bytes())
        .expect("staging write must complete");
    file.sync_all().expect("staging write must be durable");
}
