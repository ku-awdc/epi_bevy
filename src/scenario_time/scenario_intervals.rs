//! There are scenario time and user time.
//! Some systems are to run after certain processes (e.g. [System])
//! while others should run every day (default), week, month, year, etc.
//!
//!

use crate::scenario_time::scenario_timer::ScenarioTime;
use bevy::{ecs::schedule::ShouldRun, prelude::*};

/// This process updates the scenario tick
pub fn update_scenario_tick(mut scenario_tick: ResMut<ScenarioTime>) {
    scenario_tick.update_time(1);
}

/// Attach to a system as a run-criteria
///
/// ```rust
/// // .with_run_criteria(epi_bevy::scenario_intervals::run_yearly.system())
///
pub fn run_every_year(scenario_time: Res<ScenarioTime>) -> ShouldRun {
    if scenario_time.first_day_of_the_year() {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

pub fn run_every_week(scenario_time: Res<ScenarioTime>) -> ShouldRun {
    if scenario_time.first_day_of_week(None) {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

pub fn run_every_month(scenario_time: Res<ScenarioTime>) -> ShouldRun {
    if scenario_time.first_day_of_the_month() {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{prelude::*, scenario_time::scenario_timer::Time};

    #[test]
    fn test_show_run_criteria() {
        let max_timesteps = 365 * 20;
        let mut world = World::new();

        #[derive(Debug, Clone, Default)]
        struct RecordedTimes(Vec<Time>);
        world.insert_resource(RecordedTimes::default());
        world.insert_resource(ScenarioTime::new(1, None));

        let mut scenario = SystemStage::single_threaded();
        scenario.add_system(update_scenario_tick.exclusive_system().at_start());
        scenario.set_run_criteria((|| ShouldRun::Yes).system());

        fn record_scenario_time(mut records: ResMut<RecordedTimes>, tick: Res<ScenarioTime>) {
            records.0.push(tick.current_time());
        }

        scenario.add_system(
            record_scenario_time
                .system()
                .with_run_criteria(run_every_year.system()),
        );

        for _timesteps in 0..max_timesteps {
            scenario.run(&mut world);
        }

        let records = world.get_resource::<RecordedTimes>().cloned().unwrap();
        dbg!(records);
    }
}
