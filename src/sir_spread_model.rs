use bevy::prelude::*;

use crate::ScenarioConfiguration;

#[readonly::make]
#[derive(Debug, derive_more::Into, derive_more::From)]
pub struct Susceptible(pub f64);
#[readonly::make]
#[derive(Debug, derive_more::Into, derive_more::From)]
pub struct Infected(pub f64);
#[readonly::make]
#[derive(Debug, derive_more::Into, derive_more::From)]
pub struct Recovered(pub f64);
// pub struct Dead(pub usize);

#[readonly::make]
#[derive(Debug, Bundle)]
pub struct DiseaseCompartment {
    susceptible: Susceptible,
    infected: Infected,
    recovered: Recovered,
}

impl DiseaseCompartment {
    pub fn new(herd_size: usize) -> Self {
        Self {
            susceptible: (herd_size as f64).into(),
            infected: (0.).into(),
            recovered: (0.).into(),
        }
    }
}

// TODO: Add a "DiseaseParameter" that is part of the [ScenarioConfiguration]

/// Update disease dynamics
pub fn update_disease_compartments(
    scenario_configuration: Res<ScenarioConfiguration>,
    mut query: Query<(&mut Susceptible, &mut Infected, &mut Recovered)>,
) {
    let ScenarioConfiguration {
        infection_rate,
        recovery_rate,
        ..
    } = *scenario_configuration;
    for (mut susceptible, mut infected, mut recovered) in query.iter_mut() {
        let delta_infected = infection_rate * susceptible.0 * infected.0;
        // newly infected may only be atmost the number of susceptible animals
        let delta_infected = delta_infected.min(susceptible.0);
        let delta_recovered = recovery_rate * infected.0 as f64;
        // number of recovered may at most be the number of infected
        let delta_recovered = delta_recovered.min(infected.0);

        // dbg!(delta_infected, delta_recovered);

        susceptible.0 += -delta_infected;
        infected.0 += delta_infected - delta_recovered;
        recovered.0 += delta_recovered;
    }
}

/// Place one infected individual into the mix.
pub fn seed_infection(
    query: Query<(
        &mut Susceptible,
        &mut Infected,
    )>,
) {
    query.for_each_mut(|(mut susceptible, mut infected)| {
        susceptible.0 -= 1.;
        (susceptible.0 < 0.).then(|| panic!("no susceptible individuals to infect"));
        infected.0 += 1.;
    });
}
