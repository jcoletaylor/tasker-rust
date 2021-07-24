use anyhow;
use redis::{aio::Connection, AsyncCommands, Client, RedisResult};
use serde::de::DeserializeOwned;
use serde_json;
use tide::log;
use uuid::Uuid;

use crate::constants;
use crate::models::api;

#[derive(Clone, Debug)]
pub struct RedisStore {
    client: Client,
    prefix: Option<String>,
}

impl RedisStore {
    pub fn from_client(client: Client) -> Self {
        Self {
            client,
            prefix: None,
        }
    }

    pub fn new(connection_info: String) -> RedisResult<Self> {
        Ok(Self::from_client(Client::open(connection_info)?).with_prefix(constants::CACHE_PREFIX))
    }

    pub fn with_prefix(mut self, prefix: &str) -> Self {
        self.prefix = Some(prefix.to_owned());
        self
    }

    fn prefix_key(&self, key: &str) -> String {
        if let Some(ref prefix) = self.prefix {
            format!("{}:{}", prefix, key)
        } else {
            key.to_string()
        }
    }

    async fn connection(&self) -> RedisResult<Connection> {
        self.client.get_async_std_connection().await
    }

    // async fn del(self, key: &str) -> () {
    //     // NOTE This is not handling prefixing, the calling function needs to
    //     let mut connection = self.connection().await?;
    //     connection.del(key).await?;
    // }

    async fn set_json_string(self, key: String, maybe_value: serde_json::Result<String>) -> () {
        match maybe_value {
            Ok(value) => {
                let res = self.set(&key.as_str(), value).await;
                match res {
                    Ok(_) => (),
                    Err(e) => {
                        log::error!("Unable to set key to redis for {}, error: {}", key, e);
                        ()
                    }
                }
            }
            Err(e) => {
                log::error!("Unable to set key to redis for {}, error: {}", key, e);
                ()
            }
        }
    }

    async fn get_result_from_cache<T>(self, key: &String) -> Option<T>
    where
        T: DeserializeOwned,
    {
        let mut connection = match self.connection().await {
            Ok(c) => c,
            Err(e) => {
                log::error!("Unable to get redis connection, error: {}", e);
                return None;
            }
        };
        let record: Option<String> = connection.get(key.as_str()).await.unwrap_or(None);
        match record {
            Some(value) => match serde_json::from_str::<T>(&value.as_str()) {
                Ok(t) => Some(t),
                Err(e) => {
                    log::error!(
                        "Unable to deserialize record for key: {}, error: {}",
                        &key,
                        e
                    );
                    None
                }
            },
            None => None,
        }
    }

    async fn set(self, key: &str, value: String) -> anyhow::Result<bool> {
        // NOTE This is not handling prefixing, the calling function needs to
        let mut connection = self.connection().await?;
        connection
            .set_ex(key, value, constants::CACHE_EXPIRY)
            .await?;
        Ok(true)
    }
}
