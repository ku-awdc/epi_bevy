use bevy::prelude::*;
use rand::{prelude::StdRng, Rng};

#[readonly::make]
#[derive(Debug, Clone, Copy)]
pub struct DiseaseParameters {
    /// Infection rate
    infection_rate: f64,
    /// Recovery rate
    recovery_rate: f64,
}

impl DiseaseParameters {
    pub fn new(infection_rate: f64, recovery_rate: f64) -> Self {
        Self {
            infection_rate,
            recovery_rate,
        }
    }
}

#[readonly::make]
#[derive(Debug, Clone, Copy, derive_more::Into, derive_more::From, derive_more::Add)]
pub struct Susceptible(pub usize);
#[readonly::make]
#[derive(Debug, Clone, Copy, derive_more::Into, derive_more::From, derive_more::Add)]
pub struct Infected(pub usize);

impl Infected {
    pub fn new(total_infected: usize) -> Self {
        Self(total_infected)
    }

    pub fn add(&mut self) {
        self.0 += 1;
    }
}

#[readonly::make]
#[derive(Debug, derive_more::Into, derive_more::From)]
pub struct Recovered(pub usize);
// pub struct Dead(pub usize);

/// This is only used to instantiate the entites that are susceptible
/// to this disease.
#[readonly::make]
#[derive(Debug, Bundle)]
pub struct DiseaseCompartments {
    susceptible: Susceptible,
    infected: Infected,
    recovered: Recovered,
}

impl DiseaseCompartments {
    pub fn new(herd_size: usize) -> Self {
        Self {
            susceptible: herd_size.into(),
            infected: 0.into(),
            recovered: 0.into(),
        }
    }
}

// TODO: Add a "DiseaseParameter" that is part of the [ScenarioConfiguration]

/// Update disease dynamics
pub fn update_disease_compartments(
    // scenario_configuration: Res<ScenarioConfiguration>,
    mut query: Query<(
        &mut Susceptible,
        &mut Infected,
        &mut Recovered,
        &DiseaseParameters,
    )>,
    mut rng: ResMut<StdRng>,
) {
    for (mut susceptible, mut infected, mut recovered, disease_parameters) in query.iter_mut() {
        // dbg!("any");
        let DiseaseParameters {
            infection_rate,
            recovery_rate,
            ..
        } = *disease_parameters;

        // maybe no-one ever recovers..
        // and maybe no-one ever get infected, so we need to do something about this..

        let delta_infected = infection_rate * (susceptible.0 * infected.0) as f64;
        // let delta_infected = delta_infected.round() as usize;
        let delta_infected = if rng.gen_bool(delta_infected.fract()) {
            delta_infected.ceil()
        } else {
            delta_infected.floor()
        } as usize;

        // newly infected may only be atmost the number of susceptible animals
        let delta_infected = delta_infected.min(susceptible.0);
        let delta_recovered = recovery_rate * infected.0 as f64;
        // let delta_recovered = delta_recovered.round() as usize;
        let delta_recovered = if rng.gen_bool(delta_recovered.fract()) {
            delta_recovered.ceil()
        } else {
            delta_recovered.floor()
        } as usize;

        // number of recovered may at most be the number of infected
        let delta_recovered = delta_recovered.min(infected.0);

        susceptible.0 = susceptible.0.saturating_sub(delta_infected);
        infected.0 = if delta_infected < delta_recovered {
            infected.0.saturating_sub(delta_recovered - delta_infected)
        } else {
            infected.0.saturating_add(delta_infected - delta_recovered)
        };

        // infected
        //     .0
        //     .saturating_add(delta_infected.saturating_sub(delta_recovered));
        recovered.0 = recovered.0.saturating_add(delta_recovered);
    }
}

/// Place one infected individual into the mix.
pub fn seed_infection(query: Query<(&mut Susceptible, &mut Infected)>) {
    let mut empty_query = true;
    query.for_each_mut(|(mut susceptible, mut infected)| {
        susceptible.0 -= 1;
        susceptible.0 = susceptible
            .0
            .checked_sub(1)
            .expect("no susceptible individuals to infect");

        // (susceptible.0 < 0).then(|| panic!("no susceptible individuals to infect"));
        infected.0 = infected.0.saturating_add(1);

        empty_query = false;
    });

    if empty_query {
        println!("failed to seed infection, as no viable infection point was found");
    }
}
