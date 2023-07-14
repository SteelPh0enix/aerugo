/*!
# aerugo

`aerugo` is a safety-critical applications oriented Real-Time Operating System.
*/
#![no_std]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(rustdoc::missing_crate_level_docs)]

mod aerugo;
mod api;
mod boolean_condition;
mod data_provider;
mod data_receiver;
mod event;
mod execution_monitoring;
mod executor;
mod internal_cell;
mod message_queue;
mod queue;
mod task;
mod tasklet;

pub use self::aerugo::{Aerugo, AERUGO};
pub use self::api::InitApi;
pub use self::boolean_condition::{BooleanConditionSet, BooleanConditionStorage};
pub use self::event::EventStorage;
pub use self::message_queue::{MessageQueueHandle, MessageQueueStorage};
pub use self::tasklet::{TaskletConfig, TaskletStorage};

pub use fugit as time;

#[cfg(feature = "use-aerugo-cortex-m")]
pub(crate) use aerugo_cortex_m as arch;
#[cfg(feature = "use-aerugo-cortex-m")]
pub(crate) use samv71_hal as hal;

#[cfg(feature = "use-aerugo-x86")]
pub(crate) use aerugo_x86 as arch;
#[cfg(feature = "use-aerugo-x86")]
pub(crate) use x86_hal as hal;

pub use arch::log;
