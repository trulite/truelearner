#![forbid(unsafe_code)]

use lr1_modulatory_physical_return::{
    ArrowSpec, CellId, CellSpec, PlasticSubstrate, SpikeInput, TransmissionMode,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Field {
    pub space: PlasticSubstrate,
    pub source: CellId,
    pub effect: CellId,
    pub returner: CellId,
    pub source_physical: u64,
    pub effect_physical: u64,
    pub mark: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fork {
    pub space: PlasticSubstrate,
    pub sources: [CellId; 2],
    pub effects: [CellId; 2],
    pub returner: CellId,
    pub source_physical: [u64; 2],
    pub effect_physical: [u64; 2],
    pub mark: u64,
}

pub fn field(mark: u64, flip: bool, mirror: bool, mode: TransmissionMode) -> Field {
    let mut space = PlasticSubstrate::new();
    let order = if flip { [2, 1, 0] } else { [0, 1, 2] };
    let sign = if mirror { -1 } else { 1 };
    let physical = [mark + 10, mark + 20, mark + 30];
    let positions = [0, sign * 2, sign * 100];
    let regions = [10, 20, 30];
    let mut cells = [None; 3];
    for index in order {
        cells[index] =
            Some(space.add_cell(cell(physical[index], positions[index], regions[index])));
    }
    let [source, effect, returner] = cells.map(|item| item.unwrap());
    space.add_arrow(arrow(returner, source, 1, 1, 100, mode));
    Field {
        space,
        source,
        effect,
        returner,
        source_physical: physical[0],
        effect_physical: physical[1],
        mark,
    }
}

pub fn fork(mark: u64, flip: bool, mirror: bool) -> Fork {
    let mut space = PlasticSubstrate::new();
    let order = if flip {
        [4, 3, 2, 1, 0]
    } else {
        [0, 1, 2, 3, 4]
    };
    let sign = if mirror { -1 } else { 1 };
    let physical = [mark + 10, mark + 20, mark + 30, mark + 40, mark + 50];
    let positions = [0, sign * 2, sign * 20, sign * 22, sign * 100];
    let regions = [10, 20, 30, 40, 50];
    let mut cells = [None; 5];
    for index in order {
        cells[index] =
            Some(space.add_cell(cell(physical[index], positions[index], regions[index])));
    }
    let [source_a, effect_a, source_b, effect_b, returner] = cells.map(|item| item.unwrap());
    space.add_arrow(arrow(
        returner,
        source_a,
        1,
        1,
        100,
        TransmissionMode::Modulatory,
    ));
    space.add_arrow(arrow(
        returner,
        source_b,
        1,
        1,
        100,
        TransmissionMode::Modulatory,
    ));
    Fork {
        space,
        sources: [source_a, source_b],
        effects: [effect_a, effect_b],
        returner,
        source_physical: [physical[0], physical[2]],
        effect_physical: [physical[1], physical[3]],
        mark,
    }
}

pub fn arrive(space: &mut PlasticSubstrate, target: CellId, tick: i64, phase: i32, origin: u64) {
    space.enter(SpikeInput {
        arrival_tick: tick,
        phase,
        origin_physical: origin,
        target,
        impulse: 1,
    });
}

fn cell(physical_id: u64, position: i32, region: i16) -> CellSpec {
    CellSpec {
        physical_id,
        position,
        region,
        threshold: 1,
        resistance: 100,
    }
}

fn arrow(
    from: CellId,
    to: CellId,
    delay: i64,
    coupling: i32,
    resistance: u32,
    mode: TransmissionMode,
) -> ArrowSpec {
    ArrowSpec {
        from,
        to,
        delay,
        phase: 0,
        coupling,
        resistance,
        mode,
    }
}
