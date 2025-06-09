use std::env;
use std::sync::Arc;
use tokio::sync::{Mutex, Notify};
use tokio::task;
use tokio::time::{sleep, Duration};
use tracing::{info, error};

mod controllers;
mod crawler;
mod database;
mod pages;
mod utils;

#[tokio::main]
async fn main() {
    // Initialize logging (e.g. tracing_subscriber)
    tracing_subscriber::fmt::init();

    // Helper to get env var or fallback
    fn get_env(key: &str, fallback: &str) -> String {
        env::var(key).unwrap_or_else(|_| fallback.to_string())
    }

    // Parse max concurrency and max pages from env or defaults
    let max_concurrency = env::var("MAX_CONCURRENCY")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(10);

    let max_pages = env::var("MAX_PAGES")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(100);

    let redis_host = get_env("REDIS_HOST", "localhost");
    let redis_port = get_env("REDIS_PORT", "6379");
    let redis_password = get_env("REDIS_PASSWORD", "");
    let redis_db = get_env("REDIS_DB", "0");
    let starting_url = get_env("STARTING_URL", "https://en.wikipedia.org/wiki/Kamen_Rider");

    // Connect to Redis
    let db = database::Database::connect(&redis_host, &redis_port, &redis_password, &redis_db).await;
    if let Err(e) = db {
        error!("Error connecting to Redis: {:?}", e);
        return;
    }
    let db = Arc::new(db.unwrap());

    // Push starting URL with priority 0
    if let Err(e) = db.push_url(&starting_url, 0).await {
        error!("Error pushing starting URL: {:?}", e);
        return;
    }
    info!("PUSH {}", starting_url);

    // Instantiate controllers
    let page_controller = controllers::PageController::new(db.clone());
    let links_controller = controllers::LinksController::new(db.clone());
    let image_controller = controllers::ImageController::new(db.clone());

    // Shared crawler state protected by mutex
    let crawler = Arc::new(Mutex::new(crawler::CrawlerConfig {
        pages: std::collections::HashMap::new(),
        outlinks: std::collections::HashMap::new(),
        backlinks: std::collections::HashMap::new(),
        images: std::collections::HashMap::new(),
        max_pages,
        max_concurrency,
        notify: Arc::new(Notify::new()),
    }));

    loop {
        info!("Checking number of entries...");

        let queue_size = match db.get_indexer_queue_size().await {
            Ok(size) => size,
            Err(e) => {
                error!("Error getting indexer queue: {:?}", e);
                return;
            }
        };

        if queue_size >= utils::MAX_INDEXER_QUEUE_SIZE {
            info!("Indexer queue is full. Waiting...");

            // Wait for resume signal in a loop
            loop {
                match db.pop_signal_queue().await {
                    Ok(sig) if sig == utils::RESUME_CRAWL => {
                        info!("Resume crawl!");
                        break;
                    }
                    Ok(_) => { /* ignore other signals */ }
                    Err(e) => {
                        error!("Could not get signal: {:?}", e);
                        return;
                    }
                }
                sleep(Duration::from_secs(1)).await;
            }
        }

        info!("Spawning workers...");

        let mut handles = Vec::with_capacity(max_concurrency);

        for _ in 0..max_concurrency {
            let db_clone = db.clone();
            let crawler_clone = crawler.clone();

            handles.push(task::spawn(async move {
                let mut c = crawler_clone.lock().await;
                c.crawl(&db_clone).await;
            }));
        }

        for handle in handles {
            if let Err(e) = handle.await {
                error!("Worker task failed: {:?}", e);
            }
        }

        // Save results and clear crawler state
        let mut c = crawler.lock().await;
        page_controller.save_pages(&*c).await;
        links_controller.save_links(&*c).await;
        image_controller.save_images(&*c).await;

        c.pages.clear();
        c.outlinks.clear();
        c.backlinks.clear();
        c.images.clear();
    }
}
