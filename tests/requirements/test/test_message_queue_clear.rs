use assert_cmd::Command;
use test_binary::build_test_binary;

/// @SRS{ROS-FUN-RTOS-010}
/// @SRS{ROS-FUN-RTOS-130}
/// @SRS{ROS-FUN-RTOS-2010}
/// @SRS{ROS-FUN-RTOS-2020}
/// @SRS{ROS-FUN-RTOS-2030}
/// @SRS{ROS-FUN-RTOS-2050}
/// @SRS{ROS-FUN-RTOS-2060}
/// @SRS{ROS-FUN-RTOS-2070}
#[cfg_attr(not(doc), test)]
fn req_test_message_queue_clear() {
    let test_bin_path = build_test_binary("test-message-queue-clear", "testbins")
        .expect("error building test binary");

    Command::new(test_bin_path)
        .timeout(std::time::Duration::from_secs(1))
        .assert()
        .success()
        .code(0)
        .stdout(
            r"TaskB: 1
TaskB: 1
TaskB: 1
",
        );
}
