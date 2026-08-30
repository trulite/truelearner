use academy_workstation::WorkstationRecording;
use behavior_diagram::BehaviorDiagram;

#[path = "../../../tests/support/behavior_diagram.rs"]
mod behavior_diagram;

#[test]
fn recorded_workstation_behavior_forms_a_commuting_diagram() {
    let recording = WorkstationRecording::capture(91_001, 4).unwrap();
    let mut diagram = BehaviorDiagram::new("recorded workstation behavior");

    for (step, recorded) in recording.steps().iter().enumerate() {
        let world = format!("world[{step}] {}", &recorded.before.world_fingerprint[..8]);
        let harness = format!("harness[{step}]");
        let organism = format!("organism[{step}]");
        let outcome = format!("world outcome[{step}]");
        let evidence = format!(
            "evidence[{step}] {}",
            &recorded.observation.session_fingerprint[..8]
        );
        let sense = format!("{step}: sense");
        let admit = format!("{step}: admit");
        let effect = format!("{step}: effect");
        let observe = format!("{step}: observe");
        let record = format!("{step}: recorded step");

        diagram.arrow(&sense, &world, "ordinary world sample", &harness);
        diagram.arrow(
            &admit,
            &harness,
            format!(
                "{} admitted inputs",
                recorded.observation.body.admitted_inputs
            ),
            &organism,
        );
        diagram.arrow(
            &effect,
            &organism,
            format!(
                "{} crossings, {} device events",
                recorded.observation.body.crossings.len(),
                recorded.observation.device_events.len()
            ),
            &outcome,
        );
        diagram.arrow(
            &observe,
            &outcome,
            format!(
                "{} work, quiescent={}",
                recorded.observation.body.metrics.physical_work,
                recorded.observation.body.naturally_quiescent
            ),
            &evidence,
        );
        diagram.arrow(&record, &world, "frozen observer record", &evidence);
        diagram.assert_commutes(&[&sense, &admit, &effect, &observe], &[&record]);

        if let Some(next) = recording.steps().get(step + 1) {
            assert_eq!(
                recorded.observation.session_fingerprint, next.before.session_fingerprint,
                "disconnected workstation state between steps:\n{diagram}"
            );
            diagram.arrow(
                format!("{step}: continue"),
                &evidence,
                "next ordinary world step",
                format!(
                    "world[{}] {}",
                    step + 1,
                    &next.before.world_fingerprint[..8]
                ),
            );
        }
    }

    let final_evidence = format!(
        "evidence[{}] {}",
        recording.steps().len() - 1,
        &recording
            .steps()
            .last()
            .unwrap()
            .observation
            .session_fingerprint[..8]
    );
    let replay_target = if recording.verify_exact_replay().is_ok() {
        final_evidence.clone()
    } else {
        "replay diverged".to_string()
    };
    diagram.arrow(
        "live recording",
        "initial checkpoint",
        "recorded session",
        &final_evidence,
    );
    diagram.arrow(
        "replay recording",
        "initial checkpoint",
        "exact replay",
        replay_target,
    );
    diagram.assert_commutes(&["live recording"], &["replay recording"]);
    eprintln!("{diagram}");
}
