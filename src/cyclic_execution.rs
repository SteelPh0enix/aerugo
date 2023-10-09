//! Cyclic execution for tasklets.
//!
//! This module contains a structure which holds information about cyclic execution of tasklets.

use crate::aerugo::Aerugo;
use crate::data_provider::DataProvider;
use crate::mutex::Mutex;
use crate::tasklet::TaskletPtr;
use crate::time::{Duration, Instant};

/// Cyclic execution information.
pub(crate) struct CyclicExecution {
    /// Next execution time.
    next_execution_time: Mutex<Instant>,
    /// Period of cyclic execution.
    period: Option<Duration>,
    /// Tasklet subscribed for cyclic execution.
    tasklet: TaskletPtr,
}

impl CyclicExecution {
    /// Creates new instance.
    ///
    /// # Parameters
    /// * `tasklet` - Tasklet which should be executed cyclically.
    /// * `period` - Period of execution, `None` if should be awaken whenever possible.
    /// * `offset` - Offset of first execution after scheduled start, `None` if should be executed instantly.
    pub(crate) fn new(
        tasklet: TaskletPtr,
        period: Option<Duration>,
        offset: Option<Duration>,
    ) -> Self {
        let next_execution_time = match offset {
            Some(offset) => Instant::from_ticks(offset.ticks()),
            None => Instant::from_ticks(0),
        }
        .into();

        CyclicExecution {
            next_execution_time,
            period,
            tasklet,
        }
    }

    /// Wakes that stored tasklet if the time for it's execution has come.
    ///
    /// # Parameters
    /// * `current_time` - Current system time.
    pub(crate) fn wake_if_should_execute(&self, current_time: Instant) {
        if let Some(period) = self.period {
            if self.next_execution_time.lock(|next| current_time >= *next) {
                Aerugo::wake_tasklet(&self.tasklet);

                // Calculate next execution time, skipping any missed executions
                self.next_execution_time.lock(|next| {
                    while current_time >= *next {
                        *next += period
                    }
                });
            }
        } else {
            Aerugo::wake_tasklet(&self.tasklet);
        }
    }
}

impl DataProvider<()> for CyclicExecution {
    /// Returns `Some()`.
    fn get_data(&self) -> Option<()> {
        Some(())
    }

    /// Returns false, as there is no waiting data for the execution.
    ///
    /// Cyclic execution has a period for execution, and is scheduled by the
    /// [crate::cyclic_execution_manager::CyclicExecutionManager], but doesn't store any
    /// data that is 'waiting' for the scheduling purposes.
    fn data_waiting(&self) -> bool {
        false
    }
}
