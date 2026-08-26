#![forbid(unsafe_code)]

mod media;

use academy_review::{EpisodeCatalog, EpisodeClass, ReviewEpisode};
use dioxus::prelude::*;
use std::path::PathBuf;

const STYLESHEET: &str = include_str!("styles.css");

fn main() {
    let root = episode_root();
    let media_root = media::MediaRoot::new(root);
    let window = dioxus::desktop::WindowBuilder::new()
        .with_title("Academy Episodes")
        .with_inner_size(dioxus::desktop::LogicalSize::new(1440.0, 900.0))
        .with_min_inner_size(dioxus::desktop::LogicalSize::new(1080.0, 720.0));
    dioxus::LaunchBuilder::desktop()
        .with_cfg(
            dioxus::desktop::Config::new()
                .with_window(window)
                .with_custom_protocol(media::SCHEME, move |_, request| media_root.respond(request)),
        )
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
struct EpisodeLibrary {
    title: String,
    root: PathBuf,
    episodes: Vec<ReviewEpisode>,
}

#[derive(Clone)]
enum LibraryState {
    Ready(EpisodeLibrary),
    Unavailable { root: PathBuf, reason: String },
}

#[component]
fn App() -> Element {
    let root = use_hook(episode_root).clone();
    let root_for_load = root.clone();
    let library = use_resource(move || {
        let root = root_for_load.clone();
        async move { load_library(root).await }
    });
    let mut filter = use_signal(|| EpisodeFilter::All);
    let mut selected = use_signal(|| 0_usize);

    let library = library.read();
    let Some(library) = library.as_ref() else {
        return rsx! {
            document::Title { "Academy Episodes" }
            style { {STYLESHEET} }
            main { class: "empty-library",
                div { class: "empty-mark", aria_hidden: "true", i {} i {} i {} }
                h1 { "Loading episodes" }
                p { "Reading the review catalog." }
                small { "{root.display()}" }
            }
        };
    };
    let LibraryState::Ready(library) = library else {
        let LibraryState::Unavailable { root, reason } = library else {
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
        .filter(|(_, episode)| filter().accepts(episode.class))
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
                            key: "video-{episode.id}",
                            class: "episode-video",
                            controls: true,
                            autoplay: true,
                            muted: true,
                            playsinline: true,
                            preload: "metadata",
                            src: "{media::uri(&episode.video_file)}",
                            poster: "{media::uri(&episode.poster_file)}",
                        }
                    }

                    aside { class: "episode-inspector",
                        div { class: "episode-heading",
                            span { class: class_name(episode.class), "{episode.class.label()}" }
                            h2 { "{episode.title}" }
                            p { "{episode.summary}" }
                        }

                        dl { class: "evidence-metrics",
                            Metric { term: "Outcome", value: episode.outcome.label().to_string() }
                            Metric { term: "Physical work", value: episode.physical_work.to_string() }
                            Metric { term: "Crossings", value: episode.outward_crossings.to_string() }
                            Metric { term: "Learning updates", value: episode.plasticity_updates.to_string() }
                            Metric { term: "Quiescent", value: yes_no(episode.naturally_quiescent).to_string() }
                            Metric { term: "Replay", value: if episode.replay_exact { "Exact".to_string() } else { "Diverged".to_string() } }
                        }

                        div { class: "body-change",
                            span { "Body" }
                            code { "{episode.body_before} → {episode.body_after}" }
                        }

                        div { class: "episode-actions",
                            a { href: "{media::uri(&episode.record_file)}", download: "{episode.id}.json", "Episode record" }
                        }
                    }

                    section { class: "episode-gallery", aria_label: "Episode gallery",
                        for index in visible.iter().copied() {
                            if let Some(item) = library.episodes.get(index) {
                                button {
                                    key: "episode-{item.id}",
                                    class: if index == selected_index { "episode-card selected" } else { "episode-card" },
                                    r#type: "button",
                                    onclick: move |_| selected.set(index),
                                    div { class: "episode-poster",
                                        img {
                                            src: "{media::uri(&item.poster_file)}",
                                            alt: "",
                                            loading: "lazy",
                                            decoding: "async",
                                        }
                                        span { class: class_name(item.class), "{item.class.label()}" }
                                        i { class: "play-mark", aria_hidden: "true" }
                                    }
                                    div { class: "episode-card-copy",
                                        strong { "{item.title}" }
                                        span { "{item.outcome.label()}" }
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
                        onclick: move |_| filter.set(option),
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

fn episode_root() -> PathBuf {
    std::env::var("ACADEMY_EPISODE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("output/academy-episodes"))
}

async fn load_library(root: PathBuf) -> LibraryState {
    let load_root = root.clone();
    let catalog = match tokio::task::spawn_blocking(move || EpisodeCatalog::load(&load_root)).await
    {
        Ok(Ok(catalog)) => catalog,
        Ok(Err(error)) => {
            return LibraryState::Unavailable {
                root,
                reason: error.to_string(),
            }
        }
        Err(error) => {
            return LibraryState::Unavailable {
                root,
                reason: format!("catalog task failed: {error}"),
            }
        }
    };
    LibraryState::Ready(EpisodeLibrary {
        title: catalog.title,
        root,
        episodes: catalog.episodes,
    })
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
