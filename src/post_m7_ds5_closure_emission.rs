//! Development-only post-M7 DS5 physical closure-emission successor.

pub const PROTOCOL: &str = "post-m7-ds5-closure-emission-v1";
pub const AUTHORITATIVE_M7: &str = "b607ed52f640a3e202da3cc6b73ac58b180caf83";
pub const PROBE_V1_SEED: u64 = 880_000_000;
pub const FROZEN_PROTOCOL_SHA256: &str =
    "140d2392263359c666f364e1923f956a4b2b09e4107a0fd2f7f8f469d97be154";
pub const FROZEN_M7_HANDOFF_SHA256: &str =
    "b4a9012f8fbbb1fa8fdfd36921a82e162c73f4c2175c809bd48c0dae78e45520";
pub const FROZEN_M7_CSV_SHA256: &str =
    "13619c786471b34f5dc9da914c4a0f454bab8d95a87142ce6c9e35808f3dd91a";
pub const FROZEN_M7_MD_SHA256: &str =
    "d1f4d3dc6c944b8ab146a121b0fb0df7d6270b3d4363ca6d4e18b8b53925b1cd";
pub const FROZEN_M7_SOURCE_SHA256: &str =
    "67e170f12d7b7649a0a291ddfc16cd80e4b5c15564b65cd09c884f3e52b9ac5b";
pub const FROZEN_V20_SOURCE_SHA256: &str =
    "8a17e7a5fda9519ad0d4a9d29d04d2434dd5b5ee857e74c1296c5f8b3b06f897";
pub const FROZEN_V21_SOURCE_SHA256: &str =
    "85230e7b6b0d669a3b2e163f3e281975c9fbd5d98709b923efff418d36ff9f1a";
pub const FROZEN_V21B_RESULT_SHA256: &str =
    "ca4f2ffb8b77ac237bfce19d66d21820d26d34b727bbf95262003dffd93ad300";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProbeV1Report {
    pub protocol: &'static str,
    pub seed: u64,
    pub claim_eligible: bool,
    pub expected_negative: bool,
    pub exact_m7: bool,
    pub protocol_frozen: bool,
    pub frozen_parts: bool,
    pub physical_closure_path: bool,
    pub terminal_supervision_sites: usize,
    pub semantic_population_sites: usize,
    pub lawful_m6_links: usize,
    pub lawful_updates: usize,
    pub first_collapse: &'static str,
}

pub fn run_probe_v1() -> ProbeV1Report {
    let v21 = include_str!("continuation.rs");
    let old_finish = v21
        .split("pub fn run_finish_experiment")
        .nth(1)
        .unwrap_or_default();
    let successor_source = include_str!("post_m7_ds5_closure_emission.rs");
    let terminal_supervision_sites = old_finish.matches("learn_finish_from_terminal").count();
    let semantic_population_sites = [
        "NO_RESULT_CELL_ID",
        "EXPLICIT_ANSWER_CELL_ID",
        "SemanticEvent::NoResult",
        "FinishRoute::AnswerCurrent",
    ]
    .iter()
    .map(|site| v21.matches(site).count())
    .sum();
    let gate_marker = ["// POST_M7_DS5_M6_GATE_", "BEGIN"].concat();
    let lawful_m6_links = successor_source.matches(&gate_marker).count();
    let exact_m7 = AUTHORITATIVE_M7 == "b607ed52f640a3e202da3cc6b73ac58b180caf83"
        && env!("POST_M7_DS5_M7_HANDOFF_SHA256") == FROZEN_M7_HANDOFF_SHA256
        && env!("POST_M7_DS5_M7_CSV_SHA256") == FROZEN_M7_CSV_SHA256
        && env!("POST_M7_DS5_M7_MD_SHA256") == FROZEN_M7_MD_SHA256
        && env!("POST_M7_DS5_M7_SOURCE_SHA256") == FROZEN_M7_SOURCE_SHA256;
    let protocol_frozen = env!("POST_M7_DS5_PROTOCOL_SHA256") == FROZEN_PROTOCOL_SHA256;
    let frozen_parts = env!("POST_M7_DS5_V20_SOURCE_SHA256") == FROZEN_V20_SOURCE_SHA256
        && env!("POST_M7_DS5_V21_SOURCE_SHA256") == FROZEN_V21_SOURCE_SHA256
        && env!("POST_M7_DS5_V21B_RESULT_SHA256") == FROZEN_V21B_RESULT_SHA256;
    let physical_closure_path = v21.contains("self.lookup_outputs.len()")
        && v21.contains("self.current")
        && v21.contains("remaining_queued_spikes");
    let lawful_updates = usize::from(lawful_m6_links > 0);
    let expected_negative = exact_m7
        && protocol_frozen
        && frozen_parts
        && physical_closure_path
        && terminal_supervision_sites > 0
        && semantic_population_sites > 0
        && lawful_m6_links == 0
        && lawful_updates == 0;
    ProbeV1Report {
        protocol: PROTOCOL,
        seed: PROBE_V1_SEED,
        claim_eligible: false,
        expected_negative,
        exact_m7,
        protocol_frozen,
        frozen_parts,
        physical_closure_path,
        terminal_supervision_sites,
        semantic_population_sites,
        lawful_m6_links,
        lawful_updates,
        first_collapse:
            "V21b terminal answer supervision; lawful M6 closure-to-active-trace edge absent",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probe_v1_freezes_the_expected_first_collapse() {
        let report = run_probe_v1();
        assert!(report.expected_negative, "{report:#?}");
        assert!(report.physical_closure_path);
        assert!(report.terminal_supervision_sites > 0);
        assert!(report.semantic_population_sites > 0);
        assert_eq!(report.lawful_m6_links, 0);
        assert_eq!(report.lawful_updates, 0);
        assert!(!report.claim_eligible);
    }
}
