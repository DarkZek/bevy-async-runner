# Bevy Async Runner

Makes working with async in bevy easy by providing a way to 

```rust
fn welcome_user(runner: Res<AsyncRunner>) {
    runner.schedule(print_name, load_name());
}

async fn load_name() -> String {
    "John".to_string()
}

fn print_name(In(name): In<String>) {
    info!("Hello, {}", name)
}
```