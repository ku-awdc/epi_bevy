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

use crate::{
    cattle_population::{AdjacentFarms, CattleFarm, FarmId, HerdSize},
    sir_spread_model::{Infected, Susceptible},
    FarmIdEntityMap,
};

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
    let initial_contact_rate: ContactRate = initial_contact_rate
        .expect("Missing initial `ContactRate` as a resource.")
        .to_owned();
    // let initial_contact_rate = initial_contact_rate.map_or_else(|| ContactRate::new(0.001), |x| *x);

    query.for_each(|(entity, _)| {
        commands.entity(entity).insert(initial_contact_rate);
    });
}

pub fn update_between_herd_spread_model(
    mut rng: ResMut<StdRng>,
    farm_map: Res<FarmIdEntityMap>,
    mut query: QuerySet<(
        Query<(&Infected, &AdjacentFarms, &ContactRate, &HerdSize, &FarmId)>,
        Query<(&mut Susceptible, &mut Infected)>,
    )>,
) {
    // determine from farms
    // let active_infected_farms = query.iter_mut().filter(|info| info.0.0 > 0);

    //FIXME: do something with thiss
    // let new_infection_events:Vec<(FarmId, FarmId)> = active_infected_farms
    let infectious_farms: Vec<(Infected, AdjacentFarms, HerdSize, FarmId)> = query
        .q0()
        .iter()
        // first, is an infectious farm going to send out any batches of animals?
        //FIXME: ensure this works for all rates, not only for <= 1.
        // determine if there are animal movements
        .filter(|(_, _, contact_rate, _, _)| {
            let contact_rate: &&ContactRate = contact_rate;

            rng.gen_bool(contact_rate.0)
        })
        .filter(|info| {
            let infected: Infected = *info.0;
            infected.0 > 0
        })
        .map(
            |info: (&Infected, &AdjacentFarms, &ContactRate, &HerdSize, &FarmId)| {
                (*info.0, info.1.clone(), *info.3, *info.4)
            },
        )
        .collect_vec();

    let new_infection_events: Vec<(FarmId, FarmId)> = infectious_farms
        .into_iter()
        // determine destination farm (from, target)
        .filter_map(|(infected, adjacent_farms, herd_size, from_farm_id)| {
            let herd_size: HerdSize = herd_size;
            let adjacent_farms: AdjacentFarms = adjacent_farms;
            let target_farm_id = adjacent_farms.0.choose(&mut *rng).unwrap();

            //FIXME: can an infected farm infect another infected farm?

            // now will this result in an infection?
            let infection_pressure = infected.0 as f64 / herd_size.0 as f64;
            debug_assert!(
                herd_size.0 >= infected.0,
                "cannot have more infected animals than animals in the farm."
            );

            // info!(
            //     "infection pressure: {} / {} = {:.3}",
            //     infected.0, herd_size.0, infection_pressure
            // );

            if rng.gen_bool(infection_pressure) {
                // add infection to target
                //TODO: facilitate this through a common trait for between-herd
                let target_farm_entity_id = farm_map.0.get(target_farm_id).unwrap();
                // info!("target_farm id and entity.id: {:?}, {:?}", target_farm_id, target_farm_entity_id);
                // let mut target_farm_infected_count = query.q1_mut().get_component_mut::<Infected>(*target_farm_entity_id).expect("the query is not finding the target farm that needs to have its infection increased");

                //FIXME: this made the disease compartments no longer be read-only
                // incorporate that into a disease model interface
                let successful_infection = query
                    .q1_mut()
                    .get_mut(*target_farm_entity_id)
                    .map(|(mut sus, mut inf)| {
                        if sus.0 >= 1 {
                            sus.0 -= 1;
                            inf.0 += 1;
                            true
                        } else {
                            false
                        }
                    })
                    .expect("failed to find target farm to infect");
                if successful_infection {
                    Some((from_farm_id, *target_farm_id))
                } else {
                    // there wasn't any susceptible animals to infect
                    None
                }
            } else {
                // no contact between the two determined
                None
            }
        })
        .collect_vec();

    // TODO: record how much this impacts the disease spread.
    let total_new_infection_events = new_infection_events.len();

    if total_new_infection_events > 0 {
        trace!(
            "Total new between-herd infections: {}",
            new_infection_events.len()
        );
    }
}
