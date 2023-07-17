#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting as semihosting;
extern crate panic_semihosting;
extern crate samv71q21_pac as pac;

use cortex_m::peripheral::{syst, SYST};
use cortex_m::Peripherals as CortexPeripherals;
use pac::WDT;
use rt::{entry, exception};
use semihosting::hprintln;

fn setup_systick(systick: &mut SYST) {
    systick.set_clock_source(syst::SystClkSource::Core);
    systick.set_reload(12_000_000); // SAMV71Q21 runs @ 12MHz by default, this is 1s period
    systick.enable_counter();
    systick.enable_interrupt();
}

fn disable_watchdog(watchdog: &mut WDT) {
    watchdog.mr.modify(|_, w| w.wddis().set_bit());
}

#[entry]
fn main() -> ! {
    hprintln!("Hello, world!");

    let mut cpu_peripherals = unsafe { CortexPeripherals::steal() };
    let mut samv71_peripherals = unsafe { pac::Peripherals::steal() };

    disable_watchdog(&mut samv71_peripherals.WDT);
    setup_systick(&mut cpu_peripherals.SYST);

    loop {}
}

#[exception]
fn SysTick() {
    static mut COUNT: i32 = 0;
    *COUNT += 1;
    hprintln!("{}", COUNT);
}
