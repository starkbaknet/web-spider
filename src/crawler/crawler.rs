use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};
use crate::pages::{Page, PageNode, Image};
use crate::utils::{is_valid_url, normalize_url};

#[derive(Clone)]
pub struct CrawlerConfig {
    pub pages: Arc<Mutex<HashMap<String, Page>>>,
    pub outlinks: Arc<Mutex<HashMap<String, PageNode>>>,
    pub backlinks: Arc<Mutex<HashMap<String, PageNode>>>,
    pub images: Arc<Mutex<HashMap<String, Vec<Image>>>>,
    pub max_pages: usize,
    pub concurrency_limit: Arc<Semaphore>,
}

impl CrawlerConfig {
    pub fn new(max_pages: usize, max_concurrency: usize) -> Self {
        CrawlerConfig {
            pages: Arc::new(Mutex::new(HashMap::new())),
            outlinks: Arc::new(Mutex::new(HashMap::new())),
            backlinks: Arc::new(Mutex::new(HashMap::new())),
            images: Arc::new(Mutex::new(HashMap::new())),
            max_pages,
            concurrency_limit: Arc::new(Semaphore::new(max_concurrency)),
        }
    }

    pub async fn len_pages(&self) -> usize {
        self.pages.lock().await.len()
    }

    pub async fn max_pages_reached(&self) -> bool {
        self.pages.lock().await.len() >= self.max_pages
    }

    pub async fn add_page(&self, page: Page) -> Result<(), String> {
        let mut pages_guard = self.pages.lock().await;
        let normalized_url = page.normalized_url.clone();

        if pages_guard.contains_key(&normalized_url) {
            return Err("Page already visited".into());
        }

        if pages_guard.len() >= self.max_pages {
            return Err("Max pages reached".into());
        }

        pages_guard.insert(normalized_url, page);
        Ok(())
    }

    pub async fn update_links(&self, current_url: &str, outgoing_links: &[String]) {
        let mut backlinks = self.backlinks.lock().await;
        let mut outlinks = self.outlinks.lock().await;

        let mut current_node = PageNode::new(current_url.to_string());

        for link in outgoing_links {
            if !is_valid_url(link) {
                continue;
            }

            let normalized_link = match normalize_url(link) {
                Ok(url) => url,
                Err(_) => continue,
            };

            if normalized_link == current_url {
                continue;
            }

            backlinks
                .entry(normalized_link.clone())
                .or_insert_with(|| PageNode::new(normalized_link.clone()))
                .append_link(current_url.to_string());

            current_node.append_link(normalized_link);
        }

        outlinks.insert(current_url.to_string(), current_node);
    }

    pub async fn add_images(&self, page_url: &str, image_map: &HashMap<String, HashMap<String, String>>) {
        let mut images = self.images.lock().await;

        for (img_url, attrs) in image_map {
            let alt = attrs.get("alt").cloned().unwrap_or_default();
            let image = Image {
                normalized_page_url: page_url.to_string(),
                normalized_source_url: img_url.to_string(),
                alt,
            };

            images.entry(page_url.to_string()).or_default().push(image);
        }
    }
}
