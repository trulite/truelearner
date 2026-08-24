#![forbid(unsafe_code)]

use academy_core::{
    AcademyCommand, AcademyEvent, AcademyWorker, Capability, CapabilityGraph, ExperienceMode,
    ExperienceRecord, InspectorSnapshot, InteractionRequest, PhysicalInput, SessionSnapshot,
    VisualSurface, SURFACE_HEIGHT, SURFACE_WIDTH,
};
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use dioxus::prelude::*;
use futures_timer::Delay;
use std::sync::{Arc, Mutex};
use std::time::Duration;

const STYLESHEET: &str = include_str!("styles.css");

fn main() {
    dioxus::launch(App);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Speaker {
    Human,
    Organism,
    Academy,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Message {
    speaker: Speaker,
    title: String,
    body: String,
    detail: String,
}

#[derive(Clone, Debug)]
struct UiModel {
    messages: Vec<Message>,
    capabilities: CapabilityGraph,
    timeline: Vec<ExperienceRecord>,
    inspector: Option<InspectorSnapshot>,
    mode: ExperienceMode,
    selected_capability: String,
    status: String,
    busy: bool,
    debug_overlay: bool,
    replay_message: String,
    file_notice: String,
}

impl Default for UiModel {
    fn default() -> Self {
        Self {
            messages: vec![Message {
                speaker: Speaker::Academy,
                title: "Physical body readying".to_string(),
                body: "Academy is starting the bounded TrueLearner worker. No UI timing enters physical time.".to_string(),
                detail: "External instrument · awaiting worker".to_string(),
            }],
            capabilities: CapabilityGraph::default(),
            timeline: Vec::new(),
            inspector: None,
            mode: ExperienceMode::Teach,
            selected_capability: "interaction-response".to_string(),
            status: "Starting body".to_string(),
            busy: false,
            debug_overlay: true,
            replay_message: "No replay yet".to_string(),
            file_notice: String::new(),
        }
    }
}

#[component]
fn App() -> Element {
    let worker = use_hook(|| Arc::new(Mutex::new(AcademyWorker::spawn().expect("Academy worker must start"))));
    let mut model = use_signal(UiModel::default);
    let mut composer = use_signal(String::new);
    let mut shared_surface = use_signal(VisualSurface::blank);
    let mut organism_surface = use_signal(|| {
        VisualSurface::new(SURFACE_WIDTH, SURFACE_HEIGHT, [24, 31, 37, 255])
    });
    let mut drawing = use_signal(|| false);
    let mut last_point = use_signal(|| None::<(u32, u32)>);

    let polling_worker = Arc::clone(&worker);
    use_future(move || async move {
        loop {
            let mut events = Vec::new();
            if let Ok(locked) = polling_worker.lock() {
                while let Ok(Some(event)) = locked.try_event() {
                    events.push(event);
                }
            }
            if !events.is_empty() {
                for event in events {
                    apply_worker_event(event, &mut model, &mut organism_surface);
                }
            }
            Delay::new(Duration::from_millis(48)).await;
        }
    });

    use_effect(move || {
        let surface = shared_surface.read().clone();
        render_canvas("shared-canvas", &surface, model.read().debug_overlay);
    });

    use_effect(move || {
        let surface = organism_surface.read().clone();
        render_canvas("organism-canvas", &surface, false);
    });

    let selected = model
        .read()
        .capabilities
        .capability(&model.read().selected_capability)
        .cloned();
    let inspector = model.read().inspector.clone();
    let ready = inspector.is_some();

    rsx! {
        document::Title { "TrueLearner Academy" }
        style { {STYLESHEET} }
        div { class: "app-shell",
            header { class: "topbar",
                div { class: "brand-lockup",
                    div { class: "brand-mark", aria_label: "TrueLearner Academy mark",
                        span {}
                        span {}
                        span {}
                    }
                    div {
                        h1 { "TrueLearner Academy" }
                        p { "Playground · physical development instrument" }
                    }
                }
                div { class: "mode-switch", role: "group", aria_label: "Experience mode",
                    button {
                        class: if model.read().mode == ExperienceMode::Teach { "mode-button active" } else { "mode-button" },
                        onclick: move |_| model.write().mode = ExperienceMode::Teach,
                        "Teach"
                    }
                    button {
                        class: if model.read().mode == ExperienceMode::Probe { "mode-button active" } else { "mode-button" },
                        onclick: move |_| model.write().mode = ExperienceMode::Probe,
                        "Probe understanding"
                    }
                }
                div { class: "run-status",
                    span { class: if ready { "status-dot ready" } else { "status-dot" } }
                    div {
                        strong { "{model.read().status}" }
                        span {
                            if let Some(ref state) = inspector {
                                "tick {state.physical_tick} · body {state.body_fingerprint}"
                            } else {
                                "bounded worker starting"
                            }
                        }
                    }
                }
            }

            main { class: "workspace",
                section { class: "conversation-region", aria_label: "Conversation and shared world",
                    div { class: "section-heading",
                        div {
                            h2 { "Conversation & world" }
                            p { "Human semantics are rendered here; only admitted physical input reaches the body." }
                        }
                        div { class: "section-actions",
                            label { class: "toggle-control",
                                input {
                                    r#type: "checkbox",
                                    checked: model.read().debug_overlay,
                                    onchange: move |event| model.write().debug_overlay = event.checked(),
                                }
                                span { "Causally inert overlay" }
                            }
                        }
                    }

                    div { class: "conversation-stream",
                        for (index, message) in model.read().messages.iter().enumerate() {
                            article {
                                key: "message-{index}",
                                class: match message.speaker {
                                    Speaker::Human => "message human",
                                    Speaker::Organism => "message organism",
                                    Speaker::Academy => "message academy",
                                },
                                div { class: "message-meta",
                                    strong { "{message.title}" }
                                    span { "{message.detail}" }
                                }
                                p { "{message.body}" }
                            }
                        }
                    }

                    div { class: "world-layout",
                        div { class: "surface-block",
                            div { class: "surface-label",
                                div {
                                    strong { "Shared raster world" }
                                    span { "Rust-owned RGBA · {SURFACE_WIDTH}×{SURFACE_HEIGHT}" }
                                }
                                button {
                                    class: "quiet-button",
                                    onclick: move |_| shared_surface.set(VisualSurface::blank()),
                                    "Clear drawing"
                                }
                            }
                            div { class: "canvas-wrap",
                                canvas {
                                    id: "shared-canvas",
                                    width: "{SURFACE_WIDTH}",
                                    height: "{SURFACE_HEIGHT}",
                                    aria_label: "Shared raster drawing surface",
                                    onmousedown: move |event| {
                                        event.prevent_default();
                                        drawing.set(true);
                                        let point = event.element_coordinates();
                                        let current = clamp_canvas_point(point.x, point.y);
                                        last_point.set(Some(current));
                                        shared_surface.write().draw_line(current, current, [21, 92, 81, 255], 2);
                                    },
                                    onmousemove: move |event| {
                                        if !drawing() {
                                            return;
                                        }
                                        let point = event.element_coordinates();
                                        let current = clamp_canvas_point(point.x, point.y);
                                        if let Some(previous) = last_point() {
                                            shared_surface.write().draw_line(previous, current, [21, 92, 81, 255], 2);
                                        }
                                        last_point.set(Some(current));
                                    },
                                    onmouseup: move |_| {
                                        drawing.set(false);
                                        last_point.set(None);
                                    },
                                    onmouseleave: move |_| {
                                        drawing.set(false);
                                        last_point.set(None);
                                    },
                                }
                                if model.read().debug_overlay {
                                    div { class: "debug-overlay", aria_hidden: "true",
                                        span { "DISPLAY ONLY" }
                                        i {}
                                    }
                                }
                            }
                            div { class: "surface-actions",
                                button {
                                    class: "secondary-button",
                                    disabled: model.read().busy,
                                    onclick: {
                                        let worker = Arc::clone(&worker);
                                        move |_| {
                                            admit_raster(
                                                &worker,
                                                &mut model,
                                                shared_surface.read().clone(),
                                                "Human drawing",
                                            );
                                        }
                                    },
                                    "Admit drawing"
                                }
                                label { class: "file-button",
                                    "Open image"
                                    input {
                                        r#type: "file",
                                        accept: "image/png,image/jpeg,image/gif,image/webp,image/bmp",
                                        onchange: {
                                            let worker = Arc::clone(&worker);
                                            move |event| {
                                                let worker = Arc::clone(&worker);
                                                async move {
                                                    if let Some(files) = event.files() {
                                                        let names = files.files();
                                                        if let Some(name) = names.first() {
                                                            if let Some(bytes) = files.read_file(name).await {
                                                                match VisualSurface::from_encoded_image(&bytes) {
                                                                    Ok(surface) => {
                                                                        shared_surface.set(surface.clone());
                                                                        admit_raster(&worker, &mut model, surface, &format!("Image · {name}"));
                                                                    }
                                                                    Err(error) => set_error(&mut model, error.to_string()),
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        },
                                    }
                                }
                                label { class: "file-button",
                                    "Open file"
                                    input {
                                        r#type: "file",
                                        onchange: {
                                            let worker = Arc::clone(&worker);
                                            move |event| {
                                                let worker = Arc::clone(&worker);
                                                async move {
                                                    if let Some(files) = event.files() {
                                                        let names = files.files();
                                                        if let Some(name) = names.first() {
                                                            if let Some(bytes) = files.read_file(name).await {
                                                                let rendered = match VisualSurface::from_encoded_image(&bytes) {
                                                                    Ok(surface) => surface,
                                                                    Err(_) => VisualSurface::render_text(&String::from_utf8_lossy(&bytes)),
                                                                };
                                                                shared_surface.set(rendered.clone());
                                                                model.write().file_notice = format!("Rendered {name} into the raster world; path and file metadata stayed outside the organism.");
                                                                admit_raster(&worker, &mut model, rendered, &format!("Rendered file · {name}"));
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        },
                                    }
                                }
                                if !model.read().file_notice.is_empty() {
                                    span { class: "file-notice", "{model.read().file_notice}" }
                                }
                            }
                        }

                        div { class: "surface-block output-block",
                            div { class: "surface-label",
                                div {
                                    strong { "Outward raster activity" }
                                    span { "Crossings rendered for the human view" }
                                }
                            }
                            div { class: "canvas-wrap output",
                                canvas {
                                    id: "organism-canvas",
                                    width: "{SURFACE_WIDTH}",
                                    height: "{SURFACE_HEIGHT}",
                                    aria_label: "Organism raster output",
                                }
                            }
                            p { class: "surface-caption", "A dark field means no outward crossings. This renderer adds no feedback unless you explicitly admit it." }
                        }
                    }

                    form {
                        class: "composer",
                        onsubmit: {
                            let worker = Arc::clone(&worker);
                            move |event| {
                                event.prevent_default();
                                let text = composer.read().trim().to_string();
                                if text.is_empty() || model.read().busy {
                                    return;
                                }
                                let mode = model.read().mode;
                                let request = InteractionRequest {
                                    mode,
                                    input: PhysicalInput::Text(text.clone()),
                                    capability_ids: vec!["interaction-response".to_string(), "copy-symbol".to_string()],
                                    expected_text: (mode != ExperienceMode::Teach).then_some(text.clone()),
                                    academy_note: if mode == ExperienceMode::Teach {
                                        "Human-presented physical example".to_string()
                                    } else {
                                        "Fresh exact-copy probe; expectation remained Academy-side".to_string()
                                    },
                                };
                                model.write().messages.push(Message {
                                    speaker: Speaker::Human,
                                    title: "Human".to_string(),
                                    body: text,
                                    detail: format!("{} · byte/glyph embodiment affordance", mode.label()),
                                });
                                send_command(&worker, &mut model, AcademyCommand::Interact(request));
                                composer.set(String::new());
                            }
                        },
                        textarea {
                            value: "{composer}",
                            placeholder: if model.read().mode == ExperienceMode::Teach {
                                "Present a physical example…"
                            } else {
                                "Run a fresh held-out probe…"
                            },
                            aria_label: "Message to admit to TrueLearner",
                            oninput: move |event| composer.set(event.value()),
                            onkeydown: {
                                let worker = Arc::clone(&worker);
                                move |event| {
                                    if event.key() == Key::Enter && !event.modifiers().shift() {
                                        event.prevent_default();
                                        let text = composer.read().trim().to_string();
                                        if text.is_empty() || model.read().busy {
                                            return;
                                        }
                                        let mode = model.read().mode;
                                        let request = InteractionRequest {
                                            mode,
                                            input: PhysicalInput::Text(text.clone()),
                                            capability_ids: vec!["interaction-response".to_string(), "copy-symbol".to_string()],
                                            expected_text: (mode != ExperienceMode::Teach).then_some(text.clone()),
                                            academy_note: "Keyboard-admitted physical text".to_string(),
                                        };
                                        model.write().messages.push(Message {
                                            speaker: Speaker::Human,
                                            title: "Human".to_string(),
                                            body: text,
                                            detail: format!("{} · admitted explicitly", mode.label()),
                                        });
                                        send_command(&worker, &mut model, AcademyCommand::Interact(request));
                                        composer.set(String::new());
                                    }
                                }
                            },
                        }
                        button {
                            class: "primary-button",
                            r#type: "submit",
                            disabled: model.read().busy || composer.read().trim().is_empty(),
                            if model.read().busy { "Running physical activity…" } else { "Admit experience" }
                        }
                    }
                }

                aside { class: "academy-region", aria_label: "Academy evidence and body inspector",
                    section { class: "academy-section frontier-section",
                        div { class: "section-heading compact",
                            div {
                                h2 { "Development frontier" }
                                p { "External evidence, never organism state." }
                            }
                        }
                        div { class: "frontier-summary",
                            div {
                                strong { "{model.read().capabilities.stable_count()}" }
                                span { "stable" }
                            }
                            div {
                                strong { "{model.read().capabilities.frontier_count()}" }
                                span { "at frontier" }
                            }
                            div {
                                strong { "{model.read().capabilities.capabilities().count()}" }
                                span { "mapped" }
                            }
                        }
                        div { class: "capability-list",
                            for capability in model.read().capabilities.capabilities() {
                                button {
                                    key: "cap-{capability.id}",
                                    class: if capability.id == model.read().selected_capability { "capability-row selected" } else { "capability-row" },
                                    onclick: {
                                        let id = capability.id.clone();
                                        move |_| model.write().selected_capability = id.clone()
                                    },
                                    span { class: status_class(capability), "{capability.status.label()}" }
                                    strong { "{capability.title}" }
                                    small {
                                        "{capability.evidence.fresh_passes}/{capability.evidence.fresh_attempts} fresh"
                                    }
                                }
                            }
                        }
                    }

                    if let Some(capability) = selected {
                        CapabilityDetail { capability }
                    }

                    section { class: "academy-section inspector-section",
                        div { class: "section-heading compact",
                            div {
                                h2 { "Physical body" }
                                p { "Causally inert runtime view" }
                            }
                        }
                        if let Some(state) = inspector {
                            dl { class: "inspector-grid",
                                Metric { term: "Body version", value: format!("v{}", state.body_version) }
                                Metric { term: "Physical tick", value: state.physical_tick.to_string() }
                                Metric { term: "Pressure phase", value: state.pressure_phase.to_string() }
                                Metric { term: "Resident arenas", value: state.resident_arenas.to_string() }
                                Metric { term: "Active max", value: state.active_arena_max.to_string() }
                                Metric { term: "Crossings", value: state.crossing_total.to_string() }
                                Metric { term: "Last work", value: state.last_run_work.to_string() }
                                Metric { term: "Total work", value: state.physical_work_total.to_string() }
                                Metric { term: "Durable body", value: format_bytes(state.durable_bytes) }
                                Metric { term: "Resident bytes", value: format_bytes(state.last_run_bytes) }
                            }
                            div { class: "checkpoint-actions",
                                button {
                                    class: "quiet-button",
                                    onclick: {
                                        let worker = Arc::clone(&worker);
                                        move |_| send_command(&worker, &mut model, AcademyCommand::SaveCheckpoint)
                                    },
                                    "Save checkpoint"
                                }
                                button {
                                    class: "quiet-button",
                                    onclick: {
                                        let worker = Arc::clone(&worker);
                                        move |_| send_command(&worker, &mut model, AcademyCommand::RestoreCheckpoint)
                                    },
                                    "Restore"
                                }
                                button {
                                    class: "quiet-button",
                                    onclick: {
                                        let worker = Arc::clone(&worker);
                                        move |_| send_command(&worker, &mut model, AcademyCommand::ReplayLast)
                                    },
                                    "Replay last"
                                }
                            }
                            p { class: "replay-status", "{model.read().replay_message}" }
                        } else {
                            div { class: "skeleton-stack", aria_label: "Body inspector loading",
                                span {}
                                span {}
                                span {}
                            }
                        }
                    }
                }
            }

            section { class: "timeline-region", aria-label: "Development timeline",
                div { class: "timeline-heading",
                    div {
                        h2 { "Development timeline" }
                        p { "Each item joins body versions, physical admission, outward activity, work, and Academy annotation." }
                    }
                    span { "{model.read().timeline.len()} recorded experience(s)" }
                }
                div { class: "timeline-track",
                    if model.read().timeline.is_empty() {
                        div { class: "timeline-empty",
                            strong { "No admitted experience yet" }
                            span { "Teach or probe once; the physical record will appear here." }
                        }
                    }
                    for record in model.read().timeline.iter().rev().take(24) {
                        article { class: "timeline-event", key: "experience-{record.id}",
                            div { class: "timeline-topline",
                                span { class: mode_class(record.mode), "{record.mode.label()}" }
                                strong { "#{record.id}" }
                                if let Some(passed) = record.probe_passed {
                                    span { class: if passed { "probe-result pass" } else { "probe-result not-yet" },
                                        if passed { "PASS" } else { "NOT YET" }
                                    }
                                }
                            }
                            p { "{record.admission.input_summary}" }
                            dl {
                                div { dt { "clock" } dd { "{record.clock_start}→{record.clock_end}" } }
                                div { dt { "work" } dd { "{record.physical_work}" } }
                                div { dt { "body" } dd { "{record.body_before}→{record.body_after}" } }
                                div { dt { "crossings" } dd { "{record.crossings.len()}" } }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CapabilityDetail(capability: Capability) -> Element {
    let evidence = &capability.evidence;
    rsx! {
        section { class: "academy-section capability-detail",
            div { class: "detail-title",
                span { class: status_class(&capability), "{capability.status.label()}" }
                h3 { "{capability.title}" }
            }
            p { "{capability.description}" }
            if !capability.prerequisites.is_empty() {
                div { class: "prerequisites",
                    strong { "Prerequisites" }
                    span { {capability.prerequisites.join(" · ")} }
                }
            }
            dl { class: "evidence-table",
                div { dt { "Teach experiences" } dd { "{evidence.teach_experiences}" } }
                div { dt { "Fresh probes" } dd { "{evidence.fresh_passes} / {evidence.fresh_attempts}" } }
                div { dt { "Transfer probes" } dd { "{evidence.transfer_passes} / {evidence.transfer_attempts}" } }
                div { dt { "Retention probes" } dd { "{evidence.retention_passes} / {evidence.retention_attempts}" } }
                div { dt { "Median successful work" } dd { {evidence.median_work().map_or("—".to_string(), |value| value.to_string())} } }
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

fn apply_worker_event(event: AcademyEvent, model: &mut Signal<UiModel>, organism_surface: &mut Signal<VisualSurface>) {
    match event {
        AcademyEvent::Ready(snapshot) => {
            update_snapshot(model, *snapshot);
            let mut state = model.write();
            state.status = "Body quiescent".to_string();
            state.busy = false;
            state.messages.push(Message {
                speaker: Speaker::Academy,
                title: "Academy".to_string(),
                body: "The production body is resident across eight causally inert execution arenas.".to_string(),
                detail: "Ready · selected production mechanics".to_string(),
            });
        }
        AcademyEvent::Completed { record, organism_surface: output, snapshot } => {
            let record = *record;
            let result = record.probe_passed.map(|passed| if passed { "PASS" } else { "NOT YET" });
            let body = if record.organism_text.is_empty() {
                format!("{} outward crossing(s); no text decoded.", record.crossings.len())
            } else {
                record.organism_text.clone()
            };
            model.write().messages.push(Message {
                speaker: Speaker::Organism,
                title: "TrueLearner outward activity".to_string(),
                body,
                detail: result.map_or_else(
                    || format!("{} work · {} crossing(s)", record.physical_work, record.crossings.len()),
                    |result| format!("Academy probe {result} · {} work", record.physical_work),
                ),
            });
            organism_surface.set(*output);
            update_snapshot(model, *snapshot);
            let mut state = model.write();
            state.status = "Body quiescent".to_string();
            state.busy = false;
        }
        AcademyEvent::CheckpointSaved { body_version, snapshot } => {
            update_snapshot(model, *snapshot);
            let mut state = model.write();
            state.status = "Checkpoint saved".to_string();
            state.replay_message = format!("Saved exact live checkpoint at body v{body_version}");
            state.busy = false;
        }
        AcademyEvent::CheckpointRestored { body_version, snapshot } => {
            update_snapshot(model, *snapshot);
            let mut state = model.write();
            state.status = "Checkpoint restored".to_string();
            state.replay_message = format!("Restored body as v{body_version}; production mechanics reapplied");
            state.busy = false;
        }
        AcademyEvent::ReplayVerified { outcome, snapshot } => {
            let outcome = *outcome;
            update_snapshot(model, *snapshot);
            let mut state = model.write();
            state.status = if outcome.exact { "Replay exact" } else { "Replay diverged" }.to_string();
            state.replay_message = if outcome.exact {
                format!("Exact physical replay · tick {} · work {}", outcome.observed_clock, outcome.observed_work)
            } else {
                format!("Replay diverged · expected {}, observed {}", outcome.expected_body, outcome.observed_body)
            };
            state.busy = false;
        }
        AcademyEvent::Error(message) => set_error(model, message),
    }
}

fn update_snapshot(model: &mut Signal<UiModel>, snapshot: SessionSnapshot) {
    let mut state = model.write();
    state.inspector = Some(snapshot.inspector);
    state.capabilities = snapshot.capabilities;
    state.timeline = snapshot.timeline;
}

fn send_command(worker: &Arc<Mutex<AcademyWorker>>, model: &mut Signal<UiModel>, command: AcademyCommand) {
    match worker.lock() {
        Ok(locked) => match locked.try_command(command) {
            Ok(()) => {
                let mut state = model.write();
                state.busy = true;
                state.status = "Physical activity running".to_string();
            }
            Err(error) => set_error(model, error.to_string()),
        },
        Err(_) => set_error(model, "Academy worker lock is unavailable".to_string()),
    }
}

fn admit_raster(
    worker: &Arc<Mutex<AcademyWorker>>,
    model: &mut Signal<UiModel>,
    surface: VisualSurface,
    label: &str,
) {
    let mode = model.read().mode;
    let request = InteractionRequest {
        mode,
        input: PhysicalInput::Raster(surface),
        capability_ids: vec!["interaction-response".to_string(), "visual-difference".to_string()],
        expected_text: None,
        academy_note: format!("{label}; raster pixels admitted without file or DOM semantics"),
    };
    model.write().messages.push(Message {
        speaker: Speaker::Human,
        title: "Human raster world".to_string(),
        body: label.to_string(),
        detail: format!("{} · canonical pixels", mode.label()),
    });
    send_command(worker, model, AcademyCommand::Interact(request));
}

fn set_error(model: &mut Signal<UiModel>, message: String) {
    let mut state = model.write();
    state.status = "Needs attention".to_string();
    state.busy = false;
    state.messages.push(Message {
        speaker: Speaker::Academy,
        title: "Academy boundary".to_string(),
        body: message,
        detail: "No physical input was silently admitted".to_string(),
    });
}

fn render_canvas(id: &str, surface: &VisualSurface, overlay: bool) {
    let Ok(png) = surface.png_bytes() else {
        return;
    };
    let data = BASE64.encode(png);
    let overlay_script = if overlay {
        "ctx.save();ctx.strokeStyle='rgba(28,122,106,.42)';ctx.lineWidth=1;ctx.setLineDash([6,6]);ctx.strokeRect(220.5,80.5,200,200);ctx.restore();"
    } else {
        ""
    };
    document::eval(&format!(
        r#"const canvas=document.getElementById('{id}');
const ctx=canvas.getContext('2d');
const image=new Image();
image.onload=()=>{{ctx.clearRect(0,0,canvas.width,canvas.height);ctx.drawImage(image,0,0,canvas.width,canvas.height);{overlay_script}}};
image.src='data:image/png;base64,{data}';"#
    ));
}

fn clamp_canvas_point(x: f64, y: f64) -> (u32, u32) {
    (
        x.round().clamp(0.0, f64::from(SURFACE_WIDTH - 1)) as u32,
        y.round().clamp(0.0, f64::from(SURFACE_HEIGHT - 1)) as u32,
    )
}

fn status_class(capability: &Capability) -> &'static str {
    match capability.status {
        academy_core::CapabilityStatus::Unknown => "status-label unknown",
        academy_core::CapabilityStatus::Emerging => "status-label emerging",
        academy_core::CapabilityStatus::Acquired => "status-label acquired",
        academy_core::CapabilityStatus::General => "status-label general",
        academy_core::CapabilityStatus::Stable => "status-label stable",
        academy_core::CapabilityStatus::Automatic => "status-label automatic",
    }
}

fn mode_class(mode: ExperienceMode) -> &'static str {
    match mode {
        ExperienceMode::Teach => "mode-label teach",
        ExperienceMode::Probe => "mode-label probe",
        ExperienceMode::Transfer => "mode-label transfer",
        ExperienceMode::Retention => "mode-label retention",
    }
}

fn format_bytes(bytes: usize) -> String {
    if bytes >= 1024 {
        format!("{:.1} KiB", bytes as f64 / 1024.0)
    } else {
        format!("{bytes} B")
    }
}
