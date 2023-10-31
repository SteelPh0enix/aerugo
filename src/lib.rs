/*!
# aerugo

`aerugo` is a safety-critical applications oriented Real-Time Operating System.
*/
#![no_std]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(rustdoc::missing_crate_level_docs)]
#![feature(const_mut_refs)]

mod aerugo;
mod api;
mod boolean_condition;
mod cyclic_execution;
mod cyclic_execution_manager;
mod data_provider;
mod error;
mod event;
mod event_manager;
mod execution_monitor;
mod executor;
mod internal_list;
mod message_queue;
mod mutex;
mod stubs;
mod tasklet;
mod time_source;
mod utils;

#[cfg(any(doc, test))]
mod tests;

pub use self::aerugo::Aerugo;
pub use self::api::{InitApi, RuntimeApi};
pub use self::boolean_condition::{
    BooleanConditionHandle, BooleanConditionSet, BooleanConditionSetType, BooleanConditionStorage,
};
pub use self::event::{EventHandle, EventId, EventStorage};
pub use self::message_queue::{MessageQueueHandle, MessageQueueStorage};
pub use self::mutex::Mutex;
pub use self::tasklet::{TaskletConfig, TaskletStorage};

/// Module for re-exporting time structures.
pub mod time {
    pub use aerugo_hal::time::*;
    pub use aerugo_hal::{Duration, Instant};
}
pub use time::*;

pub use aerugo_hal::SystemHardwareConfig;

#[cfg(feature = "use-aerugo-cortex-m")]
#[cfg(feature = "log")]
pub(crate) use aerugo_cortex_m as arch;
#[cfg(feature = "use-aerugo-cortex-m")]
pub use aerugo_samv71_hal as hal;

#[cfg(feature = "use-aerugo-x86")]
#[cfg(feature = "log")]
pub(crate) use aerugo_x86 as arch;
#[cfg(feature = "use-aerugo-x86")]
pub use aerugo_x86_hal as hal;

#[cfg(feature = "log")]
pub use arch::{log, logln};
