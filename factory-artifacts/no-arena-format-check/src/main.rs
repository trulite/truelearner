use truelearner_core::{Arena, ArenaBody, Body};

fn main() {
    let _ = std::mem::size_of::<(Arena, ArenaBody, Body)>();
}
