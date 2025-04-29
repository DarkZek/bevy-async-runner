use bevy::app::App;
use bevy::prelude::{Commands, Plugin, Res, Update};
use crate::runner::AsyncRunner;

pub mod runner;

pub struct AsyncRunnerPlugin;

impl Plugin for AsyncRunnerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AsyncRunner::new())
            .add_systems(Update, complete_futures);
    }
}

fn complete_futures(scheduler: Res<AsyncRunner>, commands: Commands) {
    scheduler.run(commands);
}