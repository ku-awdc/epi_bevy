//!
//!
//! Infectiousness is based on total number of infected from the
//! receepient farm
//!
//!
//! The startup system [setup_between_herd_spread_model] is necessary for
//! this to make sense.



use bevy::prelude::*;
use rand::prelude::*;
use itertools::Itertools;

use crate::{FarmIdEntityMap, cattle_population::{AdjacentFarms, CattleFarm, FarmId, HerdSize}, sir_spread_model::Infected};

#[derive(Debug)]
pub struct SpreadModel {}

#[readonly::make]
#[derive(Debug, Clone, Copy)]
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
    let initial_contact_rate = initial_contact_rate.map_or_else(|| ContactRate::new(0.001), |x| *x);

    query.for_each(|(entity, _)| {
        commands.entity(entity).insert(initial_contact_rate);
    });
}

pub fn update_between_herd_spread_model(
    mut commands: Commands,
    mut rng: ResMut<StdRng>,
    farm_map: Res<FarmIdEntityMap>,
    query: Query<(&mut Infected, &AdjacentFarms, &ContactRate, &HerdSize, &FarmId)>,
    // mut target_farms_query: Query<&mut Infected>,
) {
    // determine from farms
    let active_infected_farms = query.iter_mut().filter(|info| info.0.0 > 0);
    
    //FIXME: do something with thiss
    let new_infection_events:Vec<(FarmId, FarmId)> = active_infected_farms
        // first, is an infectious farm going to send out any batches of animals?
        //FIXME: ensure this works for all rates, not only for <= 1.
        // .filter(|(_, _, contact_rate): (_, _, &ContactRate)| rng.gen_bool(contact_rate.0))
        // determine if there are animal movements
        // .filter(|(_, _, contact_rate): (_,_,&&ContactRate)| {
        .filter(|(_, _, contact_rate,_,_)| {
            let contact_rate: &&ContactRate = contact_rate;
            rng.gen_bool(contact_rate.0)
        })
        .collect_vec()
        .into_iter()
        // determine destination farm (from, target)
        .filter_map(|(infected, adjacent_farms, contact_rate, herd_size,from_farm_id)| {
            let herd_size: &HerdSize = herd_size;
            let adjacent_farms: &AdjacentFarms = adjacent_farms;
            let target_farm_id = adjacent_farms.0.choose(&mut *rng).unwrap();
            
            //FIXME: can an infected farm infect another infected farm?

            // now will this result in an infection?
            let infection_pressure = herd_size.0 as f64 / infected.0 as f64;
            //FIXME: transform to rate
            if rng.gen_bool(infection_pressure) {
                // add infection to target
                //TODO: facilitate this through a common trait for between-herd
                let target_farm_entity_id = farm_map.0.get(target_farm_id).unwrap();
                // commands.entity(target_farm_entity_id)
                let mut target_farm_infected_count = query.get_component_mut::<&mut Infected>(*target_farm_entity_id).expect("the query is not finding the target farm that needs to have its infection increased");
                target_farm_infected_count.add();

                Some((*from_farm_id, *target_farm_id))
            } else {
                None
            }
        }).collect_vec();
        info!("Between herd infections: {}", new_infection_events.len());
}
