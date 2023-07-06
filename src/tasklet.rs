//! Unit of computation in the system.
//!
//! Tasklet is a fine-grained units of computation, that execute a processing step in a finite
//! amount of time.
//!
//! Tasklet should be thought of as a small building block, which processes a given type of data,
//! one element at the time.
mod tasklet_config;
mod tasklet_handle;
mod tasklet_ptr;
mod tasklet_storage;
mod tasklet_vtable;

pub(crate) use self::tasklet_ptr::TaskletPtr;
pub(crate) use self::tasklet_vtable::{tasklet_vtable, TaskletVTable};

pub use self::tasklet_config::TaskletConfig;
pub use self::tasklet_handle::TaskletHandle;
pub use self::tasklet_storage::TaskletStorage;

use crate::aerugo::error::InitError;
use crate::arch::Mutex;
use crate::data_provider::DataProvider;
use crate::data_receiver::DataReceiver;
use crate::internal_cell::InternalCell;
use crate::task::{Task, TaskStatus};
use crate::time::TimerInstantU64;

/// Type of function that is executed by the tasklet in its step.
pub(crate) type StepFn<T, C> = fn(T, &mut C);

/// Tasklet structure.
///
/// * `T` - Type that is processed by the tasklet.
/// * `C` - Type of tasklet context data.
pub(crate) struct Tasklet<T: 'static, C> {
    /// Tasklet name.
    name: &'static str,
    /// Tasklet status.
    status: Mutex<TaskStatus>,
    /// Last execution time.
    last_execution_time: Mutex<TimerInstantU64<1_000_000>>,
    /// Step function.
    step_fn: StepFn<T, C>,
    /// Context data.
    context: InternalCell<C>,
    /// Source of the data.
    data_provider: InternalCell<Option<&'static dyn DataProvider<T>>>,
}

impl<T, C> Tasklet<T, C> {
    /// Creates new `Tasklet`.
    pub(crate) fn new(config: TaskletConfig, step_fn: StepFn<T, C>, context: C) -> Self {
        Tasklet {
            name: config.name,
            status: Mutex::new(TaskStatus::Sleeping),
            last_execution_time: Mutex::new(TimerInstantU64::<1_000_000>::from_ticks(0)),
            step_fn,
            context: context.into(),
            data_provider: InternalCell::new(None),
        }
    }
}

impl<T: Default, C> Task for Tasklet<T, C> {
    fn get_name(&self) -> &'static str {
        self.name
    }

    fn get_status(&self) -> TaskStatus {
        self.status.lock(|s| *s)
    }

    fn set_status(&self, status: TaskStatus) {
        self.status.lock(|s| *s = status)
    }

    fn get_last_execution_time(&self) -> TimerInstantU64<1_000_000> {
        self.last_execution_time.lock(|t| *t)
    }

    fn set_last_execution_time(&self, time: TimerInstantU64<1_000_000>) {
        self.last_execution_time.lock(|t| *t = time)
    }

    fn has_work(&self) -> bool {
        // SAFETY: This is safe, because this field is only mutably referenced during
        // initialization.
        match unsafe { self.data_provider.as_ref() } {
            Some(dp) => dp.data_ready(),
            None => false,
        }
    }

    fn execute(&self) {
        let value = match unsafe { self.data_provider.as_ref() } {
            Some(dp) => dp.get_data(),
            None => return,
        };

        match value {
            Some(val) => {
                // SAFETY: This is safe, because this field is only accessed here, and given tasklet can
                // be executed only once at a given time.
                let context = unsafe { self.context.as_mut_ref() };
                (self.step_fn)(val, context)
            }
            None => (),
        }
    }
}

impl<T, C> DataReceiver<T> for Tasklet<T, C> {
    fn subscribe(&self, data_provider: &'static dyn DataProvider<T>) -> Result<(), InitError> {
        // SAFETY: This is safe, because this function can be only called at the system
        // initialization, and no other reference can be created.
        unsafe { *self.data_provider.as_mut_ref() = Some(data_provider) };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_default() {
        let tasklet = Tasklet::<u8, ()>::new(TaskletConfig::default(), |_, _| {}, ());

        assert_eq!(tasklet.get_name(), "MISSING_TASKLET_NAME");
        assert_eq!(tasklet.get_status(), TaskStatus::Sleeping);
        assert_eq!(tasklet.get_last_execution_time().ticks(), 0);
    }

    #[test]
    fn create_from_config() {
        let name = "TaskName";

        let config = TaskletConfig { name };
        let tasklet = Tasklet::<u8, ()>::new(config, |_, _| {}, ());

        assert_eq!(tasklet.get_name(), name);
        assert_eq!(tasklet.get_status(), TaskStatus::Sleeping);
        assert_eq!(tasklet.get_last_execution_time().ticks(), 0);
    }

    #[test]
    fn get_set_status() {
        let tasklet = Tasklet::<u8, ()>::new(TaskletConfig::default(), |_, _| {}, ());

        assert_eq!(tasklet.get_status(), TaskStatus::Sleeping);
        tasklet.set_status(TaskStatus::Waiting);
        assert_eq!(tasklet.get_status(), TaskStatus::Waiting);
    }

    #[test]
    fn get_set_last_execution_time() {
        let tasklet = Tasklet::<u8, ()>::new(TaskletConfig::default(), |_, _| {}, ());

        assert_eq!(tasklet.get_last_execution_time().ticks(), 0);
        tasklet.set_last_execution_time(TimerInstantU64::<1_000_000>::from_ticks(42));
        assert_eq!(tasklet.get_last_execution_time().ticks(), 42);
    }
}
