use bevy::prelude::*;
use bevy_async_runner::{AsyncRunner, AsyncRunnerPlugin};

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AsyncRunnerPlugin)
        .add_systems(Startup, welcome_user)
        .run();
}

fn welcome_user(runner: Res<AsyncRunner>) {
    runner.schedule(load_name(), print_name);
}

async fn load_name() -> String {
    "John".to_string()
}

fn print_name(In(name): In<String>) {
    info!("Hello, {}", name)
}
