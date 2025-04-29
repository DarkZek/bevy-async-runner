use std::future::Future;
use std::sync::{Arc, Mutex};
use bevy::prelude::{Commands, IntoSystem, Resource, SystemInput};
use bevy::tasks::IoTaskPool;

#[derive(Resource)]
pub struct AsyncRunner {
    scheduled_runs: Arc<Mutex<Vec<Box<dyn FnOnce(&mut Commands) + Send + Sync>>>>,
    pool: &'static IoTaskPool
}

impl AsyncRunner {
    pub fn new() -> AsyncRunner {
        AsyncRunner {
            scheduled_runs: Arc::new(Mutex::new(Vec::new())),
            pool: IoTaskPool::get()
        }
    }

    /// Takes a join handle, and runs an ECS system with its value once completed
    pub fn schedule<
        S: IntoSystem<I, (), M> + Send + Sync + 'static,
        M: 'static,
        I: SystemInput<Inner<'static>: Send + Sync> + Send + Sync + 'static
    >(
        &self,
        system: S,
        task: impl Future<Output = I::Inner<'static>> + Sync + Send + 'static
    ) {
        self.pool.spawn((async |runs: Arc<Mutex<Vec<Box<dyn FnOnce(&mut Commands) + Send + Sync>>>>| {
            let result = task.await;

            let boxed_result = Box::new(result);

            let execute = move |commands: &mut Commands| {
                commands.run_system_cached_with(
                    system,
                    *boxed_result
                );
            };

            runs.lock().unwrap().push(
                Box::new(execute)
            );
        })(self.scheduled_runs.clone()))
            .detach();
    }

    /// Loop over all completed join handles and run the systems
    pub fn run(&self, mut commands: Commands) {
        for execute in self.scheduled_runs.lock().unwrap().drain(..) {
            execute(&mut commands);
        }
    }
}