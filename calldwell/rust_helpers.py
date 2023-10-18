"""Module containing some boilerplate, common to all tests that use `calldwell-rs` library."""

from __future__ import annotations

import logging
import shutil
import subprocess
from typing import TYPE_CHECKING, Any

from pygdbmi.constants import GdbTimeoutError

from .gdb_client import GDBClient
from .rtt_client import CalldwellRTTClient
from .ssh_client import SSHClient

if TYPE_CHECKING:
    from collections.abc import Callable
    from pathlib import Path

RTT_SECTION_SYMBOL_NAME = "_SEGGER_RTT"
"""Section name of RTT symbol. Hard-coded in `rtt_target` library."""
RTT_SECTION_SEARCHED_MEMORY_LENGTH = 0x800
"""This constant defines the amount of bytes OpenOCD will search, looking for RTT section.
Current value might as well be an overkill, but it works."""
RTT_SECTION_ID = "SEGGER RTT"
"""RTT section ID"""
CALLDWELL_INIT_FUNCTION_NAME = "calldwell::initialize"
"""Name of Calldwell-rs initialization function. Note that this is for `debug` binary,
as mangled `release` names are not supported (yet)."""

EXPECTED_MCU_INIT_MESSAGE = "calldwell-rs started"
"""Message that MCU should send when it finishes RTT initialization"""
HOST_HANDSHAKE_MESSAGE = "host handshake requested"
"""Response to MCU init message, request for handshake"""
EXPECTED_MCU_HANDSHAKE_MESSAGE = f"{len(HOST_HANDSHAKE_MESSAGE)}:{HOST_HANDSHAKE_MESSAGE}"
"""Expected MCU response to handshake request"""


# Yes, this is a very big function, but it's supposed to be all-in-one single-liner.
# pylint: disable=too-many-arguments,too-many-locals,too-complex
def init_remote_calldwell_rs_session(  # noqa: PLR0913,C901
    debug_host_network_path: str,
    debug_host_login: str,
    debug_host_password: str,
    gdb_server_port: int,
    rtt_server_port: int,
    local_gdb_executable: str,
    remote_gdb_server_command: str,
    path_to_test_executable: str,
    gdb_timeout: float | None = None,
    flashing_timeout: float | None = None,
    max_upload_tries: int = 5,
    log_responses: bool = False,
    log_execution: bool = False,
    pre_handshake_hook: Callable[[GDBClient, Any | None], None] | None = None,
    pre_handshake_hook_argument: Any | None = None,  # noqa: ANN401 (this argument is for the user)
) -> tuple[SSHClient, GDBClient, CalldwellRTTClient] | None:
    """Initializes Calldwell-rs test session by connecting to debug host via SSH, running GDB server
    (like OpenOCD), starting RTT server, flashing the executable, waiting until
    `calldwell::initialize` executes, and performing handshake (and optional pre-handshake hook, if
    provided).

    This function returns a tuple containing `SSHClient` connected to debug host, `GDBClient`
    connected to remote GDB server controlling running application, and `RTTClient` connected
    to running application, or `None` if starting the session fails at any point.

    Microcontroller is in running state after this function finishes execution. You must block the
    program manually (for example by putting a loop waiting for RTT message) in order to prevent
    it from continuing execution after establishing the session.

    This function can also throw one of the `pygdbmi` exceptions, like `GdbTimeoutError`.

    To check what caused the issue, you should check the logs. Writing proper error handling
    for all the scenarios is certainly possible, but usually the error is unrecoverable and
    should fail the test anyway, so there's no point in proper error handling anyway.

    This operation often fails, and therefore is a source of most false-positives
    in tests. It's caused by issues with GDB server connection.
    There's no other known fix for that, other than restarting the connection.

    Optional pre-handshake hook will be executed while program is stopped.
    Pre-handshake hook is given `GDBClient` instance as first argument, and user-provided
    `pre_handshake_hook_argument` as second.

    # Parameters
    * `debug_host_network_path` - Network path to debug host with debugger attached to target board.
                                  If ran locally, use `localhost`
    * `debug_host_login` - SSH login of debug host.
    * `debug_host_password` - SSH password of debug host.
    * `gdb_server_port` - Network port of GDB server
    * `rtt_server_port` - Port for RTT communication that will be opened by GDB
    * `local_gdb_executable` - Path to executable invoking local GDB client that will connect to
                               remote server.
    * `remote_gdb_server_command` - Command starting GDB server on remote debug host.
    * `path_to_test_executable` - Path to Calldwell test executable
    * `gdb_timeout` - Timeout for GDBClient, if `None` then default one will be used.
    * `flashing_timeout` - Timeout of binary flashing, if `None` then default one will be used.
    * `max_upload_tries` - Amount of tries this function will try to upload the binary to MCU memory
    * `log_responses` - Whether to log GDB/MI responses, or not
    * `log_execution` - Whether to log the execution of GDB commands, or not
    * `pre_handshake_hook` - Function that will be called before performing Calldwell handshake.
                             It can be used to perform some GDB operations before the program
                             starts normal execution.
    * `pre_handshake_hook_argument` - User argument passed to `pre_handshake_hook`, if present.
    """

    def try_initializing_session() -> tuple[GDBClient, SSHClient] | None:
        upload_try = 0
        remote_gdb_full_hostname = f"{debug_host_network_path}:{gdb_server_port}"

        while upload_try < max_upload_tries:
            logging.info(
                "Establishing session and uploading the binary, try "
                f"{upload_try + 1}/{max_upload_tries}",
            )

            ssh = SSHClient(debug_host_network_path, debug_host_login, debug_host_password)
            ssh.execute(remote_gdb_server_command)
            gdb = GDBClient(
                gdb_executable=local_gdb_executable,
                default_timeout=gdb_timeout,
                flashing_timeout=flashing_timeout,
                log_responses=log_responses,
                log_execution=log_execution,
            )

            if not gdb.connect_to_remote(remote_gdb_full_hostname):
                logging.error(
                    f"Could not connect to remote GDB server @ {remote_gdb_full_hostname}",
                )
                return None

            session_established = False

            try:
                if gdb.load_executable(path_to_test_executable):
                    session_established = True
                else:
                    logging.warning(
                        f"Loading executable {path_to_test_executable} failed. Restarting session",
                    )
            except GdbTimeoutError:
                logging.warning("Received GdbTimeoutError, restarting session")

            if session_established:
                logging.info(f"Session established on try # {upload_try + 1}!")
                return gdb, ssh

            upload_try += 1
            ssh.close()
            logging.info("Session closed.")

        return None

    if (session := try_initializing_session()) is None:
        logging.critical(f"Could not upload the binary {path_to_test_executable} into MCU memory")
        return None

    gdb, ssh = session

    if (rtt_symbol := gdb.get_variable(RTT_SECTION_SYMBOL_NAME)) is None:
        logging.error(f"Could not find symbol for RTT section {RTT_SECTION_SYMBOL_NAME}")
        return None

    if not gdb.start_program():
        logging.error("Could not start execution of test program")
        return None

    rtt = _initialize_rtt(gdb, debug_host_network_path, rtt_server_port, rtt_symbol.address)
    if rtt is None:
        logging.error("Couldn't initialize RTT facilities")
        return None

    if pre_handshake_hook is not None:
        pre_handshake_hook(gdb, pre_handshake_hook_argument)

    gdb.continue_program()

    if not perform_calldwell_rs_handshake(rtt):
        logging.error("Couldn't perform correct handshake with MCU")
        return None

    return ssh, gdb, rtt


def build_cargo_app(
    project_path: Path,
    target_triple: str,
    release_build: bool = False,
) -> Path | None:
    """Builds Cargo binary and returns path to it's executable, or None if Cargo is not installed.
    Throws an exception on build failure.

    Parameters:
    * `project_path` - Path to the project
    * `target_triple` - Target architecture triple, for example `thumbv7em-none-eabihf`
    * `release_build` - If `True`, a release build will be produced. If `False`, debug build
                        will be produced instead.
    """

    if (cargo := shutil.which("cargo")) is None:
        logging.error("Error: Cargo executable not found!")
        return None

    build_command = [cargo, "build"]
    if release_build:
        build_command.append("--release")

    subprocess.run(
        build_command,  # noqa: S603 (cargo existence validated, and path took via which)
        cwd=project_path,
        text=True,
        check=True,
    )

    build_type = "release" if release_build else "debug"
    exec_name = project_path.name
    return project_path / "target" / target_triple / build_type / exec_name


def perform_calldwell_rs_handshake(rtt: CalldwellRTTClient) -> bool:
    """Performs Calldwell handshake after it's RTT facilities are started.
    This acts like a mini self-test of RTT communication, to guarantee that it works correctly.
    """
    logging.info("Performing Calldwell handshake")

    if (init_message := rtt.receive_string_stream()) != EXPECTED_MCU_INIT_MESSAGE:
        logging.error(
            "Received unexpected MCU init message "
            f"(got '{init_message}', expected '{EXPECTED_MCU_INIT_MESSAGE}')",
        )
        return False

    rtt.transmit_string_stream(HOST_HANDSHAKE_MESSAGE)

    if (response := rtt.receive_string_stream()) != EXPECTED_MCU_HANDSHAKE_MESSAGE:
        logging.error(
            "MCU responded with invalid handshake message "
            f"(got '{response}', expected '{EXPECTED_MCU_HANDSHAKE_MESSAGE}')",
        )
        return False

    return True


def _initialize_rtt(
    gdb: GDBClient,
    gdb_server_hostname: str,
    rtt_server_port: int,
    rtt_address: int,
) -> CalldwellRTTClient | None:
    """Performs RTT initialization after program start, and creates Calldwell's RTT client."""
    if not gdb.set_breakpoint(CALLDWELL_INIT_FUNCTION_NAME):
        logging.error(f"Could not set breakpoint @ {CALLDWELL_INIT_FUNCTION_NAME}")
        return None

    gdb.continue_program()

    if not gdb.wait_for_breakpoint_hit():
        logging.error("Program has stopped, but not because of a breakpoint")
        return None

    gdb.finish_function_execution()

    if not gdb.start_rtt_server(rtt_server_port, 0):
        logging.error(f"Could not start RTT server @ TCP port {rtt_server_port}")
        return None

    if not gdb.setup_rtt(rtt_address, RTT_SECTION_SEARCHED_MEMORY_LENGTH, RTT_SECTION_ID):
        logging.error(
            f"Could not setup RTT for section @ {rtt_address} "
            f"(searched {RTT_SECTION_SEARCHED_MEMORY_LENGTH} bytes)",
        )
        return None

    if not gdb.start_rtt():
        logging.error("Could not start RTT (probably because the section wasn't found)")
        return None

    return CalldwellRTTClient(gdb_server_hostname, rtt_server_port)
