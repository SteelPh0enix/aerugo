//! Message queue used for exchanging data between tasklets.

mod message_queue_handle;
mod message_queue_storage;

pub use self::message_queue_handle::MessageQueueHandle;
pub use self::message_queue_storage::MessageQueueStorage;

pub(crate) use self::message_queue_storage::QueueData;

use crate::aerugo::Aerugo;
use crate::data_provider::DataProvider;
use crate::error::{RuntimeError, SystemError};
use crate::internal_list::InternalList;
use crate::mutex::Mutex;
use crate::tasklet::TaskletPtr;

/// List of tasklets registered to a queue
type TaskletList = InternalList<TaskletPtr, { Aerugo::TASKLET_COUNT }>;

/// Message queue used for exchanging data between tasklets.
///
/// # Generic Parameters
/// * `T` - Type of the stored data.
/// * `N` - Size of the queue.
#[repr(C)]
pub(crate) struct MessageQueue<T: 'static, const N: usize> {
    /// Reference to the queue data storage.
    data_queue: &'static Mutex<QueueData<T, N>>,
    /// Tasklets registered to this queue.
    registered_tasklets: TaskletList,
}

impl<T, const N: usize> MessageQueue<T, N> {
    /// Creates new `MessageQueue`.
    pub(crate) fn new(data_queue: &'static Mutex<QueueData<T, N>>) -> Self {
        MessageQueue {
            data_queue,
            registered_tasklets: TaskletList::new(),
        }
    }

    /// Registers task to this queue.
    ///
    /// # Parameters
    /// * `task` - Task to register.
    ///
    /// # Returns
    /// `()` if successful, `SystemError` otherwise.
    ///
    /// # Safety
    /// This is unsafe, because it mutably borrows the list of registered tasklets.
    /// This is safe to call during system initialization (before scheduler is started).
    /// Accessing queue from IRQ context during registering is undefined behaviour.
    pub(crate) unsafe fn register_tasklet(&self, tasklet: TaskletPtr) -> Result<(), SystemError> {
        match self.registered_tasklets.add(tasklet) {
            Ok(_) => Ok(()),
            Err(_) => Err(SystemError::TaskletListFull),
        }
    }

    /// Sends given data to this queue.
    ///
    /// # Parameters
    /// * `data` - Data to send.
    ///
    /// # Return
    /// `()` if successful, `RuntimeError` otherwise.
    fn send_data(&self, data: T) -> Result<(), RuntimeError> {
        match self.data_queue.lock(|q| q.enqueue(data)) {
            Ok(_) => (),
            Err(_) => return Err(RuntimeError::DataQueueFull),
        };

        self.wake_tasklets();

        Ok(())
    }

    /// Clears this queue.
    pub(crate) fn clear(&self) {
        self.data_queue.lock(|q| while q.dequeue().is_some() {})
    }

    /// Wakes tasklets registered to this queue.
    fn wake_tasklets(&self) {
        for t in &self.registered_tasklets {
            Aerugo::wake_tasklet(t);
        }
    }
}

impl<T, const N: usize> DataProvider<T> for MessageQueue<T, N> {
    /// Returns elements from this queue.
    ///
    /// Deqeueues next element.
    ///
    /// # Return
    /// `Some(T)` if there was data available, `None` otherwise.
    fn get_data(&self) -> Option<T> {
        self.data_queue.lock(|q| q.dequeue())
    }

    /// Checks if there is any data in the queue.
    fn data_waiting(&self) -> bool {
        self.data_queue.lock(|q| !q.is_empty())
    }
}

/// SAFETY: This is safe, because that structure is only stored by the [MessageQueueStorage]
/// which ensures safe access.
unsafe impl<T, const N: usize> Sync for MessageQueue<T, N> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn const_size() {
        type QueueStub = MessageQueue<(), 0>;
        let stub_size = core::mem::size_of::<QueueStub>();

        type Queue2u8 = MessageQueue<u8, 2>;
        let queue2u8_size = core::mem::size_of::<Queue2u8>();

        type Queue100u64 = MessageQueue<u64, 100>;
        let queue100u64_size = core::mem::size_of::<Queue100u64>();

        assert_eq!(queue2u8_size, stub_size);
        assert_eq!(queue100u64_size, stub_size);
    }
}
