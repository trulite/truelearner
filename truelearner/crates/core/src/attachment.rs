use crate::body::Body;
use crate::{Junction, JunctionId, Link, TransmissionMode};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AttachmentSite {
    position: i32,
    region: i16,
}

impl AttachmentSite {
    pub const fn new(position: i32, region: i16) -> Self {
        Self { position, region }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComponentJunction {
    relative_position: i32,
    threshold: i32,
    resistance: u32,
}

impl ComponentJunction {
    pub const fn new(
        relative_position: i32,
        threshold: i32,
        resistance: u32,
    ) -> Result<Self, ComponentSpecError> {
        if threshold <= 0 {
            Err(ComponentSpecError::NonPositiveThreshold)
        } else if resistance == 0 {
            Err(ComponentSpecError::NonPositiveResistance)
        } else {
            Ok(Self {
                relative_position,
                threshold,
                resistance,
            })
        }
    }

    pub const fn ordinary(relative_position: i32, threshold: i32) -> Self {
        assert!(
            threshold > 0,
            "ordinary junction threshold must be positive"
        );
        Self {
            relative_position,
            threshold,
            resistance: u32::MAX,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComponentLink {
    from: usize,
    to: usize,
    delay: i64,
    phase: i32,
    coupling: i32,
    resistance: u32,
    mode: TransmissionMode,
}

impl ComponentLink {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        from: usize,
        to: usize,
        delay: i64,
        phase: i32,
        coupling: i32,
        resistance: u32,
        mode: TransmissionMode,
    ) -> Result<Self, ComponentSpecError> {
        if delay < 0 {
            Err(ComponentSpecError::NegativeDelay)
        } else if resistance == 0 {
            Err(ComponentSpecError::NonPositiveResistance)
        } else {
            Ok(Self {
                from,
                to,
                delay,
                phase,
                coupling,
                resistance,
                mode,
            })
        }
    }

    pub const fn ordinary(from: usize, to: usize, coupling: i32) -> Self {
        Self {
            from,
            to,
            delay: 0,
            phase: 0,
            coupling,
            resistance: u32::MAX,
            mode: TransmissionMode::Drive,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PhysicalComponentSpec {
    junctions: Vec<ComponentJunction>,
    links: Vec<ComponentLink>,
    ports: Vec<usize>,
}

impl PhysicalComponentSpec {
    pub fn new(
        junctions: Vec<ComponentJunction>,
        links: Vec<ComponentLink>,
        ports: Vec<usize>,
    ) -> Result<Self, ComponentSpecError> {
        if junctions.is_empty() {
            return Err(ComponentSpecError::NoJunctions);
        }
        if ports.is_empty() {
            return Err(ComponentSpecError::NoPorts);
        }
        for (link_index, link) in links.iter().enumerate() {
            for (junction, endpoint) in
                [(link.from, LinkEndpoint::From), (link.to, LinkEndpoint::To)]
            {
                if junction >= junctions.len() {
                    return Err(ComponentSpecError::UnknownLinkJunction {
                        link: link_index,
                        endpoint,
                        junction,
                        junctions: junctions.len(),
                    });
                }
            }
        }
        let mut sorted_ports = ports.clone();
        sorted_ports.sort_unstable();
        for port in &sorted_ports {
            if *port >= junctions.len() {
                return Err(ComponentSpecError::UnknownPort {
                    port: *port,
                    junctions: junctions.len(),
                });
            }
        }
        if let Some(duplicate) = sorted_ports
            .windows(2)
            .find_map(|pair| (pair[0] == pair[1]).then_some(pair[0]))
        {
            return Err(ComponentSpecError::DuplicatePort(duplicate));
        }
        if let Some(unlinked) = (0..junctions.len()).find(|junction| {
            !links
                .iter()
                .any(|link| link.from == *junction || link.to == *junction)
        }) {
            return Err(ComponentSpecError::UnlinkedJunction(unlinked));
        }
        Ok(Self {
            junctions,
            links,
            ports,
        })
    }

    pub fn junction_count(&self) -> usize {
        self.junctions.len()
    }

    pub fn link_count(&self) -> usize {
        self.links.len()
    }

    pub fn port_count(&self) -> usize {
        self.ports.len()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinkEndpoint {
    From,
    To,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComponentSpecError {
    NoJunctions,
    NoPorts,
    NonPositiveThreshold,
    NonPositiveResistance,
    NegativeDelay,
    UnknownLinkJunction {
        link: usize,
        endpoint: LinkEndpoint,
        junction: usize,
        junctions: usize,
    },
    UnknownPort {
        port: usize,
        junctions: usize,
    },
    DuplicatePort(usize),
    UnlinkedJunction(usize),
}

impl fmt::Display for ComponentSpecError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "invalid physical component: {self:?}")
    }
}

impl std::error::Error for ComponentSpecError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PhysicalPort {
    target: JunctionId,
    origin_physical: u64,
}

impl PhysicalPort {
    pub const fn target(self) -> JunctionId {
        self.target
    }

    pub const fn origin_physical(self) -> u64 {
        self.origin_physical
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PhysicalAttachment {
    ports: Vec<PhysicalPort>,
}

impl PhysicalAttachment {
    pub fn len(&self) -> usize {
        self.ports.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ports.is_empty()
    }

    pub fn port(&self, index: usize) -> Option<PhysicalPort> {
        self.ports.get(index).copied()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AttachError {
    BodyNotQuiescent,
    JunctionCapacity { needed: usize, available: usize },
    LinkCapacity { needed: usize, available: usize },
    PositionOverflow,
    PhysicalIdentityExhausted,
}

impl fmt::Display for AttachError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "physical attachment failed: {self:?}")
    }
}

impl std::error::Error for AttachError {}

pub(crate) fn attach_physical(
    body: &mut Body,
    site: AttachmentSite,
    component: &PhysicalComponentSpec,
) -> Result<PhysicalAttachment, AttachError> {
    if !body.pending.is_empty() {
        return Err(AttachError::BodyNotQuiescent);
    }

    let available_junctions = body
        .arena
        .junctions
        .iter()
        .filter(|junction| !junction.live)
        .count()
        .saturating_add(
            usize::try_from(body.arena.junction_capacity)
                .unwrap_or(usize::MAX)
                .saturating_sub(body.arena.junctions.len()),
        );
    if component.junctions.len() > available_junctions {
        return Err(AttachError::JunctionCapacity {
            needed: component.junctions.len(),
            available: available_junctions,
        });
    }

    let available_links = body
        .arena
        .links
        .iter()
        .filter(|link| !link.live)
        .count()
        .saturating_add(
            usize::try_from(body.arena.link_capacity)
                .unwrap_or(usize::MAX)
                .saturating_sub(body.arena.link_slots.len()),
        );
    if component.links.len() > available_links {
        return Err(AttachError::LinkCapacity {
            needed: component.links.len(),
            available: available_links,
        });
    }

    let positions = component
        .junctions
        .iter()
        .map(|junction| {
            site.position
                .checked_add(junction.relative_position)
                .ok_or(AttachError::PositionOverflow)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let first_physical_id = body
        .arena
        .junctions
        .iter()
        .map(|junction| junction.physical_id)
        .max()
        .unwrap_or(0)
        .checked_add(1)
        .ok_or(AttachError::PhysicalIdentityExhausted)?;
    if !component.junctions.is_empty() {
        first_physical_id
            .checked_add(
                u64::try_from(component.junctions.len() - 1)
                    .map_err(|_| AttachError::PhysicalIdentityExhausted)?,
            )
            .ok_or(AttachError::PhysicalIdentityExhausted)?;
    }

    let mut next = body.clone();
    let mut ids = Vec::with_capacity(component.junctions.len());
    for (index, (junction, position)) in component.junctions.iter().zip(positions).enumerate() {
        let physical_id = first_physical_id
            .checked_add(u64::try_from(index).map_err(|_| AttachError::PhysicalIdentityExhausted)?)
            .ok_or(AttachError::PhysicalIdentityExhausted)?;
        ids.push(next.add_junction(Junction {
            physical_id,
            position,
            region: site.region,
            threshold: junction.threshold,
            resistance: junction.resistance,
        }));
    }
    for link in &component.links {
        next.add_link(Link {
            from: ids[link.from],
            to: ids[link.to],
            delay: link.delay,
            phase: link.phase,
            coupling: link.coupling,
            resistance: link.resistance,
            mode: link.mode,
        });
    }
    let attachment = PhysicalAttachment {
        ports: component
            .ports
            .iter()
            .map(|port| PhysicalPort {
                target: ids[*port],
                origin_physical: first_physical_id
                    .checked_add(
                        u64::try_from(*port)
                            .expect("component size was preflighted against physical identity"),
                    )
                    .expect("component physical identity range was preflighted"),
            })
            .collect(),
    };
    *body = next;
    Ok(attachment)
}
