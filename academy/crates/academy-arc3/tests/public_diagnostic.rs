use academy_arc3::{
    Arc3ActionCatalog, Arc3ActionOffer, Arc3ActionSchema, Arc3CapstoneAgent, Arc3CapstoneCommand,
    ARC3_FRAME_PIXELS,
};
use truelearner_workstation::WorkstationHarness;

fn checkpoint() -> Vec<u8> {
    WorkstationHarness::new(90_301)
        .unwrap()
        .save()
        .unwrap()
        .canonical_bytes()
        .unwrap()
}

fn catalog() -> Arc3ActionCatalog {
    Arc3ActionCatalog {
        offers: vec![Arc3ActionOffer {
            id: 1,
            schema: Arc3ActionSchema::Unit,
        }],
    }
}

#[test]
fn current_workstation_checkpoint_restarts_with_exact_replay() {
    let checkpoint = checkpoint();
    let mut first = Arc3CapstoneAgent::restore(&checkpoint).unwrap();
    let mut replay = Arc3CapstoneAgent::restore(&checkpoint).unwrap();
    let mut frame = vec![0; ARC3_FRAME_PIXELS];
    frame[31 * 64 + 31] = 9;

    assert_eq!(
        first.observe(frame.clone(), catalog()).unwrap(),
        replay.observe(frame, catalog()).unwrap()
    );
    assert_eq!(first.snapshot().unwrap(), replay.snapshot().unwrap());
}

#[test]
fn corrupt_or_foreign_checkpoint_fails_before_ready() {
    let mut corrupt = checkpoint();
    corrupt[0] ^= 1;
    assert!(Arc3CapstoneAgent::restore(&corrupt).is_err());
    assert!(Arc3CapstoneAgent::restore(b"not a workstation checkpoint").is_err());
}

#[test]
fn process_command_cannot_represent_evaluator_state() {
    let command = format!(
        "{{\"command\":\"observe\",\"frame\":{},\"actions\":{{\"offers\":[{{\"id\":1,\"schema\":{{\"type\":\"unit\"}}}}]}},\"score\":1}}",
        serde_json::to_string(&vec![0; ARC3_FRAME_PIXELS]).unwrap()
    );
    assert!(serde_json::from_str::<Arc3CapstoneCommand>(&command).is_err());
}
