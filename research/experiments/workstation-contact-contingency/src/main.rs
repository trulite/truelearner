use std::path::PathBuf;
use workstation_contact_contingency::capture_complete_parent;

fn main() {
    let mut output = None;
    let mut recording = None;
    let mut args = std::env::args().skip(1);
    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--output" => output = args.next().map(PathBuf::from),
            "--recording" => recording = args.next().map(PathBuf::from),
            other => panic!("unknown argument: {other}"),
        }
    }
    let (evidence, captured) = capture_complete_parent().expect("complete parent capture succeeds");
    let rendered = serde_json::to_string_pretty(&evidence).expect("evidence serializes");
    if let Some(path) = output {
        write(&path, rendered.as_bytes());
    } else {
        println!("{rendered}");
    }
    if let Some(path) = recording {
        let bytes = captured
            .canonical_bytes()
            .expect("recording encodes canonically");
        write(&path, &bytes);
    }
}

fn write(path: &PathBuf, bytes: &[u8]) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("artifact directory is created");
    }
    std::fs::write(path, bytes).expect("artifact writes");
}
