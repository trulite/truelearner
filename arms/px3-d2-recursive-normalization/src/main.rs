#![forbid(unsafe_code)]

use px0_physical_correspondence::{
    ArrowSpec, CellId, CellSpec, Execution, PlasticSubstrate, SpikeInput, WorkLedger,
};
use std::collections::BTreeSet;
use std::env;
use std::fs::{rename, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;

const PX0_SHA: &str = "3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d";
const D1_CSV_SHA: &str = "511efc76fc36c9c3c77815f1bd53bd4f8ea38411f8892bbc26005af1e7d0fecb";
const D1_AUDIT_SHA: &str = "06135ca0e63cb9dd944f172e6c072db2ae77b6c1eb1653aabba254f9247d13de";
const PROTOCOL_SHA: &str = "ce03173ca075d7abcef85da105483fa156a8de3e6afa76c97364f3f2db59d5ab";
const EXEC_SHA: &str = "dd31c40a33b2b7c7ee71d1db911bbbdc3f9f4ac3aebca9cd913b19a5ff053edd";
const PROTOCOL_V2_SHA: &str = "8f8c092668e355589fdf81e60e644719e81adabcd1db6853fa5eb585b5b0abc5";
const SEEDS: [u64; 2] = [3101, 3109];
const CSV: &str = "results/px3_d2_recursive_normalization_v1.csv";
const MD: &str = "results/px3_d2_recursive_normalization_v1.md";
const CSV_STAGE: &str = "results/.px3_d2_recursive_normalization_v1.csv.staging";
const MD_STAGE: &str = "results/.px3_d2_recursive_normalization_v1.md.staging";

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    A,
    B,
    A4,
    RepeatA,
    Ab1,
    Ab2,
    Ab4,
    Xc,
    XGapC,
    Dc,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Scenario {
    name: &'static str,
    kind: Kind,
    mature: i32,
}

impl Scenario {
    const ALL: [Self; 10] = [
        Self::new("a-alone", Kind::A, 2),
        Self::new("b-alone", Kind::B, 2),
        Self::new("a4-alone", Kind::A4, 2),
        Self::new("a-repeated", Kind::RepeatA, 2),
        Self::new("ab-mature-1", Kind::Ab1, 1),
        Self::new("ab-mature-2", Kind::Ab2, 2),
        Self::new("ab-mature-4", Kind::Ab4, 4),
        Self::new("x-plus-c-overlap", Kind::Xc, 2),
        Self::new("x-then-c-gapped", Kind::XGapC, 2),
        Self::new("d-plus-c-primitive-baseline", Kind::Dc, 2),
    ];
    const fn new(name: &'static str, kind: Kind, mature: i32) -> Self {
        Self { name, kind, mature }
    }
}

#[derive(Clone)]
struct World {
    substrate: PlasticSubstrate,
    sources: [CellId; 4],
}

#[derive(Clone, PartialEq, Eq)]
struct Row {
    seed: u64,
    scenario: &'static str,
    mirror: bool,
    mature: i32,
    scheduled: [usize; 4],
    source: [usize; 4],
    raw_impulse: [i32; 4],
    primitive_trace: [usize; 4],
    ab: usize,
    mature_crossings: usize,
    mature_impulse: i32,
    x_outlet: usize,
    x_local: usize,
    x_hub_input: usize,
    x_hub_return: usize,
    x_trace_arrivals: usize,
    x_trace_impulse: i32,
    x_trace: usize,
    xc_x: usize,
    xc_c: usize,
    xc: usize,
    dc_d: usize,
    dc_c: usize,
    dc: usize,
    work: u64,
    bytes: usize,
    fingerprint: u64,
    permanent: u64,
    quiescent: bool,
    replay: bool,
    passed: bool,
}

fn main() {
    audit_sources();
    audit_surface();
    absent(&[CSV, MD, CSV_STAGE, MD_STAGE]);
    match env::args().skip(1).collect::<Vec<_>>().as_slice() {
        [x] if x == "--preflight" => println!("PX3_D2_RECURSIVE_NORMALIZATION_PREFLIGHT_OK"),
        [x] if x == "--d2" => evidence(),
        _ => std::process::exit(2),
    }
}

fn evidence() {
    eprintln!("PX3_D2_RECURSIVE_NORMALIZATION_EVIDENCE");
    let mut rows = Vec::new();
    for seed in SEEDS {
        for scenario in Scenario::ALL {
            rows.push(replay(seed, scenario));
        }
    }
    assert_eq!(rows.len(), 20);
    publish(CSV_STAGE, CSV, &csv(&rows));
    publish(MD_STAGE, MD, &report(&rows));
}

fn audit_sources() {
    for (p, h) in [
        ("crates/px0-physical-correspondence/src/lib.rs", PX0_SHA),
        (
            "results/px3_d1_participation_gated_pair_learning_v1.csv",
            D1_CSV_SHA,
        ),
        (
            "experiments/px3_d1_participation_gated_pair_learning_result_audit_v1.md",
            D1_AUDIT_SHA,
        ),
        (
            "experiments/px3_d2_recursive_normalization_protocol_v1.md",
            PROTOCOL_SHA,
        ),
        (
            "experiments/px3_d2_recursive_normalization_protocol_v2.md",
            PROTOCOL_V2_SHA,
        ),
        (
            "experiments/px3_d2_recursive_normalization_execution_protocol_v1.md",
            EXEC_SHA,
        ),
    ] {
        assert_eq!(sha(p), h, "hash drift {p}");
    }
}

fn audit_surface() {
    assert_eq!(Scenario::ALL.len(), 10);
    assert_eq!(
        Scenario::ALL
            .iter()
            .map(|x| x.name)
            .collect::<BTreeSet<_>>()
            .len(),
        10
    );
    for p in [
        "arms/px3-d2-recursive-normalization/src/y.rs",
        "arms/px3-d2-recursive-normalization/src/z.rs",
        "results/px3_gate_v1.csv",
    ] {
        assert!(!Path::new(p).exists());
    }
}

fn replay(seed: u64, s: Scenario) -> Row {
    let a = run(seed, s);
    let b = run(seed, s);
    let eq = a == b;
    let mut r = a;
    r.replay = eq;
    r.passed &= eq;
    r
}

fn run(seed: u64, s: Scenario) -> Row {
    let mirror = seed == 3109;
    let ns = (seed << 32) | ((index(s) as u64 + 1) << 16);
    let raw = [if s.kind == Kind::A4 { 4 } else { 1 }, 1, 1, 1];
    let mut w = build(ns, raw, s.mature, mirror);
    let mut scheduled = [0; 4];
    match s.kind {
        Kind::A | Kind::A4 => {
            enter(&mut w, 0, 0, 0);
            scheduled[0] = 1;
        }
        Kind::B => {
            enter(&mut w, 1, 0, 1);
            scheduled[1] = 1;
        }
        Kind::RepeatA => {
            enter(&mut w, 0, 0, 0);
            enter(&mut w, 0, 0, 9);
            scheduled[0] = 2;
        }
        Kind::Ab1 | Kind::Ab2 | Kind::Ab4 => {
            enter(&mut w, 0, 0, 0);
            enter(&mut w, 1, 0, 1);
            scheduled = [1, 1, 0, 0];
        }
        Kind::Xc => {
            enter(&mut w, 0, 0, 0);
            enter(&mut w, 1, 0, 1);
            enter(&mut w, 2, 1, 2);
            scheduled = [1, 1, 1, 0];
        }
        Kind::XGapC => {
            enter(&mut w, 0, 0, 0);
            enter(&mut w, 1, 0, 1);
            enter(&mut w, 2, 4, 2);
            scheduled = [1, 1, 1, 0];
        }
        Kind::Dc => {
            enter(&mut w, 2, 1, 2);
            enter(&mut w, 3, 1, 3);
            scheduled = [0, 0, 1, 1];
        }
    }
    let e = w.substrate.propagate();
    let source = four(|i| fire(&e, p(ns, 10 + i as u64)));
    let raw_impulse = four(|i| imp(&e, p(ns, 10 + i as u64), p(ns, 20 + i as u64)));
    let primitive_trace = four(|i| fire(&e, p(ns, 30 + i as u64)));
    let ab = fire(&e, p(ns, 100));
    let mature_crossings = cross(&e, p(ns, 100), p(ns, 110));
    let mature_impulse = imp(&e, p(ns, 100), p(ns, 110));
    let x_outlet = fire(&e, p(ns, 110));
    let x_local = cross(&e, p(ns, 110), p(ns, 120));
    let x_hub_input = cross(&e, p(ns, 110), p(ns, 121));
    let x_hub_return = cross(&e, p(ns, 121), p(ns, 120));
    let x_trace_arrivals = arr(&e, p(ns, 120));
    let x_trace_impulse = sum_imp(&e, p(ns, 120));
    let x_trace = fire(&e, p(ns, 120));
    let xc_x = cross(&e, p(ns, 120), p(ns, 130));
    let xc_c = cross(&e, p(ns, 32), p(ns, 130));
    let xc = fire(&e, p(ns, 130));
    let dc_d = cross(&e, p(ns, 33), p(ns, 131));
    let dc_c = cross(&e, p(ns, 32), p(ns, 131));
    let dc = fire(&e, p(ns, 131));
    let expect_x = matches!(
        s.kind,
        Kind::Ab1 | Kind::Ab2 | Kind::Ab4 | Kind::Xc | Kind::XGapC
    );
    let expect_xc = usize::from(s.kind == Kind::Xc);
    let expect_dc = usize::from(s.kind == Kind::Dc);
    let active = match s.kind {
        Kind::A | Kind::A4 | Kind::RepeatA => [1, 0, 0, 0],
        Kind::B => [0, 1, 0, 0],
        Kind::Ab1 | Kind::Ab2 | Kind::Ab4 => [1, 1, 0, 0],
        Kind::Xc | Kind::XGapC => [1, 1, 1, 0],
        Kind::Dc => [0, 0, 1, 1],
    };
    let expected_raw = four(|i| i32::try_from(active[i]).expect("small") * raw[i]);
    let passed = source == active
        && raw_impulse == expected_raw
        && primitive_trace == active
        && ab == usize::from(expect_x)
        && mature_crossings == usize::from(expect_x)
        && mature_impulse == if expect_x { s.mature } else { 0 }
        && x_outlet == usize::from(expect_x)
        && x_local == usize::from(expect_x)
        && x_hub_input == usize::from(expect_x)
        && x_hub_return == usize::from(expect_x)
        && x_trace_arrivals == 2 * usize::from(expect_x)
        && x_trace_impulse == 2 * i32::from(expect_x)
        && x_trace == usize::from(expect_x)
        && xc_x == usize::from(expect_x)
        && xc_c == usize::from(matches!(s.kind, Kind::Xc | Kind::XGapC))
        && xc == expect_xc
        && dc_d == expect_dc
        && dc_c == expect_dc
        && dc == expect_dc
        && e.naturally_quiescent;
    Row {
        seed,
        scenario: s.name,
        mirror,
        mature: s.mature,
        scheduled,
        source,
        raw_impulse,
        primitive_trace,
        ab,
        mature_crossings,
        mature_impulse,
        x_outlet,
        x_local,
        x_hub_input,
        x_hub_return,
        x_trace_arrivals,
        x_trace_impulse,
        x_trace,
        xc_x,
        xc_c,
        xc,
        dc_d,
        dc_c,
        dc,
        work: e.work.total(),
        bytes: w.substrate.persistent_bytes(),
        fingerprint: e.end_fingerprint,
        permanent: e.permanent_fingerprint,
        quiescent: e.naturally_quiescent,
        replay: false,
        passed,
    }
}

fn build(ns: u64, raw: [i32; 4], mature: i32, mirror: bool) -> World {
    let mut s = PlasticSubstrate::new();
    let order = if mirror { [3, 2, 1, 0] } else { [0, 1, 2, 3] };
    let mut src = [None; 4];
    let mut out = [None; 4];
    let mut tr = [None; 4];
    for i in order {
        src[i] = Some(s.add_cell(cell(
            p(ns, 10 + i as u64),
            -20000 - i as i32 * 100,
            10 + i as i16,
            1,
        )));
    }
    for i in order {
        out[i] = Some(s.add_cell(cell(
            p(ns, 20 + i as u64),
            -10000 - i as i32 * 100,
            20 + i as i16,
            1,
        )));
    }
    for i in order {
        tr[i] = Some(s.add_cell(cell(
            p(ns, 30 + i as u64),
            -5000 - i as i32 * 100,
            30 + i as i16,
            2,
        )));
    }
    let sources = src.map(|x| x.expect("source"));
    let outlets = out.map(|x| x.expect("outlet"));
    let traces = tr.map(|x| x.expect("trace"));
    let mut hubs = [None; 4];
    for i in order {
        hubs[i] = Some(s.add_cell(cell(
            p(ns, 40 + i as u64),
            -2500 - i as i32 * 100,
            40 + i as i16,
            1,
        )));
    }
    let hubs = hubs.map(|x| x.expect("hub"));
    let ab = s.add_cell(cell(p(ns, 100), 10000, 50, 2));
    let xout = s.add_cell(cell(p(ns, 110), 12000, 51, 1));
    let xtrace = s.add_cell(cell(p(ns, 120), 14000, 52, 2));
    let xhub = s.add_cell(cell(p(ns, 121), 16000, 53, 1));
    let xc = s.add_cell(cell(p(ns, 130), 18000, 54, 2));
    let dc = s.add_cell(cell(p(ns, 131), 20000, 55, 2));
    for i in 0..4 {
        s.add_arrow(arrow(sources[i], outlets[i], 0, raw[i]));
        s.add_arrow(arrow(outlets[i], traces[i], 1, 1));
        s.add_arrow(arrow(outlets[i], hubs[i], 1, 1));
        s.add_arrow(arrow(hubs[i], traces[i], 0, 1));
    }
    s.add_arrow(arrow(traces[0], ab, 0, 1));
    s.add_arrow(arrow(traces[1], ab, 0, 1));
    s.add_arrow(arrow(ab, xout, 0, mature));
    s.add_arrow(arrow(xout, xtrace, 1, 1));
    s.add_arrow(arrow(xout, xhub, 1, 1));
    s.add_arrow(arrow(xhub, xtrace, 0, 1));
    s.add_arrow(arrow(xtrace, xc, 0, 1));
    s.add_arrow(arrow(traces[2], xc, 0, 1));
    s.add_arrow(arrow(traces[3], dc, 0, 1));
    s.add_arrow(arrow(traces[2], dc, 0, 1));
    World {
        substrate: s,
        sources,
    }
}

fn enter(w: &mut World, side: usize, tick: i64, phase: i32) {
    w.substrate.enter(SpikeInput {
        arrival_tick: tick,
        phase,
        origin_physical: 900000 + side as u64,
        target: w.sources[side],
        impulse: 1,
    });
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
fn arrow(from: CellId, to: CellId, delay: i64, coupling: i32) -> ArrowSpec {
    ArrowSpec {
        from,
        to,
        delay,
        phase: 0,
        coupling,
        resistance: 100,
    }
}
fn p(ns: u64, suffix: u64) -> u64 {
    ns + suffix
}
fn index(s: Scenario) -> usize {
    Scenario::ALL
        .iter()
        .position(|x| *x == s)
        .expect("scenario")
}
fn four<T>(mut f: impl FnMut(usize) -> T) -> [T; 4] {
    [f(0), f(1), f(2), f(3)]
}
fn fire(e: &Execution, p: u64) -> usize {
    e.trace
        .iter()
        .filter(|x| x.target_physical == p && x.fired)
        .count()
}
fn arr(e: &Execution, p: u64) -> usize {
    e.trace.iter().filter(|x| x.target_physical == p).count()
}
fn sum_imp(e: &Execution, p: u64) -> i32 {
    e.trace
        .iter()
        .filter(|x| x.target_physical == p)
        .map(|x| x.impulse)
        .sum()
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
    let mut t=String::from("seed,scenario,mirror,mature,scheduled,source,raw_impulse,primitive_trace,ab,mature_crossings,mature_impulse,x_outlet,x_local,x_hub_input,x_hub_return,x_trace_arrivals,x_trace_impulse,x_trace,xc_x,xc_c,xc,dc_d,dc_c,dc,work,bytes,fingerprint,permanent,quiescent,replay,passed\n");
    for r in rows {
        let f = vec![
            r.seed.to_string(),
            r.scenario.into(),
            r.mirror.to_string(),
            r.mature.to_string(),
            ju(&r.scheduled),
            ju(&r.source),
            ji(&r.raw_impulse),
            ju(&r.primitive_trace),
            r.ab.to_string(),
            r.mature_crossings.to_string(),
            r.mature_impulse.to_string(),
            r.x_outlet.to_string(),
            r.x_local.to_string(),
            r.x_hub_input.to_string(),
            r.x_hub_return.to_string(),
            r.x_trace_arrivals.to_string(),
            r.x_trace_impulse.to_string(),
            r.x_trace.to_string(),
            r.xc_x.to_string(),
            r.xc_c.to_string(),
            r.xc.to_string(),
            r.dc_d.to_string(),
            r.dc_c.to_string(),
            r.dc.to_string(),
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
    format!("# PX3-D2 recursive normalization v1\n\nOutcome: **{}**.\n\n- rows: `{pass}/{}` passed;\n- exact replay: `{}`;\n- quiescent: `{}`;\n- mature amplitudes: `1,2,4`;\n- mature rows with exactly one X trace: `{}`;\n- overlapping X+C firings: `{}`;\n- gapped X/C firings: `{}`;\n- primitive D+C baseline firings: `{}`;\n- learning/full recursion executed: `false`.\n",if pass==rows.len(){"D2-A POSITIVE"}else{"NEGATIVE"},rows.len(),rows.iter().all(|r|r.replay),rows.iter().all(|r|r.quiescent),rows.iter().filter(|r|matches!(r.scenario,"ab-mature-1"|"ab-mature-2"|"ab-mature-4")&&r.x_trace==1).count(),rows.iter().filter(|r|r.scenario=="x-plus-c-overlap").map(|r|r.xc).sum::<usize>(),rows.iter().filter(|r|r.scenario=="x-then-c-gapped").map(|r|r.xc).sum::<usize>(),rows.iter().filter(|r|r.scenario=="d-plus-c-primitive-baseline").map(|r|r.dc).sum::<usize>())
}
fn ju(v: &[usize]) -> String {
    v.iter().map(usize::to_string).collect::<Vec<_>>().join("|")
}
fn ji(v: &[i32]) -> String {
    v.iter().map(i32::to_string).collect::<Vec<_>>().join("|")
}
fn absent(ps: &[&str]) {
    for p in ps {
        assert!(!Path::new(p).exists(), "artifact exists {p}");
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
    rename(stage, dest).expect("rename");
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
        audit_surface();
        assert_eq!(SEEDS.len() * Scenario::ALL.len(), 20);
    }
    #[test]
    fn no_later_surface() {
        assert!(!Path::new("arms/px3-d2-recursive-normalization/src/y.rs").exists());
    }
}
