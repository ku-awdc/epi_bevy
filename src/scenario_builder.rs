//!
//!
//!

use std::collections::HashMap;

use bevy::ecs::schedule::ReportExecutionOrderAmbiguities;
use rand::{prelude::StdRng, SeedableRng};

use crate::{
    populations::{EmbeddedPopulation, Population},
    prelude::*,
    scenario_time::ScenarioTime,
};

#[derive(derive_new::new)]
struct ScenarioBuilder {
    /// Random number generator seed.
    #[new(value = "20210426")]
    seed: u64,

    #[new(default)]
    world: Option<World>,
    #[new(default)]
    entities: HashMap<EmbeddedPopulation, Vec<Entity>>,
}

impl ScenarioBuilder {
    #[must_use]
    pub fn build(self) -> Scenario {
        let mut world = self.world.unwrap_or_else(World::new);

        // default resources...
        world.insert_resource(bevy::log::LogSettings {
            level: bevy::log::Level::DEBUG,
            ..Default::default()
        });
        world.insert_resource(ReportExecutionOrderAmbiguities);

        // world.insert_resource(StdRng::seed_from_u64(20210426));
        world.insert_resource(StdRng::seed_from_u64(self.seed));
        world.insert_resource(ScenarioTime::new(1, None));

        Scenario { world }
    }

    /// Set the scenario builder's `seed`.
    #[must_use]
    pub fn set_seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }

    /// Add a population
    pub fn add_population(
        &mut self,
        population: impl Population,
        individuals: impl Iterator<Item = impl Bundle>,
    ) {
        unimplemented!()
    }

    /// Amend a population with components for simulation processes
    pub fn add_parameter(
        &mut self,
        initial_parameter: impl Bundle,
        target_population: EmbeddedPopulation,
    ) {
        // retrieve all individuals from said population
        let entities = self
            .entities
            .get(&target_population)
            .expect("population doesn't exist within the world; try adding it before this");
        // self.world.
        unimplemented!()
    }
}

struct Scenario {
    world: World,
}

// impl Scenario {
//     pub fn builder() -> ScenarioBuilder {
//         ScenarioBuilder::new()
//     }
// }

/// The [SystemStage] that the scenario is simulated within. This dictates whether
/// systems should be run in single_threaded or parallel mode as well.
#[derive(Debug, PartialEq, Eq, Hash, Clone, StageLabel)]
struct MainLoop;

#[derive(derive_new::new)]
struct ScenarioStage {
    stage: SystemStage,
}

impl ScenarioStage {
    #[must_use]
    pub fn add_process<Process, P: Population>(
        &mut self,
        initial_state: impl bevy::ecs::component::Component,
        update_routine: Process,
    ) -> &mut Self {
        // let entities_to_supplement  = self.world.query::<P>().iter_mut(&mut self.world);
        // for entity in entities_to_supplement {
        //     entity.
        // }
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_scenario() {
        let mut default_world = ScenarioBuilder::new();

        assert_eq!(default_world.seed, 20210426, "default seed has changed?");

        // let default_scenario_stage = ScenarioStage::new()
    }
}
