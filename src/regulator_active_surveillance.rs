//!
//! This module works in the following way:
//!
//! * Detection probability: 1\% x infection rate
// ! * If detected, then there is a 50% chance of eliminating
//!   the infection completely, or 90%.
//!
//!

// Note that this implementation is to showcase the presence of the central
// located parameters, and not once spread-out over the entities.

use std::marker::PhantomData;

use crate::{
    parameters::{Probability, Rate},
    prelude::*,
    sir_spread_model::Infected,
};
use bevy::ecs::system::SystemParam;
use rand::{prelude::StdRng, Rng};
use rand_distr::Distribution;

#[derive(SystemParam)]
pub struct ActiveSurveillance<'a> {
    // _secret: PhantomData<&'a ()>,
    _secret: Res<'a, ()>,
    /// For each infected animal, this is the rate of detection, typically 1%.
    #[system_param(ignore)]
    detection_rate: Rate,
    /// Proportion of remaining infected animals if the fair coin flip turned out
    /// not to wipe out the infection on a farm. Typically 10%.
    #[system_param(ignore)]
    remaining_proportion: Probability,
}

pub fn update_active_surveillance(
    active_surveillance: &ActiveSurveillance,
    mut query: Query<&mut Infected>,
    mut rng: ResMut<StdRng>,
) {
    query.for_each_mut(|infected| {
        if infected.0 > 0 {
            //infected farm
            if rand_distr::Poisson::new((infected.0 as f64) * active_surveillance.detection_rate)
                .unwrap()
                .sample(&mut *rng)
                > 0
            {
                // the infection was detected
                if rng.gen_bool(0.5) {
                    //remove all infected, and make dead
                    //FIXME: removed animals from farm, where did the
                    // animals go? they ain't recovered!
                    infected.0 = 0;
                } else {
                    // failed to remove the entire infection.
                    infected.0 =
                        ((infected.0 as f64) * active_surveillance.remaining_proportion).round() as usize;
                }
            }
        }
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_active_surveillance() {

        todo!("missing a test")
    }
}