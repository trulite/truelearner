use academy_body::{
    BodyCapability, BodyCourse, BodyCourseKind, BodyCourseOutcome, BodyEvidenceState,
    BodyExperience, BodyExperienceMode, BodyVerdict,
};
use behavior_diagram::BehaviorDiagram;
use sha2::{Digest, Sha256};
use truelearner_workstation::{WorkstationCheckpoint, WorkstationHarness};

#[path = "../../../tests/support/behavior_diagram.rs"]
mod behavior_diagram;

#[test]
fn development_commits_but_probe_is_discarded_and_replays_exactly() {
    let mut course = BodyCourse::new(41_001).unwrap();
    let development = course
        .experience(
            BodyCapability::GazeContingency,
            BodyExperienceMode::Development,
            41_002,
        )
        .unwrap();
    assert_experience_diagram(&development);
    assert!(development.replay_exact);
    assert_eq!(development.verdict, BodyVerdict::Passed);
    let durable = course.checkpoint_bytes().unwrap();

    let probe = course
        .experience(
            BodyCapability::GazeContingency,
            BodyExperienceMode::Probe,
            9_041_002,
        )
        .unwrap();
    assert_experience_diagram(&probe);
    assert!(probe.replay_exact);
    assert_eq!(probe.verdict, BodyVerdict::Passed);
    assert!(probe.durable_unchanged);
    assert_eq!(course.checkpoint_bytes().unwrap(), durable);
}

#[test]
fn the_pointer_body_course_acquires_every_capability() {
    let run = BodyCourse::new(31_001).unwrap().run().unwrap();
    for experience in &run.experiences {
        assert_experience_diagram(experience);
    }
    assert!(run.exact_replay);
    assert!(!run.experiences.is_empty());
    assert_eq!(
        run.acquired,
        [
            BodyCapability::GazeContingency,
            BodyCapability::GazeControl,
            BodyCapability::BinocularDepth,
            BodyCapability::HandContingency,
            BodyCapability::SelfWorld,
        ]
    );
    assert_eq!(run.first_failure, None);
    assert_eq!(run.schema_version, 15);
    let completed =
        WorkstationHarness::restore(WorkstationCheckpoint::decode(&run.body_checkpoint).unwrap())
            .unwrap();
    assert_eq!(
        completed.read().unwrap().body_fingerprint,
        run.final_body_fingerprint
    );
    assert_eq!(run.courses.len(), BodyCourseKind::ORDER.len());
    assert_eq!(run.courses[0].course, BodyCourseKind::EyeControl);
    assert_eq!(run.courses[0].outcome, BodyCourseOutcome::Acquired);
    assert_eq!(run.courses[1].outcome, BodyCourseOutcome::Acquired);
    assert_eq!(run.courses[2].outcome, BodyCourseOutcome::Acquired);
    for capability in BodyCapability::ORDER {
        assert_eq!(
            run.capability_evidence
                .iter()
                .find(|evidence| evidence.capability == capability)
                .unwrap()
                .state,
            BodyEvidenceState::Acquired,
            "{capability:?}"
        );
    }
}

#[test]
fn a_held_out_seed_acquires_the_whole_course() {
    let run = BodyCourse::new(31_002).unwrap().run().unwrap();

    assert!(run.exact_replay);
    assert_eq!(run.first_failure, None);
    assert_eq!(
        run.acquired,
        [
            BodyCapability::GazeContingency,
            BodyCapability::GazeControl,
            BodyCapability::BinocularDepth,
            BodyCapability::HandContingency,
            BodyCapability::SelfWorld,
        ]
    );
    assert_eq!(run.courses[0].outcome, BodyCourseOutcome::Acquired);
    assert_eq!(run.courses[1].outcome, BodyCourseOutcome::Acquired);
    assert_eq!(run.courses[2].outcome, BodyCourseOutcome::Acquired);
}

fn assert_experience_diagram(experience: &BodyExperience) {
    let mut diagram = BehaviorDiagram::new(&experience.id);
    let before = checkpoint_node("checkpoint before", &experience.checkpoint_before);
    let after = checkpoint_node("checkpoint after", &experience.checkpoint_after);
    let prepared = experience
        .perturbation
        .map(|perturbation| {
            format!(
                "prepared {:?} impulse {}",
                perturbation.control, perturbation.impulse
            )
        })
        .unwrap_or_else(|| before.clone());
    let replay_after = if experience.replay_exact {
        after.clone()
    } else {
        "checkpoint replay diverged".to_string()
    };

    if experience.perturbation.is_some() {
        diagram.arrow(
            "live setup",
            &before,
            "recorded external perturbation",
            &prepared,
        );
        diagram.arrow(
            "replay setup",
            &before,
            "same external perturbation",
            &prepared,
        );
    }
    diagram.arrow("live", &prepared, "recorded experience", &after);
    diagram.arrow("replay", &prepared, "exact replay", replay_after);
    if experience.perturbation.is_some() {
        diagram.assert_commutes(&["live setup", "live"], &["replay setup", "replay"]);
    } else {
        diagram.assert_commutes(&["live"], &["replay"]);
    }

    if !matches!(experience.mode, BodyExperienceMode::Development) {
        let durable_after = if experience.durable_unchanged {
            before.clone()
        } else {
            "durable checkpoint changed".to_string()
        };
        diagram.arrow("discard", &after, "discard probe mutation", durable_after);
        diagram.arrow("durable identity", &before, "read unchanged body", &before);
        if experience.perturbation.is_some() {
            diagram.assert_commutes(&["live setup", "live", "discard"], &["durable identity"]);
        } else {
            diagram.assert_commutes(&["live", "discard"], &["durable identity"]);
        }
    }

    assert_eq!(experience.samples.len(), experience.observations.len());
    for (step, observation) in experience.observations.iter().enumerate() {
        let world = format!("world[{step}]");
        let harness = format!("harness[{step}]");
        let organism = format!("organism[{step}]");
        let outcome = format!("world outcome[{step}]");
        let evidence = format!("evidence[{step}]");
        let admit = format!("{step}: admit");
        let run = format!("{step}: run");
        let effect = format!("{step}: effect");
        let observe = format!("{step}: observe");
        let record = format!("{step}: recorded step");

        diagram.arrow(&admit, &world, "ordinary physical sample", &harness);
        diagram.arrow(
            &run,
            &harness,
            format!("{} admitted inputs", observation.admitted_inputs),
            &organism,
        );
        diagram.arrow(
            &effect,
            &organism,
            format!(
                "{} crossings, {} movements",
                observation.crossings.len(),
                observation.movements.len()
            ),
            &outcome,
        );
        diagram.arrow(
            &observe,
            &outcome,
            format!(
                "{} returns, {} work, quiescent={}",
                observation.returned_transitions.len(),
                observation.metrics.physical_work,
                observation.naturally_quiescent
            ),
            &evidence,
        );
        diagram.arrow(&record, &world, "frozen observer record", &evidence);
        diagram.assert_commutes(&[&admit, &run, &effect, &observe], &[&record]);

        if let Some(next) = experience.observations.get(step + 1) {
            assert_eq!(
                observation.state_after, next.state_before,
                "disconnected physical state between steps:\n{diagram}"
            );
            diagram.arrow(
                format!("{step}: continue"),
                &evidence,
                "next ordinary world step",
                format!("world[{}]", step + 1),
            );
        }
    }

    if !experience.observations.is_empty() {
        diagram.arrow(
            "evaluate",
            format!("evidence[{}]", experience.observations.len() - 1),
            format!("external {:?} verdict", experience.verdict),
            format!("academy claim {:?}", experience.capability),
        );
    }
    eprintln!("{diagram}");
}

fn checkpoint_node(label: &str, bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    format!(
        "{label} {:02x}{:02x}{:02x}{:02x}",
        digest[0], digest[1], digest[2], digest[3]
    )
}

#[test]
fn serialized_organism_samples_contain_no_course_or_evaluator_fields() {
    let mut course = BodyCourse::new(51_001).unwrap();
    let experience = course
        .experience(
            BodyCapability::GazeContingency,
            BodyExperienceMode::Development,
            51_002,
        )
        .unwrap();
    let wire = serde_json::to_string(&experience.samples[0]).unwrap();
    for forbidden in [
        "capability",
        "expected",
        "target",
        "success",
        "score",
        "teaching",
        "direction",
        "action",
    ] {
        assert!(!wire.contains(forbidden), "leaked {forbidden}: {wire}");
    }
}
