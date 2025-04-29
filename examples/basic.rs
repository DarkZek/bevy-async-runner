use bevy::prelude::*;
use bevy_async_runner::AsyncRunnerPlugin;
use bevy_async_runner::runner::AsyncRunner;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AsyncRunnerPlugin)
        .add_systems(Startup, welcome_user)
        .run();
}

fn welcome_user(runner: Res<AsyncRunner>) {
    runner.schedule(print_name, load_name());
}

async fn load_name() -> String {
    "John".to_string()
}

fn print_name(In(name): In<String>) {
    info!("Hello, {}", name)
}