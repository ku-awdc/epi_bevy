//!
//! - [x] Run headless
//! - [x] Implement SIR model
//! - [ ] Add repetitions/iterations to the model
//! - [ ] Add recording through [sled]
//! - [ ] Add UI that shows progress
//! - [ ] Add CLI interface
//!

use bevy::{app::AppExit, core::FixedTimestep, prelude::*};
mod sir_spread_model;
use sir_spread_model::{update_disease_compartments, DiseaseCompartment};

mod cattle_population;

/// All the parameters for setting up a scenario-run.
#[derive(Debug)]
pub struct ScenarioConfiguration {
    /// Total number of herds in the scenario
    total_herds: usize,
    /// Herd sizes
    herd_sizes: Vec<usize>,
    /// Fail-safe for terminating the scenario.
    max_timesteps: u64,
    /// Alias: Iterations.
    max_repetitions: u64,
    /// Infection rate
    infection_rate: f64,
    /// Recovery rate
    recovery_rate: f64,
}

/// Scenario ticks
#[derive(Debug, derive_more::Display)]
struct ScenarioTick(u64);

impl ScenarioTick {
    pub fn update(&mut self) {
        self.0 = self
            .0
            .checked_add(1)
            .expect("ran out of time ticks to give")
    }
}

/// Update scenario ticks by one.
fn update_scenario_tick(mut scenario_tick: ResMut<ScenarioTick>) {
    scenario_tick.update();
}

/// Defining stages for seeding the population and the infection.
/// This is necessary to add the infection after the population has been
/// initialised.
#[derive(PartialEq, Eq, Debug, Hash, Clone, StageLabel)]
enum Seed {
    /// Seed population stage
    Population,
    /// Seed infection stage
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
            // max_timesteps: usize::MAX(),
            max_timesteps: 1_000_000,
            max_repetitions: 2,
            infection_rate: 0.03,
            recovery_rate: 0.001,
        })
        .add_startup_stage(Seed::Population, SystemStage::parallel())
        .add_startup_stage_after(Seed::Population, Seed::Infection, SystemStage::parallel())
        .add_startup_system_to_stage(Seed::Population, seed_population.system())
        .add_startup_system_to_stage(Seed::Infection, sir_spread_model::seed_infection.system())
        // TODO: add spread model
        .add_system_set(
            SystemSet::new().with_system(update_disease_compartments.system()), // .with_system(new_infections.system())
                                                                                // .with_system(recovery.system())
        )
        // TODO: add recorder
        // print state changes when they happen
        .add_system(log_changes_in_infected.system())
        // print the state of the systems every 500ms.
        .add_system(
            log_every_half_second
                .system()
                .with_run_criteria(FixedTimestep::step(0.5)),
        )
        // TODO: add application loop that displays the current estimates
        // TODO: stop if no-one is infected (or max timesteps has been reached)
        .add_system(terminate_if_outbreak_is_over.system())
        // .insert_resource(ReportExecutionOrderAmbiguities) // requires [LogPlugin]
        .run();

    println!("Finished simulation.");
}


/// Add a susceptible population to the mix.
fn seed_population(mut command: Commands, scenario_configuration: Res<ScenarioConfiguration>) {
    for herd_size in &scenario_configuration.herd_sizes {
        command
            .spawn()
            .insert_bundle(DiseaseCompartment::new(*herd_size));
    }
}

/// Print disease states if infected state every half a second;
fn log_every_half_second(
    query: Query<(
        &sir_spread_model::Susceptible,
        &sir_spread_model::Infected,
        &sir_spread_model::Recovered,
    )>,
    scenario_tick: Res<ScenarioTick>,
) {
    for (susceptible, infected, recovered) in query.iter() {
        println!(
            "{} => {:>9.3}, {:>9.3}, {:>9.3}",
            scenario_tick, susceptible.0, infected.0, recovered.0
        );
    }
}

/// Print disease states if infected state has changed.
fn log_changes_in_infected(
    query: Query<
        (
            &sir_spread_model::Susceptible,
            &sir_spread_model::Infected,
            &sir_spread_model::Recovered,
        ),
        // notice this query in comparison to [log_every_half_second]
        Changed<sir_spread_model::Infected>,
    >,
    scenario_tick: Res<ScenarioTick>,
) {
    for (susceptible, infected, recovered) in query.iter() {
        println!(
            "{} => {:>9.3}, {:>9.3}, {:>9.3}",
            scenario_tick.0, susceptible.0, infected.0, recovered.0
        );
    }
}

/// Stops the scenario if there are no active infections.
fn terminate_if_outbreak_is_over(
    scenario_configuration: Res<ScenarioConfiguration>,
    query: Query<&sir_spread_model::Infected>,
    mut event_writer: EventWriter<AppExit>,
    tick: Res<ScenarioTick>,
) {
    let any_active_infection = query
        .iter()
        .any(|x| approx::relative_ne!(x.0, 0., epsilon = 0.001));
    if
    //stop if there are no more active infections
    (!any_active_infection) ||
    // stop if max timesteps have been reached
    scenario_configuration.max_timesteps == tick.0
    {
        event_writer.send(AppExit);
    }
}
