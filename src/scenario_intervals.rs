//! There are scenario time and user time.
//! Some systems are to run after certain processes (e.g. [System])
//! while others should run every day (default), week, month, year, etc.
//!
//!

use crate::prelude::*;
use crate::scenario_time::ScenarioTime;
use bevy::ecs::schedule::ShouldRun;

pub fn run_yearly(scenario_time: Res<ScenarioTime>) -> ShouldRun {
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
