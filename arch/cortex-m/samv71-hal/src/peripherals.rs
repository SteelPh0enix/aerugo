//! Access to the hardware peripherals.

use crate::pac::Peripherals as pac_peripherals;
use crate::watchdog::Watchdog;

/// Peripherals structure.
pub struct Peripherals {
    /// Watchdog instance
    pub watchdog: Watchdog,
}

impl Peripherals {
    /// Create new instance of peripherals
    pub const fn new() -> Self {
        unsafe {
            Peripherals {
                watchdog: Watchdog::new(pac_peripherals::steal().WDT),
            }
        }
    }
}
