use crate::core::{LifeCycle, LifeCycles};

pub mod clear_memory;

pub fn get_global_lifecycles() -> LifeCycles {
    let mut lifecycles = LifeCycles::new();
    lifecycles.insert(LifeCycle::TickStart, Box::new(clear_memory::clear_memory));
    lifecycles
}
