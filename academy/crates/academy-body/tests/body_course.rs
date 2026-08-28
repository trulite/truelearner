use academy_body::{
    BodyCapability, BodyCourse, BodyCourseKind, BodyCourseOutcome, BodyExperienceMode, BodyVerdict,
};

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
    assert!(probe.replay_exact);
    assert_eq!(probe.verdict, BodyVerdict::Passed);
    assert!(probe.durable_unchanged);
    assert_eq!(course.checkpoint_bytes().unwrap(), durable);
}

#[test]
fn generated_course_preserves_first_failure_instead_of_teaching_around_it() {
    let run = BodyCourse::new(31_001).unwrap().run().unwrap();
    assert!(run.exact_replay);
    assert!(!run.experiences.is_empty());
    assert!(run.acquired.contains(&BodyCapability::BinocularDepth));
    assert!(run.acquired.contains(&BodyCapability::HandContingency));
    assert!(!run.acquired.contains(&BodyCapability::DigitSeparation));
    assert_eq!(run.first_failure, Some(BodyCapability::DigitSeparation));
    assert_eq!(run.courses.len(), BodyCourseKind::ORDER.len());
    assert_eq!(run.courses[0].course, BodyCourseKind::EyeControl);
    assert_eq!(run.courses[0].outcome, BodyCourseOutcome::Acquired);
    assert_eq!(
        run.courses[1].outcome,
        BodyCourseOutcome::Failed(BodyCapability::DigitSeparation)
    );
    assert!(run.courses[2..]
        .iter()
        .all(|course| course.outcome == BodyCourseOutcome::NotReached));
    let last = run.experiences.last().unwrap();
    assert_eq!(last.capability, BodyCapability::DigitSeparation);
    assert_ne!(last.verdict, BodyVerdict::Passed);
    assert!(run
        .experiences
        .iter()
        .filter(|experience| experience.mode != BodyExperienceMode::Development)
        .all(|experience| experience.durable_unchanged));
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
