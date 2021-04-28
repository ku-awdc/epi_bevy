//!
//! - [x] Run headless
//! - [x] Implement SIR model
//! - [ ] Add repopulation to the model
//! - [ ] Add repetitions/iterations to the model
//! - [ ] Add recording through [sled]
//! - [ ] Add UI that shows progress
//! - [ ] Add CLI interface
//!
//!
//! inspiration/formulas can be found [here](https://www.uio.no/studier/emner/matnat/ifi/IN1900/h18/ressurser/slides/disease_modeling.pdf)
//! For [SEIR-model](http://indico.ictp.it/event/7960/session/3/contribution/19/material/slides/0.pdf)

use std::collections::HashMap;

//TODO: make a framework-prelude

use bevy::{app::AppExit, core::FixedTimestep, diagnostic::{DiagnosticsPlugin, LogDiagnosticsPlugin}, log::LogPlugin, prelude::*};
mod sir_spread_model;
use cattle_population::{FarmId, HerdSize};
use itertools::Itertools;
use sir_spread_model::{
    update_disease_compartments, DiseaseCompartments,
    DiseaseParameters as WithinHerdDiseaseParameters, Infected, Susceptible,
};

use crate::between_herd_spread_model::ContactRate;

mod between_herd_spread_model;

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
#[derive(Debug, PartialEq, Eq, Hash, Clone, StageLabel)]
enum Seed {
    /// Seed population stage
    Population,
    /// Seed infection stage
    Infection,
    /// Seed contacts stage
    Contacts,
}

fn main() {
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    App::build()
        .add_plugins(MinimalPlugins)
        // .add_plugins(DefaultPlugins)
        .add_plugin(LogPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(DiagnosticsPlugin)
        .insert_resource(StdRng::seed_from_u64(20210426))
        .insert_resource(ScenarioTick(0))
        .add_system(update_scenario_tick.system())
        .insert_resource(ScenarioConfiguration {
            total_herds: 2,
            herd_sizes: vec![140, 90],
            // max_timesteps: usize::MAX(),
            min_timesteps: 3,
            max_timesteps: 1_000_000,
            max_repetitions: 2,
        })
        .insert_resource(WithinHerdDiseaseParameters::new(0.003, 0.001))
        // .insert_resource(WithinHerdDiseaseParameters::new(0.0013, 0.008333))
        .insert_resource(ContactRate::new(0.5))
        // .insert_resource(ContactRate::new(0.0))
        .add_startup_stage(Seed::Population, SystemStage::parallel())
        .add_startup_stage_after(Seed::Population, Seed::Infection, SystemStage::parallel())
        .add_startup_system_to_stage(Seed::Population, seed_cattle_population.system())
        .add_startup_system_to_stage(Seed::Infection, sir_spread_model::seed_infection.system())
        .add_startup_stage_after(Seed::Infection, Seed::Contacts, SystemStage::parallel())
        .add_startup_system_to_stage(
            Seed::Contacts,
            between_herd_spread_model::setup_between_herd_spread_model.system(),
        )
        // TODO: Add disease spread stage
        .add_system_set(
            SystemSet::new()
                .with_system(update_disease_compartments.system())
                .with_system(between_herd_spread_model::update_between_herd_spread_model.system()),
        )
        // .add_system_to_stage(CoreStage::Update, examine_population.system())
        // TODO: add recorder
        // FIXME: this doesn't work;
        // .add_system_to_stage(CoreStage::First, print_population_disease_states.system())
        // .add_system_to_stage(CoreStage::Last, print_population_disease_states.system())
        // print state changes when they happen
        // .add_system(log_changes_in_infected.system())
        // print the state of the systems every 1000ms.
        .add_system(
            log_every_half_second
                .system()
                .with_run_criteria(FixedTimestep::step(0.700)),
        )
        // TODO: add application loop that displays the current estimates
        // TODO: stop if no-one is infected (or max timesteps has been reached)
        // .add_system(print_population_disease_states.system())
        .add_system(terminate_if_outbreak_is_over.system())
        // .insert_resource(ReportExecutionOrderAmbiguities) // requires [LogPlugin]
        .run();

    println!("Finished simulation.");
}

fn print_population_disease_states(
    tick: Res<ScenarioTick>,
    query: Query<(&Infected, &Susceptible)>,
    mut event_reader: EventReader<AppExit>,
) {
    if event_reader.iter().next().is_some() {
        let (inf, sus): (Vec<Infected>, Vec<Susceptible>) =
            query.iter().map(|x| (x.0, x.1)).unzip();
        println!(
            "{} =>  \nTotal infected: {:?}/
                    \nTotal susceptible: {:?}",
            tick.0,
            inf.into_iter().fold1(|x, y| x + y),
            sus.into_iter().fold1(|x, y| x + y),
        );
    }
}

//TODO: maybe this should implement Default as a panic?
#[readonly::make]
#[derive(Debug, derive_more::Into, derive_more::From)]
pub struct FarmIdEntityMap(pub HashMap<FarmId, Entity>);

fn seed_cattle_population(
    mut commands: Commands,
    initial_disease_parameters: Option<Res<WithinHerdDiseaseParameters>>,
) {
    let initial_disease_parameters =
        initial_disease_parameters.expect("no default/initial disease parameters are set.");
    let cattle_population_bundle = cattle_population::load_ring_population();
    // FarmId and Entity id has to correspond, thus we add a resource
    // to contain this mapping.
    //TODO: maybe just collect, then find the length, and iterate further then
    let mut farm_id_to_entity_map: HashMap<FarmId, _> =
        HashMap::with_capacity(cattle_population_bundle.clone().count());
    info!("{:}", farm_id_to_entity_map.len());

    for bundle in cattle_population_bundle {
        let herd_size = bundle.herd_size;
        let farm_id = bundle.farm_id;
        info!("inserted a herd of size {:?}", herd_size);

        let farm_entity_id = commands
            .spawn_bundle(bundle)
            .insert_bundle(DiseaseCompartments::new(herd_size.0))
            .insert(initial_disease_parameters.to_owned())
            .id();

        farm_id_to_entity_map.insert(farm_id, farm_entity_id);
    }

    commands.insert_resource(FarmIdEntityMap::from(farm_id_to_entity_map));
}

fn examine_population(
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
        info!(
            "{} => {}: {:>9.3}, {:>9.3}, {:>9.3}",
            scenario_tick.0, farm_id, susceptible.0, infected.0, recovered.0
        );
    }
}

/// Print disease states if infected state has changed.
fn log_changes_in_infected(
    query: Query<
        (
            &FarmId,
            &sir_spread_model::Susceptible,
            &sir_spread_model::Infected,
            &sir_spread_model::Recovered,
        ),
        // notice this query in comparison to [log_every_half_second]
        Changed<sir_spread_model::Infected>,
    >,
    scenario_tick: Res<ScenarioTick>,
) {
    for (farm_id, susceptible, infected, recovered) in query.iter() {
        println!(
            "({}, {}) => {:>9.3}, {:>9.3}, {:>9.3}",
            scenario_tick.0, farm_id.0, susceptible.0, infected.0, recovered.0
        );
    }
}

/// Stops the scenario if there are no active infections.
fn terminate_if_outbreak_is_over(
    scenario_configuration: Res<ScenarioConfiguration>,
    query: Query<&sir_spread_model::Infected, With<sir_spread_model::Infected>>,
    mut event_writer: EventWriter<AppExit>,
    tick: Res<ScenarioTick>,
) {
    let any_active_infection = query
        .iter()
        // .any(|x| approx::relative_ne!(x.0, 0., epsilon = 0.001));
        .any(|x| x.0 != 0);
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
        info!("Terminated at tick: {}", tick.0);
        event_writer.send(AppExit);
    }
}
