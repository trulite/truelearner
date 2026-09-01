
const MAX_MOTIF_REENTRY_LINK_VISITS: u16 = 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum OutcomeForm {
    None,
    One(Junction),
    Ambiguous,
}

#[derive(Clone, Copy, Debug)]
struct RetainedMotif {
    witness: LinkId,
    parent: LinkId,
    closed: PathForm,
    prior: PathForm,
    closed_outcome: OutcomeForm,
    prior_outcome: OutcomeForm,
}

#[derive(Clone, Copy, Debug)]
struct MotifRouteCandidate {
    surface: JunctionId,
    output: JunctionId,
    through: LinkId,
    impulse: i32,
    outcome_source: Option<JunctionId>,
    form: PathForm,
    outcome: OutcomeForm,
}

fn mark_motif_reentries(
    body: ReactionView<'_>,
    paths: &mut [ReadyPath],
    present: &[JunctionId],
    construction: bool,
) {
    if construction
        || !paths
            .iter()
            .any(|path| path.executable && matches!(path.first, LinkRef::New(_)))
    {
        return;
    }

    let mut visits = 0_u16;
    let mut motifs = Vec::new();
    for slot in 0..body.link_memory.len() {
        if visits >= MAX_MOTIF_REENTRY_LINK_VISITS {
            for path in paths
                .iter_mut()
                .filter(|path| matches!(path.first, LinkRef::New(_)))
            {
                path.motif_reentries.clear();
                path.motif_routes = None;
                path.reentry_incidence_visits =
                    path.reentry_incidence_visits.saturating_add(visits);
            }
            return;
        }
        visits += 1;
        let Some(witness) = LinkId::new(slot) else {
            continue;
        };
        let Some(parent) = body.link_memory[slot].motif_parent() else {
            continue;
        };
        let Some(motif) = retained_motif(body, witness, parent) else {
            continue;
        };
        motifs.push(motif);
    }
    for index in 0..paths.len() {
        if !paths[index].executable || !matches!(paths[index].first, LinkRef::New(_)) {
            continue;
        }
        for motif in &motifs {
            if paths[index].form != motif.closed
                || outcome_form(body, paths[index].output) != motif.closed_outcome
            {
                continue;
            }
            let has_prior = paths.iter().enumerate().any(|(other, path)| {
                other != index
                    && path.surface == paths[index].surface
                    && path.output != paths[index].output
                    && matches!(path.first, LinkRef::New(_))
                    && path.form == motif.prior
                    && outcome_form(body, path.output) == motif.prior_outcome
            });
            if has_prior
                && !paths[index]
                    .motif_reentries
                    .iter()
                    .any(|support| support.witness == motif.witness)
            {
                paths[index].motif_reentries.push(MotifReentryTrace {
                    witness: motif.witness,
                    parent: motif.parent,
                });
            }
        }
    }
    for path in paths
        .iter_mut()
        .filter(|path| matches!(path.first, LinkRef::New(_)))
    {
        path.reentry_incidence_visits = path.reentry_incidence_visits.saturating_add(visits);
        if present.is_empty() || path.motif_reentries.is_empty() {
            continue;
        }
        let Some(surface) = path.outcome_source.filter(|source| *source != path.surface) else {
            continue;
        };
        let mut route_visits = visits;
        match find_motif_routes(
            body,
            surface,
            path.surface,
            present,
            &motifs,
            &mut route_visits,
        ) {
            Ok(routes) if routes.is_empty() => {}
            Ok(routes) => path.motif_routes = Some(routes.into_boxed_slice()),
            Err(()) => path.motif_route_failed = true,
        }
        path.reentry_incidence_visits = path
            .reentry_incidence_visits
            .saturating_add(route_visits.saturating_sub(visits));
    }
}

fn find_motif_routes(
    body: ReactionView<'_>,
    surface: JunctionId,
    forbidden_condition: JunctionId,
    present: &[JunctionId],
    motifs: &[RetainedMotif],
    incidence_visits: &mut u16,
) -> Result<Vec<MotifRouteTrace>, ()> {
    let mut search = MotifRouteSearch {
        body,
        forbidden_condition,
        present,
        motifs,
        incidence_visits,
        stack: Vec::new(),
        steps: Vec::new(),
        found: Vec::new(),
    };
    search.search(surface, 0)?;
    Ok(search.found)
}

struct MotifRouteSearch<'body, 'scratch> {
    body: ReactionView<'body>,
    forbidden_condition: JunctionId,
    present: &'scratch [JunctionId],
    motifs: &'scratch [RetainedMotif],
    incidence_visits: &'scratch mut u16,
    stack: Vec<JunctionId>,
    steps: Vec<MotifRouteStepTrace>,
    found: Vec<MotifRouteTrace>,
}

impl MotifRouteSearch<'_, '_> {
    fn search(&mut self, surface: JunctionId, depth: usize) -> Result<(), ()> {
        if depth >= MAX_REENTRY_DEPTH || self.stack.contains(&surface) {
            return Err(());
        }
        let Some(law) = self
            .body
            .arena
            .junction(surface)
            .map(|law| law.checkpoint_law())
        else {
            return Err(());
        };
        // Without an actual sampled event, no physical evidence selects its
        // future `Rises` or `Falls` path form.
        if matches!(law.retention, Retention::Sampled { .. })
            || !path_is_executable(self.body, surface, false)
        {
            return Ok(());
        }
        self.stack.push(surface);
        let candidates = motif_route_candidates(self.body, surface, self.incidence_visits)?;
        for (index, candidate) in candidates.iter().copied().enumerate() {
            let supports = motif_route_supports(self.motifs, &candidates, index);
            if supports.is_empty() {
                continue;
            }
            let Some(outcome_source) = candidate.outcome_source else {
                continue;
            };
            if outcome_source == surface || outcome_source == self.forbidden_condition {
                continue;
            }
            self.steps.push(MotifRouteStepTrace {
                surface: candidate.surface,
                output: candidate.output,
                through: candidate.through,
                impulse: candidate.impulse,
                outcome_source,
                supports,
            });
            if self.present.contains(&outcome_source) {
                let route = MotifRouteTrace {
                    condition: outcome_source,
                    steps: self.steps.clone(),
                };
                if !self.found.contains(&route) {
                    self.found.push(route);
                }
            } else {
                self.search(outcome_source, depth + 1)?;
            }
            self.steps.pop();
        }
        self.stack.pop();
        Ok(())
    }
}

fn motif_route_candidates(
    body: ReactionView<'_>,
    surface: JunctionId,
    incidence_visits: &mut u16,
) -> Result<Vec<MotifRouteCandidate>, ()> {
    let Some(surface_law) = body.arena.junction(surface).map(|law| law.checkpoint_law()) else {
        return Err(());
    };
    let mut candidates = Vec::new();
    let mut next = body
        .arena
        .junction(surface)
        .and_then(|junction| junction.outgoing_head);
    while let Some(through) = next {
        visit_motif_route_incidence(incidence_visits)?;
        let physical = *body
            .arena
            .link(through)
            .expect("live motif-route incidence");
        next = physical.next;
        let memory = &body.link_memory[through.slot()];
        if memory.live && memory.role == LinkRole::PathEntry {
            return Ok(Vec::new());
        }
        if !memory.live
            || memory.role != LinkRole::Drive
            || physical.impulse != 0
            || !(1..=LOCAL_RADIUS as Time).contains(&physical.delay)
        {
            continue;
        }
        let (outcome, outcome_source) = motif_route_outcome(body, physical.to, incidence_visits)?;
        for impulse in [1, -1] {
            candidates.push(MotifRouteCandidate {
                surface,
                output: physical.to,
                through,
                impulse,
                outcome_source,
                form: PathForm {
                    surface: surface_law,
                    first: LinkForm {
                        delay: physical.delay,
                        impulse: 1,
                        trigger: Trigger::SourceFires,
                    },
                    second: LinkForm {
                        delay: physical.delay,
                        impulse,
                        trigger: Trigger::SourceFires,
                    },
                },
                outcome,
            });
        }
    }
    Ok(candidates)
}

fn motif_route_outcome(
    body: ReactionView<'_>,
    output: JunctionId,
    incidence_visits: &mut u16,
) -> Result<(OutcomeForm, Option<JunctionId>), ()> {
    let mut selected = None;
    for witness in body.arena.incoming(output) {
        visit_motif_route_incidence(incidence_visits)?;
        let memory = &body.link_memory[witness.slot()];
        if !memory.live || memory.role != LinkRole::OutcomeWitness {
            continue;
        }
        let source = body.arena.link(witness).expect("live outcome witness").from;
        let Some(law) = body
            .arena
            .junction(source)
            .map(|source| source.checkpoint_law())
        else {
            return Ok((OutcomeForm::Ambiguous, None));
        };
        match selected {
            None => selected = Some((source, law)),
            Some((existing, _)) if existing != source => {
                return Ok((OutcomeForm::Ambiguous, None));
            }
            Some(_) => {}
        }
    }
    Ok(selected.map_or((OutcomeForm::None, None), |(source, law)| {
        (OutcomeForm::One(law), Some(source))
    }))
}

fn motif_route_supports(
    motifs: &[RetainedMotif],
    candidates: &[MotifRouteCandidate],
    index: usize,
) -> Vec<MotifReentryTrace> {
    let candidate = candidates[index];
    let mut supports = Vec::new();
    for motif in motifs {
        if candidate.form != motif.closed || candidate.outcome != motif.closed_outcome {
            continue;
        }
        let has_prior = candidates.iter().enumerate().any(|(other, prior)| {
            other != index
                && prior.output != candidate.output
                && prior.form == motif.prior
                && prior.outcome == motif.prior_outcome
        });
        if has_prior
            && !supports
                .iter()
                .any(|support: &MotifReentryTrace| support.witness == motif.witness)
        {
            supports.push(MotifReentryTrace {
                witness: motif.witness,
                parent: motif.parent,
            });
        }
    }
    supports
}

fn visit_motif_route_incidence(incidence_visits: &mut u16) -> Result<(), ()> {
    if *incidence_visits >= MAX_MOTIF_REENTRY_LINK_VISITS {
        return Err(());
    }
    *incidence_visits += 1;
    Ok(())
}

fn retained_motif(
    body: ReactionView<'_>,
    witness: LinkId,
    parent: LinkId,
) -> Option<RetainedMotif> {
    let child = closed_step(body, witness)?;
    let parent_step = closed_step(body, parent)?;
    if closed_step_is_valid(body, child)? != child.path.output
        || closed_step_is_valid(body, parent_step)? != parent_step.path.output
    {
        return None;
    }
    let child_prior = unique_prior_unclosed_sibling(body, child.path)?;
    let parent_prior = unique_prior_unclosed_sibling(body, parent_step.path)?;
    let closed = retained_path_form(body, child.path)?;
    let prior = retained_path_form(body, child_prior)?;
    if closed != retained_path_form(body, parent_step.path)?
        || prior != retained_path_form(body, parent_prior)?
    {
        return None;
    }
    let closed_outcome = outcome_form(body, child.path.output);
    let prior_outcome = outcome_form(body, child_prior.output);
    if !matches!(closed_outcome, OutcomeForm::One(_))
        || closed_outcome != outcome_form(body, parent_step.path.output)
        || prior_outcome != outcome_form(body, parent_prior.output)
    {
        return None;
    }
    Some(RetainedMotif {
        witness,
        parent,
        closed,
        prior,
        closed_outcome,
        prior_outcome,
    })
}

fn outcome_form(body: ReactionView<'_>, output: JunctionId) -> OutcomeForm {
    let mut selected = None;
    for witness in body.arena.incoming(output) {
        let memory = &body.link_memory[witness.slot()];
        if !memory.live || memory.role != LinkRole::OutcomeWitness {
            continue;
        }
        let source = body.arena.link(witness).expect("live outcome witness").from;
        let Some(law) = body
            .arena
            .junction(source)
            .map(|source| source.checkpoint_law())
        else {
            return OutcomeForm::Ambiguous;
        };
        match selected {
            None => selected = Some((source, law)),
            Some((existing, _)) if existing != source => return OutcomeForm::Ambiguous,
            Some(_) => {}
        }
    }
    selected.map_or(OutcomeForm::None, |(_, law)| OutcomeForm::One(law))
}

#[allow(clippy::too_many_arguments)]
fn find_reentries(
    body: ReactionView<'_>,
    path: Path,
    present: &[JunctionId],
    steps: &mut Vec<ReentryStepTrace>,
    continuations: &mut Vec<ReentryContinuation>,
    compilation: &mut ReentryCompilationScratch,
    incidence_visits: &mut u16,
    shortcut_hits: &mut u16,
) -> Result<Vec<ReentryTrace>, ()> {
    let mut search = ReentrySearch {
        body,
        present,
        steps,
        continuations,
        compilation,
        incidence_visits,
        shortcut_hits,
        found: Vec::new(),
    };
    search.search(path, 0)?;
    Ok(search.found)
}

struct ReentrySearch<'body, 'scratch> {
    body: ReactionView<'body>,
    present: &'scratch [JunctionId],
    steps: &'scratch mut Vec<ReentryStepTrace>,
    continuations: &'scratch mut Vec<ReentryContinuation>,
    compilation: &'scratch mut ReentryCompilationScratch,
    incidence_visits: &'scratch mut u16,
    shortcut_hits: &'scratch mut u16,
    found: Vec<ReentryTrace>,
}

impl ReentrySearch<'_, '_> {
    fn search(&mut self, path: Path, depth: usize) -> Result<(), ()> {
        self.compilation.frames.push(ReentryFrame {
            start: path,
            prefix_len: self.steps.len(),
            found_start: self.found.len(),
            dependencies: Vec::new(),
        });
        let result = self.search_open(path, depth);
        let frame = self.compilation.frames.pop().expect("current search frame");
        for dependency in &frame.dependencies {
            for parent in &mut self.compilation.frames {
                if !parent.dependencies.contains(dependency) {
                    parent.dependencies.push(*dependency);
                }
            }
        }
        if result.is_ok() && self.present.len() == 1 {
            let routes = self.found[frame.found_start..]
                .iter()
                .filter_map(|route| {
                    let steps = route.steps.get(frame.prefix_len..)?.to_vec();
                    (steps.first().map(|step| step.path) == Some(frame.start)).then_some(
                        ReentryTrace {
                            condition: route.condition,
                            steps,
                        },
                    )
                })
                .collect::<Vec<_>>();
            if !routes.is_empty() {
                self.compilation.rehearsals.push(ReentryRehearsal {
                    start: frame.start,
                    condition: self.present[0],
                    routes,
                    dependencies: frame.dependencies,
                });
            }
        }
        result
    }

    fn search_open(&mut self, path: Path, depth: usize) -> Result<(), ()> {
        if self.steps.iter().any(|step| step.path == path) {
            return Err(());
        }
        self.depend_on(path.middle);
        self.depend_on(path.output);
        if self.present.len() == 1 {
            if let Some(shortcut) = self.body.automaticity.and_then(|automaticity| {
                automaticity.usable_thought_shortcut(path, self.present[0])
            }) {
                return self.reuse_shortcut(shortcut);
            }
        }
        let depth = if usable_composite_for_reentry(self.body, path, self.incidence_visits)? {
            depth
        } else {
            depth.saturating_add(1)
        };
        if depth > MAX_REENTRY_DEPTH {
            return Err(());
        }
        let mut next = self
            .body
            .arena
            .junction(path.output)
            .and_then(|junction| junction.outgoing_head);
        while let Some(link) = next {
            visit_reentry_incidence(self.incidence_visits)?;
            let physical = *self.body.arena.link(link).expect("live retained incidence");
            next = physical.next;
            if physical.to != path.middle {
                continue;
            }
            let Some((returned_source, outcome_witness)) =
                self.body.link_memory[link.slot()].closed_support()
            else {
                continue;
            };
            let retained = ClosedStep {
                link,
                path,
                returned_source,
                outcome_witness,
            };
            let dependencies = &mut self
                .compilation
                .frames
                .last_mut()
                .expect("active search frame")
                .dependencies;
            let Some(outcome_target) = closed_step_is_valid_for_reentry(
                self.body,
                retained,
                dependencies,
                self.incidence_visits,
            )?
            else {
                return Err(());
            };
            self.depend_on(returned_source);
            self.depend_on(outcome_target);
            let witness = self.body.arena.link(outcome_witness).ok_or(())?;
            self.depend_on(witness.from);
            self.depend_on(witness.to);
            self.steps.push(ReentryStepTrace {
                path,
                returned_source,
                outcome_witness,
                outcome_target,
            });
            if self.present.contains(&returned_source) {
                self.found.push(ReentryTrace {
                    condition: returned_source,
                    steps: self.steps.clone(),
                });
            } else {
                let start = self.continuations.len();
                append_reentry_continuations(
                    self.body,
                    returned_source,
                    self.continuations,
                    self.incidence_visits,
                )?;
                let end = self.continuations.len();
                for index in start..end {
                    let continuation = self.continuations[index];
                    self.search(continuation.path, depth)?;
                }
                self.continuations.truncate(start);
            }
            self.steps.pop();
        }
        Ok(())
    }

    fn depend_on(&mut self, junction: JunctionId) {
        for frame in &mut self.compilation.frames {
            if !frame.dependencies.contains(&junction) {
                frame.dependencies.push(junction);
            }
        }
    }

    fn reuse_shortcut(&mut self, shortcut: &ThoughtShortcut) -> Result<(), ()> {
        for dependency in &shortcut.dependencies {
            self.depend_on(dependency.junction);
        }
        let prefix = self.steps.clone();
        for route in &shortcut.routes {
            if route.condition != shortcut.condition
                || route.steps.first().map(|step| step.path) != Some(shortcut.start)
            {
                return Err(());
            }
            for (index, step) in route.steps.iter().enumerate() {
                if prefix.iter().any(|existing| {
                    existing.path == step.path || existing.outcome_witness == step.outcome_witness
                }) || route.steps[..index].iter().any(|existing| {
                    existing.path == step.path || existing.outcome_witness == step.outcome_witness
                }) {
                    return Err(());
                }
            }
            let mut steps = prefix.clone();
            steps.extend_from_slice(&route.steps);
            self.found.push(ReentryTrace {
                condition: route.condition,
                steps,
            });
        }
        *self.shortcut_hits = self.shortcut_hits.saturating_add(1);
        Ok(())
    }
}

fn visit_reentry_incidence(incidence_visits: &mut u16) -> Result<(), ()> {
    if *incidence_visits >= MAX_REENTRY_INCIDENCE_VISITS {
        return Err(());
    }
    *incidence_visits += 1;
    Ok(())
}

fn append_reentry_continuations(
    body: ReactionView<'_>,
    surface: JunctionId,
    continuations: &mut Vec<ReentryContinuation>,
    incidence_visits: &mut u16,
) -> Result<(), ()> {
    let start = continuations.len();
    let mut next = body
        .arena
        .junction(surface)
        .and_then(|junction| junction.outgoing_head);
    while let Some(first_id) = next {
        visit_reentry_incidence(incidence_visits)?;
        let first = *body.arena.link(first_id).expect("live path incidence");
        next = first.next;
        let first_memory = &body.link_memory[first_id.slot()];
        if !first_memory.live || first_memory.role != LinkRole::PathEntry {
            continue;
        }
        let mut second = body
            .arena
            .junction(first.to)
            .and_then(|junction| junction.outgoing_head);
        while let Some(second_id) = second {
            visit_reentry_incidence(incidence_visits)?;
            let drive = *body.arena.link(second_id).expect("live drive incidence");
            second = drive.next;
            let memory = &body.link_memory[second_id.slot()];
            if !memory.live || memory.role != LinkRole::Drive || drive.impulse == 0 {
                continue;
            }
            let path = Path {
                surface,
                middle: first.to,
                output: drive.to,
                first: first_id,
                second: second_id,
            };
            if path_from_links(body, first_id, second_id) != Some(path)
                || continuations[start..]
                    .iter()
                    .any(|continuation| continuation.path == path)
            {
                continue;
            }
            continuations.push(ReentryContinuation { path });
        }
    }
    Ok(())
}

fn closed_step_is_valid_for_reentry(
    body: ReactionView<'_>,
    step: ClosedStep,
    dependencies: &mut Vec<JunctionId>,
    incidence_visits: &mut u16,
) -> Result<Option<JunctionId>, ()> {
    if path_from_links(body, step.path.first, step.path.second) != Some(step.path)
        || body.link_memory[step.path.first.slot()].participation == 0
        || body.link_memory[step.path.second.slot()].participation == 0
        || !path_is_executable_for_reentry(body, step.path.surface, dependencies, incidence_visits)?
    {
        return Ok(None);
    }
    let Some((witness, target)) = unique_outcome_witness_for_reentry(
        body,
        step.returned_source,
        step.path,
        incidence_visits,
    )?
    else {
        return Ok(None);
    };
    Ok((witness == step.outcome_witness).then_some(target))
}

fn path_is_executable_for_reentry(
    body: ReactionView<'_>,
    surface: JunctionId,
    dependencies: &mut Vec<JunctionId>,
    incidence_visits: &mut u16,
) -> Result<bool, ()> {
    let mut parent = None;
    for link in body.arena.incoming(surface) {
        visit_reentry_incidence(incidence_visits)?;
        if !is_membership_link(body, link) {
            continue;
        }
        let found = body
            .arena
            .link(link)
            .expect("live membership incidence")
            .from;
        if !dependencies.contains(&found) {
            dependencies.push(found);
        }
        if parent.is_some_and(|existing| existing != found) {
            return Ok(false);
        }
        parent = Some(found);
    }
    let Some(parent) = parent else {
        return Ok(true);
    };
    for link in body.arena.incoming(parent) {
        visit_reentry_incidence(incidence_visits)?;
        if is_membership_link(body, link) {
            return Ok(false);
        }
    }
    Ok(true)
}

fn unique_outcome_witness_for_reentry(
    body: ReactionView<'_>,
    source: JunctionId,
    path: Path,
    incidence_visits: &mut u16,
) -> Result<Option<(LinkId, JunctionId)>, ()> {
    let mut selected = None;
    for target in [path.middle, path.output] {
        for witness in body.arena.incoming(target) {
            visit_reentry_incidence(incidence_visits)?;
            let physical = body.arena.link(witness).expect("live outcome incidence");
            let memory = &body.link_memory[witness.slot()];
            if !memory.live || memory.role != LinkRole::OutcomeWitness || physical.from != source {
                continue;
            }
            if selected.is_some() {
                return Ok(None);
            }
            selected = Some((witness, target));
        }
    }
    Ok(selected)
}

fn closed_steps(body: ReactionView<'_>) -> impl Iterator<Item = ClosedStep> + '_ {
    (0..body.link_memory.len()).filter_map(move |slot| {
        let link = LinkId::new(slot)?;
        closed_step(body, link)
    })
}

fn path_from_entry(body: ReactionView<'_>, first: LinkId) -> Option<Path> {
    let entry = body.arena.link(first)?;
    let memory = body.link_memory.get(first.slot())?;
    if !memory.live || memory.role != LinkRole::PathEntry {
        return None;
    }
    let mut selected = None;
    let mut next = body
        .arena
        .junction(entry.to)
        .and_then(|junction| junction.outgoing_head);
    while let Some(second) = next {
        let physical = body.arena.link(second).expect("live path incidence");
        next = physical.next;
        let second_memory = &body.link_memory[second.slot()];
        if !second_memory.live || second_memory.role != LinkRole::Drive || physical.impulse == 0 {
            continue;
        }
        let path = path_from_links(body, first, second)?;
        if selected.replace(path).is_some() {
            return None;
        }
    }
    selected
}

fn unique_prior_unclosed_sibling(body: ReactionView<'_>, closed: Path) -> Option<Path> {
    let closed_at = body.link_memory.get(closed.second.slot())?.participated_at;
    unique_prior_unclosed_sibling_before(body, closed, closed_at)
}

fn unique_prior_unclosed_sibling_before(
    body: ReactionView<'_>,
    current: Path,
    current_at: Time,
) -> Option<Path> {
    let mut selected = None;
    let mut next = body
        .arena
        .junction(current.surface)
        .and_then(|junction| junction.outgoing_head);
    while let Some(first) = next {
        let physical = body.arena.link(first).expect("live surface incidence");
        next = physical.next;
        let Some(path) = path_from_entry(body, first) else {
            continue;
        };
        let memory = &body.link_memory[path.second.slot()];
        if path.output == current.output
            || memory.participation == 0
            || memory.participated_at >= current_at
            || memory.exact_closures != 0
            || !path_has_open_return(body, path.middle, path.output)
        {
            continue;
        }
        if selected.replace(path).is_some() {
            return None;
        }
    }
    selected
}

fn retained_path_form(body: ReactionView<'_>, path: Path) -> Option<PathForm> {
    let surface = body.arena.junction(path.surface)?.checkpoint_law();
    let first = body.arena.link(path.first)?;
    let second = body.arena.link(path.second)?;
    Some(PathForm {
        surface,
        first: LinkForm {
            delay: first.delay,
            impulse: first.impulse,
            trigger: first.trigger,
        },
        second: LinkForm {
            delay: second.delay,
            impulse: second.impulse,
            trigger: second.trigger,
        },
    })
}

fn same_path_form(body: ReactionView<'_>, left: Path, right: Path) -> bool {
    retained_path_form(body, left).is_some_and(|left| Some(left) == retained_path_form(body, right))
}

fn matching_closed_switch(
    body: ReactionView<'_>,
    current: Path,
    current_prior: Path,
    returned_source: JunctionId,
) -> Option<ClosedStep> {
    unique_outcome_witness(body, returned_source, current)?;
    for closed in closed_steps(body) {
        if closed.path.surface == current.surface
            || closed_step_is_valid(body, closed).is_none()
            || !same_path_form(body, closed.path, current)
        {
            continue;
        }
        let Some(prior) = unique_prior_unclosed_sibling(body, closed.path) else {
            continue;
        };
        if !same_path_form(body, prior, current_prior) {
            continue;
        }
        return Some(closed);
    }
    None
}

#[cold]
#[inline(never)]
fn matching_return_motif(
    body: ReactionView<'_>,
    returned: OpenReturn,
    returned_source: JunctionId,
) -> Option<ClosedStep> {
    body.link_memory[returned.link.slot()]
        .switched_from()
        .and_then(|prior| path_from_drive(body, prior))
        .and_then(|prior| matching_closed_switch(body, returned.path, prior, returned_source))
}

fn closed_step(body: ReactionView<'_>, link: LinkId) -> Option<ClosedStep> {
    let (returned_source, outcome_witness) = body.link_memory.get(link.slot())?.closed_support()?;
    let physical = body.arena.link(link)?;
    let second = outgoing_drive_to(body, physical.to, physical.from)?;
    let path = path_from_drive(body, second)?;
    (path.middle == physical.to && path.output == physical.from).then_some(ClosedStep {
        link,
        path,
        returned_source,
        outcome_witness,
    })
}

fn closed_step_is_valid(body: ReactionView<'_>, step: ClosedStep) -> Option<JunctionId> {
    if path_from_links(body, step.path.first, step.path.second) != Some(step.path)
        || body.link_memory[step.path.first.slot()].participation == 0
        || body.link_memory[step.path.second.slot()].participation == 0
        || !path_is_executable(body, step.path.surface, true)
    {
        return None;
    }
    let (witness, target) = unique_outcome_witness(body, step.returned_source, step.path)?;
    (witness == step.outcome_witness).then_some(target)
}

fn unique_outcome_witness(
    body: ReactionView<'_>,
    source: JunctionId,
    path: Path,
) -> Option<(LinkId, JunctionId)> {
    let mut selected = None;
    for target in [path.middle, path.output] {
        for witness in body.arena.incoming(target) {
            let physical = body.arena.link(witness).expect("live outcome support");
            let memory = &body.link_memory[witness.slot()];
            if !memory.live || memory.role != LinkRole::OutcomeWitness || physical.from != source {
                continue;
            }
            if selected.is_some() {
                return None;
            }
            selected = Some((witness, target));
        }
    }
    selected
}
