"""Test utilities used by Aerugo's integration test scripts.
Provides integration test's boilerplate.
"""

from __future__ import annotations

import logging
import sys
from typing import TYPE_CHECKING

from calldwell.rust_helpers import build_cargo_app, init_remote_calldwell_rs_session
from scripts.env import (
    BOARD_CLEAN_ENVIRONMENT_COMMAND,
    BOARD_GDB_PORT,
    BOARD_LOGIN,
    BOARD_NETWORK_PATH,
    BOARD_PASSWORD,
    BOARD_RTT_PORT,
    BOARD_START_GDB_SERVER_COMMAND,
    BOARD_TARGET_TRIPLE,
    HOST_GDB_EXECUTABLE,
    INTEGRATION_TESTS_DIRECTORY,
)

if TYPE_CHECKING:
    from calldwell.gdb_client import GDBClient
    from calldwell.rtt_client import CalldwellRTTClient
    from calldwell.ssh_client import SSHClient
    from calldwell.uart import RemoteUARTConnection


def init_test(
    test_name: str,
    debug_build: bool = False,
) -> tuple[GDBClient, CalldwellRTTClient, SSHClient]:
    """Creates SSH connection to target board, initializes Calldwell."""
    project_path = INTEGRATION_TESTS_DIRECTORY / test_name
    if (
        test_binary_path := build_cargo_app(
            project_path,
            BOARD_TARGET_TRIPLE,
            release_build=not debug_build,
        )
    ) is None:
        sys.exit(100)

    logging.info("Starting the test, initializing the environment...")
    session = init_remote_calldwell_rs_session(
        debug_host_network_path=BOARD_NETWORK_PATH,
        debug_host_login=BOARD_LOGIN,
        debug_host_password=BOARD_PASSWORD,
        gdb_server_port=BOARD_GDB_PORT,
        rtt_server_port=BOARD_RTT_PORT,
        local_gdb_executable=HOST_GDB_EXECUTABLE,
        remote_gdb_server_command=BOARD_START_GDB_SERVER_COMMAND,
        path_to_test_executable=str(test_binary_path.absolute()),
    )

    if session is None:
        logging.critical("Test failed, cannot initialize Calldwell session")
        sys.exit(1)

    ssh, gdb, rtt = session

    logging.info("Environment initialized!")
    return gdb, rtt, ssh


def finish_test(ssh: SSHClient) -> None:
    """Finishes integration test's execution by cleaning up resources."""
    logging.info("Finishing the test, cleaning up environment...")
    ssh.execute(BOARD_CLEAN_ENVIRONMENT_COMMAND)
    ssh.close()
    logging.info("Environment cleaned up!")


def wait_for_messages(
    rtt: CalldwellRTTClient,
    ssh: SSHClient,
    expected_messages: list[str],
) -> None:
    """Waits until list of specified messages is received, prematurely finishes
    the test with non-zero exit code if an invalid message is received, indicating
    test failure.
    """
    for message in expected_messages:
        logging.info(f"Expecting '{message}'")
        received_message = rtt.receive_string_stream()
        logging.info(f"Received '{received_message}'")
        if received_message != message:
            logging.critical(
                "TEST FAILED: UNEXPECTED MESSAGE RECEIVED "
                f"(expected '{message}', got '{received_message}')",
            )
            finish_test(ssh)
            sys.exit(2)


def wait_for_uart_messages(
    uart: RemoteUARTConnection,
    ssh: SSHClient,
    expected_messages: list[str],
) -> None:
    """Same as `wait_for_messages`, but via UART.
    Assumes that every message ends with single newline (`\\n`) character."""
    for message in expected_messages:
        logging.info(f"Expecting '{message}' via UART")
        received_message = uart.read_string(b"\n")
        logging.info(f"Received '{received_message}'")
        if (
            received_message.is_ok
            and (received_message_content := received_message.unwrap().strip()) != message
        ):
            logging.critical(
                "TEST FAILED: UNEXPECTED MESSAGE RECEIVED VIA UART "
                f"(expected '{message}', got '{received_message_content}')",
            )
            uart.close_uart()
            finish_test(ssh)
            sys.exit(2)
