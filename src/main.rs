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

use controllers::page_controller::PageController;
use controllers::page_node_controller::LinksController;
use controllers::image_controller::ImageController;
use crawler::crawler::CrawlerConfig;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    fn get_env(key: &str, fallback: &str) -> String {
        env::var(key).unwrap_or_else(|_| fallback.to_string())
    }

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
    let starting_url = get_env("STARTING_URL", "https://starkbak.net");

    let redis_db_num: i64 = redis_db.parse().unwrap_or(0);
    let db_instance = database::Database::connect(&redis_host, &redis_port, &redis_password, redis_db_num).await;
    if let Err(e) = db_instance {
        error!("Error connecting to Redis: {:?}", e);
        return;
    }
    let db = Arc::new(Mutex::new(db_instance.unwrap()));

    if let Err(e) = db.lock().await.push_url(&starting_url, 0.0).await {
        error!("Error pushing starting URL: {:?}", e);
        return;
    }
    info!("PUSH {}", starting_url);

    let page_controller = PageController::new(db.clone());
    let links_controller = LinksController::new(db.clone());
    let image_controller = ImageController::new(db.clone());

    let crawler = Arc::new(Mutex::new(CrawlerConfig::new(max_pages, max_concurrency)));

    loop {
        info!("Checking number of entries...");

        let queue_size = match db.lock().await.get_indexer_queue_size().await {
            Ok(size) => size,
            Err(e) => {
                error!("Error getting indexer queue: {:?}", e);
                return;
            }
        };

        if queue_size >= utils::MAX_INDEXER_QUEUE_SIZE as i64 {
            info!("Indexer queue is full. Waiting...");

            loop {
                match db.lock().await.pop_signal().await {
                    Ok(sig) if sig == utils::RESUME_CRAWL => {
                        info!("Resume crawl!");
                        break;
                    }
                    Ok(_) => { }
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
                let c = crawler_clone.lock().await;
                c.crawl(&db_clone).await;
            }));
        }

        for handle in handles {
            if let Err(e) = handle.await {
                error!("Worker task failed: {:?}", e);
            }
        }

        let mut c = crawler.lock().await;
        page_controller.save_pages(&*c).await;
        links_controller.save_links(&*c).await;
        if let Err(e) = image_controller.save_images(&*c).await {
            error!("Error saving images: {:?}", e);
        }

        c.pages.lock().await.clear();
        c.outlinks.lock().await.clear();
        c.backlinks.lock().await.clear();
        c.images.lock().await.clear();
    }
}
