pub mod prelude {

    pub use bevy::{
        app::AppExit,
        diagnostic::{DiagnosticsPlugin, LogDiagnosticsPlugin},
        log::LogPlugin,
        prelude::*,
    };
    // use itertools::Itertools;
}

pub mod farm_id_to_entity_map;

pub mod between_herd_spread_model;
pub mod between_herd_spread_model_record;
pub mod cattle_farm_recorder;
pub mod cattle_population;
pub mod scenario_intervals;
pub mod scenario_time;
pub mod sir_spread_model;
