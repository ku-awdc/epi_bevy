use bevy::prelude::*;
use csv::Writer;
use std::fs::File;

use crate::{
    // cattle_population::{CattleFarm, FarmId},
    populations::{Cattle, FarmId},
    scenario_time::scenario_timer::ScenarioTime,
    sir_spread_model::{Infected, Recovered, Susceptible},
};

#[derive(derive_more::From)]
pub struct CattleFarmsCSVRecorder(Writer<File>);

/// This is coupled with system [record_cattle_farm_components].
pub fn setup_cattle_farm_recorder(mut commands: Commands) {
    //TODO: determine an appropriate buffer capacity
    let buffer_capacity_in_bytes = 100_000_000; // 100 mb.
                                                // create the path (not necessarily the file)
                                                //TODO: put this somewhere else
    let path_to_csv_file: std::path::PathBuf = "outputs/cattle_farm_outputs.csv".into();
    let mut path_to_directory = path_to_csv_file.clone();
    path_to_directory.pop();
    std::fs::create_dir_all(path_to_directory).unwrap();

    let wtr = std::fs::OpenOptions::new()
        // .append(false)
        .create(true)
        .write(true)
        .truncate(true)
        .open(path_to_csv_file)
        .unwrap();

    let mut csv_writer = csv::WriterBuilder::new()
        .has_headers(false)
        .buffer_capacity(buffer_capacity_in_bytes)
        .flexible(false) // change to `false`
        .delimiter(b';')
        .from_writer(wtr);
    csv_writer
        .write_record(&[
            "scenario_time",
            "farm_id",
            "susceptible",
            "infected",
            "recovered",
        ])
        .unwrap();

    commands.insert_resource(CattleFarmsCSVRecorder::from(csv_writer));
}

/// Note that changing the components queried requires an equivalent change
/// in the setup system i.e. [setup_cattle_farm_recorder]
pub fn record_cattle_farm_components(
    // commands: Commands,
    mut csv_file: ResMut<CattleFarmsCSVRecorder>,
    scenario_time: Res<ScenarioTime>,
    query: Query<(&FarmId, &Susceptible, &Infected, &Recovered), With<Cattle>>,
) {
    // info!("Recorded to csv at {} ", *scenario_time);

    query.for_each(|x| {
        csv_file
            .0
            .serialize((scenario_time.current_time(), x))
            .unwrap();
    })

    //FIXME: flush here?
}
