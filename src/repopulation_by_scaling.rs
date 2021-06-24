//!
//!
//! Between-herd spread module that are based on animal movements would
//! create an imbalance at some point, as we do not simulate movement
//! from non-infectious farms. Thus to mitigate this imbalance, and
//! still preserve the severity of the infection in each farm, we scale
//! up to nominal herd-size.
//!
//!
//! When this repopulation even should happen is unclear.
//! It should probably occur in timesteps where no movements have happened
//! as this would technically negate the movement-effect on spread or
//! at least exasperate it.

use crate::prelude::*;
use crate::{
    populations::HerdSize,
    sir_spread_model::{Infected, Recovered, Susceptible},
};

/// This is intermittently linked with the disease spread model on the within
/// herd part and thus couldn't really be updated without some knowledge  
#[allow(dead_code)]
fn repopulate_rescale_disease_compartments(
    mut query: Query<(&mut Susceptible, &mut Infected, &mut Recovered, &HerdSize)>,
) {
    //TODO: consider a skipping if the proportions are small.

    query.for_each_mut(|(mut sus, mut inf, mut rec, herd_size)| {
        let compartments_sum = sus.0 + inf.0 + rec.0;
        sus.0 = ((sus.0 as f64) * (herd_size.0 as f64 / compartments_sum as f64)).round() as usize;
        inf.0 = ((inf.0 as f64) * (herd_size.0 as f64 / compartments_sum as f64)).round() as usize;
        rec.0 = ((rec.0 as f64) * (herd_size.0 as f64 / compartments_sum as f64)).round() as usize;
    });
}

mod tests;
