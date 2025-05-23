# Bevy Async Runner

The **Bevy Async Runner** simplifies working with asynchronous code in the Bevy game engine. It provides a mechanism to schedule and execute async tasks and provide their result to any system.

## Example Usage

Below is an example of how to use the Bevy Async Runner to schedule and execute an async task:

```rust
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
```

## Installation

Add the following to your `Cargo.toml`:

```toml
# For wasm we can use default bevy io pool
[target.'cfg(target_arch = "wasm32")'.dependencies]
bevy-async-executor = "0.1"

# For native we should use tokio so that reqwest & others work
[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
bevy-async-runner = { version = "0.1", features = ["tokio-runtime-multi-thread"]}
```

## Features

#### `tokio-runtime-multi-thread`
Use Tokio as the async runtime instead of Bevy's default IoTaskPool.
This can be especially useful when integrating libraries that depend on Tokio (such as reqwest).
Does not work on wasm.

## License

This project is licensed under the [MIT License](LICENSE).

## Contributions

Contributions are welcome! Feel free to open issues or submit pull requests to improve the project.