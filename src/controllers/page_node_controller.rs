use tracing::{info, error};

pub struct LinksController {
    db: std::sync::Arc<tokio::sync::Mutex<crate::database::Database>>,
}

impl LinksController {
    pub fn new(db: std::sync::Arc<tokio::sync::Mutex<crate::database::Database>>) -> Self {
        Self { db }
    }

    pub async fn save_links(&self, crawcfg: &crate::crawler::crawler::CrawlerConfig) {
        info!("Saving backlinks...");

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
        let mut count = 0;

        let backlinks_guard = crawcfg.backlinks.lock().await;
        for (key, backlinks) in backlinks_guard.iter() {
            let redis_key = format!("{}:{}", crate::utils::BACKLINKS_PREFIX, key);
            for link in backlinks.get_links() {
                pipe.cmd("SADD").arg(&redis_key).arg(link);
            }
            count += backlinks.get_links().len();
        }
        drop(backlinks_guard);

        info!("Saving outlinks...");

        let outlinks_guard = crawcfg.outlinks.lock().await;
        for (key, outlinks) in outlinks_guard.iter() {
            let redis_key = format!("{}:{}", crate::utils::OUTLINKS_PREFIX, key);
            for link in outlinks.get_links() {
                pipe.cmd("SADD").arg(&redis_key).arg(link);
            }
            count += outlinks.get_links().len();
        }
        drop(outlinks_guard);

        match pipe.query_async::<_, ()>(&mut conn).await {
            Ok(_) => info!("Successfully written {} entries to the db!", count),
            Err(e) => error!("Error executing pipeline: {:?}", e),
        }
    }
}
