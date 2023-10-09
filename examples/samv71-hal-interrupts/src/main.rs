#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_rtt_target;

use core::cell::RefCell;

use aerugo::hal::drivers::nvic::{Interrupt, NVIC};
use aerugo::hal::drivers::pmc::config::PeripheralId;
use aerugo::hal::drivers::timer::{
    channel_config::{ChannelClock, ChannelInterrupts},
    waveform_config::WaveformModeConfig,
    Ch0, Channel, Timer, Waveform, TC1,
};
use aerugo::hal::interrupt;
use aerugo::Mutex;

use aerugo::{
    logln, Aerugo, Duration, InitApi, RuntimeApi, SystemHardwareConfig, TaskletConfig,
    TaskletStorage,
};
use rt::entry;

static TIMER_CHANNEL: Mutex<RefCell<Option<Channel<TC1, Ch0, Waveform>>>> =
    Mutex::new(RefCell::new(None));
static COUNTER: Mutex<RefCell<u32>> = Mutex::new(RefCell::new(0));

#[derive(Default)]
struct DummyTaskContext {}

static DUMMY_TASK_STORAGE: TaskletStorage<(), DummyTaskContext, 0> = TaskletStorage::new();

fn dummy_task(_: (), _: &mut DummyTaskContext, _: &'static dyn RuntimeApi) {
    COUNTER.lock(|value_ref| logln!("Overflow IRQ happened {} times.", value_ref.borrow()));
}

fn init_timer(mut timer: Timer<TC1>) {
    let mut ch0 = timer
        .channel_0
        .take()
        .expect("Channel 0 of Timer 1 already taken")
        .into_waveform_channel(WaveformModeConfig::default());

    ch0.set_clock_source(ChannelClock::MckDividedBy32);
    ch0.enable_interrupts(ChannelInterrupts {
        counter_overflow: true,
        load_overrun: false,
        ra_compare: false,
        rb_compare: false,
        rc_compare: false,
        ra_load: false,
        rb_load: false,
        external_trigger: false,
    });
    ch0.enable();
    ch0.trigger();

    let status = ch0.status().clock_enabled;
    logln!("Clock is {}", if status { "enabled" } else { "disabled" });

    TIMER_CHANNEL.lock(|channel_ref| channel_ref.replace(Some(ch0)));
}

fn init_tasks(aerugo: &'static impl InitApi) {
    let dummy_task_config = TaskletConfig {
        name: "DummyTask",
        ..Default::default()
    };
    let dummy_task_context = DummyTaskContext::default();

    aerugo.create_tasklet_with_context(
        dummy_task_config,
        dummy_task,
        dummy_task_context,
        &DUMMY_TASK_STORAGE,
    );

    let dummy_task_handle = DUMMY_TASK_STORAGE.create_handle().unwrap();

    aerugo.subscribe_tasklet_to_cyclic(&dummy_task_handle, Some(Duration::secs(1)), None);
}

#[entry]
fn main() -> ! {
    let (aerugo, mut peripherals) = Aerugo::initialize(SystemHardwareConfig::default());

    logln!("Hello, world! Aerugo initialized!");

    logln!("Enabling timer interrupts...");
    let mut nvic = NVIC::new(peripherals.nvic.take().expect("NVIC already taken"));
    nvic.enable(Interrupt::TC1CH0);

    logln!("Interrupts enabled, initializing peripherals...");
    let timer = Timer::new(
        peripherals
            .timer_counter1
            .take()
            .expect("Timer 1 already taken"),
    );
    let mut pmc = peripherals.pmc.take().expect("PMC already taken");
    pmc.enable_peripheral_clock(PeripheralId::TC1CH0);
    init_timer(timer);

    logln!("Initializing Aerugo...");
    init_tasks(aerugo);

    logln!("Starting the system!");
    aerugo.start();
}

#[interrupt]
fn TC3() {
    COUNTER.lock(|counter_ref| {
        let counter = counter_ref.get_mut();
        *counter = counter.wrapping_add(1);
    });

    TIMER_CHANNEL.lock(|channel_ref| {
        let tc_ref = channel_ref.get_mut().as_mut().unwrap();
        // To prevent IRQ looping, we need to read IRQ status from TC registers
        tc_ref.status();
    });

    logln!("IRQ!");
}
