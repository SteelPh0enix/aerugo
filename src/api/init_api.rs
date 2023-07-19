//! System initialization API.
//!
//! This API is used for the system initialization, before the scheduler is started.
//!
//! # Safety
//! Functions from this trait shouldn't be called after the system was started.

use crate::boolean_condition::{BooleanConditionSet, BooleanConditionStorage};
use crate::event::{EventHandle, EventStorage};
use crate::hal::Peripherals;
use crate::message_queue::{MessageQueueHandle, MessageQueueStorage};
use crate::tasklet::{StepFn, TaskletHandle, TaskletStorage};

/// System initialization API
pub trait InitApi: ErrorType + TaskConfigType {
    /// Type for a duration of time.
    type Duration;

    /// Creates new tasklet in the system.
    ///
    /// # Generic Arguments
    /// * `T` - Type of the data processed by the tasklet.
    /// * `C` - Type of the structure with tasklet context data.
    ///
    /// # Arguments
    /// * `config` - Tasklet creation configuration.
    /// * `step_fn` - Tasklet step function.
    /// * `storage` - Static memory storage where the tasklet should be allocated.
    ///
    /// # Return
    /// `()` if successful, `Self::Error` othwerise.
    fn create_tasklet<T, C: Default>(
        &'static self,
        config: Self::TaskConfig,
        step_fn: StepFn<T, C>,
        storage: &'static TaskletStorage<T, C>,
    ) -> Result<(), Self::Error>;

    /// Creates new tasklet in the system with initialized context data.
    ///
    /// # Generic Arguments
    /// * `T` - Type of the data processed by the tasklet.
    /// * `C` - Type of the structure with tasklet context data.
    ///
    /// # Arguments
    /// * `config` - Tasklet creation configuration.
    /// * `step_fn` - Tasklet step function.
    /// * `context` - Tasklet context data.
    /// * `storage` - Static memory storage where the tasklet should be allocated.
    ///
    /// # Return
    /// `()` if successful, `Self::Error` othwerise.
    fn create_tasklet_with_context<T, C>(
        &'static self,
        config: Self::TaskConfig,
        step_fn: StepFn<T, C>,
        context: C,
        storage: &'static TaskletStorage<T, C>,
    ) -> Result<(), Self::Error>;

    /// Creates new message queue in the system.
    ///
    /// # Generic Arguments
    /// * `T` - Type of the data stored in the queue.
    /// * `N` - Size of the queue.
    ///
    /// # Arguments
    /// * `storage` - Static memory storage where the queue should be allocated.
    ///
    /// # Return
    /// `()` if successful, `Self::Error` othwerise.
    fn create_message_queue<T, const N: usize>(
        &'static self,
        storage: &'static MessageQueueStorage<T, N>,
    ) -> Result<(), Self::Error>;

    /// Creates new event in the system.
    ///
    /// # Arguments
    /// * `storage` - Static memory storage where the event should be allocated.
    ///
    /// # Return
    /// `()` if successful, `Self::Error` othwerise.
    fn create_event(&'static self, storage: &'static EventStorage) -> Result<(), Self::Error>;

    /// Creates new boolean condition in the system.
    ///
    /// # Arguments
    /// * `storage` - Static memory storage where the condition should be allocated.
    ///
    /// # Return
    /// `()` if successful, `Self::Error` othwerise.
    fn create_boolean_condition(
        &'static self,
        storage: &'static BooleanConditionStorage,
    ) -> Result<(), Self::Error>;

    /// Subscribes tasklet to the queue.
    ///
    /// # Generic Arguments
    /// * `T` - Type of the data.
    /// * `C` - Type of the structure with tasklet context data.
    /// * `N` - Size of the queue.
    ///
    /// # Arguments
    /// * `tasklet` - Handle to the target tasklet.
    /// * `queue` - Handle to the target queue.
    ///
    /// # Return
    /// `()` if successful, `Self::Error` othwerise.
    fn subscribe_tasklet_to_queue<T: Default, C, const N: usize>(
        &'static self,
        tasklet_handle: &TaskletHandle<T, C>,
        queue_handle: &MessageQueueHandle<T, N>,
    ) -> Result<(), Self::Error>;

    /// Subscribes tasklet to the event.
    ///
    /// # Generic Arguments
    /// * `T` - Type of the data.
    /// * `C` - Type of the structure with tasklet context data.
    ///
    /// # Arguments
    /// * `tasklet` - Handle to the target tasklet.
    /// * `event` - Target event ID.
    ///
    /// # Return
    /// `()` if successful, `Self::Error` othwerise.
    fn subscribe_tasklet_to_event<T, C>(
        &'static self,
        tasklet: &TaskletHandle<T, C>,
        event: &EventHandle,
    ) -> Result<(), Self::Error>;

    /// Subscribes tasklet to the set of conditions.
    ///
    /// # Generic Arguments
    /// * `T` - Type of the data.
    /// * `C` - Type of the structure with tasklet context data.
    ///
    /// # Arguments
    /// * `tasklet` - Handle to the target tasklet.
    /// * `condition` - Set of conditions.
    ///
    /// # Return
    /// `()` if successful, `Self::Error` othwerise.
    fn subscribe_tasklet_to_conditions<T, C>(
        &'static self,
        tasklet: &TaskletHandle<T, C>,
        conditions: BooleanConditionSet,
    ) -> Result<(), Self::Error>;

    /// Subscribes tasklet to the cyclic execution.
    ///
    /// # Generic Arguments
    /// * `T` - Type of the data.
    /// * `C` - Type of the structure with tasklet context data.
    ///
    /// # Arguments
    /// * `tasklet` - Handle to the target tasklet.
    /// * `period` - Time period of the execution.
    ///
    /// # Return
    /// `()` if successful, `Self::Error` othwerise.
    fn subscribe_tasklet_to_cyclic<T, C>(
        &'static self,
        tasklet: &TaskletHandle<T, C>,
        period: Self::Duration,
    ) -> Result<(), Self::Error>;

    /// Sets function for hardware initialization
    ///
    /// # Arguments
    /// * `init_fn` - Hardware initialization function.
    fn init_hardware(&'static self, init_fn: fn(&mut Peripherals));
}

/// Initialization error
pub trait Error: core::fmt::Debug {}

/// Initialization error type trait
pub trait ErrorType {
    /// Error type
    type Error: Error;
}

impl<T: ErrorType> ErrorType for &mut T {
    type Error = T::Error;
}

/// Configuration used for creating tasklets
pub trait TaskConfig: Default {}

/// Task configuration type trait
pub trait TaskConfigType {
    /// Task configuration type
    type TaskConfig: TaskConfig;
}
