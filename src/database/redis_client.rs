use anyhow::{anyhow, Result};
use redis::{AsyncCommands, Client};
use redis::aio::MultiplexedConnection;

pub struct Database {
    conn: MultiplexedConnection,
    pub client: Client,
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
        
        // Set connection timeout using tokio::time::timeout
        let mgr = tokio::time::timeout(
            std::time::Duration::from_secs(10),
            client.get_multiplexed_async_connection()
        ).await.map_err(|_| anyhow!("Redis connection timeout"))??;
        
        let _test_conn = tokio::time::timeout(
            std::time::Duration::from_secs(5),
            client.get_async_connection()
        ).await.map_err(|_| anyhow!("Redis test connection timeout"))?.map_err(|e| anyhow!("Redis connection test failed: {}", e))?;
        Ok(Self { conn: mgr, client: client })
    }

    pub async fn push_url(
        &mut self,
        raw_url: &str,
        score: f64,
    ) -> Result<()> {
        use crate::utils::{normalize_url, strip_url, SPIDER_QUEUE_KEY};
        
        let raw = strip_url(raw_url).map_err(|e| anyhow!("Strip URL error: {}", e))?;
        let normalized = normalize_url(&raw).map_err(|e| anyhow!("Normalize URL error: {}", e))?;
        let _: () = self.conn.zadd(SPIDER_QUEUE_KEY, normalized, score).await?;
        println!("Pushed {} to queue", raw_url);
        Ok(())
    }

    pub async fn exists_in_queue(&mut self, raw_url: &str) -> Result<Option<f64>> {
        use crate::utils::normalize_url;
        let normalized = match normalize_url(raw_url) {
            Ok(u) => u,
            Err(_) => return Ok(None),
        };
        let score: Option<f64> = self.conn.zscore(crate::utils::SPIDER_QUEUE_KEY, normalized).await.ok();
        Ok(score)
    }

    pub async fn pop_url(&mut self) -> Result<(String, f64, String)> {
        use crate::utils::{SPIDER_QUEUE_KEY};
        let values: Vec<(String, f64)> = self
            .conn
            .zpopmin(SPIDER_QUEUE_KEY, 1)
            .await
            .map_err(|e| anyhow!("ZPopMin failed: {}", e))?;
        
        let (member, score) = values.into_iter().next()
            .ok_or_else(|| anyhow!("No URLs in queue"))?;
        let raw = format!("https://{}", member);
        Ok((raw, score, member))
    }

    pub async fn pop_signal(&mut self) -> Result<String> {
        let mut arr: Vec<String> = self.conn.brpop(crate::utils::SIGNAL_QUEUE_KEY, 0).await?;
        arr
            .pop()
            .ok_or_else(|| anyhow!("BRPop returned empty"))
    }

    pub async fn get_indexer_queue_size(&mut self) -> Result<i64> {
        let size = self.conn.llen(crate::utils::INDEXER_QUEUE_KEY).await?;
        Ok(size)
    }

    pub async fn visit_page(&mut self, url: &str) -> Result<()> {
        let _: () = self.conn.set(format!("visited:{}", url), "1").await?;
        Ok(())
    }

    pub async fn has_url_been_visited(&mut self, url: &str) -> Result<bool> {
        let exists: bool = self.conn.exists(format!("visited:{}", url)).await?;
        Ok(exists)
    }
}
