use bevy::prelude::*;
use csv::Writer;
use std::fs::File;

use crate::{cattle_population::{CattleFarm, FarmId}, scenario_time::ScenarioTime, sir_spread_model::{Infected, Recovered, Susceptible}};

#[derive(derive_more::From)]
pub struct CattleFarmsCSVRecorder(Writer<File>);

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
        .append(true)
        .create(true)
        .open(path_to_csv_file)
        .unwrap();

    let csv_writer = csv::WriterBuilder::new()
        .has_headers(false)
        .buffer_capacity(buffer_capacity_in_bytes)
        .flexible(true) // change to `false`
        .from_writer(wtr);

    commands.insert_resource(CattleFarmsCSVRecorder::from(csv_writer));
}

//TODO: Some queries

pub fn record_cattle_farm_components(
    // commands: Commands,
    mut csv_file: ResMut<CattleFarmsCSVRecorder>,
    scenario_time: Res<ScenarioTime>,
    query: Query<(&FarmId, &Susceptible, &Infected, &Recovered), With<CattleFarm>>,
) {
    query.for_each(|x| {
        // info!("{:?}", x);
        // csv_file.0.write_record();
        let (farm_id, sus, inf, rec) = x;
        
        csv_file.0.serialize((*scenario_time, x)).unwrap();
    })

    //FIXME: flush here?
}
