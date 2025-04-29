use std::future::Future;
use bevy::prelude::{warn, trace, Commands, IntoSystem, Resource, SystemInput};
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
    #[cfg(all(not(feature = "tokio-runtime"), not(feature = "tokio-runtime-multi-thread")))]
    pool: &'static IoTaskPool,
    #[cfg(any(feature = "tokio-runtime-multi-thread", feature = "tokio-runtime"))]
    pool: tokio::runtime::Runtime,
}

impl AsyncRunner {
    pub fn new() -> AsyncRunner {

        #[cfg(all(not(feature = "tokio-runtime"), not(feature = "tokio-runtime-multi-thread")))]
        let pool = {
            trace!("Fetching IoTaskPool runtime via IoTaskPool::get()");
            IoTaskPool::get()
        };
        #[cfg(feature = "tokio-runtime-multi-thread")]
        let pool= {
            trace!("Starting tokio runtime via Builder::new_multi_thread()");
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
        };
        #[cfg(all(not(feature = "tokio-runtime-multi-thread"), feature = "tokio-runtime"))]
        let pool = {
            trace!("Starting tokio runtime via Builder::new_current_thread()");
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
        };

        AsyncRunner {
            channel: unbounded_channel(),
            pool
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
        #[cfg(all(not(feature = "tokio-runtime"), not(feature = "tokio-runtime-multi-thread")))]
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