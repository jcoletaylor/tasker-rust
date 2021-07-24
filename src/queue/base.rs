use crate::constants;
use anyhow;
use async_std::task;
use rsmq_async::{PoolOptions, PooledRsmq, RsmqConnection, RsmqOptions};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;
use std::fmt::Debug;
use std::future::Future;
use std::time::Duration;
use tide::log;

#[derive(Clone)]
pub struct BaseQueue {
    rsmq: PooledRsmq,
    qname: String,
}

impl BaseQueue {
    pub async fn new(
        options: RsmqOptions,
        pool_options: PoolOptions,
        qname: String,
    ) -> anyhow::Result<Self> {
        let rsmq = PooledRsmq::new(options, pool_options).await?;
        Ok(Self { rsmq, qname })
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
        let maybe_task: Option<T> = self.dequeue::<T>().await?;
        match maybe_task {
            Some(task) => {
                let handled_task = callback_fn(&task).await?;
                Ok(handled_task)
            }
            None => Ok(None),
        }
    }
    pub async fn enqueue<T>(mut self, message: &T) -> anyhow::Result<()>
    where
        T: Serialize,
    {
        let json_message = serde_json::to_string(message)?;
        let qname = self.qname.as_str().as_ref();
        self.rsmq.send_message(qname, json_message, None).await?;
        Ok(())
    }
    async fn dequeue<T>(mut self) -> anyhow::Result<Option<T>>
    where
        T: DeserializeOwned,
    {
        let qname = self.qname.as_str().as_ref();
        let message = self.rsmq.receive_message::<String>(qname, None).await?;

        match message {
            Some(value) => match serde_json::from_str::<T>(&value.message.as_str()) {
                Ok(t) => Ok(Some(t)),
                Err(e) => {
                    log::error!("Unable to deserialize record, error: {}", e);
                    Ok(None)
                }
            },
            None => Ok(None),
        }
    }
}
