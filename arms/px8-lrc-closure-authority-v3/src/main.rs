#![forbid(unsafe_code)]
use px7_lrc_arrival::{
    Activity, Arrival, Body, Form, BOUNDARY_A, BOUNDARY_B, BOUNDARY_C, BOUNDARY_D, DOWNSTREAM_ONE,
    DOWNSTREAM_ZERO, INNER_ONE, INNER_ZERO, OUTWARD_SITE, RETURN_SITE,
};
use px8_lrc_physical_closure::{CompactBody, CompactForm, Layout, Reading, RecursiveBody};
use std::{
    collections::BTreeSet,
    env,
    fmt::Write as _,
    fs::{rename, OpenOptions},
    io::Write as _,
    path::Path,
    process::Command,
};

const LAW: &str = "7226a0e4af0ff484c6fd61c46c9073ce8363692100c2a090b0ce64483f3cfc10";
const PX4: &str = "a201674f9d558b5bda20aef71e9857b632f8a6565f372aee88994a280e0fea71";
const PX7: &str = "d248a8af479872d8148115a405ae7332f7d24ca229378d3fde898ffd3d19e63e";
const PX8: &str = "8623cae7dc1b14e666140a192c49e59ae72df681bc14f0359f0c4465e0d11e8f";
const PROTOCOL: &str = "3805623f6b9ad5d138ba1c90c1b99afb9063c74381cb5545e059254996d7a227";
const WORK: u64 = 20_000;
const CAP: usize = 8_192;
const ARROW_BYTES: usize = 64;
const CSV: &str = "results/px8_lrc_closure_authority_v3.csv";
const MD: &str = "results/px8_lrc_closure_authority_v3.md";
const CSV_S: &str = "results/px8_lrc_closure_authority_v3.csv.staging";
const MD_S: &str = "results/px8_lrc_closure_authority_v3.md.staging";

const CASES: [Case; 16] = [
    Case::new(865_001, false, false, 0),
    Case::new(865_002, false, false, 137),
    Case::new(865_003, false, false, 274),
    Case::new(865_004, false, false, 411),
    Case::new(865_005, true, false, 0),
    Case::new(865_006, true, false, 137),
    Case::new(865_007, true, false, 274),
    Case::new(865_008, true, false, 411),
    Case::new(865_009, false, true, 0),
    Case::new(865_010, false, true, 137),
    Case::new(865_011, false, true, 274),
    Case::new(865_012, false, true, 411),
    Case::new(865_013, true, true, 0),
    Case::new(865_014, true, true, 137),
    Case::new(865_015, true, true, 274),
    Case::new(865_016, true, true, 411),
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Case {
    root: u64,
    reverse: bool,
    reflect: bool,
    twist: u64,
}
impl Case {
    const fn new(root: u64, reverse: bool, reflect: bool, twist: u64) -> Self {
        Self {
            root,
            reverse,
            reflect,
            twist,
        }
    }
    fn layout(self) -> Layout {
        Layout {
            namespace: self.root << 32,
            reverse: self.reverse,
            reflect: self.reflect,
            twist: self.twist,
            outward_resistance: 100,
        }
    }
    fn altered(self) -> bool {
        matches!(self.twist, 137 | 411)
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Obs {
    inner: [usize; 2],
    down: [usize; 2],
    links: [usize; 2],
    out: usize,
    modulation: u64,
    updates: u64,
    work: u64,
    quiet: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Cum {
    pass: bool,
    first: Obs,
    second: Obs,
    held: Obs,
    before: usize,
    after: usize,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Mature {
    primary: [usize; 2],
    uninterrupted: [usize; 2],
    incomplete: [usize; 2],
    duplicate: [usize; 2],
    blocked: [usize; 2],
    cumulative: [usize; 2],
}
impl Mature {
    fn stable(self) -> bool {
        [
            self.primary,
            self.uninterrupted,
            self.incomplete,
            self.duplicate,
            self.blocked,
            self.cumulative,
        ]
        .into_iter()
        .all(|p| p[0] == p[1])
    }
    fn max(self) -> usize {
        [
            self.primary,
            self.uninterrupted,
            self.incomplete,
            self.duplicate,
            self.blocked,
            self.cumulative,
        ]
        .into_iter()
        .flatten()
        .max()
        .unwrap_or(0)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Stale {
    before: usize,
    after: usize,
    delta: i64,
    capacity: usize,
    outward: usize,
    route_executions: usize,
    fresh_proposals: usize,
    queue_empty: bool,
    quiet: bool,
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct Trial {
    case: Case,
    formation: Reading,
    completed: Reading,
    incomplete: Reading,
    blocked: Reading,
    stale_reading: Reading,
    dup_rec: Reading,
    zero: Reading,
    dup_phy: Reading,
    open: Reading,
    aged: Reading,
    branch: Reading,
    cycle: Reading,
    pause: bool,
    resume: bool,
    cum: Cum,
    mature: Mature,
    stale: Stale,
    max_work: u64,
    max_bytes: usize,
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct Row {
    trial: Trial,
    claims: [bool; 14],
    replay: bool,
    passed: bool,
}

fn main() {
    audit();
    surface();
    absent();
    match env::args().skip(1).collect::<Vec<_>>().as_slice() {
        [x] if x == "--authority-preflight" => {
            println!("PX8_LRC_CLOSURE_AUTHORITY_V3_PREFLIGHT_OK")
        }
        [x] if x == "--authority-v3" => authority(),
        _ => std::process::exit(2),
    }
}
fn audit() {
    for (p, h) in [
        ("crates/lr1-modulatory-physical-return/src/lib.rs", LAW),
        ("arms/px4-lrc-lifetime/src/lib.rs", PX4),
        ("crates/px7-lrc-arrival/src/lib.rs", PX7),
        ("arms/px8-lrc-physical-closure/src/lib.rs", PX8),
        (
            "experiments/px8_lrc_cumulative_closure_authority_v3_protocol_v1.md",
            PROTOCOL,
        ),
    ] {
        assert_eq!(sha(p), h, "frozen input changed: {p}")
    }
}
fn surface() {
    assert_eq!(CASES.into_iter().collect::<BTreeSet<_>>().len(), 16);
    assert_eq!(CASES[0].root, 865_001);
    assert_eq!(CASES[15].root, 865_016);
    for r in [false, true] {
        for f in [false, true] {
            assert_eq!(
                CASES
                    .iter()
                    .filter(|c| c.reverse == r && c.reflect == f)
                    .count(),
                4
            )
        }
    }
    for t in [0, 137, 274, 411] {
        assert_eq!(CASES.iter().filter(|c| c.twist == t).count(), 4)
    }
}
fn absent() {
    for p in [CSV, MD, CSV_S, MD_S] {
        assert!(!Path::new(p).exists(), "artifact exists: {p}")
    }
}
fn authority() {
    eprintln!("PX8_LRC_CLOSURE_AUTHORITY_V3_EVIDENCE_SPENT");
    let rows = CASES.into_iter().map(replay).collect::<Vec<_>>();
    let globals = globals(&rows);
    publish(CSV_S, CSV, &csv(&rows));
    publish(MD_S, MD, &report(&rows, globals));
    assert!(rows.iter().all(|r| r.passed), "authority-v3 row failed");
    assert!(globals.into_iter().all(|x| x), "authority-v3 global failed");
    println!("PX8_LRC_CLOSURE_AUTHORITY_V3_PASS rows=16/16 clauses=230/230")
}
fn replay(case: Case) -> Row {
    let a = run(case);
    let b = run(case);
    let exact = a == b;
    let mut r = row(a, exact);
    r.replay = exact;
    r.claims[13] = exact;
    r.passed = r.claims.into_iter().all(|x| x);
    r
}

fn run(case: Case) -> Trial {
    let layout = case.layout();
    let ns = layout.namespace;
    let mut learned = RecursiveBody::new(layout);
    let formation = learned.learn_twice();
    let pb = learned.persistent_bytes();
    let pa = learned.fingerprints();
    let pp = learned.fingerprints();
    let mut uninterrupted = learned.clone();
    let ub = uninterrupted.persistent_bytes();
    let completed = learned.reuse([true; 4], 61, false);
    let pafter = learned.persistent_bytes();
    let uc = uninterrupted.reuse([true; 4], 61, false);
    let ua = uninterrupted.persistent_bytes();
    let mut inc = uninterrupted.clone();
    let ib = inc.persistent_bytes();
    let incomplete = inc.reuse([true, true, true, false], 70, false);
    let ia = inc.persistent_bytes();
    let mut dup = RecursiveBody::new(layout);
    dup.learn_twice();
    let db = dup.persistent_bytes();
    let dup_rec = dup.reuse([true; 4], 61, true);
    let da = dup.persistent_bytes();
    let mut block = RecursiveBody::new(Layout {
        outward_resistance: 0,
        ..layout
    });
    block.learn_twice();
    let bb = block.persistent_bytes();
    let blocked = block.reuse([true; 4], 61, false);
    let ba = block.persistent_bytes();
    let mut stale_body = RecursiveBody::new(layout);
    stale_body.learn_once_then_age();
    let sb = stale_body.persistent_bytes();
    let stale_reading = stale_body.reuse([true; 4], 111, false);
    let sa = stale_body.persistent_bytes();
    let mut d = CompactBody::new(ns + 10_000, CompactForm::Direct, case.reflect);
    let zero = d.flow(1, 0);
    let mut d = CompactBody::new(ns + 20_000, CompactForm::Direct, case.reflect);
    let dup_phy = d.flow(2, 0);
    let mut d = CompactBody::new(ns + 30_000, CompactForm::Open, case.reflect);
    let open = d.flow(1, 0);
    let mut d = CompactBody::new(ns + 40_000, CompactForm::Aged, case.reflect);
    let aged = d.flow(1, 10);
    let mut d = CompactBody::new(ns + 50_000, CompactForm::Fork, case.reflect);
    let branch = d.flow(1, 0);
    let mut d = CompactBody::new(ns + 60_000, CompactForm::Ring, case.reflect);
    let cycle = d.flow(1, 0);
    let cum = cumulative(case);
    let mature = Mature {
        primary: [pb, pafter],
        uninterrupted: [ub, ua],
        incomplete: [ib, ia],
        duplicate: [db, da],
        blocked: [bb, ba],
        cumulative: [cum.before, cum.after],
    };
    let delta = sa as i64 - sb as i64;
    let stale = Stale {
        before: sb,
        after: sa,
        delta,
        capacity: CAP,
        outward: stale_reading.outward,
        route_executions: stale_reading.outward + stale_reading.inward_modulation,
        fresh_proposals: sa.saturating_sub(sb) / ARROW_BYTES,
        queue_empty: stale_reading.quiet,
        quiet: stale_reading.quiet,
    };
    let readings = [
        formation,
        completed,
        incomplete,
        blocked,
        stale_reading,
        dup_rec,
        zero,
        dup_phy,
        open,
        aged,
        branch,
        cycle,
    ];
    let max_work = readings
        .into_iter()
        .map(|r| r.work)
        .chain([cum.first.work, cum.second.work, cum.held.work])
        .max()
        .unwrap_or(0);
    let max_bytes = mature.max().max(sa).max(sb);
    Trial {
        case,
        formation,
        completed,
        incomplete,
        blocked,
        stale_reading,
        dup_rec,
        zero,
        dup_phy,
        open,
        aged,
        branch,
        cycle,
        pause: pa == pp,
        resume: completed == uc,
        cum,
        mature,
        stale,
        max_work,
        max_bytes,
    }
}

fn row(t: Trial, replay: bool) -> Row {
    let quiet = [
        t.formation,
        t.completed,
        t.incomplete,
        t.blocked,
        t.stale_reading,
        t.dup_rec,
        t.zero,
        t.dup_phy,
        t.open,
        t.aged,
        t.branch,
        t.cycle,
    ]
    .into_iter()
    .all(|r| r.quiet)
        && t.cum.first.quiet
        && t.cum.second.quiet
        && t.cum.held.quiet;
    let stale_ok = t.stale.outward == 0
        && t.stale.route_executions == 0
        && t.stale.after <= CAP
        && t.stale.queue_empty
        && t.stale.quiet
        && replay;
    let claims = [
        t.formation.updates > 0 && t.formation.inward_modulation > 0,
        t.completed.outward == 1,
        t.completed.inward_modulation == 1 && t.completed.updates == 1,
        t.incomplete.outward == 0 && t.blocked.outward == 0 && t.stale_reading.outward == 0,
        t.open.outward == 0 && t.aged.outward == 0,
        t.zero.outward == 1,
        t.dup_phy.outward == 1 && t.dup_rec.outward == 1,
        t.branch.outward == 0 && t.cycle.outward == 0,
        t.pause && t.resume,
        quiet,
        t.max_work <= WORK,
        t.max_bytes <= CAP && t.mature.stable() && stale_ok,
        t.cum.pass,
        replay,
    ];
    let passed = claims.into_iter().all(|x| x);
    Row {
        trial: t,
        claims,
        replay,
        passed,
    }
}

fn cumulative(case: Case) -> Cum {
    let mut b = Body::new(Form {
        namespace: (case.root + 1_000_000) << 32,
        reverse_construction: case.reverse,
        reflected_positions: case.reflect,
        altered_pairing: case.altered(),
    });
    let a = maturation(&b, 0, case.altered());
    let x = b.participate(a);
    let first = observe(&b, x);
    let a = maturation(&b, 10, case.altered());
    let x = b.participate(a);
    let second = observe(&b, x);
    let coupling = b.link_couplings();
    let resistance = b.link_resistances();
    let before = b.persistent_bytes();
    let a = boundary(&b, 20, case.altered());
    let x = b.participate(a);
    let held = observe(&b, x);
    let after = b.persistent_bytes();
    let pass = first.modulation == 2
        && first.updates == 2
        && second.modulation == 2
        && second.updates == 2
        && coupling == [2, 2]
        && resistance == [6, 6]
        && held.inner == [1, 1]
        && held.down == [1, 1]
        && held.links == [1, 1]
        && held.out == 1
        && held.modulation == 0
        && held.updates == 0
        && first.quiet
        && second.quiet
        && held.quiet
        && before == after;
    Cum {
        pass,
        first,
        second,
        held,
        before,
        after,
    }
}
fn observe(b: &Body, a: Activity) -> Obs {
    let e = &a.execution;
    let fires = |s| {
        let p = b.physical(s);
        e.trace
            .iter()
            .filter(|x| x.target_physical == p && x.fired)
            .count()
    };
    let cross = |f, t| {
        let f = b.physical(f);
        let t = b.physical(t);
        e.crossings
            .iter()
            .filter(|x| x.from_physical == f && x.to_physical == t)
            .count()
    };
    Obs {
        inner: [INNER_ZERO, INNER_ONE].map(fires),
        down: [DOWNSTREAM_ZERO, DOWNSTREAM_ONE].map(fires),
        links: [
            cross(INNER_ZERO, DOWNSTREAM_ZERO),
            cross(INNER_ONE, DOWNSTREAM_ONE),
        ],
        out: cross(DOWNSTREAM_ONE, OUTWARD_SITE),
        modulation: e.work.modulatory_deliveries,
        updates: e.work.local_return_updates,
        work: a.work(),
        quiet: a.naturally_quiescent(),
    }
}
fn boundary(b: &Body, t: i64, alt: bool) -> [Arrival; 3] {
    let p = if alt { BOUNDARY_D } else { BOUNDARY_B };
    [
        at(b, BOUNDARY_A, t, 10),
        at(b, p, t, 11),
        at(b, BOUNDARY_C, t + 2, 12),
    ]
}
fn maturation(b: &Body, t: i64, alt: bool) -> [Arrival; 7] {
    let [a, p, c] = boundary(b, t, alt);
    [
        a,
        p,
        c,
        at(b, DOWNSTREAM_ZERO, t + 2, 20),
        at(b, RETURN_SITE, t + 3, 21),
        at(b, DOWNSTREAM_ONE, t + 4, 22),
        at(b, RETURN_SITE, t + 5, 23),
    ]
}
fn at(b: &Body, s: usize, t: i64, p: i32) -> Arrival {
    Arrival {
        tick: t,
        phase: p,
        origin: b.physical(s) + 21_000_000 + p as u64,
        position: b.coordinate(s),
        impulse: 1,
    }
}

fn csv(rows: &[Row]) -> String {
    let mut s=String::from("root,namespace,reverse,reflect,twist,formation_updates,completed_outward,completed_return,incomplete_outward,blocked_outward,stale_outward,zero_outward,duplicate_physical,duplicate_recursive,branch,cycle,max_work,max_bytes,mature_pairs,stale_memory_before,stale_memory_after,stale_delta,stale_capacity,stale_outward_crossings,stale_route_executions,fresh_proposals,queue_empty,quiescent,replay_exact,cumulative,claims,passed\n");
    for r in rows {
        let t = &r.trial;
        let fields = [
            t.case.root.to_string(),
            (t.case.root << 32).to_string(),
            t.case.reverse.to_string(),
            t.case.reflect.to_string(),
            t.case.twist.to_string(),
            t.formation.updates.to_string(),
            t.completed.outward.to_string(),
            t.completed.inward_modulation.to_string(),
            t.incomplete.outward.to_string(),
            t.blocked.outward.to_string(),
            t.stale_reading.outward.to_string(),
            t.zero.outward.to_string(),
            t.dup_phy.outward.to_string(),
            t.dup_rec.outward.to_string(),
            t.branch.outward.to_string(),
            t.cycle.outward.to_string(),
            t.max_work.to_string(),
            t.max_bytes.to_string(),
            mature_sig(t.mature),
            t.stale.before.to_string(),
            t.stale.after.to_string(),
            t.stale.delta.to_string(),
            t.stale.capacity.to_string(),
            t.stale.outward.to_string(),
            t.stale.route_executions.to_string(),
            t.stale.fresh_proposals.to_string(),
            t.stale.queue_empty.to_string(),
            t.stale.quiet.to_string(),
            r.replay.to_string(),
            format!(
                "pass={}~before={}~after={}",
                t.cum.pass, t.cum.before, t.cum.after
            ),
            r.claims
                .iter()
                .map(bool::to_string)
                .collect::<Vec<_>>()
                .join("|"),
            r.passed.to_string(),
        ];
        s.push_str(&fields.join(","));
        s.push('\n')
    }
    s
}
fn mature_sig(m: Mature) -> String {
    format!("primary={}|{};uninterrupted={}|{};incomplete={}|{};duplicate={}|{};blocked={}|{};cumulative={}|{}",m.primary[0],m.primary[1],m.uninterrupted[0],m.uninterrupted[1],m.incomplete[0],m.incomplete[1],m.duplicate[0],m.duplicate[1],m.blocked[0],m.blocked[1],m.cumulative[0],m.cumulative[1])
}
fn identity_namespaces(case: Case) -> [u64; 8] {
    let base = case.root << 32;
    [
        base,
        base + 10_000,
        base + 20_000,
        base + 30_000,
        base + 40_000,
        base + 50_000,
        base + 60_000,
        (case.root + 1_000_000) << 32,
    ]
}
fn globals(rows: &[Row]) -> [bool; 6] {
    let roots = rows
        .iter()
        .map(|r| r.trial.case.root)
        .collect::<BTreeSet<_>>();
    let balanced = [false, true].into_iter().all(|reverse| {
        [false, true].into_iter().all(|reflect| {
            rows.iter()
                .filter(|r| r.trial.case.reverse == reverse && r.trial.case.reflect == reflect)
                .count()
                == 4
        })
    }) && [0, 137, 274, 411]
        .into_iter()
        .all(|twist| rows.iter().filter(|r| r.trial.case.twist == twist).count() == 4);
    let identities = rows
        .iter()
        .flat_map(|r| identity_namespaces(r.trial.case))
        .collect::<BTreeSet<_>>();
    [
        roots.len() == 16 && roots.iter().copied().eq(865_001..=865_016),
        balanced,
        identities.len() == 128,
        rows.iter().all(|r| r.claims[12]),
        rows.iter()
            .all(|r| r.claims[9] && r.claims[10] && r.claims[11] && r.claims[13]),
        rows.len() == 16 && rows.iter().all(|r| r.claims.len() == 14),
    ]
}
fn report(rows: &[Row], g: [bool; 6]) -> String {
    let passed = rows.iter().filter(|r| r.passed).count();
    let rc = rows
        .iter()
        .map(|r| r.claims.iter().filter(|x| **x).count())
        .sum::<usize>();
    let gc = g.iter().filter(|x| **x).count();
    let mut s = String::new();
    writeln!(
        s,
        "# PX8 LR-C cumulative physical-closure/emission authority v3\n"
    )
    .unwrap();
    writeln!(
        s,
        "Outcome: **{}**.\n",
        if passed == 16 && gc == 6 {
            "DEFINITIVE POSITIVE"
        } else {
            "NEGATIVE"
        }
    )
    .unwrap();
    writeln!(s,"- rows: `{passed}/16`;\n- row clauses: `{rc}/224`;\n- global clauses: `{gc}/6`;\n- total clauses: `{}/230`;",rc+gc).unwrap();
    writeln!(
        s,
        "- maximum work: `{}`;\n- maximum bytes: `{}`;",
        rows.iter().map(|r| r.trial.max_work).max().unwrap_or(0),
        rows.iter().map(|r| r.trial.max_bytes).max().unwrap_or(0)
    )
    .unwrap();
    writeln!(
        s,
        "- mature memory stable: `{}`;",
        rows.iter().all(|r| r.trial.mature.stable())
    )
    .unwrap();
    writeln!(
        s,
        "- exact replay: `{}`;\n- final PX-C authority: `false`.\n",
        rows.iter().all(|r| r.replay)
    )
    .unwrap();
    writeln!(s, "## Unconditional stale/reproposal observations\n").unwrap();
    for r in rows {
        let t = &r.trial;
        writeln!(s,"- root `{}`: `memory_before={} memory_after={} delta={} capacity={} outward_crossings={} stale_route_executions={} fresh_proposals={} queue_empty={} quiescent={} replay_exact={} claims={} passed={}`",t.case.root,t.stale.before,t.stale.after,t.stale.delta,t.stale.capacity,t.stale.outward,t.stale.route_executions,t.stale.fresh_proposals,t.stale.queue_empty,t.stale.quiet,r.replay,r.claims.iter().map(bool::to_string).collect::<Vec<_>>().join("|"),r.passed).unwrap()
    }
    s
}
fn sha(p: &str) -> String {
    let o = Command::new("sha256sum").arg(p).output().expect("sha");
    assert!(o.status.success());
    String::from_utf8(o.stdout)
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap()
        .to_owned()
}
fn publish(stage: &str, dest: &str, text: &str) {
    let mut f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(stage)
        .expect("create");
    f.write_all(text.as_bytes()).unwrap();
    f.sync_all().unwrap();
    rename(stage, dest).unwrap()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matrix_definition_is_frozen() {
        surface();
        assert_eq!(CASES.len(), 16);
        assert_eq!(CAP, 8192);
        assert_eq!(ARROW_BYTES, 64)
    }
}
