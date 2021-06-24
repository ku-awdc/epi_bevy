//!
//!
//!
//! This should provide an interface for exogenous infection pressure onto farms.
//!

use crate::prelude::*;
use crate::{
    parameters::Rate,
    // cattle_population::CattleFarm,
    populations::Cattle,
    sir_spread_model::{Infected, Susceptible},
};

#[readonly::make]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, derive_more::Into, derive_more::Display)]
pub struct ExogenousInfectionRate(pub Rate);

pub fn setup_exogenous_infection_rate(
    mut commands: Commands,
    initial_exogenous_infection_rate: Option<ExogenousInfectionRate>,
    query: Query<Entity, With<Cattle>>,
) {
    let initial_exogenous_infection_rate =
        initial_exogenous_infection_rate.expect("initial value as a resource is not present.");

    query.for_each(|x| {
        commands.entity(x).insert(initial_exogenous_infection_rate);
    });
}

/// This is here to mimic the same process-structure everywhere.
/// But really this is belongs within the disease model, as it should
/// be a disease model that specifically handles exogenous infections within
/// its update-scheme.
pub fn update_exogenous_infection_rate() {
    unimplemented!("This doesn't do anything.")
}

///
///
/// This should be part of the disease model; Because right now, there are
/// fewer individuals in the infected compartment when recovery is being
/// considered for this timestep.
pub fn update_exogenous_infection_rate_outside_of_disease_model(
    mut query: Query<(&mut Infected, &mut Susceptible, &ExogenousInfectionRate)>,
) {
    query.for_each_mut(|(mut inf, mut sus, rate)| {
        let delta_inf = sus.0 as f64 * rate.0 .0;
        let delta_inf = delta_inf.round() as usize;

        // is it possible to infect more animals on the farm?
        if sus.0 > delta_inf {
            sus.0 -= delta_inf;

            //TODO: consider adding a step here where you consider newInf -> Recovered
            // as those would have been ignored due to this step not being part of
            // the spread model.
            // let delta_rec = todo!("recovery rate is not readily available to be accessed from another system")
            // note: this is way rates should live on the entities that they
            // are affecting, as to maintain coherence when other models into
            // the simulation as well.
            //
            // rec.0 += delta_rec;
            // inf.0 += (delta_inf - delta_rec);
            inf.0 += delta_inf;
        }
    })
}
