#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_rtt_target;

use aerugo::{
    logln, Aerugo, Duration, InitApi, RuntimeApi, SystemHardwareConfig, TaskletConfig,
    TaskletStorage,
};
use rt::entry;

#[derive(Default)]
struct DummyTaskContext {
    acc: u16,
}

fn dummy_task(_: (), context: &mut DummyTaskContext, api: &'static dyn RuntimeApi) {
    context.acc = context.acc.wrapping_add(1);

    if context.acc == 1 {
        let startup_duration = api.get_startup_duration();
        let startup_secs = startup_duration.to_secs();
        let startup_ms = startup_duration.to_millis() % 1000;
        let startup_us = startup_duration.to_micros() % (1000 * 1000);
        logln!(
            "Startup time is {}s, {}ms, {}us",
            startup_secs,
            startup_ms,
            startup_us
        );
    } else {
        let time = api.get_system_time().duration_since_epoch();
        let time_seconds = time.to_secs();
        let time_millis = time.to_millis() % 1000;
        logln!("Current time is {}s {}ms", time_seconds, time_millis);
    }
}

static DUMMY_TASK_STORAGE: TaskletStorage<(), DummyTaskContext, 0> = TaskletStorage::new();

#[entry]
fn main() -> ! {
    let (aerugo, _) = Aerugo::initialize(SystemHardwareConfig::default());

    logln!("Hello, world! Aerugo initialized!");

    logln!("Creating tasks...");
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

    logln!("Subscribing tasks...");

    aerugo.subscribe_tasklet_to_cyclic(&dummy_task_handle, Some(Duration::secs(1)), None);

    logln!("Starting the system!");

    aerugo.start();
}
