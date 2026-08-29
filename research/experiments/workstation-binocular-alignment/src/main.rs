use std::fs;
use std::path::PathBuf;
use workstation_binocular_alignment::{run_alignment_arm, AlignmentArm};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let output = args
        .next()
        .map(PathBuf::from)
        .ok_or("output path is required")?;
    let arm = match args.next().as_deref() {
        None | Some("minimal") => AlignmentArm::Minimal,
        Some("foveal-identity-only") => AlignmentArm::FovealIdentityOnly,
        Some("centered-return-only") => AlignmentArm::CenteredReturnOnly,
        Some("foveal-identity-return-composition") => AlignmentArm::FovealIdentityReturnComposition,
        Some("stable") => AlignmentArm::Stable,
        Some("complete") => AlignmentArm::Complete,
        Some("collapsed-placement") => AlignmentArm::CollapsedPlacement,
        Some("no-visual-return") => AlignmentArm::NoVisualReturn,
        Some("no-threshold-factorization") => AlignmentArm::NoThresholdFactorization,
        Some("production") => AlignmentArm::Production,
        Some(other) => return Err(format!("unknown alignment arm: {other}").into()),
    };
    let trace = run_alignment_arm(arm)?;
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&output, serde_json::to_vec_pretty(&trace)?)?;
    println!(
        "BINOCULAR_ALIGNMENT_TRACE arm={arm:?} pass={} bands={} output={}",
        if matches!(
            arm,
            AlignmentArm::Stable
                | AlignmentArm::FovealIdentityOnly
                | AlignmentArm::CenteredReturnOnly
                | AlignmentArm::FovealIdentityReturnComposition
        ) {
            trace.stable_fixation_passes()
        } else {
            trace.bounded_alignment_passes()
        },
        trace.bands.len(),
        output.display()
    );
    Ok(())
}
