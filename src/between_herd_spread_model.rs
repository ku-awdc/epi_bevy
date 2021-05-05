//!
//!
//! Infectiousness is based on total number of infected from the
//! receepient farm
//!
//!
//! The startup system [setup_between_herd_spread_model] is necessary for
//! this to make sense.

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use itertools::Itertools;
use rand::prelude::*;

use crate::{
    cattle_population::{AdjacentFarms, CattleFarm, FarmId, HerdSize},
    farm_id_to_entity_map::FarmIdEntityMap,
    scenario_time::ScenarioTime,
    sir_spread_model::{Infected, Susceptible},
};

#[readonly::make]
#[derive(Debug, Clone, Copy)]
pub struct ContactRate(pub f64);

impl ContactRate {
    pub fn new(contact_rate: f64) -> Self {
        Self { 0: contact_rate }
    }
}

/// Here we add the contact rate to each farm, as to be able to change it
/// on a pr. farm basis later on.
///
/// Add this to a startup system. Preferably after [CattleFarm]-entites has
/// been added, otherwise this query is invalid.
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

//TODO: Store the last infection events batches in the system param as a local
// for now until there appears a system that needs to use it somehow, and then
// make that available as a resource?

/// Components necessary to determine the infection pressure of actively
/// infected farms.
type InfectedFarms = (
    &'static Infected,
    &'static AdjacentFarms,
    &'static ContactRate,
    &'static HerdSize,
    &'static FarmId,
);

/// Components necessary to seed an infection
type AffectedFarm = (&'static mut Susceptible, &'static mut Infected);

#[derive(SystemParam)]
pub struct BetweenHerdSpreadModel<'a> {
    /// No. of the last batch of between-herd events that was put out by this
    /// (spread) model.
    current_batch_id: Local<'a, usize>,

    /// Get infected farms and spread through contacts to other farms.
    query: QuerySet<(Query<'a, InfectedFarms>, Query<'a, AffectedFarm>)>,
}

pub fn update_between_herd_spread_model(
    mut model: BetweenHerdSpreadModel,
    mut rng: ResMut<StdRng>,
    scenario_tick: Res<ScenarioTime>,
    farm_map: Res<FarmIdEntityMap>,
) -> Option<InfectionEvents> {
    // determine from farms
    // let active_infected_farms = query.iter_mut().filter(|info| info.0.0 > 0);

    //FIXME: do something with thiss
    // let new_infection_events:Vec<(FarmId, FarmId)> = active_infected_farms
    // let infectious_farms: Vec<(Infected, AdjacentFarms, HerdSize, FarmId)> = model.query
    // let infectious_farms = model.query
    let infectious_farms = model
        .query
        .q0()
        .iter()
        // first, is an infectious farm going to send out any batches of animals?
        //FIXME: ensure this works for all rates, not only for <= 1.
        // determine if there are animal movements
        .filter(|info| {
            // .filter(|(farm,)| {
            let infected: Infected = *info.0;
            let infected: Infected = infected;
            infected.0 > 0
        })
        .filter(|(_, _, contact_rate, _, _)| {
            // .filter(|(farm, )| {
            let contact_rate: &&ContactRate = contact_rate;

            rng.gen_bool(contact_rate.0)
        })
        .map(
            |info: (&Infected, &AdjacentFarms, &ContactRate, &HerdSize, &FarmId)| {
                (*info.0, info.1.clone(), *info.3, *info.4)
            },
        )
        // .map(|(farm,)| farm.clone())
        .collect_vec();

    let new_infection_events: Vec<(FarmId, FarmId, usize)> = infectious_farms
        .into_iter()
        // determine destination farm (from, target)
        .filter_map(|(infected, adjacent_farms, herd_size, from_farm_id)| {
            // .filter_map(|farm| {
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

                //FIXME: this made the disease compartments no longer be read-only
                // incorporate that into a disease model interface
                let successful_infection = model
                    .query
                    .q1_mut()
                    // select target farm's disease components
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
                    // `origin ~> target, #new infections`
                    Some((from_farm_id, *target_farm_id, 1))
                    // Some((farm.farm_id, *target_farm_id, 1))
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
        // since there were new infections, update the model
        *model.current_batch_id += 1;

        // debug
        trace!(
            "Total new between-herd infections: {}",
            total_new_infection_events
        );

        //export that new infections events occurred
        Some(InfectionEvents {
            scenario_tick: scenario_tick.current_time(),
            batch_id: *model.current_batch_id,
            events_values: new_infection_events,
        })
    } else {
        None
    }
}

//TODO: Maybe this should reside in its own module, as it is generally unrelated
// to the actual exectution of the between-herd spread module.

// #[readonly::make]
#[derive(Debug, Clone)]
pub struct InfectionEvents {
    /// Scenario time for the infection events
    pub scenario_tick: crate::scenario_time::Time,
    /// Infection events are put out in batches
    /// and they can be grouped according to them.
    pub batch_id: usize,
    /// An event consists an origin farm `from` and a target farm `to` and
    /// then the number of infectious animals that were introduced to the fold.
    pub events_values: Vec<(FarmId, FarmId, usize)>,
}

/// Prints the between-herd spread events as they come.
pub fn trace_between_herd_infection_events(
    In(events): In<Option<InfectionEvents>>,
) -> Option<InfectionEvents> {
    if let Some(infection_events) = events.clone() {
        let InfectionEvents {
            scenario_tick,
            batch_id,
            events_values,
        } = infection_events;
        info!("Between-herd spread events");
        info!("Batch id: {}", batch_id);
        info!("Time: {}", scenario_tick);
        // info!("{:#?}", events);
        for (origin, target, _) in events_values {
            info!("{} -> {}", origin, target)
        }
        events
    } else {
        // no events was released now, so what gives?
        None
    }
}
