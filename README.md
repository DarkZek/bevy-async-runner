# Bevy Async Runner

Makes working with async in bevy easy by providing a way to 

```rust
pub fn set_loading(
    scheduler: Res<AsyncScheduler>
) {
    if local.session.is_none() {
        scheduler.schedule(test_system, async {
            format!("Woop async done!")
        });
    }
}
```