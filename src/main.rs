//!
//! - [x] Run headless
//! - [x] Implement SIR model
//! - [ ] Add repetitions/iterations to the model
//! - [ ] Add recording through [sled]
//! - [ ] Add UI that shows progress
//! - [ ] Add CLI interface
//!

use bevy::{app::AppExit, core::FixedTimestep, ecs::system::QueryFetch, prelude::*};
mod sir_spread_model;
use cattle_population::{CattleFarm, CattleFarmBundle, FarmId, HerdSize};
use sir_spread_model::{update_disease_compartments, DiseaseCompartments, Infected, Susceptible};

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
    min_timesteps: u64,
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
            min_timesteps: 3,
            max_timesteps: 1_000_000,
            max_repetitions: 2,
            infection_rate: 0.03,
            recovery_rate: 0.001,
        })
        .add_startup_stage(Seed::Population, SystemStage::parallel())
        .add_startup_stage_after(Seed::Population, Seed::Infection, SystemStage::parallel())
        .add_startup_system_to_stage(Seed::Population, seed_cattle_population.system())
        // .add_startup_system_to_stage(Seed::Population, seed_population.system())
        .add_startup_system_to_stage(Seed::Infection, sir_spread_model::seed_infection.system())
        // TODO: add spread model
        .add_system_set_to_stage(
            CoreStage::Update,
            SystemSet::new().with_system(update_disease_compartments.system()), // .with_system(new_infections.system())
                                                                                // .with_system(recovery.system())
        )
        // .add_system_to_stage(CoreStage::Update, examine_population.system())
        // TODO: add recorder
        // print state changes when they happen
        // .add_system(log_changes_in_infected.system())
        // print the state of the systems every 500ms.
        .add_system(
            log_every_half_second
                .system()
                .with_run_criteria(FixedTimestep::step(0.5)),
        )
        // TODO: add application loop that displays the current estimates
        // TODO: stop if no-one is infected (or max timesteps has been reached)
        .add_system_to_stage(CoreStage::Update, terminate_if_outbreak_is_over.system())
        // .insert_resource(ReportExecutionOrderAmbiguities) // requires [LogPlugin]
        .run();

    println!("Finished simulation.");
}

fn seed_cattle_population(mut commands: Commands) {
    let cattle_population_bundle = cattle_population::load_ring_population();

    // command.spawn_batch(cattle_population_bundle)

    for bundle in cattle_population_bundle {
        let herd_size = bundle.herd_size.clone();

        commands
            .spawn_bundle(bundle)
            .insert_bundle(DiseaseCompartments::new(herd_size.0));
    }
}

fn examine_population(
    commands: Commands,
    scenario_tick: Res<ScenarioTick>,
    query: Query<(&HerdSize, &Susceptible, &Infected)>,
    // query: Query<(&HerdSize), WithBundle<CattleFarmBundle>>,
) {
    dbg!(scenario_tick.0);
    query.iter().take(2).for_each(|x| {
        dbg!(x);
    });
}

// /// Add a susceptible population to the mix.
// fn seed_population(mut command: Commands, scenario_configuration: Res<ScenarioConfiguration>) {
//     let population =
//     for herd_size in &scenario_configuration.herd_sizes {
//         command
//             .spawn()
//             .insert_bundle(DiseaseCompartments::new(*herd_size));
//     }
// }

/// Print disease states if infected state every half a second;
fn log_every_half_second(
    query: Query<(
        &FarmId,
        &sir_spread_model::Susceptible,
        &sir_spread_model::Infected,
        &sir_spread_model::Recovered,
    )>,
    scenario_tick: Res<ScenarioTick>,
) {
    for (farm_id, susceptible, infected, recovered) in query.iter() {
        println!(
            "{} => {}: {:>9.3}, {:>9.3}, {:>9.3}",
            scenario_tick.0, farm_id, susceptible.0, infected.0, recovered.0
        );
    }
}

// /// Print disease states if infected state has changed.
// fn log_changes_in_infected(
//     query: Query<
//         (
//             &FarmId,
//             &sir_spread_model::Susceptible,
//             &sir_spread_model::Infected,
//             &sir_spread_model::Recovered,
//         ),
//         // notice this query in comparison to [log_every_half_second]
//         Changed<sir_spread_model::Infected>,
//     >,
//     scenario_tick: Res<ScenarioTick>,
// ) {
//     for (farm_id, susceptible, infected, recovered) in query.iter() {
//         println!(
//             "({}, {}) => {:>9.3}, {:>9.3}, {:>9.3}",
//             scenario_tick.0, farm_id.0, susceptible.0, infected.0, recovered.0
//         );
//     }
// }

/// Stops the scenario if there are no active infections.
fn terminate_if_outbreak_is_over(
    scenario_configuration: Res<ScenarioConfiguration>,
    query: Query<&sir_spread_model::Infected, With<sir_spread_model::Infected>>,
    mut event_writer: EventWriter<AppExit>,
    tick: Res<ScenarioTick>,
) {
    let any_active_infection = query
        .iter()
        .any(|x| approx::relative_ne!(x.0, 0., epsilon = 0.001));
    if
    //don't stop if minimum timesteps hasn't elapsed yet
    (scenario_configuration.min_timesteps <= tick.0)
        & (
            //stop if there are no more active infections
            (!any_active_infection) ||
    // stop if max timesteps have been reached
    scenario_configuration.max_timesteps == tick.0
        )
    {
        event_writer.send(AppExit);
    }
}
