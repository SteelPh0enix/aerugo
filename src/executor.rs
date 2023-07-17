//! System scheduler.

use heapless::binary_heap::{BinaryHeap, Max};

use crate::aerugo::{error::RuntimeError, Aerugo, AERUGO};
use crate::api::RuntimeApi;
use crate::arch::Mutex;
use crate::task::TaskStatus;
use crate::tasklet::TaskletPtr;

/// Type for the tasklet execution queue
type TaskletQueue<const N: usize> = BinaryHeap<TaskletPtr, Max, N>;

/// System scheduler.
pub(crate) struct Executor {
    /// Tasklet queue.
    tasklet_queue: Mutex<TaskletQueue<{ Aerugo::TASKLET_COUNT }>>,
}

impl Executor {
    /// Creates new executor instance.
    pub(crate) const fn new() -> Self {
        Executor {
            tasklet_queue: Mutex::new(BinaryHeap::new()),
        }
    }

    /// Starts tasklet scheduling.
    pub(crate) fn run_scheduler(&'static self) -> ! {
        loop {
            self.execute_next_tasklet()
                .expect("Failure in tasklet execution");
        }
    }

    /// Schedules given task for execution.
    ///
    /// If given task is not already waiting for execution it is put to the execution queue.
    ///
    /// * `tasklet` - Tasklet to schedule.
    ///
    /// Returns `RuntimeError` in case of an error, `Ok(bool)` otherwise indicating if tasklet was
    /// scheduled.
    #[allow(dead_code)]
    pub(crate) fn schedule_tasklet(
        &'static self,
        tasklet: &TaskletPtr,
    ) -> Result<bool, RuntimeError> {
        let tasklet_status = tasklet.get_status();

        if tasklet_status == TaskStatus::Sleeping {
            self.add_tasklet_to_queue(tasklet.clone())?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Executes the next tasklet from the queue.
    ///
    /// Returns `RuntimeError` in case of an error, `Ok(bool)` otherwise indicating if tasklet was
    /// executed.
    fn execute_next_tasklet(&'static self) -> Result<bool, RuntimeError> {
        if let Some(tasklet) = self.get_tasklet_for_execution() {
            tasklet.set_status(TaskStatus::Working);

            tasklet.execute();
            tasklet.set_last_execution_time(AERUGO.get_system_time());

            if !self.reschedule_tasklet(&tasklet)? {
                tasklet.set_status(TaskStatus::Sleeping);
            }

            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Adds given tasklet to the execution queue.
    ///
    /// This marks tasklet as waiting.
    ///
    /// Returns `RuntimeError` in case of an error, `Ok(())` otherwise.
    fn add_tasklet_to_queue(&'static self, tasklet: TaskletPtr) -> Result<(), RuntimeError> {
        self.tasklet_queue.lock(|q| {
            tasklet.set_status(TaskStatus::Waiting);

            match q.push(tasklet) {
                Ok(_) => Ok(()),
                Err(_) => Err(RuntimeError::ExecutorTaskletQueueFull),
            }
        })
    }

    /// Schedules tasklet if there is more work to do.
    ///
    /// * `tasklet` - Tasklet to reschedule
    ///
    /// Returns `RuntimeError` in case of an error, `Ok(bool)` otherwise indicating if tasklet was
    /// rescheduled.
    fn reschedule_tasklet(&'static self, tasklet: &TaskletPtr) -> Result<bool, RuntimeError> {
        if tasklet.has_work() {
            self.add_tasklet_to_queue(tasklet.clone())?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Returns next tasklet that is due for execution, or `None` if the execution queue is empty.
    ///
    /// This marks returned tasklet as working.
    fn get_tasklet_for_execution(&'static self) -> Option<TaskletPtr> {
        self.tasklet_queue.lock(|q| q.pop())
    }
}
