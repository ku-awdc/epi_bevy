use std::fs::File;
use csv::Writer;

use crate::between_herd_spread_model::InfectionEvents;
use crate::prelude::*;

#[derive(derive_more::From)]
pub struct BetweenHerdInfectionEventsRecorder(Writer<File>);

/// This is coupled with system [record_between_herd_infection_events].
pub fn setup_between_herd_infection_events_recording(mut commands: Commands) {
    //TODO: determine an appropriate buffer capacity
    let buffer_capacity_in_bytes = 100_000_000; // 100 mb.
                                                // create the path (not necessarily the file)
                                                //TODO: put this somewhere else
    let path_to_csv_file: std::path::PathBuf = "outputs/between_herd_infection_events.csv".into();
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
        .flexible(false)
        .delimiter(b';') // change to `false`
        .from_writer(wtr);
    csv_writer
        .write_record(&[
            "scenario_tick",
            "batch_id",
            "origin_farm_id",
            "target_farm_id",
            "new_infections",
        ])
        .unwrap();

    commands.insert_resource(BetweenHerdInfectionEventsRecorder::from(csv_writer));
}

/// The saved fields must correspond to [setup_between_herd_infection_events_recording]
pub fn record_between_herd_infection_events(
    In(events): In<Option<InfectionEvents>>,
    mut csv_file: ResMut<BetweenHerdInfectionEventsRecorder>,
    // scenario_time: Res<ScenarioTime>,
) {
    if let Some(events) = events {
        let InfectionEvents {
            scenario_tick,
            batch_id,
            events_values,
        } = events;

        events_values.into_iter().for_each(|x| {
            csv_file.0.serialize((scenario_tick, batch_id, x)).unwrap();
        })
    } else {
        // no infection events
    }
}
