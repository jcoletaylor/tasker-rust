use crate::constants;
use crate::queue::manager::QueueManager;
use anyhow;
use async_std::task;
use rsmq_async::{PoolOptions, RsmqOptions};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;
use std::fmt::Debug;
use std::future::Future;
use std::time::Duration;

#[derive(Clone)]
pub struct WorkerQueue {
    queue_manager: QueueManager,
}

impl WorkerQueue {
    pub async fn new(
        options: RsmqOptions,
        pool_options: PoolOptions,
        qname: String,
    ) -> anyhow::Result<Self> {
        let queue = QueueManager::new(options, pool_options, qname).await?;
        Ok(Self {
            queue_manager: queue,
        })
    }
    pub async fn poll<T, F, Fut>(self, callback_fn: F) -> ()
    where
        T: DeserializeOwned + Serialize + Debug,
        F: FnOnce(&T) -> Fut,
        F: Copy,
        Fut: Future<Output = anyhow::Result<Option<T>>>,
    {
        loop {
            // figure out how to either spawn tasks or threads for this
            let result = self.clone().handle::<T, F, Fut>(callback_fn).await;
            match result {
                Ok(res) => match res {
                    Some(r) => log::info!("result handled: {:?}", r),
                    None => log::info!("no result retrieved"),
                },
                Err(e) => log::error!("error in handler: {}", e),
            };
            let dur = Duration::from_millis(constants::QUEUE_POLL_SLEEP_DURATION);
            task::sleep(dur).await;
        }
    }

    pub async fn handle<T, F, Fut>(mut self, callback_fn: F) -> anyhow::Result<Option<T>>
    where
        T: DeserializeOwned,
        T: Serialize,
        F: FnOnce(&T) -> Fut,
        Fut: Future<Output = anyhow::Result<Option<T>>>,
    {
        let maybe_task: Option<T> = self.queue_manager.dequeue::<T>().await?;
        match maybe_task {
            Some(task) => {
                let handled_task = callback_fn(&task).await?;
                Ok(handled_task)
            }
            None => Ok(None),
        }
    }
}
