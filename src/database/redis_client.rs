use anyhow::{anyhow, Result};
use chrono::Duration;
use redis::{aio::ConnectionManager, AsyncCommands, Client, RedisError};
use std::time::Duration as StdDuration;

/// Redis queue manager
pub struct Database {
    conn: ConnectionManager,
}

impl Database {
    pub async fn connect(
        host: &str,
        port: &str,
        password: &str,
        db: i64,
    ) -> Result<Self> {
        let addr = format!("redis://{}:{}?password={}&db={}", host, port, password, db);
        let client = Client::open(addr.as_str())?;
        let mgr = ConnectionManager::new(client).await?;
        // Test connection
        let mut conn = mgr.clone();
        conn.ping().await.map_err(|e| anyhow!("Redis ping failed: {}", e))?;
        Ok(Self { conn: mgr })
    }

    pub async fn push_url(
        &mut self,
        raw_url: &str,
        score: f64,
    ) -> Result<()> {
        use crate::utils::{normalize_url, strip_url, SpiderQueueKey};
        
        let raw = strip_url(raw_url)?;
        let normalized = normalize_url(&raw)?;
        let _: () = self.conn.zadd(SpiderQueueKey, normalized, score).await?;
        println!("Pushed {} to queue", raw_url);
        Ok(())
    }

    pub async fn exists_in_queue(&mut self, raw_url: &str) -> Result<Option<f64>> {
        use crate::utils::normalize_url;
        let normalized = match normalize_url(raw_url) {
            Ok(u) => u,
            Err(_) => return Ok(None),
        };
        let score: Option<f64> = self.conn.zscore(crate::utils::SpiderQueueKey, normalized).await.ok();
        Ok(score)
    }

    pub async fn pop_url(&mut self) -> Result<(String, f64, String)> {
        use crate::utils::{TBOPERATION_TIMEOUT, SpiderQueueKey};
        let (key, value): (String, (String, f64)) = self
            .conn
            .bzpopmin(SpiderQueueKey, StdDuration::from_secs(TBOPERATION_TIMEOUT))
            .await
            .map_err(|e| anyhow!("BZPopMin failed: {}", e))?;
        // value -> (member, score)
        let raw = format!("https://{}", value.0);
        Ok((raw, value.1, value.0))
    }

    pub async fn pop_signal(&mut self) -> Result<String> {
        let mut arr: Vec<String> = self.conn.brpop(crate::utils::SignalQueueKey, 0).await?;
        arr
            .pop()
            .ok_or_else(|| anyhow!("BRPop returned empty"))
    }

    pub async fn get_indexer_queue_size(&mut self) -> Result<i64> {
        let size = self.conn.llen(crate::utils::IndexerQueueKey).await?;
        Ok(size)
    }
}
