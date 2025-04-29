use bevy::app::App;
use bevy::prelude::{Commands, Plugin, ResMut, Update};
pub use crate::runner::AsyncRunner;

mod runner;

pub struct AsyncRunnerPlugin;

impl Plugin for AsyncRunnerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AsyncRunner::new())
            .add_systems(Update, complete_futures);
    }
}

fn complete_futures(mut scheduler: ResMut<AsyncRunner>, commands: Commands) {
    scheduler.run(commands);
}