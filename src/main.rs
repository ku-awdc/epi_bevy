//!
//! - [x] Run headless
//! - [ ] Implement SIR model
//! - [ ] Add UI that shows progress
//! - [ ] Add CLI interface
//!
#![feature(or_patterns)]

use bevy::{app::AppExit, prelude::*};
use disease_compartment::{update_disease_compartments, DiseaseCompartment};

#[derive(Debug)]
pub struct ScenarioConfiguration {
    total_herds: usize,
    herd_sizes: Vec<usize>,
    max_timesteps: usize,
    infection_rate: f64,
    recovery_rate: f64,
}

#[derive(Debug)]
struct ScenarioTick(usize);

impl ScenarioTick {
    pub fn update(&mut self) {
        self.0 += 1;
    }
}

fn update_scenario_tick(mut scenario_tick: ResMut<ScenarioTick>) {
    scenario_tick.update();
}

#[derive(PartialEq, Eq, Debug, Hash, Clone, StageLabel)]
enum Seed {
    Population,
    Infection,
}

fn main() {
    App::build()
        .add_plugins(MinimalPlugins)
        .insert_resource(ScenarioTick(0))
        .add_system(update_scenario_tick.system())
        .insert_resource(ScenarioConfiguration {
            total_herds: 2,
            herd_sizes: vec![140, 90],
            max_timesteps: usize::MAX,
            infection_rate: 0.03,
            recovery_rate: 0.01,
        })
        // .add_startup_stage(Seed::Population, stage)
        .add_startup_stage(Seed::Population, SystemStage::parallel())
        .add_startup_stage_after(Seed::Population, Seed::Infection, SystemStage::parallel())
        .add_startup_system_to_stage(Seed::Population, seed_population.system())
        .add_startup_system_to_stage(Seed::Infection, seed_infection.system())
        // TODO: add spread model
        .add_system_set(
            SystemSet::new().with_system(update_disease_compartments.system()), // .with_system(new_infections.system())
                                                                                // .with_system(recovery.system())
        )
        // TODO: add recorder
        // print state changes when they happen
        .add_system(log_changes_in_infected.system())
        // TODO: add application loop that displays the current estimates
        // TODO: stop if no-one is infected
        .add_system(terminate_if_outbreak_is_over.system())
        .run();

    println!("Finished simulation.");
}

/// Print disease states if infected state has changed.
fn log_changes_in_infected(
    query: Query<
        (
            &disease_compartment::Susceptible,
            &disease_compartment::Infected,
            &disease_compartment::Recovered,
        ),
        Changed<disease_compartment::Infected>,
    >,
) {
    for (S, I, R) in query.iter() {
        // dbg!(state);
        // println!("{:#.3?}", state);
        println!("{:>9.3}, {:>9.3}, {:>9.3}", S.0, I.0, R.0);
    }
}

/// Stops the scenario if there are no active infections.
fn terminate_if_outbreak_is_over(
    scenario_configuration: Res<ScenarioConfiguration>,
    query: Query<&disease_compartment::Infected>,
    mut event_writer: EventWriter<AppExit>,
    tick: Res<ScenarioTick>,
) {
    let any_active_infection = query.iter().any(|x| approx::relative_ne!(x.0, 0.));
    if
    //stop if there are no more active infections
    (!any_active_infection) ||
    // stop if max timesteps have been reached
    scenario_configuration.max_timesteps == tick.0
    {
        event_writer.send(AppExit);
    }
}

mod disease_compartment {
    use super::*;

    #[derive(Debug, derive_more::Into, derive_more::From)]
    pub struct Susceptible(pub f64);
    #[derive(Debug, derive_more::Into, derive_more::From)]
    pub struct Infected(pub f64);
    #[derive(Debug, derive_more::Into, derive_more::From)]
    pub struct Recovered(pub f64);
    // pub struct Dead(pub usize);

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

    pub fn update_disease_compartments(
        // mut commands: Commands,
        scenario_configuration: Res<ScenarioConfiguration>,
        mut query: Query<(&mut Susceptible, &mut Infected, &mut Recovered)>,
    ) {
        let ScenarioConfiguration {
            infection_rate,
            recovery_rate,
            ..
        } = *scenario_configuration;
        for (mut S, mut I, mut R) in query.iter_mut() {
            let delta_infected = infection_rate * S.0 * I.0;
            // newly infected may only be atmost the number of susceptible animals
            let delta_infected = delta_infected.min(S.0);
            let delta_recovered = recovery_rate * I.0 as f64;
            // number of recovered may at most be the number of infected
            let delta_recovered = delta_recovered.min(I.0);

            // dbg!(delta_infected, delta_recovered);

            S.0 += -delta_infected;
            I.0 += delta_infected - delta_recovered;
            R.0 += delta_recovered;
        }
    }
}

fn seed_population(mut command: Commands, scenario_configuration: Res<ScenarioConfiguration>) {
    for herd_size in &scenario_configuration.herd_sizes {
        command
            .spawn()
            .insert_bundle(DiseaseCompartment::new(*herd_size));
    }
}

fn seed_infection(
    query: Query<(
        &mut disease_compartment::Susceptible,
        &mut disease_compartment::Infected,
    )>,
) {
    query.for_each_mut(|(mut susceptible, mut infected)| {
        susceptible.0 -= 1.;
        infected.0 += 1.;
    });
}
