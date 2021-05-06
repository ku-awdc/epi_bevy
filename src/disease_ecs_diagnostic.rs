use bevy::app::AppExit;
use epi_bevy::{
    cattle_population::FarmId,
    prelude::*,
    scenario_time::ScenarioTime,
    sir_spread_model::{self, Infected, Susceptible},
};

/// Printing the disease states whenever invoked. These disease states corresponds
/// to [DiseaseCompartments].
fn print_population_disease_states_at_app_exit(
    tick: Res<ScenarioTime>,
    query: Query<(&Infected, &Susceptible)>,
    mut event_reader: EventReader<AppExit>,
) {
    if event_reader.iter().next().is_some() {
        let (inf, sus): (Vec<Infected>, Vec<Susceptible>) =
            query.iter().map(|x| (x.0, x.1)).unzip();
        println!(
            "{} =>  \nTotal infected: {:?}/
                \nTotal susceptible: {:?}",
            tick.current_time(),
            inf.into_iter().fold1(|x, y| x + y),
            sus.into_iter().fold1(|x, y| x + y),
        );
    }
}



/// Print disease states if infected state every half a second;
#[deprecated(note = "please use run criteria to handle when systems are run.")]
fn log_every_half_second(
    query: Query<(
        &FarmId,
        &sir_spread_model::Susceptible,
        &sir_spread_model::Infected,
        &sir_spread_model::Recovered,
    )>,
    scenario_tick: Res<ScenarioTime>,
) {
    for (farm_id, susceptible, infected, recovered) in query.iter() {
        info!(
            "{} => {}: {:>9.3}, {:>9.3}, {:>9.3}",
            scenario_tick.current_time(),
            farm_id,
            susceptible.0,
            infected.0,
            recovered.0
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
    scenario_tick: Res<ScenarioTime>,
) {
    for (farm_id, susceptible, infected, recovered) in query.iter() {
        println!(
            "({}, {}) => {:>9.3}, {:>9.3}, {:>9.3}",
            scenario_tick.current_time(),
            farm_id.0,
            susceptible.0,
            infected.0,
            recovered.0
        );
    }
}
