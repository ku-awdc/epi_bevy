//!
//!
//!

use bevy::ecs::schedule::ReportExecutionOrderAmbiguities;
use rand::{SeedableRng, prelude::StdRng};

use crate::{prelude::*, scenario_time::ScenarioTime};

#[derive(derive_new::new)]
struct ScenarioBuilder {
    #[new(value = "20210426")]
    seed: u64,
}

impl ScenarioBuilder {
    
    #[must_use]
    pub fn build(self) -> Scenario {
        let mut world = World::new();

        // default resources...
        world.insert_resource(bevy::log::LogSettings {
            level: bevy::log::Level::DEBUG,
            ..Default::default()
        });
        world.insert_resource(ReportExecutionOrderAmbiguities);

        // world.insert_resource(StdRng::seed_from_u64(20210426));
        world.insert_resource(StdRng::seed_from_u64(self.seed));
        world.insert_resource(ScenarioTime::new(1, None));

        
        Scenario {
            world,
        }
    }

    /// Set the scenario builder's `seed`.
    #[must_use]
    fn set_seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }
}

struct Scenario {
    world: World,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_scenario() {
        let mut default_world = ScenarioBuilder::new();
        
        assert_eq!(default_world.seed, 20210426, "default seed has changed?");
    }
}