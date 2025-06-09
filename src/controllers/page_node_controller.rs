use redis::AsyncCommands;
use tracing::{info, error};

pub struct LinksController {
    db: database::Database,
}

impl LinksController {
    pub fn new(db: database::Database) -> Self {
        Self { db }
    }

    /// Save backlinks and outlinks from crawler config into Redis sets with pipeline
    pub async fn save_links(&self, crawcfg: &crawler::CrawlerConfig) {
        info!("Saving backlinks...");

        let mut conn = match seluse redis::AsyncCommands;
        use tracing::{info, error};
        
        pub struct LinksController {
            db: database::Database,
        }
        
        impl LinksController {
            pub fn new(db: database::Database) -> Self {
                Self { db }
            }
        
            /// Save backlinks and outlinks from crawler config into Redis sets with pipeline
            pub async fn save_links(&self, crawcfg: &crawler::CrawlerConfig) {
                info!("Saving backlinks...");
        
                let mut conn = match self.db.client.get_async_connection().await {
                    Ok(c) => c,
                    Err(e) => {
                        error!("Failed to get Redis connection: {:?}", e);
                        return;
                    }
                };
        
                let mut pipe = redis::pipe();
        
                let mut count = 0;
        
                // Save backlinks
                for (key, backlinks) in &crawcfg.backlinks {
                    let redis_key = format!("{}:{}", utils::BACKLINKS_PREFIX, key);
                    for link in backlinks.get_links() {
                        pipe.cmd("SADD").arg(&redis_key).arg(link);
                    }
                    count += backlinks.get_links().len();
                }
        
                info!("Saving outlinks...");
        
                // Save outlinks
                for (key, outlinks) in &crawcfg.outlinks {
                    let redis_key = format!("{}:{}", utils::OUTLINKS_PREFIX, key);
                    for link in outlinks.get_links() {
                        pipe.cmd("SADD").arg(&redis_key).arg(link);
                    }
                    count += outlinks.get_links().len();
                }
        
                match pipe.query_async::<_, ()>(&mut conn).await {
                    Ok(_) => info!("Successfully written {} entries to the db!", count),
                    Err(e) => error!("Error executing pipeline: {:?}", e),
                }
            }
        }
        f.db.client.get_async_connection().await {
            Ok(c) => c,
            Err(e) => {
                error!("Failed to get Redis connection: {:?}", e);
                return;
            }
        };

        let mut pipe = redis::pipe();

        let mut count = 0;

        // Save backlinks
        for (key, backlinks) in &crawcfg.backlinks {
            let redis_key = format!("{}:{}", utils::BACKLINKS_PREFIX, key);
            for link in backlinks.get_links() {
                pipe.cmd("SADD").arg(&redis_key).arg(link);
            }
            count += backlinks.get_links().len();
        }

        info!("Saving outlinks...");

        // Save outlinks
        for (key, outlinks) in &crawcfg.outlinks {
            let redis_key = format!("{}:{}", utils::OUTLINKS_PREFIX, key);
            for link in outlinks.get_links() {
                pipe.cmd("SADD").arg(&redis_key).arg(link);
            }
            count += outlinks.get_links().len();
        }

        match pipe.query_async::<_, ()>(&mut conn).await {
            Ok(_) => info!("Successfully written {} entries to the db!", count),
            Err(e) => error!("Error executing pipeline: {:?}", e),
        }
    }
}
