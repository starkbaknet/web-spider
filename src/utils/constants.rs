pub mod utils {
    use std::time::Duration;
    
    // Crawler settings
    pub const TIMEOUT: Duration = Duration::from_secs(5);
    pub const MAX_SCORE: i32 = 10_000;
    pub const MIN_SCORE: i32 = -1_000;

    // Redis message queues
    pub const SPIDER_QUEUE_KEY: &str = "spider_queue";
    pub const INDEXER_QUEUE_KEY: &str = "pages_queue";
    pub const SIGNAL_QUEUE_KEY: &str = "signal_queue";
    pub const RESUME_CRAWL: &str = "RESUME_CRAWL";
    pub const MAX_INDEXER_QUEUE_SIZE: usize = 5_000;

    // Redis data keys
    pub const NORMALIZED_URL_PREFIX: &str = "normalized_url";
    pub const PAGE_PREFIX: &str = "page_data";                
    pub const IMAGE_PREFIX: &str = "image_data";              
    pub const PAGE_IMAGES_PREFIX: &str = "page_images";       
    pub const BACKLINKS_PREFIX: &str = "backlinks";           
    pub const OUTLINKS_PREFIX: &str = "outlinks";             
}
