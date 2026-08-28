#![forbid(unsafe_code)]

use academy_workstation::{
    CONTACT_DEPTH, SessionObservation, WorkstationRecording, WorkstationWorld, WorldError,
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use truelearner_workstation::{BodyAxis, Digit, HandPoint, WorkstationState};

pub const EVIDENCE_STEPS: usize = 120;
pub const EVIDENCE_SEED: u64 = 82_001;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct PointEvidence {
    x: i16,
    y: i16,
    depth: i16,
}

impl From<HandPoint> for PointEvidence {
    fn from(point: HandPoint) -> Self {
        Self {
            x: point.x(),
            y: point.y(),
            depth: point.depth(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct SiteReachSummary {
    site: &'static str,
    max_depth: i16,
    min_signed_depth_gap_while_over_surface: Option<i16>,
    surface_entries: u64,
    positive_contact_samples: u64,
    max_pressure: u16,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct SurfaceEntryEvidence {
    sequence: u64,
    site: &'static str,
    before: PointEvidence,
    after: PointEvidence,
    next_sample_sequence: Option<u64>,
    next_pressure: Option<u16>,
    matched_local_contact: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ContactObservation {
    sequence: u64,
    site: &'static str,
    pressure: u16,
    slip: i16,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ContactContingencyEvidence {
    schema: &'static str,
    outcome: &'static str,
    first_failure: Option<&'static str>,
    seed: u64,
    steps: usize,
    contact_depth: i16,
    recording_sha256: String,
    exact_replay: bool,
    naturally_quiescent: bool,
    max_step_work: u64,
    output_crossings: u64,
    returned_transition_count: u64,
    device_event_count: u64,
    isolated_finger_steps: u64,
    five_finger_steps: u64,
    moved_fingers: Vec<&'static str>,
    first_contact: Option<ContactObservation>,
    sites: Vec<SiteReachSummary>,
    surface_entries: Vec<SurfaceEntryEvidence>,
}

pub fn capture_complete_parent()
-> Result<(ContactContingencyEvidence, WorkstationRecording), WorldError> {
    capture(EVIDENCE_SEED, EVIDENCE_STEPS)
}

pub fn capture(
    seed: u64,
    steps: usize,
) -> Result<(ContactContingencyEvidence, WorkstationRecording), WorldError> {
    let recording = WorkstationRecording::capture(seed, steps)?;
    let bytes = recording.canonical_bytes()?;
    let evidence = project_with(
        seed,
        recording.steps().len(),
        hex_digest(&bytes),
        true,
        |index| &recording.steps()[index].observation,
    )?;
    Ok((evidence, recording))
}

pub fn project_observations(
    seed: u64,
    trace_sha256: String,
    observations: &[SessionObservation],
    exact_replay: bool,
) -> Result<ContactContingencyEvidence, WorldError> {
    project_with(
        seed,
        observations.len(),
        trace_sha256,
        exact_replay,
        |index| &observations[index],
    )
}

fn project_with<'a, F>(
    seed: u64,
    steps: usize,
    trace_sha256: String,
    exact_replay: bool,
    observation_at: F,
) -> Result<ContactContingencyEvidence, WorldError>
where
    F: Fn(usize) -> &'a SessionObservation,
{
    if steps == 0 {
        return Err(WorldError::InvalidRecording);
    }
    let geometry = WorkstationWorld::new()?.geometry().clone();
    let mut sites = site_points(&observation_at(0).body.state_before)
        .into_iter()
        .map(|(site, point)| SiteReachSummary {
            site,
            max_depth: point.depth(),
            min_signed_depth_gap_while_over_surface: over_surface_xy(&geometry, point)
                .then_some(CONTACT_DEPTH.saturating_sub(point.depth())),
            surface_entries: 0,
            positive_contact_samples: 0,
            max_pressure: 0,
        })
        .collect::<Vec<_>>();
    let mut surface_entries = Vec::new();
    let mut first_contact = None;
    let mut moved_fingers = BTreeSet::new();
    let mut isolated_finger_steps = 0_u64;
    let mut five_finger_steps = 0_u64;
    let mut output_crossings = 0_u64;
    let mut returned_transition_count = 0_u64;
    let mut device_event_count = 0_u64;
    let mut max_step_work = 0_u64;
    let mut naturally_quiescent = true;

    for step_index in 0..steps {
        let observation = observation_at(step_index);
        naturally_quiescent &= observation.body.naturally_quiescent;
        max_step_work = max_step_work.max(observation.body.metrics.physical_work);
        output_crossings =
            output_crossings.saturating_add(as_u64(observation.body.crossings.len()));
        returned_transition_count = returned_transition_count
            .saturating_add(as_u64(observation.body.returned_transitions.len()));
        device_event_count =
            device_event_count.saturating_add(as_u64(observation.device_events.len()));

        let changed_fingers = observation
            .body
            .movements
            .iter()
            .filter_map(|movement| match movement.axis {
                BodyAxis::FingerFlexion { digit } if movement.changed => Some(digit),
                _ => None,
            })
            .collect::<Vec<_>>();
        isolated_finger_steps += u64::from(changed_fingers.len() == 1);
        five_finger_steps += u64::from(changed_fingers.len() == 5);
        moved_fingers.extend(changed_fingers.into_iter().map(digit_name));

        let before = site_points(&observation.body.state_before);
        let after = site_points(&observation.body.state_after);
        for site_index in 0..sites.len() {
            let (site, before_point) = before[site_index];
            let (_, after_point) = after[site_index];
            let summary = &mut sites[site_index];
            summary.max_depth = summary.max_depth.max(after_point.depth());
            if over_surface_xy(&geometry, after_point) {
                let gap = CONTACT_DEPTH.saturating_sub(after_point.depth());
                summary.min_signed_depth_gap_while_over_surface = Some(
                    summary
                        .min_signed_depth_gap_while_over_surface
                        .map_or(gap, |prior| prior.min(gap)),
                );
            }

            let contact = observation.sample.contacts()[site_index];
            if contact.pressure() > 0 {
                summary.positive_contact_samples =
                    summary.positive_contact_samples.saturating_add(1);
                summary.max_pressure = summary.max_pressure.max(contact.pressure());
                if first_contact.is_none() {
                    first_contact = Some(ContactObservation {
                        sequence: observation.sequence,
                        site,
                        pressure: contact.pressure(),
                        slip: contact.slip(),
                    });
                }
            }

            if !on_surface(&geometry, before_point) && on_surface(&geometry, after_point) {
                summary.surface_entries = summary.surface_entries.saturating_add(1);
                let next = (step_index + 1 < steps).then(|| observation_at(step_index + 1));
                let next_pressure =
                    next.map(|observation| observation.sample.contacts()[site_index].pressure());
                surface_entries.push(SurfaceEntryEvidence {
                    sequence: observation.sequence,
                    site,
                    before: before_point.into(),
                    after: after_point.into(),
                    next_sample_sequence: next.map(|observation| observation.sequence),
                    next_pressure,
                    matched_local_contact: next_pressure.is_some_and(|pressure| pressure > 0),
                });
            }
        }
    }

    let matched_entry = surface_entries
        .iter()
        .any(|entry| entry.matched_local_contact);
    let entry_with_following_sample = surface_entries
        .iter()
        .any(|entry| entry.next_sample_sequence.is_some());
    let (outcome, first_failure) = if matched_entry {
        ("contact-established", None)
    } else if entry_with_following_sample {
        ("contact-sensation-wall", Some("local-next-sample-contact"))
    } else if surface_entries.is_empty() {
        ("surface-entry-wall", Some("unguided-surface-entry"))
    } else {
        ("entry-at-horizon", Some("next-sample-observation"))
    };

    Ok(ContactContingencyEvidence {
        schema: "workstation-contact-contingency/v1",
        outcome,
        first_failure,
        seed,
        steps,
        contact_depth: CONTACT_DEPTH,
        recording_sha256: trace_sha256,
        exact_replay,
        naturally_quiescent,
        max_step_work,
        output_crossings,
        returned_transition_count,
        device_event_count,
        isolated_finger_steps,
        five_finger_steps,
        moved_fingers: Digit::ALL
            .into_iter()
            .map(digit_name)
            .filter(|digit| moved_fingers.contains(digit))
            .collect(),
        first_contact,
        sites,
        surface_entries,
    })
}

fn site_points(state: &WorkstationState) -> Vec<(&'static str, HandPoint)> {
    let mut points = Vec::with_capacity(6);
    points.push(("palm", state.hand().palm()));
    points.extend(
        Digit::ALL
            .into_iter()
            .map(|digit| (digit_name(digit), state.hand().fingertip(digit))),
    );
    points
}

fn over_surface_xy(geometry: &academy_workstation::WorldGeometry, point: HandPoint) -> bool {
    geometry.touchpad.contains_hand(point)
        || geometry
            .keys()
            .iter()
            .any(|key| key.rect.contains_hand(point))
}

fn on_surface(geometry: &academy_workstation::WorldGeometry, point: HandPoint) -> bool {
    point.depth() >= CONTACT_DEPTH && over_surface_xy(geometry, point)
}

fn digit_name(digit: Digit) -> &'static str {
    match digit {
        Digit::Thumb => "thumb",
        Digit::Index => "index",
        Digit::Middle => "middle",
        Digit::Ring => "ring",
        Digit::Little => "little",
    }
}

fn as_u64(value: usize) -> u64 {
    u64::try_from(value).unwrap_or(u64::MAX)
}

fn hex_digest(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use academy_workstation::WorkstationSession;

    #[test]
    fn initial_hand_is_over_a_real_surface_but_below_contact_depth() {
        let session = WorkstationSession::new(EVIDENCE_SEED).unwrap();
        let state = session.read().unwrap().body.state;
        let geometry = WorkstationWorld::new().unwrap().geometry().clone();
        assert!(site_points(&state).into_iter().any(|(_, point)| {
            over_surface_xy(&geometry, point) && point.depth() < CONTACT_DEPTH
        }));
        assert!(
            site_points(&state)
                .into_iter()
                .all(|(_, point)| !on_surface(&geometry, point))
        );
    }

    #[test]
    fn short_projection_uses_the_complete_inert_recording() {
        let (evidence, recording) = capture(EVIDENCE_SEED, 2).unwrap();
        assert_eq!(evidence.steps, 2);
        assert_eq!(recording.steps().len(), 2);
        assert!(evidence.exact_replay);
        assert!(evidence.naturally_quiescent);
        assert_eq!(evidence.sites.len(), 6);
        assert_eq!(evidence.recording_sha256.len(), 64);
    }
}
