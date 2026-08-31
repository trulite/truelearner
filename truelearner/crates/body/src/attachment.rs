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
    let mut part_link_memory = std::mem::take(&mut part.body.link_memory);
    let part_automaticity = std::mem::take(&mut part.body.automaticity);
    let (junction_base, link_base) = host.arena.append(part_arena);
    debug_assert_eq!(host.link_memory.len(), link_base);
    for memory in &mut part_link_memory {
        memory.remap_links(link_base);
    }
    if let Some(mut part_automaticity) = part_automaticity {
        part_automaticity.remap_links(link_base);
        if let Some(host_automaticity) = &mut host.automaticity {
            host_automaticity.append(*part_automaticity);
        } else {
            host.automaticity = Some(part_automaticity);
        }
    }
    host.link_memory.append(&mut part_link_memory);
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
mod tests {
    use super::*;
    use crate::{Junction, LinkRole};

    #[test]
    fn attachment_preserves_live_returns_when_rebuilding_the_index() {
        let mut host = Body::default();
        let host_junction = host.add_junction(Junction::integrating(1)).unwrap();
        let host_link = host
            .add_link(Link::new(host_junction, host_junction, 0, 0))
            .unwrap();
        host.set_link_role(
            host_link,
            LinkRole::Return {
                cause: 8,
                cohort: 8,
            },
        )
        .unwrap();

        let mut part = Body::default();
        let part_junction = part.add_junction(Junction::integrating(1)).unwrap();
        let part_link = part
            .add_link(Link::new(part_junction, part_junction, 0, 0))
            .unwrap();
        part.set_link_role(
            part_link,
            LinkRole::Return {
                cause: 3,
                cohort: 3,
            },
        )
        .unwrap();
        let part = OpenBody::new(part, vec![part_junction]).unwrap();

        attach(&mut host, part, &[]).unwrap();

        assert_eq!(host.returns.live_count, 2);
        assert!(host.returns.by_source.iter().all(Vec::is_empty));
    }
}
