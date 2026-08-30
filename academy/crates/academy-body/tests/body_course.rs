use academy_body::{
    BodyCapability, BodyCourse, BodyCourseKind, BodyCourseOutcome, BodyExperience,
    BodyExperienceMode, BodyVerdict,
};
use behavior_diagram::BehaviorDiagram;
use sha2::{Digest, Sha256};

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
fn generated_course_preserves_first_failure_instead_of_teaching_around_it() {
    let run = BodyCourse::new(31_001).unwrap().run().unwrap();
    for experience in &run.experiences {
        assert_experience_diagram(experience);
    }
    assert!(run.exact_replay);
    assert!(!run.experiences.is_empty());
    assert!(run.acquired.contains(&BodyCapability::BinocularDepth));
    assert!(run.acquired.contains(&BodyCapability::HandContingency));
    assert!(run.acquired.contains(&BodyCapability::DigitSeparation));
    assert!(run.acquired.contains(&BodyCapability::SelfWorld));
    assert!(!run.acquired.contains(&BodyCapability::Contact));
    assert_eq!(run.first_failure, Some(BodyCapability::Contact));
    assert_eq!(run.courses.len(), BodyCourseKind::ORDER.len());
    assert_eq!(run.courses[0].course, BodyCourseKind::EyeControl);
    assert_eq!(run.courses[0].outcome, BodyCourseOutcome::Acquired);
    assert_eq!(run.courses[1].outcome, BodyCourseOutcome::Acquired);
    assert_eq!(run.courses[2].outcome, BodyCourseOutcome::Acquired);
    assert_eq!(
        run.courses[3].outcome,
        BodyCourseOutcome::Failed(BodyCapability::Contact)
    );
    let contact_probe = run
        .experiences
        .iter()
        .find(|experience| {
            experience.capability == BodyCapability::Contact
                && experience.mode == BodyExperienceMode::Probe
        })
        .unwrap();
    assert_ne!(contact_probe.verdict, BodyVerdict::Passed);
    assert!(run
        .experiences
        .iter()
        .filter(|experience| experience.mode != BodyExperienceMode::Development)
        .all(|experience| experience.durable_unchanged));
}

fn assert_experience_diagram(experience: &BodyExperience) {
    let mut diagram = BehaviorDiagram::new(&experience.id);
    let before = checkpoint_node("checkpoint before", &experience.checkpoint_before);
    let after = checkpoint_node("checkpoint after", &experience.checkpoint_after);
    let replay_after = if experience.replay_exact {
        after.clone()
    } else {
        "checkpoint replay diverged".to_string()
    };

    diagram.arrow("live", &before, "recorded experience", &after);
    diagram.arrow("replay", &before, "exact replay", replay_after);
    diagram.assert_commutes(&["live"], &["replay"]);

    if experience.mode != BodyExperienceMode::Development {
        let durable_after = if experience.durable_unchanged {
            before.clone()
        } else {
            "durable checkpoint changed".to_string()
        };
        diagram.arrow("discard", &after, "discard probe mutation", durable_after);
        diagram.arrow("durable identity", &before, "read unchanged body", &before);
        diagram.assert_commutes(&["live", "discard"], &["durable identity"]);
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
