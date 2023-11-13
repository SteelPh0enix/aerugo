#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_rtt_target;

use aerugo::{
    hal::{
        drivers::{
            nvic::{Interrupt, NVIC},
            pio::{pin::Peripheral as PioPeripheral, Port},
            pmc::config::PeripheralId,
            uart::{Bidirectional, Config, NotConfigured, ReceiverConfig, Uart},
            xdmac::{
                channel::{Channel, Configured},
                channel_status::ChannelStatusReader,
                events::ChannelEvents,
                status::StatusReader,
                transfer::{
                    AddressingMode, DataWidth, MicroblockLength, Peripheral as XdmacPeripheral,
                    SystemBus, TransferBlock, TransferLocation, TransferType, TriggerSource,
                },
                Xdmac,
            },
        },
        interrupt,
        user_peripherals::{PIOD, PMC, UART4},
    },
    time::RateExtU32,
    Aerugo, BooleanConditionStorage, InitApi, RuntimeApi, SystemHardwareConfig, TaskletConfig,
    TaskletStorage,
};
use rt::entry;

struct TaskUartReaderContext {}

fn task_uart_reader(_: bool, _: &mut TaskUartReaderContext, _: &'static dyn RuntimeApi) {}

const TRANSFER_LENGTH: usize = 7;
static mut MESSAGE_BUFFER: [u8; TRANSFER_LENGTH] = [0; TRANSFER_LENGTH];

/// This is used for passing XDMAC's status reader to IRQ.
/// It must be initialized before starting an IRQ-synchronized XDMAC transaction, otherwise the
/// program may panic.
/// This can be safely accessed outside of XDMAC IRQ only when no XDMAC transactions are in progress.
static mut XDMAC_STATUS_READER: Option<StatusReader> = None;
/// This is used for passing XDMAC's channel status reader to IRQ.
/// It must be initialized before starting an IRQ-synchronized XDMAC transaction, otherwise the
/// program may panic.
/// This can be safely accessed outside of XDMAC IRQ only when no XDMAC transactions are in
/// progress.
static mut XDMAC_CHANNEL_STATUS_READER: Option<ChannelStatusReader> = None;
/// This is used for passing XDMAC's channel to IRQ.
/// It must be initialized before starting an IRQ-synchronized XDMAC transaction, otherwise the
/// program may panic.
/// This can be safely accessed outside of XDMAC IRQ only when no XDMAC transactions are in
/// progress.
static mut XDMAC_RX_CHANNEL: Option<Channel<Configured>> = None;

static TASK_UART_READER_STORAGE: TaskletStorage<bool, TaskUartReaderContext, 0> =
    TaskletStorage::new();
static CONDITION_COMMAND_READY_STORAGE: BooleanConditionStorage = BooleanConditionStorage::new();

#[entry]
fn main() -> ! {
    let (aerugo, mut peripherals) = Aerugo::initialize(SystemHardwareConfig::default());

    let pmc = peripherals.pmc.unwrap();
    init_clocks(pmc);

    let port = Port::new(peripherals.pio_d.take().unwrap());
    init_pio(port);

    let uart = Uart::new(peripherals.uart_4.take().unwrap());
    let mut uart = init_uart(uart);

    let xdmac = Xdmac::new(peripherals.xdmac.take().unwrap());
    init_xdmac(xdmac, &mut uart);

    aerugo.create_boolean_condition(false, &CONDITION_COMMAND_READY_STORAGE);
    let condition_command_ready_handle = CONDITION_COMMAND_READY_STORAGE.create_handle().unwrap();

    let task_uart_reader_config = TaskletConfig {
        name: "UartReader",
        ..Default::default()
    };
    let task_uart_reader_context = TaskUartReaderContext {};

    aerugo.create_tasklet_with_context(
        task_uart_reader_config,
        task_uart_reader,
        task_uart_reader_context,
        &TASK_UART_READER_STORAGE,
    );
    let task_uart_reader_handle = TASK_UART_READER_STORAGE.create_handle().unwrap();

    aerugo
        .subscribe_tasklet_to_condition(&task_uart_reader_handle, &condition_command_ready_handle);

    let mut nvic = NVIC::new(peripherals.nvic.take().unwrap());
    nvic.enable(Interrupt::XDMAC);

    unsafe {
        XDMAC_RX_CHANNEL.as_mut().unwrap().enable();
    }

    aerugo.start();
}

fn init_clocks(mut pmc: PMC) {
    pmc.enable_peripheral_clock(PeripheralId::PIOD);
    pmc.enable_peripheral_clock(PeripheralId::UART4);
    pmc.enable_peripheral_clock(PeripheralId::XDMAC);
}

fn init_pio(port: Port<PIOD>) {
    let mut pins = port.into_pins();
    pins[18]
        .take()
        .unwrap()
        .into_peripheral_pin(PioPeripheral::C);
    pins[19]
        .take()
        .unwrap()
        .into_peripheral_pin(PioPeripheral::D);
}

fn init_uart(uart: Uart<UART4, NotConfigured>) -> Uart<UART4, Bidirectional> {
    let uart_config = Config::new(9600, 12.MHz()).unwrap();
    let recv_config = ReceiverConfig {
        rx_filter_enabled: true,
    };

    uart.into_bidirectional(uart_config, recv_config)
}

fn init_xdmac(mut xdmac: Xdmac, uart: &mut Uart<UART4, Bidirectional>) {
    // Place XDMAC status reader in IRQ storage.
    // This is safe, because XDMAC IRQ should be disabled.
    unsafe { XDMAC_STATUS_READER.replace(xdmac.take_status_reader().unwrap()) };

    let rx_source_location = TransferLocation {
        address: uart.xdmac_rx_address(),
        interface: SystemBus::Interface1,
        addressing_mode: AddressingMode::Fixed,
    };

    let rx_destination_location = TransferLocation {
        address: unsafe { MESSAGE_BUFFER.as_mut_ptr() as *const () },
        interface: SystemBus::Interface1,
        addressing_mode: AddressingMode::Incremented,
    };

    let transfer_microblock_length = MicroblockLength::new(TRANSFER_LENGTH as u32).unwrap();

    let rx_transfer = TransferBlock::new(
        rx_source_location,
        rx_destination_location,
        TransferType::PeripheralToMemory(XdmacPeripheral::UART4_RX, TriggerSource::Hardware),
        DataWidth::Byte,
    )
    .unwrap()
    .with_microblock_length(transfer_microblock_length);

    let mut rx_channel = xdmac.take_next_free_channel().unwrap();

    // RX channel will be queried using an interrupt. End of RX implies end of TX, as both transfers
    // have the same data length.
    rx_channel.set_events_state(ChannelEvents {
        end_of_block: true,
        end_of_list: true,
        end_of_disable: false,
        end_of_flush: true,
        read_bus_error: true,
        write_bus_error: true,
        request_overflow_error: true,
    });
    rx_channel.enable_interrupt();

    let rx_channel = rx_channel.configure_transfer(rx_transfer);

    unsafe { XDMAC_CHANNEL_STATUS_READER.replace(rx_channel.take_status_reader().unwrap()) };

    unsafe {
        XDMAC_RX_CHANNEL.replace(rx_channel);
    }
}

#[interrupt]
fn XDMAC() {
    let rx_channel = unsafe { XDMAC_RX_CHANNEL.as_mut().unwrap() };

    let channel_status_reader = unsafe { XDMAC_CHANNEL_STATUS_READER.as_mut().unwrap() };

    let status = unsafe { XDMAC_STATUS_READER.as_mut().unwrap().get_pending_channels() };

    if status[channel_status_reader.id()] {
        let events = channel_status_reader.get_pending_events();

        if events.read_bus_error {
            panic!("XDMAC read bus error detected");
        }
        if events.write_bus_error {
            panic!("XDMAC write bus error detected");
        }
        if events.request_overflow_error {
            panic!("XDMAC request overflow error detected");
        }
    }

    rx_channel.repeat_transfer();
    rx_channel.enable();
}
