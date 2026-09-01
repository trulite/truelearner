use crate::{Body, Impulse, JunctionId, Link, Time, Trigger};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Port(usize);

#[derive(Clone, Debug)]
pub struct OpenBody {
    body: Body,
    ports: Vec<JunctionId>,
}

impl OpenBody {
    pub fn new(body: Body, ports: Vec<JunctionId>) -> Result<Self, OpenBodyError> {
        for (index, port) in ports.iter().copied().enumerate() {
            if body.arena.require(port).is_err() {
                return Err(OpenBodyError::UnknownPort(port));
            }
            if ports[..index].contains(&port) {
                return Err(OpenBodyError::DuplicatePort(port));
            }
        }
        Ok(Self { body, ports })
    }

    pub fn port(&self, index: usize) -> Option<Port> {
        (index < self.ports.len()).then_some(Port(index))
    }

    pub fn into_body(self) -> Body {
        self.body
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpenBodyError {
    UnknownPort(JunctionId),
    DuplicatePort(JunctionId),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    IntoHost,
    IntoPart,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Join {
    host: JunctionId,
    part: Port,
    direction: Direction,
    delay: Time,
    impulse: Impulse,
    trigger: Trigger,
}

impl Join {
    pub const fn into_host(host: JunctionId, part: Port, delay: Time, impulse: Impulse) -> Self {
        Self {
            host,
            part,
            direction: Direction::IntoHost,
            delay,
            impulse,
            trigger: Trigger::SourceFires,
        }
    }

    pub const fn into_part(host: JunctionId, part: Port, delay: Time, impulse: Impulse) -> Self {
        Self {
            host,
            part,
            direction: Direction::IntoPart,
            delay,
            impulse,
            trigger: Trigger::SourceFires,
        }
    }

    pub const fn when(mut self, trigger: Trigger) -> Self {
        self.trigger = trigger;
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Attachment {
    ports: Vec<JunctionId>,
}

impl Attachment {
    pub fn port(&self, port: Port) -> Option<JunctionId> {
        self.ports.get(port.0).copied()
    }

    pub fn is_empty(&self) -> bool {
        self.ports.is_empty()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AttachError {
    HostActive,
    PartActive,
    UnknownHost(JunctionId),
    UnknownPartPort(Port),
    CapacityExhausted,
}

#[derive(Clone, Debug)]
pub struct AttachFailure {
    error: AttachError,
    part: Box<OpenBody>,
}

impl AttachFailure {
    pub const fn error(&self) -> AttachError {
        self.error
    }

    pub fn into_part(self) -> OpenBody {
        *self.part
    }
}

pub fn attach(
    host: &mut Body,
    mut part: OpenBody,
    joins: &[Join],
) -> Result<Attachment, AttachFailure> {
    let validation = validate(host, &part, joins);
    if let Err(error) = validation {
        return Err(AttachFailure {
            error,
            part: Box::new(part),
        });
    }

    let part_junctions = part.body.arena.junction_count();
    let at = host.attachment_time().max(part.body.attachment_time());
    let part_arena = std::mem::take(&mut part.body.arena);
    let mut part_arrows = std::mem::take(&mut part.body.arrows);
    let part_consolidation = std::mem::take(&mut part.body.consolidation);
    let part_reentry = std::mem::take(&mut part.body.reentry);
    host.has_composites |= part.body.has_composites;
    let (junction_base, link_base) = host.arena.append(part_arena);
    debug_assert_eq!(host.arrows.len(), link_base);
    for memory in &mut part_arrows {
        memory.remap(junction_base, link_base);
    }
    if let Some(mut part_consolidation) = part_consolidation {
        part_consolidation.remap(junction_base, link_base);
        if let Some(host_consolidation) = &mut host.consolidation {
            host_consolidation.append(*part_consolidation);
        } else {
            host.consolidation = Some(part_consolidation);
        }
    }
    if let Some(mut part_reentry) = part_reentry {
        part_reentry.remap(junction_base, link_base);
        if let Some(host_reentry) = &mut host.reentry {
            host_reentry.append(*part_reentry);
        } else {
            host.reentry = Some(part_reentry);
        }
    }
    host.arrows.append(&mut part_arrows);
    host.prepare_attachment(part_junctions, at);
    host.rebuild_live_returns();

    let ports = part
        .ports
        .iter()
        .copied()
        .map(|port| remap_junction(port, junction_base))
        .collect::<Vec<_>>();
    for join in joins {
        let part = ports[join.part.0];
        let (from, to) = match join.direction {
            Direction::IntoHost => (part, join.host),
            Direction::IntoPart => (join.host, part),
        };
        host.add_link(Link::new(from, to, join.delay, join.impulse).when(join.trigger))
            .expect("attachment was fully validated before mutation");
    }

    Ok(Attachment { ports })
}

fn validate(host: &Body, part: &OpenBody, joins: &[Join]) -> Result<(), AttachError> {
    if !host.is_quiet() {
        return Err(AttachError::HostActive);
    }
    if !part.body.is_quiet() {
        return Err(AttachError::PartActive);
    }
    for join in joins {
        host.arena
            .require(join.host)
            .map_err(AttachError::UnknownHost)?;
        if join.part.0 >= part.ports.len() {
            return Err(AttachError::UnknownPartPort(join.part));
        }
    }
    let links = part
        .body
        .arena
        .link_count()
        .checked_add(joins.len())
        .ok_or(AttachError::CapacityExhausted)?;
    if !host
        .arena
        .has_junction_capacity(part.body.arena.junction_count())
        || !host.arena.has_link_capacity(links)
    {
        return Err(AttachError::CapacityExhausted);
    }
    Ok(())
}

fn remap_junction(id: JunctionId, base: usize) -> JunctionId {
    JunctionId::new(base + id.slot()).expect("validated attachment identity")
}

#[cfg(test)]
#[path = "tests/attachment.rs"]
mod tests;
