//! System time manager.
//!
//! This module contains a system time manager. It's responsibility is to keep track of tasklets
//! and events that are based on time.

mod cyclic_execution;

use self::cyclic_execution::CyclicExecution;

use crate::aerugo::Aerugo;
use crate::api::InitError;
use crate::internal_list::InternalList;
use crate::tasklet::TaskletPtr;
use crate::time::MillisDurationU32;

/// List of cyclic executions registered in the system.
type CyclicExecutions = InternalList<CyclicExecution, { Aerugo::TASKLET_COUNT }>;

/// System time manager.
///
/// This shouldn't be created by hand by the user or anywhere else in the code.
/// It should be used as a singleton (crate::aerugo::TIME_MANAGER) and shouldn't be directly accessed
/// by any other part of the system.
pub(crate) struct TimeManager {
    /// Registered cyclic executions.
    cyclic_executions: CyclicExecutions,
}

impl TimeManager {
    /// Creates new time manager instance.
    ///
    /// # Safety
    /// This shouldn't be called more than once.
    pub(crate) const fn new() -> Self {
        TimeManager {
            cyclic_executions: CyclicExecutions::new(),
        }
    }

    /// Creates new cyclic execution and registers it in the manager.
    ///
    /// # Parameters
    /// * `tasklet`: Tasklet that will be executed
    /// * `period`: Period for execution, `None` if tasklet shall be executed without waits
    ///
    /// # Return
    /// Reference to the cyclic execution data if successful, `InitError` otherwise.
    ///
    /// # Safety
    /// This is unsafe, because it mutably borrows the list of cyclic executions.
    /// This is safe to call before the system initialization.
    pub(crate) unsafe fn create_cyclic_execution(
        &'static self,
        tasklet: TaskletPtr,
        period: Option<MillisDurationU32>,
    ) -> Result<&'static CyclicExecution, InitError> {
        let cyclic_execution = CyclicExecution::new(tasklet, period);

        match self.cyclic_executions.add(cyclic_execution) {
            Ok(_) => (),
            Err(_) => return Err(InitError::CyclicExecutionListFull),
        };

        Ok(self.cyclic_executions.last().unwrap())
    }

    /// Wakes all cyclic tasklets.
    pub(crate) fn wake_tasklets(&'static self) {
        for ce in &self.cyclic_executions {
            ce.wake_tasklet();
        }
    }
}

unsafe impl Sync for TimeManager {}
