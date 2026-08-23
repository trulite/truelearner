use std::env;
use std::fs;
use std::path::PathBuf;

const AUTHORITATIVE_SOURCE: &str = "../../crates/px0-physical-correspondence/src/lib.rs";

fn replace_once(source: &mut String, before: &str, after: &str, label: &str) {
    let occurrences = source.matches(before).count();
    assert_eq!(occurrences, 1, "{label}: expected one insertion point");
    *source = source.replacen(before, after, 1);
}

fn main() {
    println!("cargo:rerun-if-changed={AUTHORITATIVE_SOURCE}");
    println!("cargo:rerun-if-changed=build.rs");

    let mut source = fs::read_to_string(AUTHORITATIVE_SOURCE)
        .expect("read exact authoritative PX0-PX2 substrate source");
    let header = "#![forbid(unsafe_code)]\n//! Experimental substrate-native CELL/ARROW/SPIKE physics for PX0.\n//!\n//! Active state contains only cells, arrows, spikes, and local physical\n//! timing. The module contains no evaluator types and has no dependency on the\n//! historical mechanism suite.\n";
    assert!(source.starts_with(header), "authoritative header changed");
    source.replace_range(
        ..header.len(),
        "// Exact authoritative PX0-PX2 substrate body plus the isolated CJ0-D delta.\n",
    );

    replace_once(
        &mut source,
        "    pub physical_deallocations: u64,\n}",
        "    pub physical_deallocations: u64,\n    pub local_subunit_integrations: u64,\n    pub local_subunit_spikes_emitted: u64,\n}",
        "work-ledger fields",
    );
    replace_once(
        &mut source,
        "            + self.physical_deallocations\n",
        "            + self.physical_deallocations\n            + self.local_subunit_integrations\n            + self.local_subunit_spikes_emitted\n",
        "work-ledger total",
    );
    replace_once(
        &mut source,
        "    pub fired: bool,\n}",
        "    pub fired: bool,\n    pub local_subunit_integration: bool,\n}",
        "trace local event",
    );
    replace_once(
        &mut source,
        "            self.apply_local_return(spike.target, self.tick, &mut work);\n            self.decay_cell(spike.target, self.tick);\n",
        r#"            let local_subunit_integration = self.local_subunit_condition(&spike);
            self.apply_local_return(spike.target, self.tick, &mut work);
            self.decay_cell(spike.target, self.tick);
"#,
        "local condition",
    );
    replace_once(
        &mut source,
        "                impulse: spike.impulse,\n                fired: fires,\n",
        "                impulse: spike.impulse,\n                fired: fires,\n                local_subunit_integration: local_subunit_integration && !fires,\n",
        "trace event value",
    );
    replace_once(
        &mut source,
        "            if !fires {\n                continue;\n            }\n\n            target.state = 0;",
        r#"            if !fires {
                if local_subunit_integration {
                    self.emit_local_subunit(spike.target, &mut work);
                }
                continue;
            }

            target.state = 0;"#,
        "subthreshold local emission",
    );
    replace_once(
        &mut source,
        "    pub fn arrow_resistance(&self, arrow: ArrowId) -> u32 {",
        r#"    pub fn current_tick(&self) -> i64 {
        self.tick
    }

    pub fn arrow_coupling(&self, arrow: ArrowId) -> i32 {
        self.require_arrow(arrow);
        self.arrows[arrow.0].coupling
    }

    pub fn cell_state(&self, cell: CellId) -> i32 {
        self.require_cell(cell);
        self.cells[cell.0].state
    }

    pub fn arrow_resistance(&self, arrow: ArrowId) -> u32 {"#,
        "read-only physical observations",
    );
    replace_once(
        &mut source,
        "    fn apply_local_return(&mut self, cell: CellId, tick: i64, work: &mut WorkLedger) {",
        r#"    // CJ0_D_LOCAL_SUBUNIT_LAW_BEGIN
    fn local_subunit_condition(&self, spike: &Spike) -> bool {
        let Some((current_id, current_generation)) = spike.arrow else {
            return false;
        };
        if spike.impulse <= 0 {
            return false;
        }
        let target = &self.cells[spike.target.0];
        if target.state <= 0
            || target.state >= target.threshold
            || target.last_update_tick != self.tick
        {
            return false;
        }
        let current = &self.arrows[current_id.0];
        if !current.live
            || current.generation != current_generation
            || current.to != spike.target
            || current.coupling <= 0
        {
            return false;
        }
        self.arrows.iter().enumerate().any(|(index, other)| {
            index != current_id.0
                && other.live
                && other.to == spike.target
                && other.from != current.from
                && other.coupling > 0
                && other.eligible_until.is_some()
                && other.eligible_until == current.eligible_until
        })
    }

    fn emit_local_subunit(&mut self, source: CellId, work: &mut WorkLedger) {
        work.local_subunit_integrations += 1;
        let source_cell = &self.cells[source.0];
        let source_position = source_cell.position;
        let source_region = source_cell.region;
        let source_generation = source_cell.generation;
        let origin_physical = source_cell.physical_id;
        let outgoing = self
            .arrows
            .iter()
            .enumerate()
            .map(|(index, arrow)| (ArrowId(index), arrow.clone()))
            .collect::<Vec<_>>();
        for (arrow_id, arrow) in outgoing {
            work.arrow_checks += 1;
            if !arrow.live
                || arrow.from != source
                || arrow.source_generation != source_generation
                || arrow.coupling <= 0
            {
                continue;
            }
            let target = &self.cells[arrow.to.0];
            let distance = target.position.saturating_sub(source_position).abs();
            if target.region != source_region
                || !(1..=LOCAL_VARIATION_RADIUS).contains(&distance)
            {
                continue;
            }
            self.arrows[arrow_id.0].eligible_until =
                Some(self.tick.saturating_add(LOCAL_WINDOW));
            work.local_eligibility_writes += 1;
            self.pending.push(Spike {
                arrival_tick: self.tick.saturating_add(arrow.delay),
                phase: arrow.phase,
                origin_physical,
                target: arrow.to,
                target_generation: target.generation,
                impulse: arrow.coupling,
                serial: self.next_serial,
                arrow: Some((arrow_id, arrow.generation)),
            });
            self.next_serial = self.next_serial.wrapping_add(1);
            work.spikes_emitted += 1;
            work.local_subunit_spikes_emitted += 1;
        }
    }
    // CJ0_D_LOCAL_SUBUNIT_LAW_END

    fn apply_local_return(&mut self, cell: CellId, tick: i64, work: &mut WorkLedger) {"#,
        "candidate law block",
    );

    let output =
        PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR")).join("physical_substrate.rs");
    fs::write(output, source).expect("write isolated generated substrate");
}
