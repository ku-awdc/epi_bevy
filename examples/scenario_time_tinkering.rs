use bevy::app::ScheduleRunnerPlugin;
use epi_bevy::prelude::*;
use epi_bevy::scenario_time::ScenarioTime;

fn main() {
    App::build()
        .insert_resource(bevy::log::LogSettings {
            level: bevy::log::Level::DEBUG,
            ..Default::default()
        })
        .insert_resource(bevy::ecs::schedule::ReportExecutionOrderAmbiguities)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(DiagnosticsPlugin)
        .add_plugin(ScheduleRunnerPlugin::default())
        .add_plugin(LogPlugin)
        .add_plugins(MinimalPlugins)
        .insert_resource(ScenarioTime::new(1, 10))
        .add_system_set(
            SystemSet::new()
                .with_system(update_scenario_tick.system())
                .with_system(print_current_time.system())
                .label("main loop"),
        )
        .add_system(terminate_if_scenario_ended.system().after("main loop"))
        .run();
}

fn update_scenario_tick(mut scenario_tick: ResMut<ScenarioTime>) {
    scenario_tick.update_time(1);
}

fn print_current_time(scenario_time: Res<ScenarioTime>) {
    info!("{}", scenario_time.current_time())
}

/// Stops the scenario if there are no active infections.
fn terminate_if_scenario_ended(mut event_writer: EventWriter<AppExit>, tick: Res<ScenarioTime>) {
    if tick.ended() {
        info!("Terminated at tick: {}", tick.current_time());
        event_writer.send(AppExit);
    }
}