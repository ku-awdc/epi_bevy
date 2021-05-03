//! There are scenario time and user time. 
//! Some systems are to run after certain processes (e.g. [System])
//! while others should run every day (default), week, month, year, etc.
//! 
//! 

use bevy::{ecs::schedule::ShouldRun, prelude::*};

use crate::scenario_time::ScenarioTime;

pub fn run_yearly(scenario_time: Res<ScenarioTime>) -> ShouldRun {
    if scenario_time.first_day_of_the_year() {
        info!("first day of the year");
        ShouldRun::YesAndCheckAgain
    } else {
        ShouldRun::NoAndCheckAgain
    }
}