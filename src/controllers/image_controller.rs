use std::time::Duration;
use redis::AsyncCommands;
use tracing::info;
use tokio::time::timeout;

pub struct ImageController {
    db: database::Database, // your database struct wrapping Redis client & context
}

impl ImageController {
    pub fn new(db: database::Database) -> Self {
        Self { db }
    }

    pub async fn save_images(&self, crawcfg: &crawler::CrawlerConfig) -> redis::RedisResult<()> {
        let mut pipe = redis::pipe();

        info!("Saving images...");

        let mut count = 0;

        for (normalized_url, image_data) in &crawcfg.images {
            for image in image_data {
                let image_key = format!("{}:{}", utils::IMAGE_PREFIX, image.normalized_source_url);

                // HSET image key with page_url and alt
                pipe.hset(&image_key, "page_url", &image.normalized_page_url)
                    .hset(&image_key, "alt", &image.alt)
                    // EXPIRE key after 1 hour
                    .expire(&image_key, 3600u64);

                count += 1;

                // Add image source URL to a set of page images
                let page_images_key = format!("{}:{}", utils::PAGE_IMAGES_PREFIX, normalized_url);
                pipe.sadd(&page_images_key, &image.normalized_source_url);
            }
        }

        // Get async Redis connection
        let mut conn = self.db.client.get_async_connection().await?;

        // Execute pipeline
        let result = pipe.query_async::<_, ()>(&mut conn).await;

        match result {
            Ok(_) => info!("Successfully written {} entries to the db!", count),
            Err(e) => tracing::error!("Error saving images: {:?}", e),
        }

        result
    }
}
