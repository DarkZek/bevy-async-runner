use std::time::Duration;
use bevy::prelude::*;
use bevy_async_runner::{AsyncRunner, AsyncRunnerPlugin};
use serde::Deserialize;
use tokio::time::sleep;

#[cfg(all(not(feature = "tokio-runtime-multi-thread"), not(target_arch = "wasm32")))]
compile_error!("Tokio feature required to use `reqwest`. Try `--features=\"tokio-runtime-multi-thread\"");

#[cfg(all(target_arch = "wasm32", any(feature = "tokio-runtime", feature = "tokio-runtime-multi-thread")))]
compile_error!("Tokio single threaded runtime will be blocked when running Bevy, so this example does not work on non wasm targets");

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AsyncRunnerPlugin)
        .insert_resource(IpAddress(String::default()))
        .insert_resource(Time::<Fixed>::from_hz(4.0))
        .add_systems(Startup, fetch_ip)
        .add_systems(FixedUpdate, print_ip)
        .run();
}

#[derive(Resource)]
struct IpAddress(String);

fn fetch_ip(runner: Res<AsyncRunner>) {
    runner.schedule(load_name(), |In(name): In<Result<String>>, mut stored_ip: ResMut<IpAddress>| {
        stored_ip.0 = name.unwrap();
    });
}

#[derive(Deserialize)]
struct IpifyResponse {
    ip: String
}

async fn load_name() -> Result<String> {

    sleep(Duration::from_secs_f64(1.0)).await;

    let response = reqwest::Client::new()
        .get("https://api.ipify.org?format=json")
        .send()
        .await?;

    let data = response
        .json::<IpifyResponse>()
        .await?;

    Ok(data.ip)
}

fn print_ip(res: Res<IpAddress>) {
    info!("Ip Address: {}", res.0);
}