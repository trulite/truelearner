#![allow(dead_code)]
use std::env;
#[path = "../ds_r0_anonymous_post_action_evidence_return.rs"]
mod ds_r0_anonymous_post_action_evidence_return;
#[path = "../research_runtime.rs"]
mod research_runtime;
use ds_r0_anonymous_post_action_evidence_return::run;
use research_runtime::HarnessMode;
fn main() {
    let arg = env::args().nth(1).unwrap_or_else(|| "--micro".into());
    let mode = match arg.as_str() {
        "--micro" => HarnessMode::Micro,
        "--gate" => HarnessMode::Gate,
        "--definitive" => {
            eprintln!("DS-R0: definitive execution is forbidden");
            std::process::exit(2)
        }
        _ => {
            eprintln!("usage: ds_r0_anonymous_post_action_evidence_return [--micro|--gate]");
            std::process::exit(2)
        }
    };
    let r = run(mode);
    println!("{} mode={} audit={} claim_eligible={} M0_authoritative={} enabling_only={} M1_exists={} protocol={}",r.label,r.mode,if r.audit_passed{"PASS"}else{"FAIL"},r.claim_eligible,r.m0_authoritative,r.enabling_only,r.m1_exists,r.protocol);
    println!("stages={:?} source={:?}", r.stages, r.source);
    for s in &r.seeds {
        println!("seed={} E0={} exact={} fresh={} A1_candidates={} A1_templates={} A1_roots={} A1_structural={} A1_handles={} choice={} choose_calls={} DS1_updates={} effect_known={} activity_pulses={} activity_relations={} spikes={} arrows={} mutations={} mature_shapes={} temporary_relations={} bridge_fields={} controls={} E0_work={} A1_work={} R0_work={} E0_bytes={} A1_bytes={} DS1_bytes={} R0_bytes={} temporary_peak={}",s.seed,s.actual,s.exact,s.fresh_target,s.candidates,s.templates,s.roots,s.structural,s.handles,s.choice,s.choose_calls,s.ds1_updates,s.effect_known,s.activity_pulses,s.activity_relations,s.spikes,s.arrows,s.mutations,s.mature_shapes,s.temporary_relations,s.bridge_fields,s.controls.passed(),s.e0_work,s.a1_work,s.return_work.organism_work(),s.e0_bytes,s.a1_bytes,s.ds1_bytes,s.return_bytes,s.temporary_peak);
        println!(
            "seed={} controls={:?} return_work={:?}",
            s.seed, s.controls, s.return_work
        )
    }
    println!(
        "hashes parent={} protocol={} M0={} E0={} A1={} DS1={} stage6={} handoff={} results={}",
        ds_r0_anonymous_post_action_evidence_return::EXACT_PARENT,
        ds_r0_anonymous_post_action_evidence_return::PROTOCOL_COMMIT,
        ds_r0_anonymous_post_action_evidence_return::AUTHORITATIVE_M0,
        ds_r0_anonymous_post_action_evidence_return::FROZEN_E0_SHA256,
        ds_r0_anonymous_post_action_evidence_return::FROZEN_A1_SHA256,
        ds_r0_anonymous_post_action_evidence_return::FROZEN_DS1_SHA256,
        ds_r0_anonymous_post_action_evidence_return::FROZEN_PARENT_SHA256,
        ds_r0_anonymous_post_action_evidence_return::FROZEN_PARENT_HANDOFF_SHA256,
        ds_r0_anonymous_post_action_evidence_return::FROZEN_RESULTS_DIGEST
    );
    if !r.audit_passed {
        std::process::exit(1)
    }
}
