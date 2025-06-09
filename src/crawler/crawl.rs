use std::sync::Arc;
use tokio::sync::Mutex;
use log::{info, error};
use crate::database::Database;
use crate::pages::{Page, create_page};
use crate::utils::{is_valid_url, MIN_SCORE, MAX_SCORE};
use super::crawler::CrawlerConfig;

impl CrawlerConfig {
    pub async fn crawl(&self, db: Arc<Mutex<Database>>) {
        let _guard = self.wg.lock().await;

        loop {
            info!("Crawling...");

            if self.max_pages_reached().await {
                info!("Maximum number of pages reached");
                return;
            }

            info!("Waiting for message queue...");

            let (raw_url, depth, normalized_url) = match db.lock().await.pop_url().await {
                Ok(tuple) => tuple,
                Err(err) => {
                    error!("No more URLs in the queue: {}", err);
                    return;
                }
            };

            info!("Popped URL: {} | Depth Level: {} | Normalized URL: {}", raw_url, depth, normalized_url);

            match db.lock().await.has_url_been_visited(&normalized_url).await {
                Ok(true) => {
                    info!("Skipping {} - already visited", normalized_url);
                    continue;
                }
                Ok(false) => {}
                Err(err) => {
                    error!("Error checking visit status: {}", err);
                    continue;
                }
            }

            info!("Crawling from {} ({})...", normalized_url, raw_url);

            let (html, status_code, content_type) = match get_page_data(&raw_url).await {
                Ok(data) => data,
                Err(err) => {
                    error!("Error fetching page data: {}", err);
                    continue;
                }
            };

            let (links, images_map) = match get_urls_from_html(&html, &raw_url).await {
                Ok(data) => data,
                Err(err) => {
                    error!("Error extracting URLs from HTML: {}", err);
                    continue;
                }
            };

            self.add_images(&normalized_url, images_map).await;
            self.update_links(&normalized_url, &links).await;

            let page = create_page(normalized_url.clone(), html, content_type, status_code);

            if let Err(err) = self.add_page(&page).await {
                error!("Error adding page: {}", err);
                continue;
            }

            if let Err(err) = db.lock().await.visit_page(&normalized_url).await {
                error!("Error marking page visited: {}", err);
                continue;
            }

            info!("Adding links from {}...", normalized_url);

            for raw_link in links {
                if !is_valid_url(&raw_link) {
                    continue;
                }

                let score = match db.lock().await.exists_in_queue(&raw_link).await {
                    Some(existing_score) => existing_score,
                    None => depth + 1.0,
                };

                let bounded_score = score.clamp(MIN_SCORE, MAX_SCORE);

                let _ = db.lock().await.push_url(&raw_link, bounded_score).await;
            }
        }
    }
}
