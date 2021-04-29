use std::collections::{BTreeMap, BTreeSet};

use bevy::prelude::*;
use bevy::{ecs::system::SystemParam, utils::HashSet};

use crate::{sir_spread_model::Infected, ScenarioTick};

//TODO: Add `Debug` after it is added to `Bevy`.

#[derive(SystemParam)]
pub struct Recorder<'a> {
    previously_active_infections: Local<'a, HashSet<Entity>>,
    total_infected_farms: Local<'a, BTreeMap<ScenarioTick, usize>>,
}
pub fn record_total_infected_farms(mut recorder: Recorder, query: Query<(Entity, &Infected)>) {
    let actively_infected_farms: HashSet<_> = query
        .iter()
        .filter(|x| {
            let infected: &Infected = x.1;
            infected.0 > 1
        })
        .map(|x| x.0)
        .collect();
    let tmp = recorder.previously_active_infections.clone();
    let mut newly_infected_farms =
        (&actively_infected_farms).difference(&tmp);

    //FIXME: update previously

    recorder
        .previously_active_infections
        .extend(newly_infected_farms.clone());

    if newly_infected_farms.next().is_some() {
        // there are some newly infected farms.

        info!("Total infected farms: {}", &actively_infected_farms.len());
    }
}
