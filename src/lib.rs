//! Simulation framework for modelling spread and control of
//! animal diseases. This framework relies heavily on an ECS architecture
//! for its processes, and the specific ECS framework chosen here is the
//! [bevy_ecs]()
//!
//!

//! - [ ] TODO: Add support for multiple populations.
//!
//! A population can be defined as a shallow generic parameter and amended
//! to all places where [CattleFarm] is currently used.
//! But this is not enough as the parameters/components onto the farms would
//! be initialized from the common resource. This can of course be avoided
//! by also adding a Population generic onto all parameters, and thus use
//! that as a marker for initial parameters that are related to certain parameters
//! but then I foresee using the [std::marker::PhantomData].
//
//

pub mod prelude {
    //! Major packages used in this simulation has `prelude`-modules and these
    //! are all conjoined here.
    //!
    //!
    pub use bevy::prelude::*;

    pub use itertools::Itertools;
    #[cfg(feature = "serialize")]
    pub use serde::{Deserialize, Serialize};

    //TODO: add [bevy::ecs::system::SystemParam] as I believe this should
    // be included in the prelude, but it isn't.
}

// scenario builder
pub mod populations;
pub mod scenario_builder;

// ecs tools
pub mod chain_tools;
pub mod scenario_intervals;

// math
pub mod disease_parameters;
pub mod parameters;
pub mod tools;

// generic simulation modules
pub mod farm_id_to_entity_map;
pub mod scenario_time;

// (cattle) population model
pub mod cattle_population;
// within-herd spread model(s?)
pub mod sir_spread_model;
// TODO: Add a population that depends on the animal type and the disease
// compartments.

// between-herd infection modules
pub mod between_herd_spread_exogenous_model;
pub mod between_herd_spread_model;
pub mod between_herd_spread_model_record;
pub mod cattle_farm_recorder;
pub mod population_model_record;

// regulators
pub mod regulator_active_surveillance;
pub mod regulator_passive_surveillance;

// animal movements
pub mod repopulation_by_scaling;

// deprecated
pub mod deprecated_active_surveillance;
