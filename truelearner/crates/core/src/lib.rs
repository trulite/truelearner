#![forbid(unsafe_code)]

mod arena;
mod body;
mod checkpoint;
mod choose;
mod core;
mod format;
mod hold;
mod input;
mod junction;
mod link;
mod outcome;
mod output;
mod path;
mod physics;
mod reuse;
mod schedule;
mod snapshot;
mod strength;
mod timing_wheel;
mod trace;

mod prelude {
    pub(crate) use crate::arena::*;
    pub(crate) use crate::body::*;
    pub(crate) use crate::core::*;
    pub(crate) use crate::junction::*;
    pub(crate) use crate::link::*;
    pub(crate) use crate::schedule::*;
    pub(crate) use crate::trace::*;
    pub(crate) use serde::{Deserialize, Serialize};
    pub(crate) use std::collections::HashSet;
    pub(crate) use truelearner_arena_format::{
        ArenaBody, ArenaId, ArenaVersion, ArrowId as LinkId, BodyVersion, CellId as JunctionId,
        CellRef as JunctionRef, ContentHash, DurableArrow as DurableLink,
        DurableCell as DurableJunction, FormatError, Generation,
    };
}

pub use core::*;

#[cfg(test)]
mod tests;
