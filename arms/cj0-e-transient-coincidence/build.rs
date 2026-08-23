use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const FROZEN_SHA256: &str = "3ee8b2bfc9c9ac2d4b9726d60d93759c66eaeec6cd2e61db7041bde753aad12d";

fn main() {
    let manifest = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("manifest directory"));
    let frozen = manifest.join("../../crates/px0-physical-correspondence/src/lib.rs");
    println!("cargo:rerun-if-changed={}", frozen.display());
    let source = fs::read_to_string(&frozen).expect("read frozen PX0--PX2 substrate law");
    assert_eq!(sha256_file(&frozen), FROZEN_SHA256, "frozen law hash drift");

    let old_update = r#"            let target = &mut self.cells[spike.target.0];
            target.state = target.state.saturating_add(spike.impulse);
            work.state_updates += 1;
            work.threshold_checks += 1;
            let fires = self.tick >= target.refractory_until && target.state >= target.threshold;"#;
    let new_update = r#"            let target = &mut self.cells[spike.target.0];
            let live_before = target.state > 0;
            target.state = target.state.saturating_add(spike.impulse);
            work.state_updates += 1;
            work.threshold_checks += 1;
            let transient_completed = target.threshold > 1 && live_before;
            let fires = self.tick >= target.refractory_until
                && target.state >= target.threshold
                && (target.threshold == 1 || transient_completed);"#;
    assert_eq!(source.matches(old_update).count(), 1, "firing block drift");
    let source = source.replace(old_update, new_update);

    let old_proposal = "            if external_arrival {\n                self.propose_local_arrows(source, &mut work);\n            }";
    let new_proposal = "            if external_arrival || transient_completed {\n                self.propose_local_arrows(source, &mut work);\n            }";
    assert_eq!(
        source.matches(old_proposal).count(),
        1,
        "proposal block drift"
    );
    let source = source.replace(old_proposal, new_proposal);

    let accessor_anchor =
        "    fn apply_local_return(&mut self, cell: CellId, tick: i64, work: &mut WorkLedger) {";
    let accessors = r#"    pub fn arrow_coupling(&self, arrow: ArrowId) -> i32 {
        self.require_arrow(arrow);
        self.arrows[arrow.0].coupling
    }

    pub fn current_tick(&self) -> i64 {
        self.tick
    }

"#;
    assert_eq!(
        source.matches(accessor_anchor).count(),
        1,
        "accessor anchor drift"
    );
    let source = source.replace(accessor_anchor, &format!("{accessors}{accessor_anchor}"));
    let source = source.replacen("#![forbid(unsafe_code)]\n", "", 1);
    let inherited_docs = "//! Experimental substrate-native CELL/ARROW/SPIKE physics for PX0.\n//!\n//! Active state contains only cells, arrows, spikes, and local physical\n//! timing. The module contains no evaluator types and has no dependency on the\n//! historical mechanism suite.\n\n";
    assert_eq!(source.matches(inherited_docs).count(), 1, "header drift");
    let source = source.replacen(inherited_docs, "", 1);

    let out = PathBuf::from(env::var_os("OUT_DIR").expect("build output"));
    fs::write(out.join("substrate.rs"), source).expect("write isolated candidate law");
}

fn sha256_file(path: &Path) -> String {
    use std::process::Command;
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
