//! Implementation of HAL for Watchdog

use embedded_hal::watchdog::WatchdogDisable;

use crate::embedded_hal::watchdog;
use crate::internal_cell::InternalCell;
use crate::pac::WDT;

/// Structure representing a watchdog
pub struct Watchdog {
    /// Watchdog instance
    wdt: InternalCell<WDT>,
    /// Indicates whether the watchdog has been configured (or disabled).
    ///
    /// Note that watchdog can be configured only once, and that includes disabling it.
    /// Configuration is locked until MCU performs hard restart.
    /// This flag prevents it from being configured twice.
    configured: bool,
}

/// Structure representing Watchdog configuration.
///
/// Note that watchdog can be configured only once.
/// Configuration is locked until MCU performs hard restart.
pub struct WatchdogConfiguration {
    /// If true, watchdog stays enabled.
    pub enabled: bool,
    /// If true, watchdog will reset the MCU on overflowing.
    pub reset_enabled: bool,
    /// Defines the
    pub counter: u16,
}

impl Watchdog {
    /// Create a watchdog instance
    pub const fn new(wdt: WDT) -> Self {
        Self {
            wdt: InternalCell::new(wdt),
            configured: false,
        }
    }

    /// Set watchdog configuration
    ///
    /// Note that watchdog can be configured only once.
    /// Configuration is locked until MCU performs hard restart.
    pub fn configure(&mut self, configuration: WatchdogConfiguration) {
        if self.configured {
            return;
        }

        // It's unsafe to modify configuration and enable/disable watchdog at the same time,
        // therefore disabling is handled separately.
        if !configuration.enabled {
            self.disable();
            return;
        }

        let clamped_counter_value = configuration.counter.clamp(0, 2u16.pow(12) - 1);

        // SAFETY: WDV is 12-bit field, value from configuration is clamped to (2^12)-1
        unsafe {
            self.wdt.as_mut_ref().mr.modify(|_, w| {
                w.wdrsten()
                    .bit(configuration.reset_enabled)
                    .wdv()
                    .bits(clamped_counter_value)
            });
        }
    }
}

impl watchdog::Watchdog for Watchdog {
    fn feed(&mut self) {
        unsafe {
            self.wdt
                .as_mut_ref()
                .cr
                .write(|w| w.key().passwd().wdrstt().set_bit());
        }
    }
}

impl watchdog::WatchdogDisable for Watchdog {
    /// Disables the watchdog.
    ///
    /// Note that watchdog can be configured only once, and that includes disabling it.
    /// Once disabled, it's off until the MCU performs a full restart.
    fn disable(&mut self) {
        if self.configured {
            return;
        }

        unsafe {
            self.wdt.as_mut_ref().mr.modify(|_, w| w.wddis().set_bit());
        }
        self.configured = true;
    }
}
