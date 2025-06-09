use redis::AsyncCommands;
use tracing::{info, error};

pub struct PageController {
    db: database::Database, // your db wrapper with async Redis client & context
}

impl PageController {
    pub fn new(db: database::Database) -> Self {
        Self { db }
    }

    /// Get all pages stored in Redis under keys with prefix utils::PAGE_PREFIX
    pub async fn get_all_pages(&self) -> Option<std::collections::HashMap<String, pages::Page>> {
        info!("Fetching data from Redis...");

        let mut conn = match self.db.client.get_async_connection().await {
            Ok(c) => c,
            Err(e) => {
                error!("Failed to get Redis connection: {:?}", e);
                return None;
            }
        };

        let keys_result: redis::RedisResult<Vec<String>> = conn.keys(format!("{}:*", utils::PAGE_PREFIX)).await;

        let keys = match keys_result {
            Ok(k) => k,
            Err(e) => {
                error!("Error fetching keys from Redis: {:?}", e);
                return None;
            }
        };

        if keys.is_empty() {
            return Some(std::collections::HashMap::new());
        }

        // Pipeline to get all pages (HGETALL)
        let mut pipe = redis::pipe();
        for key in &keys {
            pipe.cmd("HGETALL").arg(key);
        }

        // Execute pipeline, returns Vec<Vec<(String, String)>>
        let results: redis::RedisResult<Vec<Vec<(String, String)>>> = pipe.query_async(&mut conn).await;

        let redis_pages = match results {
            Ok(results) => {
                let mut pages_map = std::collections::HashMap::new();

                for data in results {
                    match pages::dehash_page(data) {
                        Ok(page) => {
                            pages_map.insert(page.normalized_url.clone(), page);
                        }
                        Err(e) => {
                            error!("Error dehashing page from Redis: {:?}", e);
                            return None;
                        }
                    }
                }
                pages_map
            }
            Err(e) => {
                error!("Error fetching data from Redis pipeline: {:?}", e);
                return None;
            }
        };

        Some(redis_pages)
    }

    /// Save pages from crawler config into Redis and push page keys to indexer queue
    pub async fn save_pages(&self, crawcfg: &crawler::CrawlerConfig) {
        let data = &crawcfg.pages;
        info!("Writing {} entries to the db...", data.len());

        let mut conn = match self.db.client.get_async_connection().await {
            Ok(c) => c,
            Err(e) => {
                error!("Failed to get Redis connection: {:?}", e);
                return;
            }
        };

        let mut pipe = redis::pipe();

        for page in data {
            match pages::hash_page(page) {
                Ok(page_hash) => {
                    let page_key = format!("{}:{}", utils::PAGE_PREFIX, page.normalized_url);

                    pipe.hset(&page_key, &page_hash);

                    // Push the page key to the indexer queue asynchronously, ignoring result here
                    let _ = conn.lpush(utils::INDEXER_QUEUE_KEY, &page_key).await;
                }
                Err(e) => {
                    error!("Error hashing page {}: {:?}", page.normalized_url, e);
                }
            }
        }

        if let Err(e) = pipe.query_async::<_, ()>(&mut conn).await {
            error!("Error executing pipeline: {:?}", e);
        } else {
            info!("Successfully written {} entries to the db!", data.len());
        }
    }
}
