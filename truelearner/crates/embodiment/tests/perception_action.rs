use truelearner_embodiment::{
    interact, Availability, CommandCollector, EffectMode, InteractionStep, OpposedEffort, Port,
};

#[derive(Clone, Debug, PartialEq, Eq)]
struct AxisState {
    position: i16,
    calls: u8,
}

#[test]
fn absent_command_is_the_identity_action() {
    let mut state = AxisState {
        position: 7,
        calls: 0,
    };
    let step: InteractionStep<i16, i16> = interact(
        &mut state,
        |state| state.position,
        |_| None,
        |state, requested| {
            state.calls += 1;
            state.position += requested;
            requested
        },
    );

    assert_eq!(step.before, 7);
    assert_eq!(step.effect, None);
    assert_eq!(step.after, 7);
    assert_eq!(state.calls, 0);
}

#[test]
fn return_is_sensed_from_actual_state_not_copied_from_the_request() {
    let mut state = AxisState {
        position: 9,
        calls: 0,
    };
    let step = interact(
        &mut state,
        |state| state.position,
        |_| Some(5_i16),
        |state, requested| {
            state.calls += 1;
            let before = state.position;
            state.position = state.position.saturating_add(requested).clamp(0, 10);
            state.position - before
        },
    );

    assert_eq!(step.before, 9);
    assert_eq!(step.effect, Some(1));
    assert_eq!(step.after, 10);
    assert_eq!(state.calls, 1);
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct BinocularState {
    feature: i16,
    gazes: [i16; 2],
    visible: [bool; 2],
}

fn binocular_reading(state: &BinocularState) -> [Availability<i16>; 2] {
    std::array::from_fn(|eye| {
        if state.visible[eye] {
            Availability::Available(state.feature - state.gazes[eye])
        } else {
            Availability::Unavailable
        }
    })
}

#[test]
fn one_binocular_cycle_preserves_occlusion_and_eye_locality() {
    let mut state = BinocularState {
        feature: 4,
        gazes: [1, -3],
        visible: [true, false],
    };
    let step = interact(
        &mut state,
        binocular_reading,
        |views| {
            let mut commands = CommandCollector::<OpposedEffort, 2>::new();
            for (eye, view) in views.iter().enumerate() {
                if let Availability::Available(offset) = view {
                    let command = if *offset < 0 {
                        OpposedEffort::new(1, 0)
                    } else if *offset > 0 {
                        OpposedEffort::new(0, 1)
                    } else {
                        continue;
                    };
                    commands
                        .add(Port(eye as u32), command, |left, right| {
                            left.combine_bounded(right, 8)
                        })
                        .unwrap();
                }
            }
            Some(commands.finish())
        },
        |state, commands| {
            let mut actual = [0_i16; 2];
            for (eye, command) in commands.into_commands().into_iter().enumerate() {
                if let Some(command) = command {
                    let before = state.gazes[eye];
                    state.gazes[eye] = state.gazes[eye]
                        .saturating_add(command.net() as i16)
                        .clamp(-8, 8);
                    actual[eye] = state.gazes[eye] - before;
                }
            }
            actual
        },
    );

    assert_eq!(
        step.before,
        [Availability::Available(3), Availability::Unavailable]
    );
    assert_eq!(step.effect, Some([1, 0]));
    assert_eq!(
        step.after,
        [Availability::Available(2), Availability::Unavailable]
    );
    assert_eq!(state.gazes, [2, -3]);
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct HandState {
    depth: [u16; 5],
    surface: u16,
    calls: u8,
}

fn pressures(state: &HandState) -> [u16; 5] {
    state.depth.map(|depth| {
        if depth >= state.surface {
            depth - state.surface + 1
        } else {
            0
        }
    })
}

fn finger_cycle(state: &mut HandState) -> InteractionStep<[u16; 5], [u16; 5]> {
    interact(
        state,
        pressures,
        |contact| {
            let mut commands = CommandCollector::<OpposedEffort, 5>::new();
            for finger in 0..contact.len() {
                commands
                    .add(
                        Port(finger as u32),
                        OpposedEffort::new(0, 1),
                        |left, right| left.combine_bounded(right, 8),
                    )
                    .unwrap();
            }
            let mut frame = commands.finish();
            for (finger, pressure) in contact.iter().enumerate() {
                if *pressure > 0 {
                    frame
                        .constrain(Port(finger as u32), EffectMode::Identity)
                        .unwrap();
                }
            }
            Some(frame)
        },
        |state, commands| {
            state.calls += 1;
            let mut actual = [0_u16; 5];
            for (finger, command) in commands.into_commands().into_iter().enumerate() {
                if command.is_some() {
                    let before = state.depth[finger];
                    state.depth[finger] = state.depth[finger].saturating_add(1).min(state.surface);
                    actual[finger] = state.depth[finger] - before;
                }
            }
            actual
        },
    )
}

#[test]
fn five_fingers_use_contact_to_make_only_the_touching_effect_identity() {
    let mut state = HandState {
        depth: [0, 1, 2, 1, 0],
        surface: 2,
        calls: 0,
    };

    let step = finger_cycle(&mut state);

    assert_eq!(step.before, [0, 0, 1, 0, 0]);
    assert_eq!(step.effect, Some([1, 1, 0, 1, 1]));
    assert_eq!(step.after, [0, 1, 1, 1, 0]);
    assert_eq!(state.depth, [1, 2, 2, 2, 1]);
    assert_eq!(state.calls, 1);
}

#[test]
fn repeated_cycles_close_orientation_then_become_identity() {
    let mut state = AxisState {
        position: -3,
        calls: 0,
    };
    let mut steps = Vec::new();
    for _ in 0..5 {
        steps.push(interact(
            &mut state,
            |state| state.position,
            |position| match position.cmp(&0) {
                std::cmp::Ordering::Less => Some(1_i16),
                std::cmp::Ordering::Greater => Some(-1_i16),
                std::cmp::Ordering::Equal => None,
            },
            |state, movement| {
                state.calls += 1;
                state.position += movement;
                movement
            },
        ));
    }

    assert_eq!(state.position, 0);
    assert_eq!(state.calls, 3);
    assert_eq!(steps[2].after, 0);
    assert_eq!(steps[3].effect, None);
    assert_eq!(steps[4].effect, None);
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct MixedBody {
    views: [Availability<i16>; 2],
    contact: u16,
    axes: [i16; 3],
}

#[test]
fn held_out_mixed_body_composes_visual_contact_and_three_actuators() {
    let mut body = MixedBody {
        views: [Availability::Available(-2), Availability::Unavailable],
        contact: 0,
        axes: [0, 4, 9],
    };
    let step = interact(
        &mut body,
        |body| (body.views, body.contact, body.axes),
        |(views, contact, _)| {
            let mut commands = CommandCollector::<i16, 3>::new();
            if matches!(views[0], Availability::Available(_)) {
                commands.add(Port(0), -1, i16::saturating_add).unwrap();
            }
            if *contact == 0 {
                commands.add(Port(2), 1, i16::saturating_add).unwrap();
            }
            Some(commands.finish())
        },
        |body, commands| {
            let before = body.axes;
            for (axis, command) in commands.into_commands().into_iter().enumerate() {
                if let Some(command) = command {
                    body.axes[axis] = body.axes[axis].saturating_add(command);
                }
            }
            std::array::from_fn(|axis| body.axes[axis] - before[axis])
        },
    );

    assert_eq!(step.effect, Some([-1, 0, 1]));
    assert_eq!(step.after.2, [-1, 4, 10]);
    assert_eq!(step.after.0[1], Availability::Unavailable);
}
