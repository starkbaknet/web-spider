use tracing::{info, error};

pub struct ImageController {
    db: std::sync::Arc<tokio::sync::Mutex<crate::database::Database>>,
}

impl ImageController {
    pub fn new(db: std::sync::Arc<tokio::sync::Mutex<crate::database::Database>>) -> Self {
        Self { db }
    }

    pub async fn save_images(&self, crawcfg: &crate::crawler::crawler::CrawlerConfig) -> redis::RedisResult<()> {
        let mut pipe = redis::pipe();

        info!("Saving images...");

        let mut count = 0;
        
        let images_guard = crawcfg.images.lock().await;
        for (normalized_url, image_data) in images_guard.iter() {
            for image in image_data {
                let image_key = format!("{}:{}", crate::utils::IMAGE_PREFIX, image.normalized_source_url);

                pipe.hset(&image_key, "page_url", &image.normalized_page_url)
                    .hset(&image_key, "alt", &image.alt)
                    .expire(&image_key, 3600usize);

                count += 1;

                let page_images_key = format!("{}:{}", crate::utils::PAGE_IMAGES_PREFIX, normalized_url);
                pipe.sadd(&page_images_key, &image.normalized_source_url);
            }
        }

        let db_guard = self.db.lock().await;
        let mut conn = db_guard.client.get_async_connection().await?;
        drop(db_guard); 

        let result = pipe.query_async::<_, ()>(&mut conn).await;

        match &result {
            Ok(_) => info!("Successfully written {} entries to the db!", count),
            Err(e) => tracing::error!("Error saving images: {:?}", e),
        }

        result
    }
}
