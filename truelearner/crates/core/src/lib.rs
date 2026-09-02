#![deny(unsafe_code)]
//! Executable physical identities and laws shared by the TrueLearner body.

use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;
use vstd::prelude::*;

verus! {

pub type Time = u64;
pub type Impulse = i32;
pub type Cause = u64;
pub const DRIVE_MAX: u16 = 1_023;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(not(verus_only), derive(Serialize, Deserialize))]
pub struct JunctionId(NonZeroU32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(not(verus_only), derive(Serialize, Deserialize))]
pub struct LinkId(NonZeroU32);

} // verus!

impl JunctionId {
    #[doc(hidden)]
    #[inline(always)]
    pub fn new(slot: usize) -> Option<Self> {
        u32::try_from(slot)
            .ok()?
            .checked_add(1)
            .and_then(NonZeroU32::new)
            .map(Self)
    }

    #[doc(hidden)]
    #[inline(always)]
    pub const fn slot(self) -> usize {
        self.0.get() as usize - 1
    }
}

impl LinkId {
    #[doc(hidden)]
    #[inline(always)]
    pub fn new(slot: usize) -> Option<Self> {
        u32::try_from(slot)
            .ok()?
            .checked_add(1)
            .and_then(NonZeroU32::new)
            .map(Self)
    }

    #[doc(hidden)]
    #[inline(always)]
    pub const fn slot(self) -> usize {
        self.0.get() as usize - 1
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(not(verus_only), derive(Serialize, Deserialize))]
pub enum Retention {
    Integrating,
    Sampled { lifetime: Time, range: u32 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(not(verus_only), derive(Serialize, Deserialize))]
pub struct Junction {
    pub threshold: Impulse,
    pub retention: Retention,
}

impl Junction {
    #[inline(always)]
    pub const fn integrating(threshold: Impulse) -> Self {
        Self {
            threshold,
            retention: Retention::Integrating,
        }
    }

    #[inline(always)]
    pub const fn sampled(lifetime: Time) -> Self {
        Self::sampled_in(lifetime, DRIVE_MAX as u32)
    }

    #[inline(always)]
    pub const fn sampled_in(lifetime: Time, range: u32) -> Self {
        Self {
            threshold: 1,
            retention: Retention::Sampled { lifetime, range },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(not(verus_only), derive(Serialize, Deserialize))]
pub enum Trigger {
    SourceFires,
    RisesThrough(Impulse),
    FallsThrough(Impulse),
    Rises,
    Falls,
}

impl Trigger {
    #[inline(always)]
    pub const fn opens(self, before: Impulse, after: Impulse) -> bool {
        match self {
            Self::SourceFires => true,
            Self::Rises => after > before,
            Self::Falls => after < before,
            Self::RisesThrough(level) => before < level && after >= level,
            Self::FallsThrough(level) => before > level && after <= level,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(not(verus_only), derive(Serialize, Deserialize))]
pub struct Link {
    pub from: JunctionId,
    pub to: JunctionId,
    pub delay: Time,
    pub impulse: Impulse,
    pub trigger: Trigger,
}

impl Link {
    #[inline(always)]
    pub const fn new(from: JunctionId, to: JunctionId, delay: Time, impulse: Impulse) -> Self {
        Self {
            from,
            to,
            delay,
            impulse,
            trigger: Trigger::SourceFires,
        }
    }

    #[inline(always)]
    pub const fn when(mut self, trigger: Trigger) -> Self {
        self.trigger = trigger;
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(not(verus_only), derive(Serialize, Deserialize))]
pub struct Path {
    pub surface: JunctionId,
    pub middle: JunctionId,
    pub output: JunctionId,
    pub first: LinkId,
    pub second: LinkId,
}

impl Path {
    #[inline(always)]
    pub const fn links(self) -> [LinkId; 2] {
        [self.first, self.second]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(not(verus_only), derive(Serialize, Deserialize))]
pub struct Occurrence {
    pub cause: Cause,
    pub at: Time,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(not(verus_only), derive(Serialize, Deserialize))]
pub struct Outcome {
    pub at: Time,
    pub caused_transition: bool,
    pub available_until_choice: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(not(verus_only), derive(Serialize, Deserialize))]
pub struct PathEvidence {
    participation: u64,
    last_participation: Occurrence,
    outcome_at: Time,
    boundary_closed: bool,
    boundary_inhibited: bool,
    outcome_present: bool,
    outcome_caused_transition: bool,
    outcome_available: bool,
    exact_closures: u8,
    strength: i64,
}

impl Default for PathEvidence {
    fn default() -> Self {
        Self {
            participation: 0,
            last_participation: Occurrence { cause: 0, at: 0 },
            outcome_at: 0,
            boundary_closed: false,
            boundary_inhibited: false,
            outcome_present: false,
            outcome_caused_transition: false,
            outcome_available: false,
            exact_closures: 0,
            strength: 1,
        }
    }
}

impl PathEvidence {
    #[inline(always)]
    pub const fn participation(&self) -> u64 {
        self.participation
    }

    #[inline(always)]
    pub const fn last_participation(&self) -> Option<Occurrence> {
        if self.participation == 0 {
            None
        } else {
            Some(self.last_participation)
        }
    }

    #[inline(always)]
    pub const fn outcome(&self) -> Option<Outcome> {
        if self.outcome_present {
            Some(Outcome {
                at: self.outcome_at,
                caused_transition: self.outcome_caused_transition,
                available_until_choice: self.outcome_available,
            })
        } else {
            None
        }
    }

    #[inline(always)]
    pub const fn boundary_closed(&self) -> bool {
        self.boundary_closed
    }

    #[inline(always)]
    pub const fn boundary_inhibited(&self) -> bool {
        self.boundary_inhibited
    }

    #[inline(always)]
    pub const fn exact_closures(&self) -> u8 {
        self.exact_closures
    }

    #[inline(always)]
    pub const fn strength(&self) -> i64 {
        self.strength
    }

    #[inline(always)]
    pub fn participate(&mut self, occurrence: Occurrence) -> bool {
        let first = self.participation == 0;
        self.participation = self.participation.saturating_add(1);
        self.last_participation = occurrence;
        self.boundary_closed = false;
        first
    }

    #[inline(always)]
    pub fn remember_outcome(&mut self, outcome: Outcome) {
        self.outcome_at = outcome.at;
        self.outcome_present = true;
        self.outcome_caused_transition = outcome.caused_transition;
        self.outcome_available = outcome.available_until_choice;
    }

    #[inline(always)]
    pub fn consume_outcome(&mut self) {
        if self.outcome_present {
            self.outcome_available = false;
        }
    }

    #[inline(always)]
    pub fn clear_outcome(&mut self) {
        self.outcome_present = false;
        self.outcome_available = false;
    }

    #[inline(always)]
    pub fn close_boundary(&mut self) {
        self.boundary_closed = true;
    }

    #[inline(always)]
    pub fn inhibit_boundary(&mut self) {
        self.boundary_inhibited = true;
    }

    #[inline(always)]
    pub fn consume_boundary_inhibition(&mut self) {
        self.boundary_inhibited = false;
    }

    #[inline(always)]
    pub fn increment_exact_closures(&mut self) -> u8 {
        self.exact_closures = self.exact_closures.saturating_add(1);
        self.exact_closures
    }

    #[inline(always)]
    pub fn strengthen(&mut self, amount: i64) -> (i64, i64) {
        let before = self.strength;
        self.strength = self.strength.saturating_add(amount);
        (before, self.strength)
    }

    fn learn_closure(&mut self, at: Time, offers_choice: bool, exact: bool) -> (u8, i64, i64) {
        self.remember_outcome(Outcome {
            at,
            caused_transition: true,
            available_until_choice: offers_choice,
        });
        if !offers_choice {
            self.close_boundary();
        }
        if exact {
            self.increment_exact_closures();
        }
        let (before, after) = self.strengthen(1);
        (self.exact_closures, before, after)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(not(verus_only), derive(Serialize, Deserialize))]
pub enum PropagationMode {
    Entry,
    Drive {
        boundary_crossing: bool,
        locally_plastic: bool,
        factors: Option<[LinkId; 2]>,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(not(verus_only), derive(Serialize, Deserialize))]
pub enum WitnessKind {
    Progress,
    Closure { offers_choice: bool },
}

verus! {

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(not(verus_only), derive(Serialize, Deserialize))]
pub struct ClosedSupport {
    pub source: JunctionId,
    pub witness: LinkId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(not(verus_only), derive(Serialize, Deserialize))]
pub enum ReturnStatus {
    Open {
        switched_from: Option<LinkId>,
    },
    Closed {
        at: Time,
        support: ClosedSupport,
        motif_parent: Option<LinkId>,
    },
    Ambiguous {
        at: Time,
    },
    Expired,
}

pub open spec fn return_is_open(status: ReturnStatus) -> bool {
    status is Open
}

pub open spec fn return_is_closed_with(status: ReturnStatus, expected: ClosedSupport) -> bool {
    match status {
        ReturnStatus::Closed { support, .. } => support == expected,
        _ => false,
    }
}

pub open spec fn return_is_ambiguous_at(status: ReturnStatus, expected_at: Time) -> bool {
    match status {
        ReturnStatus::Ambiguous { at } => at == expected_at,
        _ => false,
    }
}

pub open spec fn return_is_expired(status: ReturnStatus) -> bool {
    status is Expired
}

pub open spec fn return_has_no_support(status: ReturnStatus) -> bool {
    !(status is Closed)
}

#[inline(always)]
fn close_return_transition(
    active: &mut bool,
    status: &mut ReturnStatus,
    at: Time,
    support: ClosedSupport,
    motif_parent: Option<LinkId>,
) -> (changed: bool)
    ensures
        changed == return_is_open(*old(status)),
        changed ==> return_is_closed_with(*final(status), support),
        changed ==> *final(active),
        !changed ==> *final(status) == *old(status),
        !changed ==> *final(active) == *old(active),
{
    if !matches!(status, ReturnStatus::Open { .. }) {
        return false;
    }
    *status = ReturnStatus::Closed {
        at,
        support,
        motif_parent,
    };
    *active = true;
    true
}

#[inline(always)]
fn mark_ambiguous_transition(
    active: &mut bool,
    status: &mut ReturnStatus,
    at: Time,
) -> (changed: bool)
    ensures
        changed == return_is_open(*old(status)),
        changed ==> return_is_ambiguous_at(*final(status), at),
        changed ==> return_has_no_support(*final(status)),
        changed ==> !*final(active),
        !changed ==> *final(status) == *old(status),
        !changed ==> *final(active) == *old(active),
{
    if !matches!(status, ReturnStatus::Open { .. }) {
        return false;
    }
    *status = ReturnStatus::Ambiguous { at };
    *active = false;
    true
}

#[inline(always)]
fn expire_return_transition(
    active: &mut bool,
    status: &mut ReturnStatus,
) -> (changed: bool)
    ensures
        changed == return_is_open(*old(status)),
        changed ==> return_is_expired(*final(status)),
        changed ==> return_has_no_support(*final(status)),
        changed ==> !*final(active),
        !changed ==> *final(status) == *old(status),
        !changed ==> *final(active) == *old(active),
{
    if !matches!(status, ReturnStatus::Open { .. }) {
        return false;
    }
    *status = ReturnStatus::Expired;
    *active = false;
    true
}

#[cfg(verus_only)]
fn verify_terminal_returns_are_absorbing(
    mut active: bool,
    mut status: ReturnStatus,
    at: Time,
    support: ClosedSupport,
    motif_parent: Option<LinkId>,
)
    requires
        return_is_ambiguous_at(status, at) || return_is_expired(status),
    ensures
        return_has_no_support(status),
{
    let closed = close_return_transition(
        &mut active,
        &mut status,
        at,
        support,
        motif_parent,
    );
    assert(!closed);
    let ambiguous = mark_ambiguous_transition(&mut active, &mut status, at);
    assert(!ambiguous);
    let expired = expire_return_transition(&mut active, &mut status);
    assert(!expired);
}

} // verus!

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(not(verus_only), derive(Serialize, Deserialize))]
pub enum ArrowKind {
    Propagation {
        mode: PropagationMode,
        last_transmission: Option<Occurrence>,
        evidence: PathEvidence,
    },
    Witness {
        kind: WitnessKind,
        last_transmission: Option<Occurrence>,
    },
    Return {
        path: Path,
        cause: Cause,
        opened_at: Time,
        status: ReturnStatus,
    },
    Membership,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(not(verus_only), derive(Serialize, Deserialize))]
pub struct ArrowState {
    active: bool,
    kind: ArrowKind,
}

impl Default for ArrowState {
    fn default() -> Self {
        Self::drive()
    }
}

impl ArrowState {
    #[inline(always)]
    pub const fn drive() -> Self {
        Self {
            active: true,
            kind: ArrowKind::Propagation {
                mode: PropagationMode::Drive {
                    boundary_crossing: false,
                    locally_plastic: false,
                    factors: None,
                },
                last_transmission: None,
                evidence: PathEvidence {
                    participation: 0,
                    last_participation: Occurrence { cause: 0, at: 0 },
                    outcome_at: 0,
                    boundary_closed: false,
                    boundary_inhibited: false,
                    outcome_present: false,
                    outcome_caused_transition: false,
                    outcome_available: false,
                    exact_closures: 0,
                    strength: 1,
                },
            },
        }
    }

    #[inline(always)]
    pub const fn entry() -> Self {
        Self {
            active: true,
            kind: ArrowKind::Propagation {
                mode: PropagationMode::Entry,
                last_transmission: None,
                evidence: PathEvidence {
                    participation: 0,
                    last_participation: Occurrence { cause: 0, at: 0 },
                    outcome_at: 0,
                    boundary_closed: false,
                    boundary_inhibited: false,
                    outcome_present: false,
                    outcome_caused_transition: false,
                    outcome_available: false,
                    exact_closures: 0,
                    strength: 1,
                },
            },
        }
    }

    #[inline(always)]
    pub const fn witness(kind: WitnessKind) -> Self {
        Self {
            active: true,
            kind: ArrowKind::Witness {
                kind,
                last_transmission: None,
            },
        }
    }

    #[inline(always)]
    pub const fn open_return(path: Path, cause: Cause, opened_at: Time) -> Self {
        Self {
            active: true,
            kind: ArrowKind::Return {
                path,
                cause,
                opened_at,
                status: ReturnStatus::Open {
                    switched_from: None,
                },
            },
        }
    }

    #[inline(always)]
    pub const fn membership() -> Self {
        Self {
            active: true,
            kind: ArrowKind::Membership,
        }
    }

    #[inline(always)]
    pub const fn active(&self) -> bool {
        self.active
    }

    #[inline(always)]
    pub const fn kind(&self) -> ArrowKind {
        self.kind
    }

    #[inline(always)]
    pub const fn is_drive(&self) -> bool {
        self.active
            && matches!(
                self.kind,
                ArrowKind::Propagation {
                    mode: PropagationMode::Drive { .. },
                    ..
                }
            )
    }

    #[inline(always)]
    pub const fn is_entry(&self) -> bool {
        self.active
            && matches!(
                self.kind,
                ArrowKind::Propagation {
                    mode: PropagationMode::Entry,
                    ..
                }
            )
    }

    #[inline(always)]
    pub const fn witness_kind(&self) -> Option<WitnessKind> {
        if !self.active {
            return None;
        }
        match self.kind {
            ArrowKind::Witness { kind, .. } => Some(kind),
            _ => None,
        }
    }

    #[inline(always)]
    pub const fn is_membership(&self) -> bool {
        self.active && matches!(self.kind, ArrowKind::Membership)
    }

    #[inline(always)]
    pub const fn propagation_mode(&self) -> Option<PropagationMode> {
        match self.kind {
            ArrowKind::Propagation { mode, .. } if self.active => Some(mode),
            _ => None,
        }
    }

    #[inline(always)]
    pub const fn evidence(&self) -> Option<&PathEvidence> {
        match &self.kind {
            ArrowKind::Propagation { evidence, .. } => Some(evidence),
            _ => None,
        }
    }

    #[inline(always)]
    pub const fn transmitted(&self) -> bool {
        self.last_transmission().is_some()
    }

    #[inline(always)]
    pub const fn participation(&self) -> u64 {
        match self.evidence() {
            Some(evidence) => evidence.participation(),
            None => 0,
        }
    }

    #[inline(always)]
    pub const fn locally_plastic(&self) -> bool {
        matches!(
            self.kind,
            ArrowKind::Propagation {
                mode: PropagationMode::Drive {
                    locally_plastic: true,
                    ..
                },
                ..
            }
        )
    }

    #[inline(always)]
    pub fn mark_locally_plastic(&mut self) -> bool {
        let ArrowKind::Propagation {
            mode: PropagationMode::Drive {
                locally_plastic, ..
            },
            ..
        } = &mut self.kind
        else {
            return false;
        };
        *locally_plastic = true;
        true
    }

    #[inline(always)]
    pub const fn occurrence(&self) -> Option<Occurrence> {
        let transmitted = self.last_transmission();
        let participated = match self.evidence() {
            Some(evidence) => evidence.last_participation(),
            None => None,
        };
        match (transmitted, participated) {
            (Some(left), Some(right)) if left.at > right.at => Some(left),
            (_, Some(right)) => Some(right),
            (Some(left), None) => Some(left),
            (None, None) => None,
        }
    }

    #[inline(always)]
    pub const fn outcome(&self) -> Option<Outcome> {
        match self.evidence() {
            Some(evidence) => evidence.outcome(),
            None => None,
        }
    }

    #[inline(always)]
    pub const fn boundary_closed(&self) -> bool {
        match self.evidence() {
            Some(evidence) => evidence.boundary_closed(),
            None => false,
        }
    }

    #[inline(always)]
    pub const fn boundary_inhibited(&self) -> bool {
        match self.evidence() {
            Some(evidence) => evidence.boundary_inhibited(),
            None => false,
        }
    }

    #[inline(always)]
    pub const fn exact_closures(&self) -> u8 {
        match self.evidence() {
            Some(evidence) => evidence.exact_closures(),
            None => 0,
        }
    }

    #[inline(always)]
    pub const fn strength(&self) -> i64 {
        match self.evidence() {
            Some(evidence) => evidence.strength(),
            None => 1,
        }
    }

    #[inline(always)]
    pub fn participate(&mut self, occurrence: Occurrence) -> bool {
        self.evidence_mut()
            .is_some_and(|evidence| evidence.participate(occurrence))
    }

    #[inline(always)]
    pub fn remember_outcome(&mut self, outcome: Outcome) -> bool {
        let Some(evidence) = self.evidence_mut() else {
            return false;
        };
        evidence.remember_outcome(outcome);
        true
    }

    #[inline(always)]
    pub fn consume_outcome(&mut self) -> bool {
        let Some(evidence) = self.evidence_mut() else {
            return false;
        };
        evidence.consume_outcome();
        true
    }

    #[inline(always)]
    pub fn clear_outcome(&mut self) -> bool {
        let Some(evidence) = self.evidence_mut() else {
            return false;
        };
        evidence.clear_outcome();
        true
    }

    #[inline(always)]
    pub fn close_boundary(&mut self) -> bool {
        let Some(evidence) = self.evidence_mut() else {
            return false;
        };
        evidence.close_boundary();
        true
    }

    #[inline(always)]
    pub fn inhibit_boundary(&mut self) -> bool {
        let Some(evidence) = self.evidence_mut() else {
            return false;
        };
        evidence.inhibit_boundary();
        true
    }

    #[inline(always)]
    pub fn consume_boundary_inhibition(&mut self) -> bool {
        let Some(evidence) = self.evidence_mut() else {
            return false;
        };
        evidence.consume_boundary_inhibition();
        true
    }

    #[inline(always)]
    pub fn increment_exact_closures(&mut self) -> Option<u8> {
        self.evidence_mut()
            .map(PathEvidence::increment_exact_closures)
    }

    #[inline(always)]
    pub fn strengthen(&mut self, amount: i64) -> Option<(i64, i64)> {
        self.evidence_mut()
            .map(|evidence| evidence.strengthen(amount))
    }

    #[inline(always)]
    pub fn learn_closure(
        &mut self,
        at: Time,
        offers_choice: bool,
        exact: bool,
    ) -> Option<(u8, i64, i64)> {
        self.evidence_mut()
            .map(|evidence| evidence.learn_closure(at, offers_choice, exact))
    }

    #[inline(always)]
    pub fn evidence_mut(&mut self) -> Option<&mut PathEvidence> {
        match &mut self.kind {
            ArrowKind::Propagation { evidence, .. } => Some(evidence),
            _ => None,
        }
    }

    #[inline(always)]
    pub const fn last_transmission(&self) -> Option<Occurrence> {
        match self.kind {
            ArrowKind::Propagation {
                last_transmission, ..
            }
            | ArrowKind::Witness {
                last_transmission, ..
            } => last_transmission,
            _ => None,
        }
    }

    #[inline(always)]
    pub fn record_transmission(&mut self, occurrence: Occurrence) {
        match &mut self.kind {
            ArrowKind::Propagation {
                last_transmission, ..
            }
            | ArrowKind::Witness {
                last_transmission, ..
            } => *last_transmission = Some(occurrence),
            _ => {}
        }
    }

    #[inline(always)]
    pub const fn boundary_crossing(&self) -> bool {
        matches!(
            self.kind,
            ArrowKind::Propagation {
                mode: PropagationMode::Drive {
                    boundary_crossing: true,
                    ..
                },
                ..
            }
        )
    }

    #[inline(always)]
    pub fn mark_boundary_crossing(&mut self) -> bool {
        let ArrowKind::Propagation {
            mode: PropagationMode::Drive {
                boundary_crossing, ..
            },
            ..
        } = &mut self.kind
        else {
            return false;
        };
        *boundary_crossing = true;
        true
    }

    #[inline(always)]
    pub const fn factors(&self) -> Option<[LinkId; 2]> {
        match self.kind {
            ArrowKind::Propagation {
                mode: PropagationMode::Drive { factors, .. },
                ..
            } => factors,
            _ => None,
        }
    }

    #[inline(always)]
    pub fn retain_factors(&mut self, factors: [LinkId; 2]) -> bool {
        let ArrowKind::Propagation {
            mode: PropagationMode::Drive { factors: slot, .. },
            ..
        } = &mut self.kind
        else {
            return false;
        };
        *slot = Some(factors);
        true
    }

    #[inline(always)]
    pub const fn open_return_data(&self) -> Option<(Path, Cause, Time, Option<LinkId>)> {
        match self.kind {
            ArrowKind::Return {
                path,
                cause,
                opened_at,
                status: ReturnStatus::Open { switched_from },
            } if self.active => Some((path, cause, opened_at, switched_from)),
            _ => None,
        }
    }

    #[inline(always)]
    pub const fn closed_support(&self) -> Option<ClosedSupport> {
        match self.kind {
            ArrowKind::Return {
                status: ReturnStatus::Closed { support, .. },
                ..
            } if self.active => Some(support),
            _ => None,
        }
    }

    #[inline(always)]
    pub const fn motif_parent(&self) -> Option<LinkId> {
        match self.kind {
            ArrowKind::Return {
                status: ReturnStatus::Closed { motif_parent, .. },
                ..
            } => motif_parent,
            _ => None,
        }
    }

    #[inline(always)]
    pub fn remember_switched_from(&mut self, prior: LinkId) -> bool {
        let ArrowKind::Return {
            status: ReturnStatus::Open { switched_from },
            ..
        } = &mut self.kind
        else {
            return false;
        };
        *switched_from = Some(prior);
        true
    }

    #[inline(always)]
    pub const fn switched_from(&self) -> Option<LinkId> {
        match self.kind {
            ArrowKind::Return {
                status: ReturnStatus::Open { switched_from },
                ..
            } if self.active => switched_from,
            _ => None,
        }
    }

    #[inline(always)]
    pub fn close_return(
        &mut self,
        at: Time,
        support: ClosedSupport,
        motif_parent: Option<LinkId>,
    ) -> bool {
        let ArrowKind::Return { status, .. } = &mut self.kind else {
            return false;
        };
        close_return_transition(&mut self.active, status, at, support, motif_parent)
    }

    #[inline(always)]
    pub fn mark_ambiguous(&mut self, at: Time) -> bool {
        let ArrowKind::Return { status, .. } = &mut self.kind else {
            return false;
        };
        mark_ambiguous_transition(&mut self.active, status, at)
    }

    #[inline(always)]
    pub fn expire_return(&mut self) -> bool {
        let ArrowKind::Return { status, .. } = &mut self.kind else {
            return false;
        };
        expire_return_transition(&mut self.active, status)
    }

    #[inline(always)]
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    #[inline(always)]
    pub fn remap(&mut self, junction_base: usize, link_base: usize) {
        let remap_junction = |junction: JunctionId| {
            JunctionId::new(junction_base + junction.slot())
                .expect("validated attachment junction identity")
        };
        let remap_link = |link: LinkId| {
            LinkId::new(link_base + link.slot()).expect("validated attachment link identity")
        };
        match &mut self.kind {
            ArrowKind::Propagation {
                mode: PropagationMode::Drive { factors, .. },
                ..
            } => {
                if let Some([first, second]) = factors {
                    *first = remap_link(*first);
                    *second = remap_link(*second);
                }
            }
            ArrowKind::Return { path, status, .. } => {
                path.surface = remap_junction(path.surface);
                path.middle = remap_junction(path.middle);
                path.output = remap_junction(path.output);
                path.first = remap_link(path.first);
                path.second = remap_link(path.second);
                match status {
                    ReturnStatus::Open { switched_from } => {
                        *switched_from = switched_from.map(remap_link);
                    }
                    ReturnStatus::Closed {
                        support,
                        motif_parent,
                        ..
                    } => {
                        support.source = remap_junction(support.source);
                        support.witness = remap_link(support.witness);
                        *motif_parent = motif_parent.map(remap_link);
                    }
                    ReturnStatus::Ambiguous { .. } | ReturnStatus::Expired => {}
                }
            }
            ArrowKind::Propagation { .. } | ArrowKind::Witness { .. } | ArrowKind::Membership => {}
        }
    }
}

#[doc(hidden)]
pub const fn opens(trigger: Trigger, before: Impulse, after: Impulse) -> bool {
    trigger.opens(before, after)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn path() -> Path {
        Path {
            surface: JunctionId::new(0).unwrap(),
            middle: JunctionId::new(1).unwrap(),
            output: JunctionId::new(2).unwrap(),
            first: LinkId::new(0).unwrap(),
            second: LinkId::new(1).unwrap(),
        }
    }

    #[test]
    fn return_history_closes_once_from_open() {
        let mut returned = ArrowState::open_return(path(), 7, 11);
        let support = ClosedSupport {
            source: JunctionId::new(3).unwrap(),
            witness: LinkId::new(2).unwrap(),
        };

        assert!(returned.close_return(12, support, None));
        assert_eq!(returned.closed_support(), Some(support));
        assert!(!returned.close_return(13, support, None));
        assert!(!returned.mark_ambiguous(13));
        assert!(!returned.expire_return());
    }

    #[test]
    fn ambiguous_and_expired_returns_retain_no_support() {
        let mut ambiguous = ArrowState::open_return(path(), 7, 11);
        let mut expired = ArrowState::open_return(path(), 7, 11);

        assert!(ambiguous.mark_ambiguous(12));
        assert!(expired.expire_return());
        assert_eq!(ambiguous.closed_support(), None);
        assert_eq!(expired.closed_support(), None);
        assert!(!ambiguous.active());
        assert!(!expired.active());
    }

    #[test]
    fn factorization_does_not_change_a_drive_into_another_arrow_kind() {
        let mut drive = ArrowState::drive();
        let factors = [LinkId::new(0).unwrap(), LinkId::new(1).unwrap()];

        assert!(drive.retain_factors(factors));
        assert!(drive.is_drive());
        assert_eq!(drive.factors(), Some(factors));
    }
}
