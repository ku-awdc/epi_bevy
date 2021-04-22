//!
//! - [x] Run headless
//! - [x] Implement SIR model
//! - [ ] Add UI that shows progress
//! - [ ] Add CLI interface
//!
#![feature(or_patterns)]

use bevy::{app::AppExit, core::FixedTimestep, prelude::*};
use disease_compartment::{update_disease_compartments, DiseaseCompartment};

/// All the parameters for setting up a scenario-run.
#[derive(Debug)]
pub struct ScenarioConfiguration {
    /// Total number of herds in the scenario
    total_herds: usize,
    /// Herd sizes
    herd_sizes: Vec<usize>,
    /// Fail-safe for terminating the scenario.
    max_timesteps: u64,
    /// Infection rate
    infection_rate: f64,
    /// Recovery rate
    recovery_rate: f64,
}

/// Scenario ticks
#[derive(Debug)]
struct ScenarioTick(u64);

impl ScenarioTick {
    pub fn update(&mut self) {
        // self.0 += 1;
        self.0 = self
            .0
            .checked_add(1)
            .expect("ran out of time ticks to give")
    }
}

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
        // .add_plugins(MinimalPlugins)
        .add_plugins(DefaultPlugins)

        .add_startup_system(setup.system())
        
        .insert_resource(ScenarioTick(0))
        .add_system(update_scenario_tick.system())
        .insert_resource(ScenarioConfiguration {
            total_herds: 2,
            herd_sizes: vec![140, 90],
            max_timesteps: u64::MAX,
            infection_rate: 0.03,
            recovery_rate: 0.001,
        })
        .add_startup_stage(Seed::Population, SystemStage::parallel())
        .add_startup_stage_after(Seed::Population, Seed::Infection, SystemStage::parallel())
        .add_startup_system_to_stage(Seed::Population, seed_population.system())
        .add_startup_system_to_stage(
            Seed::Infection,
            disease_compartment::seed_infection.system(),
        )
        // TODO: add spread model
        .add_system_set(
            SystemSet::new().with_system(update_disease_compartments.system()), // .with_system(new_infections.system())
                                                                                // .with_system(recovery.system())
        )
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
        .add_system(terminate_if_outbreak_is_over.system())
        .run();

    println!("Finished simulation.");
}


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 2d camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    
    commands.spawn()
        .insert_bundle(
            TextSection
        );
    commands.spawn_bundle(Text2dBundle {
        text: Text::with_section(
            "This text is in the 2D scene.",
            TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 13.0,
                color: Color::BLACK,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        ..Default::default()
    });
}

/// Print disease states if infected state every half a second;
fn log_every_half_second(
    query: Query<(
        &disease_compartment::Susceptible,
        &disease_compartment::Infected,
        &disease_compartment::Recovered,
    )>,
) {
    for (susceptible, infected, recovered) in query.iter() {
        // dbg!(state);
        // println!("{:#.3?}", state);
        println!(
            "{:>9.3}, {:>9.3}, {:>9.3}",
            susceptible.0, infected.0, recovered.0
        );
    }
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
    for (susceptible, infected, recovered) in query.iter() {
        // dbg!(state);
        // println!("{:#.3?}", state);
        println!(
            "{:>9.3}, {:>9.3}, {:>9.3}",
            susceptible.0, infected.0, recovered.0
        );
    }
}

/// Stops the scenario if there are no active infections.
fn terminate_if_outbreak_is_over(
    scenario_configuration: Res<ScenarioConfiguration>,
    query: Query<&disease_compartment::Infected>,
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

mod disease_compartment {
    use super::*;

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

    pub fn seed_infection(
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
}

fn seed_population(mut command: Commands, scenario_configuration: Res<ScenarioConfiguration>) {
    for herd_size in &scenario_configuration.herd_sizes {
        command
            .spawn()
            .insert_bundle(DiseaseCompartment::new(*herd_size));
    }
}
