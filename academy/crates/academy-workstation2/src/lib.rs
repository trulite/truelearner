#![forbid(unsafe_code)]
//! A tablet-like external world. Only light and hand contact enter the body.

mod application;
mod display;
mod draw;
mod game_surface;
mod screen;
mod session;
mod target;
mod world;

pub use display::{
    display_from_screen, DisplayPoint, DisplayRect, Viewport, ARC_VIEWPORT_MARGIN,
    ARC_VIEWPORT_SIDE, DISPLAY_SIDE,
};
pub use draw::Rect;
pub use screen::{BezelControl, DeviceEvent, ScreenPoint, TouchId, CONTACT_DEPTH};
pub use session::{Workstation2Observation, Workstation2Session};
pub use target::{TargetApp, TargetLayout, TARGET_SIDE};
pub use world::Workstation2;

/// Maximum Manhattan path travel classified as one generic screen tap.
pub const TAP_TRAVEL: i16 = 32;

#[cfg(test)]
mod tests {
    use super::*;
    use truelearner_workstation::{BodyAxis, BodyControl, Direction, Eye, WorkstationHarness};

    #[test]
    fn gaze_refines_local_detail_without_removing_global_context() {
        let mut world = Workstation2::new(0);
        let mut body = WorkstationHarness::new(1).unwrap();
        let before = world.sense(body.state()).unwrap();
        body.perturb_body(
            BodyControl::new(
                BodyAxis::EyeHorizontal { eye: Eye::Left },
                Direction::Increase,
            ),
            1,
        )
        .unwrap();
        let after = world.sense(body.state()).unwrap();

        assert_eq!(
            before.eye(Eye::Left).global(),
            after.eye(Eye::Left).global()
        );
        assert_eq!(
            before.eye(Eye::Left).changed_values(),
            after.eye(Eye::Left).changed_values()
        );
        assert_ne!(
            before.eye(Eye::Left).foveal(),
            after.eye(Eye::Left).foveal()
        );
        assert_eq!(before.eye(Eye::Right), after.eye(Eye::Right));
    }

    #[test]
    fn an_opposite_arc_edge_change_enters_the_fovea_within_thirty_two_steps() {
        let frame = |x: usize| {
            let mut pixels = vec![0_u8; 64 * 64];
            pixels[32 * 64 + x] = 255;
            truelearner_workstation::LightField::new(64, 64, pixels).unwrap()
        };
        let mut body = WorkstationHarness::new(1).unwrap();
        for _ in 0..12 {
            body.perturb_body(
                BodyControl::new(
                    BodyAxis::EyeHorizontal { eye: Eye::Left },
                    Direction::Increase,
                ),
                4,
            )
            .unwrap();
        }
        assert!(body.state().eye(Eye::Left).gaze().x() >= 896);

        let mut world = Workstation2::with_pixels_in_viewport(frame(63), Viewport::arc()).unwrap();
        body.observe(world.sense(body.state()).unwrap()).unwrap();
        world.replace_pixels(frame(0)).unwrap();
        let mut gazes = Vec::new();
        let mut detected = false;
        for _ in 0..32 {
            let sample = world.sense(body.state()).unwrap();
            detected |= sample.eye(Eye::Left).foveal().pixels().contains(&255);
            body.step(sample).unwrap();
            let gaze = body.state().eye(Eye::Left).gaze();
            gazes.push((gaze.x(), gaze.y()));
        }
        assert!(detected, "gazes {gazes:?}");
    }

    #[test]
    fn a_real_fingertip_contact_reaches_the_virtual_keyboard() {
        let mut world = Workstation2::new(0);
        let mut body = WorkstationHarness::new(2).unwrap();
        while body.state().hand().palm().depth() < CONTACT_DEPTH - 16 {
            body.perturb_body(
                BodyControl::new(BodyAxis::PalmDepth, Direction::Increase),
                1,
            )
            .unwrap();
        }
        let palm_depth = body.state().hand().palm().depth();
        body.perturb_body(
            BodyControl::new(BodyAxis::FingerFlexion, Direction::Increase),
            1,
        )
        .unwrap();
        let started = world.advance(body.state());
        assert!(started
            .iter()
            .any(|event| matches!(event, DeviceEvent::TouchStarted { .. })));
        assert_eq!(body.state().hand().palm().depth(), palm_depth);

        let finger_flexion = body.state().hand().finger_flexion();
        body.perturb_body(
            BodyControl::new(BodyAxis::PalmHorizontal, Direction::Increase),
            1,
        )
        .unwrap();
        let moved = world.advance(body.state());
        assert!(moved
            .iter()
            .any(|event| matches!(event, DeviceEvent::TouchMoved { .. })));
        assert_eq!(body.state().hand().finger_flexion(), finger_flexion);
        assert_eq!(body.state().hand().palm().depth(), palm_depth);

        body.perturb_body(
            BodyControl::new(BodyAxis::FingerFlexion, Direction::Decrease),
            1,
        )
        .unwrap();
        let ended = world.advance(body.state());
        assert!(ended
            .iter()
            .any(|event| matches!(event, DeviceEvent::TouchEnded { .. })));
        assert_eq!(body.state().hand().palm().depth(), palm_depth);
        assert!(!world.text().is_empty());
    }

    #[test]
    fn two_screen_contacts_change_scale_but_one_does_not() {
        let mut world = Workstation2::new(0);
        let first = TouchId::new(0).unwrap();
        let second = TouchId::new(1).unwrap();
        world.apply_device_events(&[
            DeviceEvent::TouchStarted {
                touch: first,
                at: ScreenPoint { x: 400, y: 400 },
            },
            DeviceEvent::TouchMoved {
                touch: first,
                from: ScreenPoint { x: 400, y: 400 },
                to: ScreenPoint { x: 350, y: 400 },
            },
        ]);
        assert_eq!(world.scale(), 128);

        world.apply_device_events(&[
            DeviceEvent::TouchStarted {
                touch: second,
                at: ScreenPoint { x: 600, y: 400 },
            },
            DeviceEvent::TouchMoved {
                touch: second,
                from: ScreenPoint { x: 600, y: 400 },
                to: ScreenPoint { x: 700, y: 400 },
            },
        ]);
        assert!(world.scale() > 128);
    }

    #[test]
    fn session_has_no_device_event_input_to_the_body() {
        let checkpoint = WorkstationHarness::new(3).unwrap().save().unwrap();
        let mut first = Workstation2Session::from_checkpoint(checkpoint.clone(), 0).unwrap();
        let mut second = Workstation2Session::from_checkpoint(checkpoint, 0).unwrap();

        let left = first.step().unwrap();
        let right = second.step().unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn an_application_event_cannot_mutate_the_body() {
        let body = WorkstationHarness::new(4).unwrap();
        let before = body.read().unwrap();
        let mut world = Workstation2::new(0);
        let touch = TouchId::new(0).unwrap();
        let key = ScreenPoint { x: 448, y: 736 };

        world.apply_device_events(&[
            DeviceEvent::TouchStarted { touch, at: key },
            DeviceEvent::TouchEnded { touch, at: key },
        ]);

        assert_eq!(body.read().unwrap(), before);
        assert_eq!(world.text(), "A");
    }

    #[test]
    fn pixels_are_replaceable_and_ignore_device_events() {
        let dark = truelearner_workstation::LightField::filled(64, 64, 0).unwrap();
        let light = truelearner_workstation::LightField::filled(64, 64, 255).unwrap();
        let body = WorkstationHarness::new(5).unwrap();
        let mut world = Workstation2::with_pixels(dark);
        let before = world.sense(body.state()).unwrap();
        world.apply_device_events(&[DeviceEvent::TouchEnded {
            touch: TouchId::new(0).unwrap(),
            at: ScreenPoint { x: 512, y: 512 },
        }]);
        assert_eq!(world.sense(body.state()).unwrap(), before);
        world.replace_pixels(light).unwrap();
        assert_ne!(world.sense(body.state()).unwrap(), before);
    }

    #[test]
    fn traced_session_step_preserves_the_ordinary_observation() {
        let checkpoint = WorkstationHarness::new(6).unwrap().save().unwrap();
        let frame = truelearner_workstation::LightField::filled(64, 64, 0).unwrap();
        let world = Workstation2::with_pixels(frame);
        let mut ordinary =
            Workstation2Session::with_world(checkpoint.clone(), world.clone()).unwrap();
        let mut traced = Workstation2Session::with_world(checkpoint, world).unwrap();

        let observation = ordinary.step().unwrap();
        let (traced_observation, trace) = traced.step_traced().unwrap();
        assert_eq!(traced_observation, observation);
        truelearner_workstation::verify_choice_contract(&trace).unwrap();
    }
}
