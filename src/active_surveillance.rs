//! Passive surveillance regulator
//!
//! Detection happen as 1% x infected
//!
//! And the concrete animals being removed should be 50% x infected
//!
//! Ideas
//!
//! - [ ] Add delay of maybe 7 days before removing those dead animals
//!

use std::convert::TryFrom;

use crate::{
    cattle_population::CattleFarm,
    parameters::{Probability, Rate},
    prelude::*,
    sir_spread_model::{Infected, Susceptible},
};

use rand::{prelude::StdRng, Rng};
use rand_distr::{Binomial, Distribution};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, derive_more::Into, derive_more::Display)]
pub struct DetectionRatePerAnimal(pub crate::parameters::Rate);

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, derive_more::Into, derive_more::Display)]
pub struct DetectionRatePerFarm(pub crate::parameters::Rate);

// #[derive(SystemParam)]
pub struct PassiveRegulator {}

//TODO: this relies on [CattleFarm] being part of the entity.

/// Here we add the detection rate to each farm, as to be able to change it
/// on a pr. farm basis later on.
///
/// Add this to a startup system. Preferably after [CattleFarm]-entities has
/// been added, otherwise this query is invalid.
pub fn setup_passive_surveillance(
    mut commands: Commands,
    initial_detection_rate_pr_farm: Option<Res<DetectionRatePerFarm>>,
    initial_detection_rate_pr_animal: Option<Res<DetectionRatePerAnimal>>,
    query: Query<(Entity, &CattleFarm)>,
) {
    let initial_detection_rate_pr_farm: DetectionRatePerFarm = initial_detection_rate_pr_farm
        .expect("Missing initial `DetectionRatePerFarm` as a resource.")
        .to_owned();
    let initial_detection_rate_pr_animal: DetectionRatePerAnimal = initial_detection_rate_pr_animal
        .expect("Missing initial `DetectionRatePerAnimal` as a resource.")
        .to_owned();

    query.for_each(|(entity, _)| {
        commands.entity(entity).insert_bundle((
            initial_detection_rate_pr_animal,
            initial_detection_rate_pr_farm,
        ));
    });
}

pub fn active_surveillance(
    // regulator: PassiveRegulator,
    mut query: Query<(
        &mut Infected,
        &mut Susceptible,
        &DetectionRatePerFarm,
        &DetectionRatePerAnimal,
    )>,
    mut rng: ResMut<StdRng>,
) {
    query.for_each_mut(|(mut infected, mut susceptible, dfarm, danimal)| {
        if infected.0 > 0
            && rng.gen_bool(
                Probability::try_from(Rate::new(infected.0 as f64 * (dfarm.0).0).unwrap())
                    .map(|x| x.0)
                    .unwrap(),
            )
        {
            // detected infection now to remove animals
            let delta = Binomial::new(
                infected.0 as _,
                Probability::try_from(danimal.0).map(|x| x.0).unwrap(),
            )
            .unwrap();
            let delta = delta.sample(&mut *rng) as usize;
            infected.0 -= delta;
            susceptible.0 += delta;
        }
    });
}
