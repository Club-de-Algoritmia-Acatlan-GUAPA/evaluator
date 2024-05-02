use core::panic;

use fred::prelude::*;

use crate::configuration::RedisSettings;

pub struct RedisConnection {
    pool: RedisPool,
}

impl RedisConnection {
    pub async fn new(notif: &RedisSettings, cache: Option<&RedisSettings>) -> Self {
        let notif_addr = format!("redis://{}:{}", notif.host, notif.port);
        let mut pool_vec = vec![];
        let notif_redis = RedisClient::new(
            RedisConfig::from_url(&notif_addr).expect("Unable to parse config"),
            None,
            None,
            None,
        );
        pool_vec.push(notif_redis);

        if let Some(cache) = cache {
            let cache_addr = &format!("redis://{}:{}", cache.host, cache.port);
            let cache_redis = RedisClient::new(
                RedisConfig::from_url(cache_addr).expect("Unable to parse config"),
                None,
                None,
                None,
            );

            pool_vec.push(cache_redis);
        }
        let pool = RedisPool::from_clients(pool_vec).expect("Unable to build poool");
        pool.connect_pool();
        pool.wait_for_connect()
            .await
            .unwrap_or_else(|_| panic!("unable to connect to {}", notif_addr.as_str()));

        Self { pool }
    }

    pub fn get_notifier(&self) -> RedisNotifier {
        if !self.pool.clients().is_empty() {
            return RedisNotifier::new(&self.pool.clients()[0], "channel_1".to_string());
        }
        panic!("Notif redis not configured");
    }

    pub fn get_cache(&self) -> &RedisClient {
        if self.pool.clients().len() > 1 {
            return &self.pool.clients()[1];
        }
        panic!("Cache not configured");
    }
}
pub fn get_redis_tls() -> fred::types::TlsConfig {
    let config: TlsConfig = TlsConnector::default_rustls().unwrap().into();
    config
}

pub struct RedisNotifier<'redis> {
    inner: &'redis RedisClient,
    channel: String,
}

impl<'redis> RedisNotifier<'redis> {
    pub fn new(client: &'redis RedisClient, channel: String) -> Self {
        Self {
            inner: client,
            channel,
        }
    }

    pub async fn notify(&self, message: &str) -> Result<(), RedisError> {
        self.inner.publish(&self.channel, message).await
    }
}

