/*!
SAMV71 implementation of aerugo HAL.
*/
#![no_std]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(rustdoc::missing_crate_level_docs)]

pub(crate) use embedded_hal;
pub(crate) use fugit as time;
pub(crate) use samv71q21_pac as pac;

pub mod hal;
pub mod peripherals;
pub mod watchdog;

pub use self::hal::Hal;
pub use self::peripherals::Peripherals;

mod internal_cell;
