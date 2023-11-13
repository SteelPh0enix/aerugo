"""Helper script for Aerugo example that creates echo server on UART interface."""
import logging
import sys
import time

from calldwell import init_default_logger
from calldwell.ssh_client import SSHClient
from calldwell.uart import RemoteUARTConfig, RemoteUARTConnection
from scripts.env import (
    BOARD_LOGIN,
    BOARD_NETWORK_PATH,
    BOARD_PASSWORD,
    BOARD_UART_DEVICE,
    BOARD_UART_PORT,
)

ECHO_EXAMPLE_BAUDRATE = 57600


def main() -> None:
    """Main function."""
    ssh = SSHClient(BOARD_NETWORK_PATH, BOARD_LOGIN, BOARD_PASSWORD)
    uart_config = RemoteUARTConfig(
        device_path=BOARD_UART_DEVICE,
        port=BOARD_UART_PORT,
        baudrate=ECHO_EXAMPLE_BAUDRATE,
    )
    uart = RemoteUARTConnection(ssh, uart_config)

    if uart.open_uart():
        logging.info("UART opened, enter data to send:")
    else:
        logging.critical("UART connection couldn't be established, quitting...")
        sys.exit(1)

    msg = "Hello, world!"
    i = 0

    while True:
        uart.write_string(f"{msg} # {i}\n")
        print(uart.read_string(b"\n"))
        time.sleep(0.1)
        i += 1


if __name__ == "__main__":
    init_default_logger()
    main()
