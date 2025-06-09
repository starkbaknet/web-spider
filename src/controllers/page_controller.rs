use redis::AsyncCommands;
use tracing::{info, error};

pub struct PageController {
    db: std::sync::Arc<tokio::sync::Mutex<crate::database::Database>>,
}

impl PageController {
    pub fn new(db: std::sync::Arc<tokio::sync::Mutex<crate::database::Database>>) -> Self {
        Self { db }
    }

    pub async fn get_all_pages(&self) -> Option<std::collections::HashMap<String, crate::pages::Page>> {
        info!("Fetching data from Redis...");

        let db_guard = self.db.lock().await;
        let mut conn = match db_guard.client.get_async_connection().await {
            Ok(c) => c,
            Err(e) => {
                error!("Failed to get Redis connection: {:?}", e);
                return None;
            }
        };
        drop(db_guard); 

        let keys_result: redis::RedisResult<Vec<String>> = conn.keys(format!("{}:*", crate::utils::PAGE_PREFIX)).await;

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

        let mut pipe = redis::pipe();
        for key in &keys {
            pipe.cmd("HGETALL").arg(key);
        }

        let results: redis::RedisResult<Vec<Vec<(String, String)>>> = pipe.query_async(&mut conn).await;

        let redis_pages = match results {
            Ok(results) => {
                let mut pages_map = std::collections::HashMap::new();

                for data in results {
                    match crate::pages::dehash_page(&data.into_iter().collect()) {
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

    pub async fn save_pages(&self, crawcfg: &crate::crawler::crawler::CrawlerConfig) {
        let data = crawcfg.pages.lock().await;
        info!("Writing {} entries to the db...", data.len());

        let db_guard = self.db.lock().await;
        let mut conn = match db_guard.client.get_async_connection().await {
            Ok(c) => c,
            Err(e) => {
                error!("Failed to get Redis connection: {:?}", e);
                return;
            }
        };
        drop(db_guard); 

        let mut pipe = redis::pipe();

        for (_, page) in data.iter() {
            let page_hash = crate::pages::hash_page(page);
            let page_key = format!("{}:{}", crate::utils::PAGE_PREFIX, page.normalized_url);

            for (field, value) in &page_hash {
                pipe.hset(&page_key, field, value);
            }

            let _: Result<(), _> = conn.lpush(crate::utils::INDEXER_QUEUE_KEY, &page_key).await;
        }

        if let Err(e) = pipe.query_async::<_, ()>(&mut conn).await {
            error!("Error executing pipeline: {:?}", e);
        } else {
            info!("Successfully written {} entries to the db!", data.len());
        }
    }
}
