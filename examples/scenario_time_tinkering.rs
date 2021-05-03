use bevy::app::ScheduleRunnerSettings;
use epi_bevy::prelude::*;
use epi_bevy::scenario_time::ScenarioTime;

fn main() {
    App::build()
        .add_plugins(MinimalPlugins)
        // .add_plugins(DefaultPlugins)
        .add_plugin(LogPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(DiagnosticsPlugin)
        .insert_resource(ScheduleRunnerSettings::run_once())
        // .insert_resource(StdRng::seed_from_u64(20210426))
        // .insert_resource(ScenarioTick(0))
        .insert_resource(ScenarioTime::new(1, 10))
        .run();
}
