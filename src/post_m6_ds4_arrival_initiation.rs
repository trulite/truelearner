//! Development-only post-M6 DS4 physical-arrival initiation successor.

use crate::research_runtime::HarnessMode;

pub const PROTOCOL: &str = "post-m6-ds4-arrival-initiation-v2";
pub const AUTHORITATIVE_M6: &str = "aa4e22efd8a65b7694956a53cfaa970582695215";
pub const PROBE_V1_SEED: u64 = 140_000_000;
pub const FROZEN_PROTOCOL_SHA256: &str =
    "01c47af6fe1be9dc1e48a4b81a94e194df36d1631e1d650c8f2e94284bd42d6b";
pub const FROZEN_M6_CSV_SHA256: &str =
    "0cb9ba779fca1899cf030d30358fe9354cfb7b2cccf87f32df3f6ea9ddfe91e4";
pub const FROZEN_M6_MD_SHA256: &str =
    "6a5d938c3e021344b00f3a559593fee860b5f6cceb777c409ad8d59a2dd71872";
pub const FROZEN_M6_HANDOFF_SHA256: &str =
    "6cdd015d6b20f10a95f26c33dfe30ceb834b2663f9912926752fa1fb204c9ca9";
pub const FROZEN_OLD_NEGATIVE_CSV_SHA256: &str =
    "c6b626650fc199a8ebe2feae8115a8b27071088f963f919393f55e24fbe44a3a";
pub const FROZEN_OLD_NEGATIVE_MD_SHA256: &str =
    "97f1fc665e03be1ccd398dcdf34fbb262c525aabc23a812c8740c350dd890659";

#[allow(dead_code)]
mod frozen_pre_m6_ds4 {
    include!(concat!(
        env!("OUT_DIR"),
        "/ds4_cumulative_request_start_port.rs"
    ));
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProbeV1Report {
    pub protocol: &'static str,
    pub seed: u64,
    pub claim_eligible: bool,
    pub expected_negative: bool,
    pub exact_m6: bool,
    pub immutable_old_negative: bool,
    pub protocol_frozen: bool,
    pub physical_arrival_path: bool,
    pub learned_event_activity: usize,
    pub occurrence_selections: usize,
    pub semantic_feedback_calls: usize,
    pub m6_differential_links: usize,
    pub lawful_updates: usize,
    pub first_collapse: &'static str,
}

pub fn run_probe_v1() -> ProbeV1Report {
    let old = frozen_pre_m6_ds4::run_probe();
    let old_source = include_str!("ds4_cumulative_request_start_port.rs");
    let linker = old_source
        .split("// DS4_LINKER_START")
        .nth(1)
        .and_then(|tail| tail.split("// DS4_LINKER_END").next())
        .unwrap_or_default();
    let semantic_feedback_calls = linker
        .matches("learner.feedback(choice.pattern_cell, functional)")
        .count();
    let m6_differential_links = linker
        .matches("delayed_experience(differential)")
        .count();
    let exact_m6 = AUTHORITATIVE_M6 == "aa4e22efd8a65b7694956a53cfaa970582695215"
        && env!("POST_M6_DS4_M6_CSV_SHA256") == FROZEN_M6_CSV_SHA256
        && env!("POST_M6_DS4_M6_MD_SHA256") == FROZEN_M6_MD_SHA256
        && env!("POST_M6_DS4_M6_HANDOFF_SHA256") == FROZEN_M6_HANDOFF_SHA256;
    let immutable_old_negative = env!("POST_M6_DS4_OLD_NEGATIVE_CSV_SHA256")
        == FROZEN_OLD_NEGATIVE_CSV_SHA256
        && env!("POST_M6_DS4_OLD_NEGATIVE_MD_SHA256") == FROZEN_OLD_NEGATIVE_MD_SHA256;
    let protocol_frozen =
        env!("POST_M6_DS4_PROTOCOL_SHA256") == FROZEN_PROTOCOL_SHA256;
    let physical_arrival_path = old.path_exists
        && old.learned_m3_uses > 0
        && old.completion_activity > 0
        && old.request_selection_activations > 0;
    let lawful_updates = if semantic_feedback_calls == 0 && m6_differential_links == 1 {
        old.request_update_activations
    } else {
        0
    };
    let expected_negative = exact_m6
        && immutable_old_negative
        && protocol_frozen
        && physical_arrival_path
        && semantic_feedback_calls == 1
        && m6_differential_links == 0
        && lawful_updates == 0;
    ProbeV1Report {
        protocol: PROTOCOL,
        seed: PROBE_V1_SEED,
        claim_eligible: false,
        expected_negative,
        exact_m6,
        immutable_old_negative,
        protocol_frozen,
        physical_arrival_path,
        learned_event_activity: old.completion_activity,
        occurrence_selections: old.request_selection_activations,
        semantic_feedback_calls,
        m6_differential_links,
        lawful_updates,
        first_collapse: "P4 semantic terminal credit; M6 differential-to-active-trace edge absent",
    }
}

pub fn definitive_rejected() -> bool {
    let _ = HarnessMode::Definitive;
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probe_v1_freezes_the_expected_first_collapse() {
        let report = run_probe_v1();
        assert!(report.expected_negative, "{report:#?}");
        assert!(report.physical_arrival_path);
        assert_eq!(report.semantic_feedback_calls, 1);
        assert_eq!(report.m6_differential_links, 0);
        assert_eq!(report.lawful_updates, 0);
        assert!(!report.claim_eligible);
    }

    #[test]
    fn definitive_is_inert() {
        assert!(definitive_rejected());
    }
}
