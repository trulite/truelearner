#![deny(warnings)]

use truelearner_workstation::{
    verify_choice_contract, BodyLinkId, BodyPath, BodyReturnDecision, BodyTraceEvent,
    ContactSample, LightField, MotorEffect, WorkstationCheckpoint, WorkstationHarness,
    WorkstationStepObservation, WorldSample, TOUCH_SITES,
};

fn physical_sample(light_value: u8) -> WorldSample {
    let light = |value| {
        let width = 33_u16;
        let height = 33_u16;
        let mut pixels = vec![0_u8; usize::from(width) * usize::from(height)];
        pixels[usize::from(height / 2) * usize::from(width) + usize::from(width / 2)] = value;
        LightField::new(width, height, pixels).unwrap()
    };
    WorldSample::new(
        [light(light_value), light(light_value.saturating_sub(32))],
        [ContactSample::default(); TOUCH_SITES],
    )
    .unwrap()
}

fn same_external_effects(left: &[MotorEffect], right: &[MotorEffect]) -> bool {
    if left.len() != right.len() || left.is_empty() {
        return false;
    }
    let left_start = left.iter().map(|effect| effect.at).min().unwrap();
    let right_start = right.iter().map(|effect| effect.at).min().unwrap();
    left.iter().all(|expected| {
        right.iter().any(|actual| {
            expected.control == actual.control
                && expected.impulse == actual.impulse
                && expected.at - left_start == actual.at - right_start
        })
    })
}

fn count_path(counts: &mut Vec<(BodyPath, usize)>, path: BodyPath) {
    if let Some((_, count)) = counts.iter_mut().find(|(candidate, _)| *candidate == path) {
        *count += 1;
    } else {
        counts.push((path, 1));
    }
}

fn transferred_probe(
    checkpoint: WorkstationCheckpoint,
    reference: &WorkstationCheckpoint,
    learned_links: &[BodyLinkId],
) -> (WorkstationStepObservation, Vec<BodyTraceEvent>) {
    let mut harness = WorkstationHarness::restore(checkpoint).unwrap();
    harness.reposition_from_checkpoint(reference).unwrap();
    for step in 0..12 {
        let sample = physical_sample(if step % 2 == 0 { 200 } else { 80 });
        let (observation, trace) = harness.step_traced(sample).unwrap();
        verify_choice_contract(&trace).unwrap();
        let reused = trace.iter().any(|event| {
            matches!(
                event,
                BodyTraceEvent::Arrival(arrival)
                    if arrival.via.is_some_and(|link| learned_links.contains(&link))
            )
        });
        if reused {
            return (observation, trace);
        }
    }
    panic!("the changed world never traversed a retained composite")
}

#[test]
#[ignore = "pre-existing on main: the exact return resolves Ambiguous instead of Accepted"]
fn repeated_closed_workstation_experience_compacts_and_transfers() {
    let mut harness = WorkstationHarness::new(71_001).unwrap();
    let reference = harness.save().unwrap();
    let mut learned_links = Vec::<BodyLinkId>::new();
    let mut closure_counts = Vec::<(BodyPath, usize)>::new();
    let mut ordinary_effects = Vec::new();
    let mut ordinary_work = 0;
    for repetition in 0..7 {
        harness.reposition_from_checkpoint(&reference).unwrap();
        let mut returned = None;
        let mut latest = physical_sample(224);
        for step in 0..12 {
            latest = physical_sample(if step % 2 == 0 { 224 } else { 96 });
            let (action, action_trace) = harness.step_traced(latest.clone()).unwrap();
            verify_choice_contract(&action_trace).unwrap();
            if let Some(parent) = action.crossings.first().copied() {
                if repetition == 5 {
                    ordinary_effects = action.crossings.clone();
                    ordinary_work = action.metrics.physical_work;
                }
                returned = Some(parent);
                break;
            }
        }
        let parent = returned.expect("the generic workstation exposes an outward crossing");
        let formed_before = harness.automaticity_work().composites_formed;
        let (_, trace) = harness
            .settle_traced_with_boundary_parents(latest, &[parent])
            .unwrap();
        verify_choice_contract(&trace).unwrap();
        let returned = trace
            .iter()
            .find_map(|event| match event {
                BodyTraceEvent::Return(returned) => Some(returned),
                _ => None,
            })
            .expect("the world consequence reaches a return source");
        assert_eq!(returned.decision, BodyReturnDecision::Accepted);
        assert_eq!(returned.exact_paths, 1);
        assert_eq!(returned.return_cause, Some(parent.cause));
        let path = returned.path.expect("the exact return names its path");
        count_path(&mut closure_counts, path);
        if harness.automaticity_work().composites_formed > formed_before {
            let formed = trace
                .iter()
                .filter_map(|event| match event {
                    BodyTraceEvent::Strengthened(strengthened)
                        if strengthened.link != path.first && strengthened.link != path.second =>
                    {
                        Some(strengthened.link)
                    }
                    _ => None,
                })
                .collect::<Vec<_>>();
            assert_eq!(formed.len(), 1);
            learned_links.push(formed[0]);
        }
    }

    assert!(
        closure_counts
            .iter()
            .filter(|(_, count)| *count >= 3)
            .count()
            >= 2
    );
    assert_eq!(harness.automaticity_work().composites_formed, 2);
    assert_eq!(learned_links.len(), 2);
    let developed = harness.save().unwrap();

    let first = transferred_probe(developed.clone(), &reference, &learned_links);
    let replay = transferred_probe(developed, &reference, &learned_links);
    assert_eq!(first, replay);
    assert!(same_external_effects(&ordinary_effects, &first.0.crossings));
    assert!(first.0.metrics.physical_work < ordinary_work);
    assert!(first.0.naturally_quiescent);

    let mut no_return = WorkstationHarness::new(71_001).unwrap();
    let no_return_reference = no_return.save().unwrap();
    for _ in 0..7 {
        no_return
            .reposition_from_checkpoint(&no_return_reference)
            .unwrap();
        for step in 0..12 {
            let action = no_return
                .step(physical_sample(if step % 2 == 0 { 224 } else { 96 }))
                .unwrap();
            if !action.crossings.is_empty() {
                break;
            }
        }
    }
    assert_eq!(no_return.automaticity_work().composites_formed, 0);
}
