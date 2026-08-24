use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const FROZEN_PX0_SHA256: &str = "3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d";

fn main() {
    let manifest = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("manifest directory"));
    let frozen = manifest.join("../../crates/px0-physical-correspondence/src/lib.rs");
    println!("cargo:rerun-if-changed={}", frozen.display());

    let source = fs::read_to_string(&frozen).expect("read frozen PX0 law");
    assert_eq!(
        sha256_file(&frozen),
        FROZEN_PX0_SHA256,
        "frozen PX0 law hash drift"
    );

    let old = r#"                let live_arrow = &mut self.arrows[arrow_id.0];
                live_arrow.eligible_until = Some(self.tick.saturating_add(LOCAL_WINDOW));
                work.local_eligibility_writes += 1;
                self.pending.push(Spike {
                    arrival_tick: self.tick.saturating_add(arrow.delay),
                    phase: arrow.phase,
                    origin_physical,
                    target: arrow.to,
                    target_generation: to.generation,
                    impulse: arrow.coupling,
                    serial: self.next_serial,
                    arrow: Some((arrow_id, arrow.generation)),
                });"#;
    let new = r#"                let live_arrow = &mut self.arrows[arrow_id.0];
                let path_contribution_live = live_arrow
                    .eligible_until
                    .is_some_and(|end| self.tick <= end);
                let local_contribution = if to.threshold == 1 {
                    arrow.coupling
                } else if path_contribution_live {
                    0
                } else {
                    1
                };
                live_arrow.eligible_until = Some(self.tick.saturating_add(LOCAL_WINDOW));
                work.local_eligibility_writes += 1;
                self.pending.push(Spike {
                    arrival_tick: self.tick.saturating_add(arrow.delay),
                    phase: arrow.phase,
                    origin_physical,
                    target: arrow.to,
                    target_generation: to.generation,
                    impulse: local_contribution,
                    serial: self.next_serial,
                    arrow: Some((arrow_id, arrow.generation)),
                });"#;
    assert_eq!(
        source.matches(old).count(),
        1,
        "candidate traversal block drift"
    );
    let source = source.replace(old, new);

    let source = source.replacen("#![forbid(unsafe_code)]\n", "", 1);
    let inherited_docs = "//! Experimental substrate-native CELL/ARROW/SPIKE physics for PX0.\n//!\n//! Active state contains only cells, arrows, spikes, and local physical\n//! timing. The module contains no evaluator types and has no dependency on the\n//! historical mechanism suite.\n\n";
    assert_eq!(
        source.matches(inherited_docs).count(),
        1,
        "PX0 header drift"
    );
    let source = source.replacen(inherited_docs, "", 1);

    let out = PathBuf::from(env::var_os("OUT_DIR").expect("build output"));
    fs::write(out.join("candidate_substrate.rs"), source).expect("write isolated candidate law");
}

fn sha256_file(path: &Path) -> String {
    let output = Command::new("sha256sum")
        .arg(path)
        .output()
        .expect("run sha256sum");
    assert!(output.status.success(), "sha256sum failed");
    String::from_utf8(output.stdout)
        .expect("sha256 output")
        .split_whitespace()
        .next()
        .expect("sha256 digest")
        .to_string()
}
