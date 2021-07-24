use anyhow;
use rsmq_async::{PoolOptions, PooledRsmq, RsmqConnection, RsmqOptions};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;
use tide::log;

pub struct WorkerQueue {
    rsmq: PooledRsmq,
    qname: String,
}

impl WorkerQueue {
    pub async fn new(
        options: RsmqOptions,
        pool_options: PoolOptions,
        qname: String,
    ) -> anyhow::Result<Self> {
        let rsmq = PooledRsmq::new(options, pool_options).await?;
        Ok(Self { rsmq, qname })
    }
    pub async fn receive_message<T>(mut self) -> anyhow::Result<Option<T>>
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
    pub async fn enqueue<T>(mut self, message: &T) -> anyhow::Result<()>
    where
        T: Serialize,
    {
        let json_message = serde_json::to_string(message)?;
        let qname = self.qname.as_str().as_ref();
        self.rsmq.send_message(qname, json_message, None).await?;
        Ok(())
    }
}
