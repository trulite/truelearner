#![forbid(unsafe_code)]

use academy_core::{
    A1Experience, A1ExperienceKind, A1ProbeFamily, AcademyCommand, AcademyEvent, AcademyWorker,
    Capability, CapabilityGraph, ExperienceMode, ExperienceRecord, InspectorSnapshot,
    InteractionRequest, PhysicalInput, SessionSnapshot, TeachingCase, VisualSurface, SURFACE_HEIGHT,
    SURFACE_WIDTH,
};
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use dioxus::prelude::*;
use futures_timer::Delay;
use std::sync::{Arc, Mutex};
use std::time::Duration;

const STYLESHEET: &str = include_str!("styles.css");

fn main() {
    let window = dioxus::desktop::WindowBuilder::new()
        .with_title("Academy")
        .with_inner_size(dioxus::desktop::LogicalSize::new(1440.0, 900.0))
        .with_min_inner_size(dioxus::desktop::LogicalSize::new(1080.0, 720.0));
    dioxus::LaunchBuilder::desktop()
        .with_cfg(dioxus::desktop::Config::new().with_window(window))
        .launch(App);
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
    a1_timeline: Vec<A1Experience>,
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
            messages: Vec::new(),
            capabilities: CapabilityGraph::default(),
            timeline: Vec::new(),
            a1_timeline: Vec::new(),
            inspector: None,
            mode: ExperienceMode::Teach,
            selected_capability: "interaction-response".to_string(),
            status: "Starting".to_string(),
            busy: false,
            debug_overlay: true,
            replay_message: String::new(),
            file_notice: String::new(),
        }
    }
}

#[component]
fn App() -> Element {
    let worker = use_hook(|| {
        Arc::new(Mutex::new(
            AcademyWorker::spawn().expect("Academy worker must start"),
        ))
    });
    let mut model = use_signal(UiModel::default);
    let mut composer = use_signal(String::new);
    let mut history_open = use_signal(|| false);
    let mut skills_open = use_signal(|| false);
    let mut shared_surface = use_signal(VisualSurface::blank);
    let mut organism_surface =
        use_signal(|| VisualSurface::new(SURFACE_WIDTH, SURFACE_HEIGHT, [24, 31, 37, 255]));
    let mut drawing = use_signal(|| false);
    let mut last_point = use_signal(|| None::<(u32, u32)>);
    let mut canvas_extent = use_signal(|| (f64::from(SURFACE_WIDTH), f64::from(SURFACE_HEIGHT)));
    let mut canvas_element = use_signal(|| None::<MountedEvent>);

    let polling_worker = Arc::clone(&worker);
    use_future(move || {
        let polling_worker = Arc::clone(&polling_worker);
        async move {
            loop {
                let mut events = Vec::new();
                if let Ok(locked) = polling_worker.lock() {
                    while let Ok(Some(event)) = locked.try_event() {
                        events.push(event);
                    }
                }
                if !events.is_empty() {
                    for event in events {
                        apply_worker_event(
                            event,
                            &mut model,
                            &mut shared_surface,
                            &mut organism_surface,
                        );
                    }
                }
                Delay::new(Duration::from_millis(48)).await;
            }
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
        document::Title { "Academy" }
        style { {STYLESHEET} }
        div { class: "app-shell",
            header { class: "topbar",
                div { class: "run-status",
                    span { class: if ready { "status-dot ready" } else { "status-dot" } }
                    div {
                        strong { "{model.read().status}" }
                        span {
                            if let Some(ref state) = inspector {
                                "t{state.physical_tick} · {state.body_fingerprint}"
                            } else {
                                "Starting"
                            }
                        }
                    }
                }
            }

            main { class: "workspace",
                section { class: "runtime-strip", aria_label: "Runtime",
                    h2 { "Runtime" }
                    if let Some(state) = inspector {
                        dl { class: "runtime-metrics",
                            Metric { term: "Version", value: format!("v{}", state.body_version) }
                            Metric { term: "Tick", value: state.physical_tick.to_string() }
                            Metric { term: "Phase", value: state.pressure_phase.to_string() }
                            Metric { term: "Arenas", value: state.resident_arenas.to_string() }
                            Metric { term: "Peak", value: state.active_arena_max.to_string() }
                            Metric { term: "Crossings", value: state.crossing_total.to_string() }
                            Metric { term: "Last work", value: state.last_run_work.to_string() }
                            Metric { term: "Total work", value: state.physical_work_total.to_string() }
                            Metric { term: "Body", value: format_bytes(state.durable_bytes) }
                            Metric { term: "Resident", value: format_bytes(state.last_run_bytes) }
                        }
                    } else {
                        div { class: "runtime-loading", "Starting…" }
                    }
                }

                section { class: "conversation-region", aria_label: "World",
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
                                div { class: "section-actions",
                                    label { class: "toggle-control",
                                        input {
                                            r#type: "checkbox",
                                            checked: model.read().debug_overlay,
                                            onchange: move |event| model.write().debug_overlay = event.checked(),
                                        }
                                        span { "Overlay" }
                                    }
                                    button {
                                        class: "quiet-button",
                                        onclick: move |_| shared_surface.set(VisualSurface::blank()),
                                        "Clear"
                                    }
                                }
                            }
                            div { class: "canvas-wrap",
                                canvas {
                                    id: "shared-canvas",
                                    width: "{SURFACE_WIDTH}",
                                    height: "{SURFACE_HEIGHT}",
                                    aria_label: "Shared raster drawing surface",
                                    onmounted: move |event| {
                                        canvas_element.set(Some(event));
                                    },
                                    onmousedown: move |event| {
                                        event.prevent_default();
                                        let point = event.element_coordinates();
                                        let mounted = canvas_element.read().clone();
                                        async move {
                                            if let Some(element) = mounted {
                                                if let Ok(rect) = element.get_client_rect().await {
                                                    canvas_extent.set((rect.width(), rect.height()));
                                                }
                                            }
                                            drawing.set(true);
                                            let (width, height) = canvas_extent();
                                            let current = map_canvas_point(point.x, point.y, width, height);
                                            last_point.set(Some(current));
                                            shared_surface.write().draw_line(current, current, [21, 92, 81, 255], 2);
                                        }
                                    },
                                    onmousemove: move |event| {
                                        if !drawing() {
                                            return;
                                        }
                                        let point = event.element_coordinates();
                                        let (width, height) = canvas_extent();
                                        let current = map_canvas_point(point.x, point.y, width, height);
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
                                                "Drawing",
                                            );
                                        }
                                    },
                                    "Admit"
                                }
                                label { class: "file-button",
                                    "Image"
                                    input {
                                        r#type: "file",
                                        accept: "image/png,image/jpeg,image/gif,image/webp,image/bmp",
                                        onchange: {
                                            let worker = Arc::clone(&worker);
                                            move |event| {
                                                let worker = Arc::clone(&worker);
                                                async move {
                                                    if let Some(file) = event.files().first().cloned() {
                                                        let name = file.name();
                                                        match file.read_bytes().await {
                                                            Ok(bytes) => {
                                                                match VisualSurface::from_encoded_image(bytes.as_ref()) {
                                                                    Ok(surface) => {
                                                                        shared_surface.set(surface.clone());
                                                                        admit_raster(&worker, &mut model, surface, &format!("Image · {name}"));
                                                                    }
                                                                    Err(error) => set_error(&mut model, error.to_string()),
                                                                }
                                                            }
                                                            Err(error) => set_error(&mut model, error.to_string()),
                                                        }
                                                    }
                                                }
                                            }
                                        },
                                    }
                                }
                                label { class: "file-button",
                                    "File"
                                    input {
                                        r#type: "file",
                                        onchange: {
                                            let worker = Arc::clone(&worker);
                                            move |event| {
                                                let worker = Arc::clone(&worker);
                                                async move {
                                                    if let Some(file) = event.files().first().cloned() {
                                                        let name = file.name();
                                                        match file.read_bytes().await {
                                                            Ok(bytes) => {
                                                                let rendered = match VisualSurface::from_encoded_image(bytes.as_ref()) {
                                                                    Ok(surface) => surface,
                                                                    Err(_) => VisualSurface::render_text(&String::from_utf8_lossy(&bytes)),
                                                                };
                                                                shared_surface.set(rendered.clone());
                                                                model.write().file_notice = format!("Loaded {name}");
                                                                admit_raster(&worker, &mut model, rendered, &format!("Rendered file · {name}"));
                                                            }
                                                            Err(error) => set_error(&mut model, error.to_string()),
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

                    }

                }

                aside { class: "academy-region", aria_label: "Output",
                    section { class: "academy-section output-section",
                        div { class: "canvas-wrap output",
                            canvas {
                                id: "organism-canvas",
                                width: "{SURFACE_WIDTH}",
                                height: "{SURFACE_HEIGHT}",
                                aria_label: "Organism raster output",
                            }
                            div { class: "output-toolbar",
                                button {
                                    class: "quiet-button",
                                    onclick: {
                                        let worker = Arc::clone(&worker);
                                        move |_| send_command(&worker, &mut model, AcademyCommand::SaveCheckpoint)
                                    },
                                    "Save"
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
                                    "Replay"
                                }
                            }
                        }
                    }

                }
            }

            form {
                class: "composer",
                onsubmit: {
                    let worker = Arc::clone(&worker);
                    move |event| {
                        event.prevent_default();
                        let text = composer.read().trim().to_string();
                        let mode = model.read().mode;
                        if model.read().busy
                            || (mode == ExperienceMode::Teach && text.is_empty())
                        {
                            return;
                        }
                        submit_text(&worker, &mut model, &text);
                        composer.set(String::new());
                    }
                },
                button {
                    class: "dock-button",
                    r#type: "button",
                    aria_label: "History",
                    aria_expanded: history_open(),
                    title: "History",
                    onclick: move |_| {
                        let opening = !history_open();
                        history_open.set(opening);
                        if opening {
                            skills_open.set(false);
                        }
                    },
                    span { class: "history-icon", aria_hidden: "true",
                        i {}
                        i {}
                        i {}
                        i {}
                    }
                }
                button {
                    class: "dock-button",
                    r#type: "button",
                    aria_label: "Skills",
                    aria_expanded: skills_open(),
                    title: "Skills",
                    onclick: move |_| {
                        let opening = !skills_open();
                        skills_open.set(opening);
                        if opening {
                            history_open.set(false);
                        }
                    },
                    span { class: "skills-icon", aria_hidden: "true",
                        i {}
                        i {}
                        i {}
                    }
                }
                div { class: "mode-switch", role: "group", aria_label: "Experience mode",
                    button {
                        r#type: "button",
                        class: if model.read().mode == ExperienceMode::Teach { "mode-button active" } else { "mode-button" },
                        onclick: move |_| model.write().mode = ExperienceMode::Teach,
                        "Teach"
                    }
                    button {
                        r#type: "button",
                        class: if model.read().mode == ExperienceMode::Probe { "mode-button active" } else { "mode-button" },
                        onclick: move |_| model.write().mode = ExperienceMode::Probe,
                        "Probe"
                    }
                }
                textarea {
                    value: "{composer}",
                    placeholder: if model.read().mode == ExperienceMode::Teach {
                        "Name a generated teaching case…"
                    } else {
                        "Run a fresh held-out probe"
                    },
                    aria_label: "Experience",
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
                                submit_text(&worker, &mut model, &text);
                                composer.set(String::new());
                            }
                        }
                    },
                }
                button {
                    class: "primary-button",
                    r#type: "submit",
                    disabled: model.read().busy
                        || (model.read().mode == ExperienceMode::Teach
                            && composer.read().trim().is_empty()),
                    if model.read().busy { "Running…" } else { "Run" }
                }
            }

            if history_open() {
                section { class: "timeline-region", aria_label: "History",
                    div { class: "timeline-heading",
                        div {
                            h2 { "History" }
                        }
                        div { class: "timeline-actions",
                            span { "{model.read().timeline.len() + model.read().a1_timeline.len()} records" }
                            button {
                                class: "quiet-button",
                                r#type: "button",
                                onclick: move |_| history_open.set(false),
                                "Close"
                            }
                        }
                    }
                    div { class: "timeline-track",
                        if model.read().timeline.is_empty() && model.read().a1_timeline.is_empty() {
                            div { class: "timeline-empty",
                                strong { "No activity yet" }
                            }
                        }
                        for record in model.read().a1_timeline.iter().rev().take(24) {
                            article { class: "history-card", key: "a1-experience-{record.id}",
                                div { class: match record.kind {
                                    A1ExperienceKind::Teach => "history-preview teach",
                                    A1ExperienceKind::Probe(_) => "history-preview probe",
                                },
                                    div { class: "history-topline",
                                        span { class: match record.kind {
                                            A1ExperienceKind::Teach => "mode-label teach",
                                            A1ExperienceKind::Probe(_) => "mode-label probe",
                                        },
                                            {match record.kind {
                                                A1ExperienceKind::Teach => "Teach",
                                                A1ExperienceKind::Probe(_) => "Probe",
                                            }}
                                        }
                                        strong { "{record.case_id}" }
                                        if record.replay_exact == Some(true) {
                                            span { class: "probe-result pass", "Exact" }
                                        }
                                    }
                                    p { "{record.capability_id}" }
                                    div { class: "history-signal", aria_hidden: "true",
                                        i {}
                                        i {}
                                        i {}
                                        i {}
                                    }
                                }
                                dl { class: "history-facts",
                                    div { dt { "Work" } dd { "{record.observation.physical_work}" } }
                                    div { dt { "Crossings" } dd { "{record.crossings.len()}" } }
                                    div { dt { "Clock" } dd { "{record.observation.clock_start}→{record.observation.clock_end}" } }
                                }
                                button {
                                    class: "quiet-button",
                                    r#type: "button",
                                    onclick: {
                                        let worker = Arc::clone(&worker);
                                        let id = record.id.clone();
                                        move |_| send_command(
                                            &worker,
                                            &mut model,
                                            AcademyCommand::ReplayExperience(id.clone()),
                                        )
                                    },
                                    "Replay"
                                }
                            }
                        }
                        for record in model.read().timeline.iter().rev().take(24) {
                            article { class: "history-card", key: "experience-{record.id}",
                                div { class: if record.mode == ExperienceMode::Teach { "history-preview teach" } else { "history-preview probe" },
                                    div { class: "history-topline",
                                        span { class: mode_class(record.mode), "{record.mode.label()}" }
                                        strong { "#{record.id}" }
                                        if let Some(passed) = record.probe_passed {
                                            span { class: if passed { "probe-result pass" } else { "probe-result not-yet" },
                                                if passed { "Pass" } else { "Not yet" }
                                            }
                                        }
                                    }
                                    p { "{record.admission.input_summary}" }
                                    div { class: "history-signal", aria_hidden: "true",
                                        i {}
                                        i {}
                                        i {}
                                        i {}
                                    }
                                }
                                dl { class: "history-facts",
                                    div { dt { "Work" } dd { "{record.physical_work}" } }
                                    div { dt { "Crossings" } dd { "{record.crossings.len()}" } }
                                    div { dt { "Clock" } dd { "{record.clock_start}→{record.clock_end}" } }
                                }
                            }
                        }
                    }
                }
            }

            if skills_open() {
                section { class: "skills-space", aria_label: "Skills",
                    div { class: "timeline-heading",
                        div {
                            h2 { "Skills" }
                        }
                        div { class: "timeline-actions",
                            span {
                                "{model.read().capabilities.stable_count()} stable · {model.read().capabilities.frontier_count()} learning"
                            }
                            button {
                                class: "quiet-button",
                                r#type: "button",
                                onclick: move |_| skills_open.set(false),
                                "Close"
                            }
                        }
                    }
                    div { class: "skills-space-layout",
                        div { class: "skills-gallery",
                            for capability in model.read().capabilities.capabilities() {
                                button {
                                    key: "cap-{capability.id}",
                                    class: if capability.id == model.read().selected_capability { "skill-card selected" } else { "skill-card" },
                                    onclick: {
                                        let id = capability.id.clone();
                                        move |_| model.write().selected_capability = id.clone()
                                    },
                                    div { class: "skill-card-topline",
                                        span { class: status_class(capability), "{capability.status.label()}" }
                                        small {
                                            "{capability.evidence.fresh_passes}/{capability.evidence.fresh_attempts}"
                                        }
                                    }
                                    strong { "{capability.title}" }
                                    span { class: "skill-meter", aria_hidden: "true",
                                        i {
                                            style: "width: {capability_progress(capability)}%"
                                        }
                                    }
                                }
                            }
                        }
                        div { class: "skill-detail-space",
                            if let Some(capability) = selected {
                                CapabilityDetail { capability }
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
            if !capability.prerequisites.is_empty() {
                div { class: "prerequisites",
                    strong { "Requires" }
                    span { {capability.prerequisites.join(" · ")} }
                }
            }
            dl { class: "evidence-table",
                div { dt { "Teach" } dd { "{evidence.teach_experiences}" } }
                div { dt { "Fresh" } dd { "{evidence.fresh_passes} / {evidence.fresh_attempts}" } }
                div { dt { "Transfer" } dd { "{evidence.transfer_passes} / {evidence.transfer_attempts}" } }
                div { dt { "Retention" } dd { "{evidence.retention_passes} / {evidence.retention_attempts}" } }
                div { dt { "Median work" } dd { {evidence.median_work().map_or("—".to_string(), |value| value.to_string())} } }
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

fn apply_worker_event(
    event: AcademyEvent,
    model: &mut Signal<UiModel>,
    shared_surface: &mut Signal<VisualSurface>,
    organism_surface: &mut Signal<VisualSurface>,
) {
    match event {
        AcademyEvent::Ready(snapshot) => {
            eprintln!("ACADEMY_PLAYGROUND_READY");
            update_snapshot(model, *snapshot);
            let mut state = model.write();
            state.status = "Ready".to_string();
            state.busy = false;
        }
        AcademyEvent::Completed {
            record,
            organism_surface: output,
            snapshot,
        } => {
            let record = *record;
            let result = record
                .probe_passed
                .map(|passed| if passed { "PASS" } else { "NOT YET" });
            let body = if record.organism_text.is_empty() {
                format!("{} crossings", record.crossings.len())
            } else {
                record.organism_text.clone()
            };
            model.write().messages.push(Message {
                speaker: Speaker::Organism,
                title: "Output".to_string(),
                body,
                detail: result.map_or_else(
                    || {
                        format!(
                            "{} work · {} crossings",
                            record.physical_work,
                            record.crossings.len()
                        )
                    },
                    |result| format!("{result} · {} work", record.physical_work),
                ),
            });
            organism_surface.set(*output);
            update_snapshot(model, *snapshot);
            let mut state = model.write();
            state.status = "Ready".to_string();
            state.busy = false;
        }
        AcademyEvent::TeachingCaseReady { case, snapshot } => {
            update_snapshot(model, *snapshot);
            let mut state = model.write();
            state.status = "Teaching case ready".to_string();
            state.replay_message = format!("{} · seed {}", case.id, case.seed);
            state.busy = false;
        }
        AcademyEvent::A1Completed { record, snapshot } => {
            let record = *record;
            shared_surface.set(record.shared_world_surface.clone());
            organism_surface.set(record.organism_surface.clone());
            let (title, body) = match record.kind {
                A1ExperienceKind::Teach => (
                    "Teaching".to_string(),
                    format!(
                        "Completed physical consequence loop · {} updates",
                        record.observation.plasticity_updates
                    ),
                ),
                A1ExperienceKind::Probe(family) => (
                    "Fresh probe".to_string(),
                    format!(
                        "{family:?} · {} relation crossing(s) · no teaching",
                        record.observation.outward_relation_crossings
                    ),
                ),
            };
            model.write().messages.push(Message {
                speaker: Speaker::Academy,
                title,
                body,
                detail: format!(
                    "{} work · replay {}",
                    record.observation.physical_work,
                    if record.replay_exact == Some(true) {
                        "exact"
                    } else {
                        "pending"
                    }
                ),
            });
            update_snapshot(model, *snapshot);
            let mut state = model.write();
            state.status = match record.kind {
                A1ExperienceKind::Teach => "Taught",
                A1ExperienceKind::Probe(A1ProbeFamily::LearnedRelation)
                    if record.observation.outward_relation_crossings == 1 =>
                {
                    "Probe passed"
                }
                A1ExperienceKind::Probe(_) => "Control observed",
            }
            .to_string();
            state.busy = false;
        }
        AcademyEvent::CheckpointSaved {
            body_version,
            snapshot,
        } => {
            update_snapshot(model, *snapshot);
            let mut state = model.write();
            state.status = "Saved".to_string();
            state.replay_message = format!("Saved v{body_version}");
            state.busy = false;
        }
        AcademyEvent::CheckpointRestored {
            body_version,
            snapshot,
        } => {
            update_snapshot(model, *snapshot);
            let mut state = model.write();
            state.status = "Restored".to_string();
            state.replay_message = format!("Restored v{body_version}");
            state.busy = false;
        }
        AcademyEvent::ReplayVerified { outcome, snapshot } => {
            let outcome = *outcome;
            update_snapshot(model, *snapshot);
            let mut state = model.write();
            state.status = if outcome.exact {
                "Replay exact"
            } else {
                "Replay diverged"
            }
            .to_string();
            state.replay_message = if outcome.exact {
                format!(
                    "Exact · t{} · {} work",
                    outcome.observed_clock, outcome.observed_work
                )
            } else {
                format!(
                    "Replay diverged · expected {}, observed {}",
                    outcome.expected_body, outcome.observed_body
                )
            };
            state.busy = false;
        }
        AcademyEvent::A1ReplayVerified { outcome, snapshot } => {
            let outcome = *outcome;
            update_snapshot(model, *snapshot);
            let mut state = model.write();
            state.status = if outcome.exact {
                "Replay exact"
            } else {
                "Replay diverged"
            }
            .to_string();
            state.replay_message = format!(
                "{} · crossings {} · body {} · clock {} · work {}",
                outcome.experience_id,
                outcome.crossings_exact,
                outcome.body_exact,
                outcome.clock_exact,
                outcome.work_exact,
            );
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
    state.a1_timeline = snapshot.a1_timeline;
}

fn submit_text(
    worker: &Arc<Mutex<AcademyWorker>>,
    model: &mut Signal<UiModel>,
    text: &str,
) {
    let mode = model.read().mode;
    let command = if mode == ExperienceMode::Teach {
        AcademyCommand::StartAndTeach(TeachingCase::generated_text(stable_seed(text)))
    } else {
        AcademyCommand::ProbeActiveCase(A1ProbeFamily::LearnedRelation)
    };
    model.write().messages.push(Message {
        speaker: Speaker::Human,
        title: "You".to_string(),
        body: if mode == ExperienceMode::Teach {
            text.to_string()
        } else {
            "Run a fresh held-out relation probe".to_string()
        },
        detail: mode.label().to_string(),
    });
    send_command(worker, model, command);
}

fn stable_seed(text: &str) -> u64 {
    text.as_bytes()
        .iter()
        .fold(0xcbf29ce484222325_u64, |state, byte| {
            state.wrapping_mul(0x100000001b3) ^ u64::from(*byte)
        })
}

fn send_command(
    worker: &Arc<Mutex<AcademyWorker>>,
    model: &mut Signal<UiModel>,
    command: AcademyCommand,
) {
    match worker.lock() {
        Ok(locked) => match locked.try_command(command) {
            Ok(()) => {
                let mut state = model.write();
                state.busy = true;
                state.status = "Running".to_string();
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
        academy_note: format!("Unscored raster interaction · {label}"),
    };
    model.write().messages.push(Message {
        speaker: Speaker::Human,
        title: "You".to_string(),
        body: label.to_string(),
        detail: mode.label().to_string(),
    });
    send_command(worker, model, AcademyCommand::Interact(request));
}

fn set_error(model: &mut Signal<UiModel>, message: String) {
    let mut state = model.write();
    state.status = "Error".to_string();
    state.busy = false;
    state.messages.push(Message {
        speaker: Speaker::Academy,
        title: "Error".to_string(),
        body: message,
        detail: String::new(),
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

fn map_canvas_point(x: f64, y: f64, displayed_width: f64, displayed_height: f64) -> (u32, u32) {
    let width = displayed_width.max(1.0);
    let height = displayed_height.max(1.0);
    (
        (x / width * f64::from(SURFACE_WIDTH))
            .floor()
            .clamp(0.0, f64::from(SURFACE_WIDTH - 1)) as u32,
        (y / height * f64::from(SURFACE_HEIGHT))
            .floor()
            .clamp(0.0, f64::from(SURFACE_HEIGHT - 1)) as u32,
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

fn capability_progress(capability: &Capability) -> u8 {
    match capability.status {
        academy_core::CapabilityStatus::Unknown => 8,
        academy_core::CapabilityStatus::Emerging => 28,
        academy_core::CapabilityStatus::Acquired => 48,
        academy_core::CapabilityStatus::General => 66,
        academy_core::CapabilityStatus::Stable => 84,
        academy_core::CapabilityStatus::Automatic => 100,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displayed_canvas_coordinates_map_to_physical_raster() {
        assert_eq!(map_canvas_point(0.0, 0.0, 1280.0, 720.0), (0, 0));
        assert_eq!(map_canvas_point(640.0, 360.0, 1280.0, 720.0), (320, 180));
        assert_eq!(map_canvas_point(1280.0, 720.0, 1280.0, 720.0), (639, 359));
    }

    #[test]
    fn coordinate_mapping_is_independent_of_display_scale() {
        assert_eq!(map_canvas_point(160.0, 90.0, 320.0, 180.0), (320, 180));
        assert_eq!(map_canvas_point(960.0, 540.0, 1920.0, 1080.0), (320, 180));
    }
}
