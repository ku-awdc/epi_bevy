//! Simulation framework for modelling spread and control of 
//! animal diseases. This framework relies heavily on an ECS architecture
//! for its processes, and the specific ECS framework chosen here is the
//! [bevy_ecs]()
pub mod prelude {
    //! Major packages used in this simulation has `prelude`-modules and these
    //! are all conjoined here.
    //! 
    //! 
    pub use bevy::prelude::*;
    pub use itertools::Itertools;

    //TODO: add [bevy::ecs::system::SystemParam] as I believe this should
    // be included in the prelude, but it isn't.

}

pub mod tools;

pub mod farm_id_to_entity_map;

pub mod between_herd_spread_model;
pub mod between_herd_spread_model_record;
pub mod cattle_farm_recorder;
pub mod cattle_population;
pub mod parameters;
pub mod population_model_record;
pub mod regulator_passive_surveillance;
pub mod scenario_intervals;
pub mod scenario_time;
pub mod sir_spread_model;
