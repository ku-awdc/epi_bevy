//! This module works in the following way:
//!
//! * Detection probability: 1\% x infection rate
//! * If detected, then there is a 50% chance of eliminating
//!   the infection completely, or 90%.
//!
//!
//!
//! This module works in the following way:
//!
//! * Detection probability: 1\% x infection rate
//! * If detected, then there is a 50% chance of eliminating
//!   the infection completely, or 90%.
//!
//!

// Note that this implementation is to showcase the presence of the central
// located parameters, and not once spread-out over the entities.

use std::convert::TryFrom;

use crate::{
    parameters::{Probability, Rate},
    prelude::*,
    sir_spread_model::Infected,
    tools::FloatExt,
};
use bevy::ecs::system::SystemParam;
use rand::{prelude::StdRng, Rng};

/// Rate of detection
#[readonly::make]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, derive_more::Into, derive_more::Display, derive_new::new)]
pub struct DetectionRate(pub Rate);

/// Proportion of animals that remain infected 50% of the time, if the outbreak
/// on farm was detected. For the latter, see [DetectionRate].
#[readonly::make]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, derive_more::Into, derive_more::Display, derive_new::new)]
pub struct RemainingProportion(pub Probability);

#[derive(SystemParam)]
pub struct ActiveSurveillance<'a> {
    /// For each infected animal, this is the rate of detection, typically 1%.
    // #[system_param(ignore)]
    detection_rate: Option<Res<'a, DetectionRate>>,
    /// Proportion of remaining infected animals if the fair coin flip turned out
    /// not to wipe out the infection on a farm. Typically 10%.
    // #[system_param(ignore)]
    remaining_proportion: Option<Res<'a, RemainingProportion>>,
}

pub fn update_active_surveillance(
    active_surveillance: ActiveSurveillance,
    mut query: Query<&mut Infected>,
    mut rng: ResMut<StdRng>,
) {
    let detection_rate = active_surveillance.detection_rate.as_ref().unwrap().0;
    let remaining_proportion = active_surveillance.remaining_proportion.as_ref().unwrap().0;

    // dbg!(detection_rate, remaining_proportion);

    query.for_each_mut(|mut infected| {
        if infected.0 > 0 {
            //infected farm
            if rng.gen_bool(
                Probability::try_from(Rate::new((infected.0 as f64) * detection_rate.0).unwrap())
                    .unwrap()
                    .0,
            ) {
                // the infection was detected
                if rng.gen_bool(0.5) {
                    //remove all infected, and make dead
                    //FIXME: removed animals from farm, where did the
                    // animals go? they ain't recovered!
                    infected.0 = 0;
                } else {
                    // failed to remove the entire infection.
                    infected.0 = ((infected.0 as f64) * remaining_proportion.0)
                        .round_stoch(&mut *rng) as usize;
                }
            }
        }
    })
}

mod tests;
