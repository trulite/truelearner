#![forbid(unsafe_code)]

use px0_physical_correspondence::{
    ArrowId, ArrowSpec, CellId, CellSpec, Execution, PlasticSubstrate, SpikeInput,
};
use std::collections::BTreeSet;
use std::env;
use std::fs::{rename, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;
const PX0: &str = "3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d";
const D1: &str = "511efc76fc36c9c3c77815f1bd53bd4f8ea38411f8892bbc26005af1e7d0fecb";
const AUDIT: &str = "06135ca0e63cb9dd944f172e6c072db2ae77b6c1eb1653aabba254f9247d13de";
const PROTO: &str = "8273aa6569cafec456f06fb677d07f2854b3b7512f181a2eeef7abd24cf96e20";
const EXEC: &str = "5f410813cbc47f7aa9b3fe98a5854f255e647b3ab96ddfff5e0366cdefa4dc23";
const SEEDS: [u64; 2] = [3001, 3007];
const PAIRS: [(usize, usize); 6] = [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];
const CSV: &str = "results/px3_d1_r2_closed_loop_return_attribution_v1.csv";
const MD: &str = "results/px3_d1_r2_closed_loop_return_attribution_v1.md";
const CS: &str = "results/.px3_d1_r2_closed_loop_return_attribution_v1.csv.staging";
const MS: &str = "results/.px3_d1_r2_closed_loop_return_attribution_v1.md.staging";
#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    ReturnOnly,
    Real,
    BlockedLate,
    LateConsequence,
    NoAbConsequence,
    Swap,
    TwoLoops,
    Real21,
    Real44,
}
#[derive(Clone, Copy, PartialEq, Eq)]
struct Scenario {
    name: &'static str,
    kind: Kind,
    raw: [i32; 4],
}
impl Scenario {
    const ALL: [Self; 9] = [
        Self::n("return-only", Kind::ReturnOnly, [1; 4]),
        Self::n("ab-real-loop", Kind::Real, [1; 4]),
        Self::n("ab-blocked-late-a", Kind::BlockedLate, [1; 4]),
        Self::n("ab-late-consequence", Kind::LateConsequence, [1; 4]),
        Self::n("no-ab-genuine-consequence", Kind::NoAbConsequence, [1; 4]),
        Self::n("ab-swapped-return-to-ac", Kind::Swap, [1; 4]),
        Self::n("ab-then-cd-two-loops", Kind::TwoLoops, [1; 4]),
        Self::n("ab-real-loop-2-1", Kind::Real21, [2, 1, 1, 1]),
        Self::n("ab-real-loop-4-4", Kind::Real44, [4, 4, 1, 1]),
    ];
    const fn n(name: &'static str, kind: Kind, raw: [i32; 4]) -> Self {
        Self { name, kind, raw }
    }
}
#[derive(Clone)]
struct World {
    s: PlasticSubstrate,
    sources: [CellId; 4],
    context: CellId,
    driver: CellId,
    broadcast: CellId,
    candidates: [ArrowId; 6],
    connectors: [ArrowId; 6],
}
#[derive(Clone, PartialEq, Eq)]
struct Row {
    seed: u64,
    scenario: &'static str,
    source: [usize; 4],
    traces: [usize; 4],
    o: [usize; 6],
    p: [usize; 6],
    candidate: [usize; 6],
    candidate_impulse: [i32; 6],
    effect: [usize; 6],
    returns: [usize; 6],
    candidate_r: [u32; 6],
    connector_r: [u32; 6],
    after_pressure: [u32; 6],
    proposals: u64,
    work: u64,
    bytes: usize,
    fingerprint: u64,
    permanent: u64,
    quiescent: bool,
    replay: bool,
    passed: bool,
}
fn main() {
    audit();
    surface();
    absent(&[CSV, MD, CS, MS]);
    match env::args().skip(1).collect::<Vec<_>>().as_slice() {
        [x] if x == "--preflight" => {
            println!("PX3_D1_R2_CLOSED_LOOP_RETURN_ATTRIBUTION_PREFLIGHT_OK")
        }
        [x] if x == "--r2" => evidence(),
        _ => std::process::exit(2),
    }
}
fn evidence() {
    eprintln!("PX3_D1_R2_CLOSED_LOOP_RETURN_ATTRIBUTION_EVIDENCE");
    let mut rows = Vec::new();
    for seed in SEEDS {
        for s in Scenario::ALL {
            rows.push(replay(seed, s));
        }
    }
    assert_eq!(rows.len(), 18);
    publish(CS, CSV, &csv(&rows));
    publish(MS, MD, &report(&rows));
}
fn audit() {
    for (p, h) in [
        ("crates/px0-physical-correspondence/src/lib.rs", PX0),
        (
            "results/px3_d1_participation_gated_pair_learning_v1.csv",
            D1,
        ),
        (
            "experiments/px3_d1_participation_gated_pair_learning_result_audit_v1.md",
            AUDIT,
        ),
        (
            "experiments/px3_d1_r2_closed_loop_return_attribution_protocol_v1.md",
            PROTO,
        ),
        (
            "experiments/px3_d1_r2_closed_loop_return_attribution_execution_protocol_v1.md",
            EXEC,
        ),
    ] {
        assert_eq!(sha(p), h)
    }
}
fn surface() {
    assert_eq!(
        Scenario::ALL
            .iter()
            .map(|x| x.name)
            .collect::<BTreeSet<_>>()
            .len(),
        9
    );
    for p in [
        "arms/px3-d1-r2-closed-loop-return-attribution/src/d2.rs",
        "results/px3_gate_v1.csv",
    ] {
        assert!(!Path::new(p).exists())
    }
}
fn replay(seed: u64, sc: Scenario) -> Row {
    let a = run(seed, sc);
    let b = run(seed, sc);
    let eq = a == b;
    let mut r = a;
    r.replay = eq;
    r.passed &= eq;
    r
}
fn run(seed: u64, sc: Scenario) -> Row {
    let ns = (seed << 32) | ((idx(sc) as u64 + 1) << 16);
    let mut w = build(ns, sc.raw, sc.kind);
    match sc.kind {
        Kind::ReturnOnly => named_pulse(&mut w, 0, 3, 1, 90),
        Kind::Real | Kind::Real21 | Kind::Real44 => {
            sources(&mut w, [0, 1], 2);
            named_pulse(&mut w, 1, 3, 1, 91)
        }
        Kind::BlockedLate => {
            sources(&mut w, [0, 1], 2);
            named_pulse(&mut w, 1, 3, 1, 91);
            source_pulse(&mut w, 0, 4, 92)
        }
        Kind::LateConsequence => {
            sources(&mut w, [0, 1], 2);
            named_pulse(&mut w, 2, 9, 1, 93)
        }
        Kind::NoAbConsequence => named_pulse(&mut w, 2, 3, 1, 93),
        Kind::Swap => {
            sources(&mut w, [0, 1], 2);
            named_pulse(&mut w, 1, 3, 1, 91)
        }
        Kind::TwoLoops => {
            sources(&mut w, [0, 1], 2);
            named_pulse(&mut w, 1, 3, 1, 91);
            sources(&mut w, [2, 3], 5);
            named_pulse(&mut w, 1, 6, 1, 94)
        }
    }
    let e = w.s.propagate();
    let candidate_r = res(&w, w.candidates);
    let connector_r = res(&w, w.connectors);
    let mut later = w.clone();
    later.s.advance_time(50);
    let after_pressure = res(&later, later.candidates);
    let source = four(|i| fire(&e, p(ns, 10 + i as u64)));
    let traces = four(|i| fire(&e, p(ns, 30 + i as u64)));
    let o = six(|i| fire(&e, p(ns, 100 + i as u64)));
    let pp = six(|i| fire(&e, p(ns, 200 + i as u64)));
    let candidate = six(|i| cross(&e, p(ns, 200 + i as u64), p(ns, 300 + i as u64)));
    let candidate_impulse = six(|i| imp(&e, p(ns, 200 + i as u64), p(ns, 300 + i as u64)));
    let effect = six(|i| fire(&e, p(ns, 300 + i as u64)));
    let returns = six(|i| ret_count(&e, ns, i));
    let expected = expected(sc.kind);
    let passed = source == expected.0
        && traces == expected.0
        && o == expected.1
        && pp == expected.1
        && candidate == expected.1
        && candidate_impulse == expected.2
        && effect == expected.3
        && returns == expected.4
        && candidate_r == expected.5
        && connector_r == expected.6
        && e.work.local_structural_proposals == 0
        && e.naturally_quiescent;
    Row {
        seed,
        scenario: sc.name,
        source,
        traces,
        o,
        p: pp,
        candidate,
        candidate_impulse,
        effect,
        returns,
        candidate_r,
        connector_r,
        after_pressure,
        proposals: e.work.local_structural_proposals,
        work: e.work.total(),
        bytes: w.s.persistent_bytes(),
        fingerprint: e.end_fingerprint,
        permanent: e.permanent_fingerprint,
        quiescent: e.naturally_quiescent,
        replay: false,
        passed,
    }
}

#[allow(clippy::type_complexity)]
fn expected(
    k: Kind,
) -> (
    [usize; 4],
    [usize; 6],
    [i32; 6],
    [usize; 6],
    [usize; 6],
    [u32; 6],
    [u32; 6],
) {
    let z4 = [0; 4];
    let z6 = [0; 6];
    let z6i = [0_i32; 6];
    let i1 = [1; 6];
    let c100 = [100; 6];
    match k {
        Kind::ReturnOnly => (z4, z6, z6i, z6, [1; 6], i1, c100),
        Kind::Real | Kind::Real21 | Kind::Real44 => (
            [1, 1, 0, 0],
            [1, 0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0, 0],
            [4, 1, 1, 1, 1, 1],
            c100,
        ),
        Kind::BlockedLate => (
            [2, 1, 0, 0],
            [1, 0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0, 0],
            z6,
            i1,
            [103, 100, 100, 100, 100, 100],
        ),
        Kind::LateConsequence => (
            [1, 1, 0, 0],
            [1, 0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0, 0],
            [1; 6],
            [1; 6],
            [0, 1, 1, 1, 1, 1],
            c100,
        ),
        Kind::NoAbConsequence => (z4, z6, z6i, [1; 6], [1; 6], i1, c100),
        Kind::Swap => (
            [1, 1, 0, 0],
            [1, 0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0, 0],
            [0, 1, 0, 0, 0, 0],
            i1,
            c100,
        ),
        Kind::TwoLoops => (
            [1, 1, 1, 1],
            [1, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 1],
            [4, 1, 1, 1, 1, 4],
            c100,
        ),
    }
}

fn build(ns: u64, raw: [i32; 4], kind: Kind) -> World {
    let mut s = PlasticSubstrate::new();
    let sources = four(|i| {
        s.add_cell(cell(
            p(ns, 10 + i as u64),
            -20000 - i as i32 * 100,
            10 + i as i16,
            1,
        ))
    });
    let outlets = four(|i| {
        s.add_cell(cell(
            p(ns, 20 + i as u64),
            -15000 - i as i32 * 100,
            20 + i as i16,
            1,
        ))
    });
    let traces = four(|i| {
        s.add_cell(cell(
            p(ns, 30 + i as u64),
            -10000 - i as i32 * 100,
            30 + i as i16,
            2,
        ))
    });
    let hubs = four(|i| {
        s.add_cell(cell(
            p(ns, 40 + i as u64),
            -5000 - i as i32 * 100,
            40 + i as i16,
            1,
        ))
    });
    let os = six(|i| {
        s.add_cell(cell(
            p(ns, 100 + i as u64),
            10000 + i as i32 * 100,
            50 + i as i16,
            2,
        ))
    });
    let ps = six(|i| {
        s.add_cell(cell(
            p(ns, 200 + i as u64),
            15000 + i as i32 * 100,
            60 + i as i16,
            1,
        ))
    });
    let effects = six(|i| {
        s.add_cell(cell(
            p(ns, 300 + i as u64),
            20000 + i as i32 * 100,
            70 + i as i16,
            2,
        ))
    });
    let relays = six(|i| {
        s.add_cell(cell(
            p(ns, 400 + i as u64),
            25000 + i as i32 * 100,
            80 + i as i16,
            1,
        ))
    });
    let context = s.add_cell(cell(p(ns, 500), 30000, 90, 1));
    let driver = s.add_cell(cell(p(ns, 501), 35000, 91, 1));
    let broadcast = s.add_cell(cell(p(ns, 502), 40000, 92, 1));
    for i in 0..4 {
        s.add_arrow(fixed(sources[i], outlets[i], 0, raw[i]));
        s.add_arrow(fixed(outlets[i], traces[i], 1, 1));
        s.add_arrow(fixed(outlets[i], hubs[i], 1, 1));
        s.add_arrow(fixed(hubs[i], traces[i], 0, 1));
    }
    let mut connectors = [None; 6];
    let mut candidates = [None; 6];
    for i in 0..6 {
        let (a, b) = PAIRS[i];
        s.add_arrow(fixed(traces[a], os[i], 0, 1));
        s.add_arrow(fixed(traces[b], os[i], 0, 1));
        connectors[i] = Some(s.add_arrow(fixed(os[i], ps[i], 0, 1)));
        candidates[i] = Some(s.add_arrow(weak(ps[i], effects[i])));
        s.add_arrow(fixed(context, effects[i], 2, 1));
        s.add_arrow(fixed(driver, effects[i], 0, 2));
        if !(kind == Kind::BlockedLate && i == 0) {
            s.add_arrow(fixed(effects[i], relays[i], 0, 1));
        }
        let target = if kind == Kind::Swap && i == 0 { 1 } else { i };
        s.add_arrow(fixed(relays[i], ps[target], 1, 1));
        s.add_arrow(fixed(broadcast, ps[i], 1, 1));
    }
    World {
        s,
        sources,
        context,
        driver,
        broadcast,
        candidates: candidates.map(|x| x.expect("candidate")),
        connectors: connectors.map(|x| x.expect("connector")),
    }
}
fn sources(w: &mut World, sides: [usize; 2], tick: i64) {
    for side in sides {
        source_pulse(w, side, tick, side as i32)
    }
}
fn source_pulse(w: &mut World, side: usize, tick: i64, phase: i32) {
    let t = w.sources[side];
    pulse(w, t, tick, 1, phase)
}
fn named_pulse(w: &mut World, which: u8, tick: i64, impulse: i32, phase: i32) {
    let t = match which {
        0 => w.broadcast,
        1 => w.context,
        2 => w.driver,
        _ => unreachable!(),
    };
    pulse(w, t, tick, impulse, phase)
}
fn pulse(w: &mut World, target: CellId, tick: i64, impulse: i32, phase: i32) {
    w.s.enter(SpikeInput {
        arrival_tick: tick,
        phase,
        origin_physical: 900000 + phase as u64,
        target,
        impulse,
    })
}
fn res(w: &World, ids: [ArrowId; 6]) -> [u32; 6] {
    six(|i| w.s.arrow_resistance(ids[i]))
}
fn ret_count(e: &Execution, ns: u64, target: usize) -> usize {
    (0..6)
        .map(|r| cross(e, p(ns, 400 + r as u64), p(ns, 200 + target as u64)))
        .sum()
}
fn cell(physical_id: u64, position: i32, region: i16, threshold: i32) -> CellSpec {
    CellSpec {
        physical_id,
        position,
        region,
        threshold,
        resistance: 100,
    }
}
fn fixed(from: CellId, to: CellId, delay: i64, coupling: i32) -> ArrowSpec {
    ArrowSpec {
        from,
        to,
        delay,
        phase: 0,
        coupling,
        resistance: 100,
    }
}
fn weak(from: CellId, to: CellId) -> ArrowSpec {
    ArrowSpec {
        from,
        to,
        delay: 2,
        phase: 0,
        coupling: 1,
        resistance: 1,
    }
}
fn p(ns: u64, x: u64) -> u64 {
    ns + x
}
fn idx(s: Scenario) -> usize {
    Scenario::ALL
        .iter()
        .position(|x| *x == s)
        .expect("scenario")
}
fn four<T>(mut f: impl FnMut(usize) -> T) -> [T; 4] {
    [f(0), f(1), f(2), f(3)]
}
fn six<T>(mut f: impl FnMut(usize) -> T) -> [T; 6] {
    [f(0), f(1), f(2), f(3), f(4), f(5)]
}
fn fire(e: &Execution, p: u64) -> usize {
    e.trace
        .iter()
        .filter(|x| x.target_physical == p && x.fired)
        .count()
}
fn cross(e: &Execution, a: u64, b: u64) -> usize {
    e.crossings
        .iter()
        .filter(|x| x.from_physical == a && x.to_physical == b)
        .count()
}
fn imp(e: &Execution, a: u64, b: u64) -> i32 {
    e.crossings
        .iter()
        .filter(|x| x.from_physical == a && x.to_physical == b)
        .map(|x| x.impulse)
        .sum()
}
fn csv(rows: &[Row]) -> String {
    let mut t=String::from("seed,scenario,source,traces,o,p,candidate,candidate_impulse,effect,returns,candidate_r,connector_r,after_pressure,proposals,work,bytes,fingerprint,permanent,quiescent,replay,passed\n");
    for r in rows {
        let f = vec![
            r.seed.to_string(),
            r.scenario.into(),
            ju(&r.source),
            ju(&r.traces),
            ju(&r.o),
            ju(&r.p),
            ju(&r.candidate),
            ji(&r.candidate_impulse),
            ju(&r.effect),
            ju(&r.returns),
            jr(&r.candidate_r),
            jr(&r.connector_r),
            jr(&r.after_pressure),
            r.proposals.to_string(),
            r.work.to_string(),
            r.bytes.to_string(),
            r.fingerprint.to_string(),
            r.permanent.to_string(),
            r.quiescent.to_string(),
            r.replay.to_string(),
            r.passed.to_string(),
        ];
        t.push_str(&f.join(","));
        t.push('\n');
    }
    t
}
fn report(rows: &[Row]) -> String {
    let pass = rows.iter().filter(|r| r.passed).count();
    let late = rows
        .iter()
        .filter(|r| r.scenario == "ab-blocked-late-a")
        .map(|r| r.candidate_r[0].to_string())
        .collect::<Vec<_>>()
        .join("|");
    let conn = rows
        .iter()
        .filter(|r| r.scenario == "ab-blocked-late-a")
        .map(|r| r.connector_r[0].to_string())
        .collect::<Vec<_>>()
        .join("|");
    format!("# PX3-D1-R2 closed-loop return attribution v1\n\nOutcome: **{}**.\n\n- rows: `{pass}/{}` passed;\n- exact replay: `{}`;\n- quiescent: `{}`;\n- blocked-return + late-A candidate resistance: `{late}`;\n- blocked-return + late-A connector resistance: `{conn}`;\n- structural proposals: `{}`;\n- D2/full PX3 executed: `false`.\n",if pass==rows.len(){"R2-A POSITIVE"}else{"NEGATIVE"},rows.len(),rows.iter().all(|r|r.replay),rows.iter().all(|r|r.quiescent),rows.iter().map(|r|r.proposals).sum::<u64>())
}
fn ju(v: &[usize]) -> String {
    v.iter().map(usize::to_string).collect::<Vec<_>>().join("|")
}
fn ji(v: &[i32]) -> String {
    v.iter().map(i32::to_string).collect::<Vec<_>>().join("|")
}
fn jr(v: &[u32]) -> String {
    v.iter().map(u32::to_string).collect::<Vec<_>>().join("|")
}
fn absent(ps: &[&str]) {
    for p in ps {
        assert!(!Path::new(p).exists())
    }
}
fn publish(stage: &str, dest: &str, c: &str) {
    let mut f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(stage)
        .expect("stage");
    f.write_all(c.as_bytes()).expect("write");
    f.sync_all().expect("sync");
    rename(stage, dest).expect("rename")
}
fn sha(p: &str) -> String {
    let o = Command::new("sha256sum").arg(p).output().expect("sha");
    assert!(o.status.success());
    String::from_utf8(o.stdout)
        .expect("utf8")
        .split_whitespace()
        .next()
        .expect("digest")
        .into()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matrix() {
        surface();
        assert_eq!(SEEDS.len() * Scenario::ALL.len(), 18)
    }
    #[test]
    fn pairs() {
        assert_eq!(PAIRS.into_iter().collect::<BTreeSet<_>>().len(), 6)
    }
}
