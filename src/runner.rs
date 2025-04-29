use std::future::Future;
use bevy::prelude::{warn, Commands, IntoSystem, Resource, SystemInput};
use bevy::tasks::IoTaskPool;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender, UnboundedReceiver};
use tokio::sync::mpsc::error::TryRecvError;

type ExecuteSystemFn = Box<dyn FnOnce(&mut Commands) + Send + Sync>;

#[derive(Resource)]
pub struct AsyncRunner {
    channel: (
        UnboundedSender<ExecuteSystemFn>,
        UnboundedReceiver<ExecuteSystemFn>
    ),
    #[cfg(not(all(feature = "tokio-runtime", not(target_arch = "wasm32"))))]
    pool: &'static IoTaskPool,
    #[cfg(all(feature = "tokio-runtime", not(target_arch = "wasm32")))]
    pool: tokio::runtime::Runtime,
}

impl AsyncRunner {
    pub fn new() -> AsyncRunner {
        AsyncRunner {
            channel: unbounded_channel(),

            #[cfg(not(all(feature = "tokio-runtime", not(target_arch = "wasm32"))))]
            pool: IoTaskPool::get(),
            #[cfg(all(feature = "tokio-runtime", not(target_arch = "wasm32")))]
            pool: tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap(),
        }
    }

    /// Takes a join handle, and runs an ECS system with its value once completed
    pub fn schedule<
        S: IntoSystem<I, (), M> + Send + Sync + 'static,
        M: 'static,
        I: SystemInput<Inner<'static>: Send + Sync> + Send + Sync + 'static
    >(
        &self,
        task: impl Future<Output = I::Inner<'static>> + Sync + Send + 'static,
        system: S,
    ) {
        let task = self.pool.spawn((async |sender: UnboundedSender<ExecuteSystemFn>| {
            let result = task.await;

            let boxed_result = Box::new(result);

            let execute = move |commands: &mut Commands| {
                commands.run_system_cached_with(
                    system,
                    *boxed_result
                );
            };

            sender.send(Box::new(execute)).unwrap();
        })(self.channel.0.clone()));

        // IoTaskPool won't finish it unless we detach
        #[cfg(not(all(feature = "tokio-runtime", not(target_arch = "wasm32"))))]
        task.detach();
    }

    /// Loop over all completed join handles and run the systems
    pub fn run(&mut self, mut commands: Commands) {
        loop {
            match self.channel.1.try_recv() {
                Ok(execute) => {
                    execute(&mut commands);
                }
                Err(e) => {
                    match e {
                        TryRecvError::Empty => {}
                        TryRecvError::Disconnected => {
                            warn!("AsyncRunner communication channel terminated");
                            break;
                        }
                    }

                    break;
                }
            }
        }
    }
}