use academy_workstation_review::{record_and_render, VIDEO_HEIGHT, VIDEO_WIDTH};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static NEXT_DIRECTORY: AtomicU64 = AtomicU64::new(0);

struct TestDirectory(PathBuf);

impl TestDirectory {
    fn new() -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "academy-workstation-video-{}-{nonce}-{}",
            std::process::id(),
            NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn frozen_recording_encodes_as_a_decodable_video() {
    if !available("ffmpeg") || !available("ffprobe") {
        eprintln!("ffmpeg or ffprobe unavailable; skipping external video smoke check");
        return;
    }
    let directory = TestDirectory::new();
    let manifest = record_and_render(&directory.0, 82_201, 2).unwrap();
    let video = directory.0.join(&manifest.video_file);
    let probe = Command::new("ffprobe")
        .args([
            "-v",
            "error",
            "-select_streams",
            "v:0",
            "-show_entries",
            "stream=width,height",
            "-of",
            "csv=p=0:s=x",
        ])
        .arg(&video)
        .output()
        .unwrap();

    assert!(probe.status.success());
    assert_eq!(
        String::from_utf8(probe.stdout).unwrap().trim(),
        format!("{VIDEO_WIDTH}x{VIDEO_HEIGHT}")
    );
    assert_eq!(manifest.step_count, 2);
    assert!(manifest.replay_exact);
    assert!(fs::metadata(video).unwrap().len() > 0);
}

fn available(command: &str) -> bool {
    Command::new(command)
        .arg("-version")
        .output()
        .is_ok_and(|output| output.status.success())
}
