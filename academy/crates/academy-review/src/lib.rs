#![forbid(unsafe_code)]
//! Portable descriptions of causally inert Academy review evidence.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EpisodeClass {
    Development,
    Test,
    Control,
}

impl EpisodeClass {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Development => "Development",
            Self::Test => "Test",
            Self::Control => "Control",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EpisodeOutcome {
    StructureFormed,
    LearnedResponse,
    ExpectedSilence,
    ScaffoldedAction,
    MappingFollowed,
    RetainedResponse,
}

impl EpisodeOutcome {
    pub const fn label(self) -> &'static str {
        match self {
            Self::StructureFormed => "Structure formed",
            Self::LearnedResponse => "Learned response",
            Self::ExpectedSilence => "Correctly silent",
            Self::ScaffoldedAction => "Scaffolded action",
            Self::MappingFollowed => "Physical mapping followed",
            Self::RetainedResponse => "Retained response",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EpisodeFrame {
    pub file: String,
    pub duration_ms: u32,
    pub caption: String,
    pub world_fingerprint: String,
    pub output_fingerprint: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewEpisode {
    pub schema_version: u16,
    pub id: String,
    pub title: String,
    pub summary: String,
    pub display_name: String,
    pub class: EpisodeClass,
    pub outcome: EpisodeOutcome,
    pub seed: u64,
    pub physical_work: u64,
    pub plasticity_updates: u64,
    pub outward_crossings: usize,
    pub naturally_quiescent: bool,
    pub replay_exact: bool,
    pub body_before: String,
    pub body_after: String,
    pub video_file: String,
    pub poster_file: String,
    pub record_file: String,
    pub frames: Vec<EpisodeFrame>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EpisodeCatalog {
    pub schema_version: u16,
    pub title: String,
    pub episodes: Vec<ReviewEpisode>,
}

impl EpisodeCatalog {
    pub fn load(root: &Path) -> Result<Self, CatalogError> {
        let path = root.join("catalog.json");
        let bytes = fs::read(&path).map_err(|source| CatalogError::Read {
            path: path.clone(),
            source,
        })?;
        serde_json::from_slice(&bytes).map_err(|source| CatalogError::Decode { path, source })
    }
}

#[derive(Debug)]
pub enum CatalogError {
    Read {
        path: PathBuf,
        source: std::io::Error,
    },
    Decode {
        path: PathBuf,
        source: serde_json::Error,
    },
}

impl fmt::Display for CatalogError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read { path, source } => write!(formatter, "{}: {source}", path.display()),
            Self::Decode { path, source } => write!(formatter, "{}: {source}", path.display()),
        }
    }
}

impl std::error::Error for CatalogError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Read { source, .. } => Some(source),
            Self::Decode { source, .. } => Some(source),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    static NEXT_TEST_DIRECTORY: AtomicU64 = AtomicU64::new(0);

    struct TestDirectory(PathBuf);

    impl TestDirectory {
        fn new() -> Self {
            let nonce = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system clock is after the Unix epoch")
                .as_nanos();
            let path = std::env::temp_dir().join(format!(
                "academy-review-{}-{nonce}-{}",
                std::process::id(),
                NEXT_TEST_DIRECTORY.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&path).expect("create isolated review test directory");
            Self(path)
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn catalog_round_trip_preserves_review_vocabulary() {
        let catalog = sample_catalog();
        let bytes = serde_json::to_vec(&catalog).expect("serialize catalog");
        let decoded: EpisodeCatalog = serde_json::from_slice(&bytes).expect("deserialize catalog");

        assert_eq!(decoded, catalog);
        assert_eq!(EpisodeClass::Development.label(), "Development");
        assert_eq!(EpisodeOutcome::LearnedResponse.label(), "Learned response");
    }

    #[test]
    fn load_reads_only_the_catalog_file() {
        let directory = TestDirectory::new();
        let catalog = sample_catalog();
        fs::write(
            directory.0.join("catalog.json"),
            serde_json::to_vec(&catalog).expect("serialize catalog"),
        )
        .expect("write catalog");

        assert_eq!(EpisodeCatalog::load(&directory.0).unwrap(), catalog);
    }

    #[test]
    fn absent_catalog_reports_the_attempted_path() {
        let directory = TestDirectory::new();
        let error = EpisodeCatalog::load(&directory.0).unwrap_err();

        assert!(error.to_string().contains("catalog.json"));
    }

    fn sample_catalog() -> EpisodeCatalog {
        EpisodeCatalog {
            schema_version: 1,
            title: "Review".to_string(),
            episodes: vec![ReviewEpisode {
                schema_version: 1,
                id: "development-one".to_string(),
                title: "A path forms".to_string(),
                summary: "A physical path forms during supported participation.".to_string(),
                display_name: "Momo".to_string(),
                class: EpisodeClass::Development,
                outcome: EpisodeOutcome::StructureFormed,
                seed: 7,
                physical_work: 11,
                plasticity_updates: 1,
                outward_crossings: 1,
                naturally_quiescent: true,
                replay_exact: true,
                body_before: "before".to_string(),
                body_after: "after".to_string(),
                video_file: "episodes/development-one/episode.mp4".to_string(),
                poster_file: "episodes/development-one/poster.png".to_string(),
                record_file: "episodes/development-one/record.json".to_string(),
                frames: Vec::new(),
            }],
        }
    }
}
