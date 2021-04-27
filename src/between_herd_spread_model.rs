//!
//!
//! Infectiousness is based on total number of infected from the
//! receepient farm
//!
//! 
//! The startup system [setup_between_herd_spread_model] is necessary for
//! this to make sense.

use bevy::prelude::*;
use itertools::Itertools;
use rand::prelude::*;
use rand_distr::WeightedAliasIndex;

use crate::{FarmIdEntityMap, cattle_population::{AdjacentFarms, CattleFarm, CattleFarmBundle}, sir_spread_model::Infected};

#[derive(Debug)]
pub struct SpreadModel {}

#[readonly::make]
#[derive(Debug,Clone, Copy)]
pub struct ContactRate(pub f64);

impl ContactRate {
    pub fn new(contact_rate: f64) -> Self {
        Self { 0: contact_rate }
    }
}

/// Add this to startup. 
pub fn setup_between_herd_spread_model(
    mut commands: Commands,
    initial_contact_rate: Option<Res<ContactRate>>,
    query: Query<(Entity, &CattleFarm)>,
) {
    let initial_contact_rate = initial_contact_rate
        .map_or_else(||ContactRate::new(0.001), |x| *x);
    
    query.for_each(|(entity,_)| {
        commands.entity(entity).insert(initial_contact_rate);
    });
}

fn update_between_herd_spread_model(
    rng: ResMut<StdRng>,
    farm_map: Res<FarmIdEntityMap>,
    query: Query<(&Infected, &AdjacentFarms)>,
) {
    let active_infected_farms = query.iter().filter(|info| info.0 .0 > 0);

    let infectious_contacts = active_infected_farms
    // .collect_vec().choose
    .map(|info| {

        let adjacent_farms: &AdjacentFarms = info.1;
        // let  = adjacent_farms.0.choose(rng);
        todo!()
    });
}
