pub mod matter {
    include!(concat!(env!("OUT_DIR"), "/authority.rs"));
    include!("addition.rs");
}

pub use matter::{
    ArrowId, ArrowSpec, CellId, CellSpec, Execution, PlasticSubstrate, SpikeInput, WorkLedger,
};
