//!
//! - [x] Run headless
//! - [x] Implement SIR model
//! - [ ] Add repopulation to the model
//!     Repopulation can happen where every time there isn't a an active change
//!     in the map, then the compartments gets "scaled back up" so as to revert
//!     back to nominal animal counts on the farm
//! - [ ] Add repetitions/iterations to the model
//! - [ ] Add recording through [sled]
//! - [ ] Add UI that shows progress
//! - [ ] Add CLI interface
//! - [ ] Add a between-herd infection that add a proportion of infected animals
//!       from one farm onto another.
//! - [ ] Implement true passive surveillance, which is the true/observed
//!       prevalence "watcher". Maybe set that to weekly or similar.
//! 
//! - [ ] Add the 50% -> {0% infected, or 90% recovered} regulator
//! - [ ] Ensure that the simulation is "actually" deterministic, when everything
//!       is set. 
//!
//!
//! inspiration/formulas can be found [here](https://www.uio.no/studier/emner/matnat/ifi/IN1900/h18/ressurser/slides/disease_modeling.pdf)
//! For [SEIR-model](http://indico.ictp.it/event/7960/session/3/contribution/19/material/slides/0.pdf)

use epi_bevy::{
    between_herd_spread_model,
    farm_id_to_entity_map::FarmIdEntityMap,
    parameters::{Probability, Rate},
    prelude::*,
    deprecated_active_surveillance::{DetectionRatePerAnimal, DetectionRatePerFarm},
    sir_spread_model,
};
use std::{collections::HashMap, convert::TryFrom};

use bevy::{
    app::{AppExit, ScheduleRunnerPlugin},
    diagnostic::{DiagnosticsPlugin, LogDiagnosticsPlugin},
    log::LogPlugin,
};

use epi_bevy::between_herd_spread_model::print_between_herd_infection_events;
use epi_bevy::cattle_population::{FarmId, HerdSize};
use epi_bevy::scenario_time::ScenarioTime;
use epi_bevy::sir_spread_model::{
    DiseaseCompartments, DiseaseParameters as WithinHerdDiseaseParameters,
};

mod disease_ecs_diagnostic;

//TODO: make this into a bundle and add all those "resources" that represent
// parameters that end up residing in the entities, by using the scenario
// configuration as the initial value for these.
//
// Note: this makes it so that the scenario configuration dictates how all
// these components look for all farms (initially). They can of course
// be amended afterwards, but remember, commands are executed at the end
// of a stage.
//
/// All the parameters for setting up a scenario-run.
#[derive(Debug)]
pub struct ScenarioConfiguration {
    /// Total number of herds in the scenario
    // total_herds: usize,
    /// Herd sizes
    // herd_sizes: Vec<usize>,
    /// Fail-safe for terminating the scenario.
    max_timesteps: u64,
    min_timesteps: u64,
    /// Alias: Iterations.
    max_repetitions: u64,
}

/// Update scenario ticks by one.
fn update_scenario_tick(mut scenario_tick: ResMut<ScenarioTime>) {
    scenario_tick.update_time(1);
    // info!("{:?}", *scenario_tick);
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, SystemLabel)]
enum Processes {
    Disease,
    Recording,
    Regulators,
}

fn main() {
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    //TODO: Add a CSV plugin
    // - [ ] Hide the CSV behind a mutex.
    // - [ ] Write all components of a specific entities (e.g. [CattleFarm])
    //       to a CSV file
    // - [ ] Save every other scenario / physical time.
    // - [ ] Save also in other modules (e.g. between-herd spread & a regulators).

    /// Used to fuse the chained systems, such that one doesn't get a compilation
    /// error even though the result of the last system isn't used.
    ///
    /// As [SystemStage] requires that the system-chain ends with unit-type `()`
    /// we have to use this to fuse the chain.
    ///
    /// Author: @TheRuwuMeatball
    pub fn dispose<T>(_: In<T>) {}

    App::build()
        .insert_resource(bevy::log::LogSettings {
            level: bevy::log::Level::DEBUG,
            ..Default::default()
        })
        .insert_resource(bevy::ecs::schedule::ReportExecutionOrderAmbiguities)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(DiagnosticsPlugin::default())
        .add_plugin(ScheduleRunnerPlugin::default())
        .add_plugin(LogPlugin::default())
        .add_plugins(MinimalPlugins)


        // TODO: Things that follow here 

        .insert_resource(StdRng::seed_from_u64(20210426))
        // .insert_resource(ScenarioTick(0))
        .insert_resource(ScenarioTime::new(1, None))
        .insert_resource(ScenarioConfiguration {
            // max_timesteps: usize::MAX(),
            // FIXME: make these part of the [ScenarioTime] resource
            min_timesteps: 3,
            max_timesteps: 10_000,
            //FIXME: this is currently unused
            max_repetitions: 2,
        })
        // .insert_resource(WithinHerdDiseaseParameters::new(0.0013, 0.008333))
        // .insert_resource(WithinHerdDiseaseParameters::new(0.0013, 0.008333))
        //TODO: this block adds parameters, but what I'd ideally want is for the SceneConfiguration to add
        // them. So is there such a thing as a Bundle of Resources?
        .insert_resource(WithinHerdDiseaseParameters::new(0.03, 0.01))
        .insert_resource(between_herd_spread_model::ContactRate::new(0.095))
        .insert_resource(DetectionRatePerAnimal(Rate::try_from(Probability::new(0.5).unwrap()).unwrap()))
        .insert_resource(DetectionRatePerFarm(Rate::try_from(Probability::new(0.01).unwrap()).unwrap()))
        // .insert_resource(DetectionRatePerAnimal(Rate::try_from(Probability::new(0.0).unwrap()).unwrap()))
        // .insert_resource(DetectionRatePerFarm(Rate::try_from(Probability::new(0.00).unwrap()).unwrap()))
        // .insert_resource(ContactRate::new(0.0))
        //TODO: Every time a new module is added to the mix, it needs a new setup
        // procedure to amend the farms with components pertaining to those new systems
        // they can be regulators, and at least any other thing that should be
        // extended somehow...
        .add_startup_system(epi_bevy::cattle_farm_recorder::setup_cattle_farm_recorder.system())
        .add_startup_system(epi_bevy::between_herd_spread_model_record::setup_between_herd_infection_events_recording.system())
        //TODO: this stage doesn't need to be parallel.. but it is?
        .add_startup_stage(Seed::Population, SystemStage::parallel())
        .add_startup_stage_after(Seed::Population, Seed::Infection, SystemStage::parallel())
        .add_startup_system_to_stage(Seed::Population, seed_cattle_population.system())
        .add_startup_system_to_stage(
            Seed::Infection,
            epi_bevy::sir_spread_model::seed_infection_random.system(),
        )
        .add_startup_stage_after(Seed::Infection, Seed::Contacts, SystemStage::parallel())
        .add_startup_system_to_stage(
            Seed::Contacts,
            between_herd_spread_model::setup_between_herd_spread_model.system(),
        )
        .add_startup_system_to_stage(Seed::Contacts, epi_bevy::deprecated_active_surveillance::setup_passive_surveillance.system())

        // Main-loop 

        .add_system(update_scenario_tick.system().before(Processes::Disease))
        .add_system_set(
            SystemSet::new()
                .label(Processes::Disease)
                .with_system(epi_bevy::sir_spread_model::update_disease_compartments.system())
                .with_system(
                    between_herd_spread_model::update_between_herd_spread_model
                        .system()
                        // .chain(print_between_herd_infection_events.system())
                        .chain(epi_bevy::between_herd_spread_model_record::record_between_herd_infection_events.system()),
                ),
        )
        .add_system_set(
            SystemSet::new()
                .label(Processes::Recording)
                .after(Processes::Disease)
                .with_system(
                    epi_bevy::population_model_record::print_total_infected_farms
                        .system(),
                ),
        )
        //TODO: Add a regulators system set! (and finish it)
        .add_system_set(SystemSet::new()
            // .with_system(epi_bevy::active_surveillance::active_surveillance.system())
            .label(Processes::Regulators))
        .add_system_set(
            SystemSet::new()
                // .with_run_criteria(epi_bevy::scenario_intervals::run_yearly.system())
                .with_run_criteria(epi_bevy::scenario_intervals::run_every_week.system())
                .with_system(
                    epi_bevy::cattle_farm_recorder::record_cattle_farm_components.system(),
                ),
        )
        // .add_system_to_stage(CoreStage::Update, examine_population.system())
        // TODO: add recorder
        // FIXME: this doesn't work;
        // .add_system_to_stage(CoreStage::First, print_population_disease_states.system())
        // .add_system_to_stage(CoreStage::Last, print_population_disease_states.system())
        // print state changes when they happen
        // .add_system(log_changes_in_infected.system())
        // print the state of the systems every 1000ms.
        // .add_system(
        //     log_every_half_second
        //         .system()
        //         .with_run_criteria(FixedTimestep::step(0.700)),
        // )
        // TODO: add application loop that displays the current estimates
        // .add_system(print_population_disease_states.system())
        .add_system(terminate_if_outbreak_is_over.system().after(Processes::Regulators))
        .run();

    info!("Finished simulation.");
}

// TODO: Maybe. Extract disease parameters from the cattle parameters. It is
// there right now, as because Herd-size is necessary to setup the disease
// compartments..
fn seed_cattle_population(
    mut commands: Commands,
    initial_disease_parameters: Option<Res<WithinHerdDiseaseParameters>>,
) {
    let initial_disease_parameters =
        initial_disease_parameters.expect("no default/initial disease parameters are set.");
    let cattle_population_bundle = epi_bevy::cattle_population::load_ring_population();
    // FarmId and Entity id has to correspond, thus we add a resource
    // to contain this mapping.
    //TODO: maybe just collect, then find the length, and iterate further then
    let mut farm_id_to_entity_map: HashMap<FarmId, _> =
        HashMap::with_capacity(cattle_population_bundle.clone().count());
    // info!("{:}", farm_id_to_entity_map.len());

    for bundle in cattle_population_bundle {
        let herd_size = bundle.herd_size;
        let farm_id = bundle.farm_id;
        // info!("inserted a herd of size {:?}", herd_size);

        let farm_entity_id = commands
            .spawn_bundle(bundle)
            .insert_bundle(DiseaseCompartments::new(herd_size.0))
            .insert(initial_disease_parameters.to_owned())
            .id();

        farm_id_to_entity_map.insert(farm_id, farm_entity_id);
    }

    commands.insert_resource(FarmIdEntityMap::from(farm_id_to_entity_map));
}

/// Stops the scenario if there are no active infections.
fn terminate_if_outbreak_is_over(
    scenario_configuration: Res<ScenarioConfiguration>,
    query: Query<&sir_spread_model::Infected, With<sir_spread_model::Infected>>,
    mut event_writer: EventWriter<AppExit>,
    tick: Res<ScenarioTime>,
) {
    let any_active_infection = query
        .iter()
        // .any(|x| approx::relative_ne!(x.0, 0., epsilon = 0.001));
        .any(|x| x.0 != 0);
    if
    //don't stop if minimum timesteps hasn't elapsed yet
    (scenario_configuration.min_timesteps <= tick.current_time())
        & (
            //stop if there are no more active infections
            (!any_active_infection) ||
    // stop if max timesteps have been reached
    scenario_configuration.max_timesteps == tick.current_time()
        )
    // || tick.ended()
    //TODO: Add the case where `tick.ended()` alone as to specifically
    // inform the runner that the convergence criteria was not met.
    {
        info!("Terminated at tick: {}", tick.current_time());
        event_writer.send(AppExit);
    }
}
