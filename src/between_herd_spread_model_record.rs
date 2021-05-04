///
/// The [record_total_infected_farms] is very ineffective, due to having to
/// query every farm for its disease state, in order to determine if
/// these are new outbreaks, or just old ones that are progressing.
///
/// - [ ] However, from here we cannot have a system that records between-herd
/// incidence outside of the between-herd spread model...
///
use std::collections::BTreeMap;

use bevy::prelude::*;
use bevy::{ecs::system::SystemParam, utils::HashSet};

use crate::{scenario_time::ScenarioTime, sir_spread_model::Infected};

//TODO: Add `Debug` after it is added to `Bevy`.

#[derive(SystemParam)]
pub struct Recorder<'a> {
    previously_active_infections: Local<'a, HashSet<Entity>>,
    total_infected_farms: Local<'a, BTreeMap<ScenarioTime, usize>>,
}
pub fn record_total_infected_farms(
    mut recorder: Recorder,
    scenario_tick: Res<ScenarioTime>,
    query: Query<(Entity, &Infected)>,
) {
    let actively_infected_farms: HashSet<_> = query
        .iter()
        .filter(|x| {
            let infected: &Infected = x.1;
            infected.0 > 1
        })
        .map(|x| x.0)
        .collect();
    let tmp = recorder.previously_active_infections.clone();
    let mut newly_infected_farms = (&actively_infected_farms).difference(&tmp);

    //TODO: update previously infected farms in an "efficient" way.
    // There must be a way to register these things and see
    // if that is computationally better than doing this, maybe through
    // "tag" like components.
    *recorder.previously_active_infections = actively_infected_farms.clone();

    if newly_infected_farms.next().is_some() {
        // there are some newly infected farms.
        let total_infected_farms = actively_infected_farms.len();

        recorder
            .total_infected_farms
            .insert(*scenario_tick, total_infected_farms);
        info!("Total infected farms: {}", total_infected_farms);
    }
}
