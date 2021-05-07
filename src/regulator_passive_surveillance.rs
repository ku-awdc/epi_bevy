//!
//!
//!
//! Prints the true prevalence right now.
//!
//! Could use [DetectionRate] and report an observed preference
//!

use std::convert::TryFrom;

use rand::{prelude::StdRng, Rng};

use crate::{
    parameters::{Probability, Rate},
    prelude::*,
    regulator_active_surveillance::DetectionRate,
    scenario_time::ScenarioTime,
    sir_spread_model::Infected,
};

pub struct TotalFarms(pub usize);

// Reports the population prevalence of the disease.
pub fn update_passive_surveillance(
    mut commands: Commands,
    query: Query<&Infected>,
    total_farms: Option<Res<TotalFarms>>,
    scenario_time: Res<ScenarioTime>,
    mut rng: ResMut<StdRng>,
    detection_rate: Res<DetectionRate>,
) {
    // if the number of total farms isn't available then write it down.
    let total_farms = if let Some(total_farms) = total_farms {
        total_farms.0
    } else {
        let total_farms = query.iter().count();
        commands.insert_resource(TotalFarms(total_farms));

        total_farms
    };

    // info!("{:>5} => True prevalence: {:.4}", scenario_time.current_time(), true_prevalence);

    let observed_prevalence = query
        .iter()
        .filter(|x| {
            // no false positives

            (x.0 > 0)
                && rng.gen_bool(
                    Probability::try_from(Rate::new((x.0 as f64) * detection_rate.0 .0).unwrap())
                        .unwrap()
                        .0,
                )
        })
        .count() as f64
        / total_farms as f64;
    // dbg!(total_farms);

    let true_prevalence = query.iter().filter(|x| x.0 > 0).count() as f64 / total_farms as f64;

    info!(
        "{:>5} => True prevalence: {:.4}\tObserved_prevalence: {:.4}",
        scenario_time.current_time(),
        true_prevalence,
        observed_prevalence,
    );
}
