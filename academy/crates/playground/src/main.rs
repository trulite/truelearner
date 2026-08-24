#![forbid(unsafe_code)]

use academy_episodes::{EpisodeCatalog, EpisodeClass, ReviewEpisode};
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use dioxus::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

const STYLESHEET: &str = include_str!("styles.css");

fn main() {
    let window = dioxus::desktop::WindowBuilder::new()
        .with_title("Academy Episodes")
        .with_inner_size(dioxus::desktop::LogicalSize::new(1440.0, 900.0))
        .with_min_inner_size(dioxus::desktop::LogicalSize::new(1080.0, 720.0));
    dioxus::LaunchBuilder::desktop()
        .with_cfg(dioxus::desktop::Config::new().with_window(window))
        .launch(App);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EpisodeFilter {
    All,
    Development,
    Tests,
    Controls,
}

impl EpisodeFilter {
    const ALL: [Self; 4] = [Self::All, Self::Development, Self::Tests, Self::Controls];

    const fn label(self) -> &'static str {
        match self {
            Self::All => "All",
            Self::Development => "Development",
            Self::Tests => "Tests",
            Self::Controls => "Controls",
        }
    }

    const fn accepts(self, class: EpisodeClass) -> bool {
        match self {
            Self::All => true,
            Self::Development => matches!(class, EpisodeClass::Development),
            Self::Tests => matches!(class, EpisodeClass::Test),
            Self::Controls => matches!(class, EpisodeClass::Control),
        }
    }
}

#[derive(Clone)]
struct LoadedEpisode {
    evidence: ReviewEpisode,
    video_uri: String,
    poster_uri: String,
    record_uri: String,
}

struct EpisodeLibrary {
    title: String,
    root: PathBuf,
    episodes: Vec<LoadedEpisode>,
}

enum LibraryState {
    Ready(EpisodeLibrary),
    Unavailable { root: PathBuf, reason: String },
}

#[component]
fn App() -> Element {
    let library = use_hook(load_library);
    let mut filter = use_signal(|| EpisodeFilter::All);
    let mut selected = use_signal(|| 0_usize);

    let LibraryState::Ready(library) = library else {
        let LibraryState::Unavailable { root, reason } = &library else {
            unreachable!()
        };
        return rsx! {
            document::Title { "Academy Episodes" }
            style { {STYLESHEET} }
            main { class: "empty-library",
                div { class: "empty-mark", aria_hidden: "true", i {} i {} i {} }
                h1 { "No episodes yet" }
                p { "Run the headless Academy suite, then reopen this viewer." }
                code { "cargo run --manifest-path academy/Cargo.toml -p academy-runner" }
                small { "{root.display()} · {reason}" }
            }
        };
    };

    let visible = library
        .episodes
        .iter()
        .enumerate()
        .filter(|(_, episode)| filter().accepts(episode.evidence.class))
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
    let selected_index = if visible.contains(&selected()) {
        selected()
    } else {
        visible.first().copied().unwrap_or(0)
    };
    let episode = library.episodes.get(selected_index);

    rsx! {
        document::Title { "Academy Episodes" }
        style { {STYLESHEET} }
        div { class: "app-shell",
            header { class: "topbar",
                div { class: "collection-title",
                    h1 { "{library.title}" }
                    span { "{library.episodes.len()} episodes" }
                }
                div { class: "library-state",
                    span { class: "status-dot" }
                    div {
                        strong { "Exact evidence" }
                        span { "{library.root.display()}" }
                    }
                }
            }

            if let Some(episode) = episode {
                main { class: "episode-layout",
                    section { class: "player-stage", aria_label: "Selected episode",
                        video {
                            key: "video-{episode.evidence.id}",
                            class: "episode-video",
                            controls: true,
                            autoplay: true,
                            muted: true,
                            playsinline: true,
                            preload: "auto",
                            poster: "{episode.poster_uri}",
                            source { src: "{episode.video_uri}", r#type: "video/mp4" }
                        }
                    }

                    aside { class: "episode-inspector",
                        div { class: "episode-heading",
                            span { class: class_name(episode.evidence.class), "{episode.evidence.class.label()}" }
                            h2 { "{episode.evidence.title}" }
                            p { "{episode.evidence.summary}" }
                        }

                        dl { class: "evidence-metrics",
                            Metric { term: "Outcome", value: episode.evidence.outcome.label().to_string() }
                            Metric { term: "Physical work", value: episode.evidence.physical_work.to_string() }
                            Metric { term: "Crossings", value: episode.evidence.outward_crossings.to_string() }
                            Metric { term: "Learning updates", value: episode.evidence.plasticity_updates.to_string() }
                            Metric { term: "Quiescent", value: yes_no(episode.evidence.naturally_quiescent).to_string() }
                            Metric { term: "Replay", value: if episode.evidence.replay_exact { "Exact".to_string() } else { "Diverged".to_string() } }
                        }

                        div { class: "body-change",
                            span { "Body" }
                            code { "{episode.evidence.body_before} → {episode.evidence.body_after}" }
                        }

                        div { class: "episode-actions",
                            a { href: "{episode.record_uri}", download: "{episode.evidence.id}.json", "Episode record" }
                        }
                    }

                    section { class: "episode-gallery", aria_label: "Episode gallery",
                        for index in visible.iter().copied() {
                            if let Some(item) = library.episodes.get(index) {
                                button {
                                    key: "episode-{item.evidence.id}",
                                    class: if index == selected_index { "episode-card selected" } else { "episode-card" },
                                    r#type: "button",
                                    onclick: move |_| selected.set(index),
                                    div { class: "episode-poster",
                                        img { src: "{item.poster_uri}", alt: "" }
                                        span { class: class_name(item.evidence.class), "{item.evidence.class.label()}" }
                                        i { class: "play-mark", aria_hidden: "true" }
                                    }
                                    div { class: "episode-card-copy",
                                        strong { "{item.evidence.title}" }
                                        span { "{item.evidence.outcome.label()}" }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                main { class: "empty-filter",
                    h2 { "No matching episodes" }
                    p { "Choose another collection." }
                }
            }

            nav { class: "filter-dock", aria_label: "Episode collections",
                for option in EpisodeFilter::ALL {
                    button {
                        r#type: "button",
                        class: if filter() == option { "filter-button active" } else { "filter-button" },
                        onclick: move |_| {
                            filter.set(option);
                            if let Some(index) = library
                                .episodes
                                .iter()
                                .position(|episode| option.accepts(episode.evidence.class))
                            {
                                selected.set(index);
                            }
                        },
                        "{option.label()}"
                    }
                }
            }
        }
    }
}

#[component]
fn Metric(term: String, value: String) -> Element {
    rsx! {
        div {
            dt { "{term}" }
            dd { "{value}" }
        }
    }
}

fn load_library() -> LibraryState {
    let root = std::env::var("ACADEMY_EPISODE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("output/academy-episodes"));
    let catalog = match EpisodeCatalog::load(&root) {
        Ok(catalog) => catalog,
        Err(error) => {
            return LibraryState::Unavailable {
                root,
                reason: error.to_string(),
            }
        }
    };
    match load_episodes(&root, catalog) {
        Ok(library) => LibraryState::Ready(library),
        Err(reason) => LibraryState::Unavailable { root, reason },
    }
}

fn load_episodes(root: &Path, catalog: EpisodeCatalog) -> Result<EpisodeLibrary, String> {
    let episodes = catalog
        .episodes
        .into_iter()
        .map(|episode| {
            let video_uri = media_uri(&root.join(&episode.video_file), "video/mp4")?;
            let poster_uri = media_uri(&root.join(&episode.poster_file), "image/png")?;
            let record_uri = media_uri(&root.join(&episode.record_file), "application/json")?;
            Ok(LoadedEpisode {
                evidence: episode,
                video_uri,
                poster_uri,
                record_uri,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(EpisodeLibrary {
        title: catalog.title,
        root: root.to_path_buf(),
        episodes,
    })
}

fn media_uri(path: &Path, mime: &str) -> Result<String, String> {
    let bytes = fs::read(path).map_err(|error| format!("{}: {error}", path.display()))?;
    Ok(format!("data:{mime};base64,{}", BASE64.encode(bytes)))
}

fn class_name(class: EpisodeClass) -> &'static str {
    match class {
        EpisodeClass::Development => "episode-class development",
        EpisodeClass::Test => "episode-class test",
        EpisodeClass::Control => "episode-class control",
    }
}

fn yes_no(value: bool) -> &'static str {
    if value {
        "Yes"
    } else {
        "No"
    }
}
