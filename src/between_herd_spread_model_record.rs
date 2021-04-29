use std::collections::BTreeMap;

use bevy::prelude::*;
use bevy::{ecs::system::SystemParam, utils::HashSet};

use crate::{sir_spread_model::Infected, ScenarioTick};

//TODO: Add `Debug` after it is added to `Bevy`.

#[derive(SystemParam)]
pub struct Recorder<'a> {
    previously_active_infections: Res<'a, HashSet<Entity>>,
    total_infected_farms: Res<'a, BTreeMap<ScenarioTick, usize>>,
}
pub fn record_total_infected_farms<'a>(
    recorder: &'static Local<Recorder<'a>>,
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
    let previously_actively_infected = recorder.previously_active_infections;
    let mut newly_infected_farms =
        (&actively_infected_farms).difference(&*previously_actively_infected);

    if newly_infected_farms.next().is_some() {
        // there are some newly infected farms.

        info!("Total infected farms: {}", &actively_infected_farms.len());
    }
}
