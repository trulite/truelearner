//! The target app: one reactive lit rectangle, an optional inert decoy, a
//! drag goal, ordered pairs, and blank screens. Everything it knows stays
//! outside the organism.
use crate::draw::{background, distance, fill_rect, frame, Rect};
use crate::{DeviceEvent, ScreenPoint, TAP_TRAVEL};
use truelearner_workstation::{LightField, BODY_MAX};

pub const TARGET_SIDE: i16 = 192;
const SCREEN_AREA: u32 = (BODY_MAX as u32 + 1) * (BODY_MAX as u32 + 1);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TargetLayout {
    /// The reactive target, or rect A of a sequence pair.
    pub target: Option<Rect>,
    /// The inert decoy, or rect B of a sequence pair.
    pub decoy: Option<Rect>,
    /// The drag goal: a touch that starts on the target and ends here
    /// completes a drag.
    pub goal: Option<Rect>,
    /// Brightness of the target; the background stays under 60.
    pub target_band: u8,
    pub decoy_band: u8,
    pub goal_band: u8,
    /// A reactive target jumps when tapped. A dead target only sits there.
    pub reactive: bool,
    /// The zero-contrast control keeps the rectangle for counting but draws
    /// nothing, so any preference for it is chance.
    pub visible: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Rewarded {
    Ab,
    Ba,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TargetApp {
    layout: TargetLayout,
    side: i16,
    rng: u64,
    started: [Option<ScreenPoint>; 5],
    taps: u32,
    target_taps: u32,
    decoy_taps: u32,
    hits: u32,
    /// The target reacts until this many hits, then goes dead in place.
    dies_after: Option<u32>,
    /// The sequence order that produces the visible change, if any.
    rewarded: Option<Rewarded>,
    /// Which rect was tapped last: `true` for A (target), `false` for B.
    last_tapped: Option<bool>,
    ab_pairs: u32,
    ba_pairs: u32,
    drag_attempts: u32,
    drag_hits: u32,
}

impl TargetApp {
    pub fn new(layout: TargetLayout, seed: u64) -> Self {
        Self {
            layout,
            side: TARGET_SIDE,
            rng: seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1),
            started: [None; 5],
            taps: 0,
            target_taps: 0,
            decoy_taps: 0,
            hits: 0,
            dies_after: None,
            rewarded: None,
            last_tapped: None,
            ab_pairs: 0,
            ba_pairs: 0,
            drag_attempts: 0,
            drag_hits: 0,
        }
    }

    /// A target somewhere on the screen, fully visible, reactive.
    pub fn lit(seed: u64) -> Self {
        Self::lit_with_side(seed, TARGET_SIDE)
    }

    /// A target of `side` world units somewhere on the screen, fully
    /// visible, reactive. A large side is the big toy: the first hits
    /// present early, so the consequence loop can find them.
    pub fn lit_with_side(seed: u64, side: i16) -> Self {
        let mut app = Self::new(Self::bare_layout(seed), seed);
        app.side = side.clamp(64, TARGET_SIDE.max(512));
        app.layout.target = Some(app.random_rect());
        app
    }

    /// The live-key pair: a reactive target and an inert decoy, equally
    /// bright, at distinct places. Only the consequence distinguishes them.
    pub fn dual(seed: u64) -> Self {
        Self::dual_with_side(seed, TARGET_SIDE)
    }

    /// The live-key pair at `side` world units. A large side is the big
    /// toy: the two rectangles abut or overlap, so a midpoint tap still
    /// lands on a key and the asymmetry presents to the learner. The
    /// probes use the standard side.
    pub fn dual_with_side(seed: u64, side: i16) -> Self {
        let mut app = Self::new(Self::bare_layout(seed), seed);
        app.side = side.clamp(64, TARGET_SIDE.max(512));
        app.layout.decoy_band = 230;
        app.layout.target = Some(app.random_rect());
        let target = app.layout.target.unwrap();
        let mut decoy = app.random_rect();
        while if side < 512 {
            decoy.overlaps(target)
        } else {
            decoy.contains(ScreenPoint {
                x: (target.left + target.right) / 2,
                y: (target.top + target.bottom) / 2,
            })
        } {
            decoy = app.random_rect();
        }
        app.layout.decoy = Some(decoy);
        app
    }

    /// The live-key control: the same two rectangles with their roles
    /// exchanged, so a learner that tracks position instead of reaction
    /// fails the probe.
    pub fn swapped(mut self) -> Self {
        std::mem::swap(&mut self.layout.target, &mut self.layout.decoy);
        self
    }

    /// The dead-key toy: the target reacts until `limit` hits, then goes
    /// dead in place. The stop is the visible change.
    pub fn dies_after(mut self, limit: u32) -> Self {
        self.dies_after = Some(limit);
        self
    }

    /// The quiet-hand screen: nothing drawn, nothing reacts.
    pub fn blank(seed: u64) -> Self {
        Self::new(Self::bare_layout(seed), seed)
    }

    /// The sequence pair: two equally bright rectangles where only tapping
    /// A then B produces the visible change — both jump.
    pub fn sequence(seed: u64) -> Self {
        Self::dual(seed).into_sequence(Rewarded::Ab)
    }

    /// The sequence control: the reversed order produces the change.
    pub fn reversed(mut self) -> Self {
        self.rewarded = Some(Rewarded::Ba);
        self
    }

    fn into_sequence(mut self, rewarded: Rewarded) -> Self {
        self.layout.reactive = false;
        self.rewarded = Some(rewarded);
        self
    }

    /// The drag toy: a touch that starts on the target and ends on the
    /// goal completes a drag, and both jump.
    pub fn drag(seed: u64) -> Self {
        let mut app = Self::dual(seed);
        app.layout.reactive = false;
        app.layout.decoy = None;
        app.layout.decoy_band = 176;
        app.layout.goal = Some(app.random_rect());
        app
    }

    fn bare_layout(seed: u64) -> TargetLayout {
        let _ = seed;
        TargetLayout {
            target: None,
            decoy: None,
            goal: None,
            target_band: 230,
            decoy_band: 176,
            goal_band: 176,
            reactive: true,
            visible: true,
        }
    }

    /// Same rectangles and reactions, drawn at zero contrast.
    pub fn blind(mut self) -> Self {
        self.layout.visible = false;
        self
    }

    pub fn layout(&self) -> TargetLayout {
        self.layout
    }

    pub const fn taps(&self) -> u32 {
        self.taps
    }

    pub const fn target_taps(&self) -> u32 {
        self.target_taps
    }

    pub const fn decoy_taps(&self) -> u32 {
        self.decoy_taps
    }

    pub const fn hits(&self) -> u32 {
        self.hits
    }

    pub const fn ab_pairs(&self) -> u32 {
        self.ab_pairs
    }

    pub const fn ba_pairs(&self) -> u32 {
        self.ba_pairs
    }

    pub const fn drag_attempts(&self) -> u32 {
        self.drag_attempts
    }

    pub const fn drag_hits(&self) -> u32 {
        self.drag_hits
    }

    /// Share of the screen a blind tap lands on the target.
    pub fn chance(&self) -> f64 {
        self.layout
            .target
            .map_or(0.0, |rect| f64::from(rect.area()) / f64::from(SCREEN_AREA))
    }

    /// Share of the screen a blind release lands on the drag goal.
    pub fn goal_chance(&self) -> f64 {
        self.layout
            .goal
            .map_or(0.0, |rect| f64::from(rect.area()) / f64::from(SCREEN_AREA))
    }

    pub(crate) fn apply(&mut self, events: &[DeviceEvent]) {
        for event in events {
            match *event {
                DeviceEvent::TouchStarted { touch, at } => {
                    self.started[touch.index()] = Some(at);
                }
                DeviceEvent::TouchMoved { .. } => {}
                DeviceEvent::TouchEnded { touch, at } => {
                    let Some(start) = self.started[touch.index()].take() else {
                        continue;
                    };
                    // The drag toy: a touch that starts on the target,
                    // however far it travels. Releasing on the goal
                    // completes a drag; both jump.
                    if self.layout.goal.is_some() {
                        if self.layout.target.is_some_and(|rect| rect.contains(start)) {
                            self.drag_attempts += 1;
                            if self.layout.goal.is_some_and(|rect| rect.contains(at)) {
                                self.drag_hits += 1;
                                self.hits += 1;
                                self.layout.target = Some(self.random_rect());
                                self.layout.goal = Some(self.random_rect());
                            }
                        }
                        continue;
                    }
                    if distance(start, at) > TAP_TRAVEL {
                        continue;
                    }
                    self.taps += 1;
                    let on_a = self.layout.target.is_some_and(|rect| rect.contains(at));
                    let on_b = self.layout.decoy.is_some_and(|rect| rect.contains(at));
                    if on_b {
                        self.decoy_taps += 1;
                    }
                    if on_a {
                        self.target_taps += 1;
                        if self.layout.reactive {
                            self.hits += 1;
                            if self.dies_after.is_some_and(|limit| self.hits >= limit) {
                                self.layout.reactive = false;
                            } else {
                                self.move_target();
                            }
                        }
                    }
                    if self.rewarded.is_some() {
                        match (on_a, on_b) {
                            (true, _) => {
                                if self.last_tapped == Some(false) {
                                    self.ba_pairs += 1;
                                    if self.rewarded == Some(Rewarded::Ba) {
                                        self.rewarded_pair();
                                    }
                                }
                                self.last_tapped = Some(true);
                            }
                            (false, true) => {
                                if self.last_tapped == Some(true) {
                                    self.ab_pairs += 1;
                                    if self.rewarded == Some(Rewarded::Ab) {
                                        self.rewarded_pair();
                                    }
                                }
                                self.last_tapped = Some(false);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    fn rewarded_pair(&mut self) {
        self.hits += 1;
        self.layout.target = Some(self.random_rect());
        self.layout.decoy = Some(self.random_rect());
    }

    fn move_target(&mut self) {
        let mut target = self.random_rect();
        if self.side < 512 {
            while self
                .layout
                .decoy
                .is_some_and(|decoy| target.overlaps(decoy))
                || self.layout.goal.is_some_and(|goal| target.overlaps(goal))
            {
                target = self.random_rect();
            }
        }
        self.layout.target = Some(target);
    }

    pub(crate) fn frame(&self) -> LightField {
        let mut pixels = background();
        if !self.layout.visible {
            return frame(pixels);
        }
        if let Some(decoy) = self.layout.decoy {
            fill_rect(&mut pixels, decoy, self.layout.decoy_band);
        }
        if let Some(goal) = self.layout.goal {
            fill_rect(&mut pixels, goal, self.layout.goal_band);
        }
        if let Some(target) = self.layout.target {
            fill_rect(&mut pixels, target, self.layout.target_band);
        }
        frame(pixels)
    }

    fn random_rect(&mut self) -> Rect {
        self.rng = self
            .rng
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        let span = i64::from(BODY_MAX - self.side);
        let left = ((self.rng >> 33) as i64 % span) as i16;
        let top = ((self.rng >> 13) as i64 % span) as i16;
        Rect {
            left,
            top,
            right: left + self.side,
            bottom: top + self.side,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TouchId;

    fn tap(app: &mut TargetApp, at: ScreenPoint) {
        let touch = TouchId::new(0).unwrap();
        app.apply(&[
            DeviceEvent::TouchStarted { touch, at },
            DeviceEvent::TouchEnded { touch, at },
        ]);
    }

    fn centre(rect: Rect) -> ScreenPoint {
        ScreenPoint {
            x: (rect.left + rect.right) / 2,
            y: (rect.top + rect.bottom) / 2,
        }
    }

    #[test]
    fn a_tap_inside_the_target_moves_it_and_counts_a_hit() {
        let mut app = TargetApp::lit(7);
        let before = app.layout().target.unwrap();
        tap(&mut app, centre(before));
        assert_eq!((app.taps(), app.target_taps(), app.hits()), (1, 1, 1));
        assert_ne!(app.layout().target.unwrap(), before);
        assert!(app.frame() != TargetApp::lit(7).frame());
    }

    #[test]
    fn a_drag_is_not_a_tap_and_a_miss_is_not_a_hit() {
        let mut app = TargetApp::lit(7);
        let rect = app.layout().target.unwrap();
        let touch = TouchId::new(1).unwrap();
        let start = centre(rect);
        let far = ScreenPoint {
            x: start.x,
            y: start.y.saturating_sub(100).max(0),
        };
        app.apply(&[
            DeviceEvent::TouchStarted { touch, at: start },
            DeviceEvent::TouchMoved {
                touch,
                from: start,
                to: far,
            },
            DeviceEvent::TouchEnded { touch, at: far },
        ]);
        assert_eq!(app.taps(), 0);
        tap(
            &mut app,
            ScreenPoint {
                x: (rect.left + 600) % 1000,
                y: (rect.top + 600) % 1000,
            },
        );
        assert_eq!(app.taps(), 1);
        assert_eq!(app.hits(), 0);
    }

    #[test]
    fn the_blind_control_counts_but_shows_nothing() {
        let mut app = TargetApp::lit(9).blind();
        assert_eq!(app.frame(), frame(background()));
        let rect = app.layout().target.unwrap();
        tap(&mut app, centre(rect));
        assert_eq!((app.target_taps(), app.hits()), (1, 1));
        assert_eq!(app.frame(), frame(background()));
    }

    #[test]
    fn the_dual_pair_is_equally_bright_and_distinct() {
        let app = TargetApp::dual(7);
        let layout = app.layout();
        let target = layout.target.unwrap();
        let decoy = layout.decoy.unwrap();
        assert_eq!(layout.target_band, layout.decoy_band);
        assert_ne!(target, decoy);
        assert!(!target.overlaps(decoy));
        // Only the target reacts.
        let mut app = app;
        let before = app.layout().target.unwrap();
        tap(&mut app, centre(decoy));
        assert_eq!(app.decoy_taps(), 1);
        assert_eq!(app.hits(), 0);
        tap(&mut app, centre(before));
        assert_eq!(app.hits(), 1);
        assert_ne!(app.layout().target.unwrap(), before);
    }

    #[test]
    fn a_live_target_never_moves_over_its_decoy() {
        let mut app = TargetApp::dual(7);
        for _ in 0..64 {
            let target = app.layout().target.unwrap();
            tap(&mut app, centre(target));
            assert!(!app
                .layout()
                .target
                .unwrap()
                .overlaps(app.layout().decoy.unwrap()));
        }
    }

    #[test]
    fn swapping_the_dual_pair_exchanges_roles() {
        let app = TargetApp::dual(7);
        let swapped = app.clone().swapped().layout();
        let plain = app.layout();
        assert_eq!(swapped.target, plain.decoy);
        assert_eq!(swapped.decoy, plain.target);
    }

    #[test]
    fn the_target_goes_dead_in_place_after_its_limit() {
        let mut app = TargetApp::lit(7).dies_after(2);
        let first = app.layout().target.unwrap();
        tap(&mut app, centre(first));
        let second = app.layout().target.unwrap();
        assert_ne!(second, first);
        tap(&mut app, centre(second));
        assert!(!app.layout().reactive);
        assert_eq!(app.layout().target, Some(second));
        // Taps on the dead target still count; nothing reacts.
        tap(&mut app, centre(second));
        assert_eq!(app.target_taps(), 3);
        assert_eq!(app.hits(), 2);
        assert_eq!(app.layout().target, Some(second));
    }

    #[test]
    fn the_sequence_pair_only_rewards_the_required_order() {
        let mut app = TargetApp::sequence(7);
        let layout = app.layout();
        let a = layout.target.unwrap();
        let b = layout.decoy.unwrap();
        let a_centre = centre(a);
        let b_centre = centre(b);
        // B then A is a pair but produces no change.
        tap(&mut app, b_centre);
        let before = app.layout();
        tap(&mut app, a_centre);
        assert_eq!(app.ba_pairs(), 1);
        assert_eq!(app.ab_pairs(), 0);
        assert_eq!(app.hits(), 0);
        assert_eq!(app.layout().target, before.target);
        // A then B is the rewarded order: both jump.
        tap(&mut app, a_centre);
        let before = app.layout();
        tap(&mut app, b_centre);
        assert_eq!(app.ab_pairs(), 1);
        assert_eq!(app.hits(), 1);
        assert_ne!(app.layout().target, before.target);
        assert_ne!(app.layout().decoy, before.decoy);
    }

    #[test]
    fn the_reversed_sequence_rewards_the_other_order() {
        let mut app = TargetApp::sequence(7).reversed();
        let layout = app.layout();
        let a_centre = centre(layout.target.unwrap());
        let b_centre = centre(layout.decoy.unwrap());
        tap(&mut app, b_centre);
        tap(&mut app, a_centre);
        assert_eq!((app.ba_pairs(), app.hits()), (1, 1));
    }

    #[test]
    fn a_drag_from_the_target_to_the_goal_jumps_both() {
        let mut app = TargetApp::drag(7);
        let layout = app.layout();
        let target = layout.target.unwrap();
        let goal = layout.goal.unwrap();
        let touch = TouchId::new(2).unwrap();
        let start = centre(target);
        let end = centre(goal);
        // Releasing outside the goal is an attempt without a hit.
        app.apply(&[
            DeviceEvent::TouchStarted { touch, at: start },
            DeviceEvent::TouchMoved {
                touch,
                from: start,
                to: end,
            },
            DeviceEvent::TouchEnded {
                touch,
                at: ScreenPoint { x: 8, y: 8 },
            },
        ]);
        assert_eq!((app.drag_attempts(), app.drag_hits()), (1, 0));
        app.apply(&[
            DeviceEvent::TouchStarted { touch, at: start },
            DeviceEvent::TouchMoved {
                touch,
                from: start,
                to: end,
            },
            DeviceEvent::TouchEnded { touch, at: end },
        ]);
        assert_eq!((app.drag_attempts(), app.drag_hits()), (2, 1));
        assert_ne!(app.layout().target, Some(target));
        assert_ne!(app.layout().goal, Some(goal));
        // A touch that does not start on the target is not a drag attempt.
        app.apply(&[
            DeviceEvent::TouchStarted { touch, at: end },
            DeviceEvent::TouchEnded { touch, at: end },
        ]);
        assert_eq!(app.drag_attempts(), 2);
    }

    #[test]
    fn the_blank_screen_draws_only_the_background() {
        let app = TargetApp::blank(7);
        assert_eq!(app.frame(), frame(background()));
        assert_eq!(app.chance(), 0.0);
    }
}
