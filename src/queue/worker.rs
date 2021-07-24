use crate::queue::base::BaseQueue;
use rsmq_async::{PoolOptions, RsmqOptions};

pub struct WorkerQueue {
    queue: BaseQueue,
}

impl WorkerQueue {
    pub async fn new(
        options: RsmqOptions,
        pool_options: PoolOptions,
        qname: String,
    ) -> anyhow::Result<Self> {
        let queue = BaseQueue::new(options, pool_options, qname).await?;
        Ok(Self { queue })
    }
    // TODO: implement callbacks
}
