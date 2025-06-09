use std::fmt;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub normalized_url: String,
    pub html: String,
    pub content_type: String,
    pub status_code: i32,
    pub last_crawled: DateTime<Utc>,
}

impl fmt::Display for Page {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let html_preview = if self.html.len() > 15 {
            format!("{}...", &self.html[..15])
        } else {
            self.html.clone()
        };

        write!(
            f,
            "-------------------------------------------------\n\
             Normalized URL:    {:<10}\n\
             HTML:              {:<40}\n\
             Last Crawled:      {:<30}\n\
             Status Code:       {:<10}\n\
             Content Type:      {:<20}\n\
             -------------------------------------------------",
            self.normalized_url,
            html_preview,
            self.last_crawled.to_rfc2822(),
            self.status_code,
            self.content_type
        )
    }
}

impl Page {
    pub fn new(normalized_url: String, html: String, content_type: String, status_code: i32) -> Self {
        Self {
            normalized_url,
            html,
            content_type,
            status_code,
            last_crawled: Utc::now(),
        }
    }

    pub fn to_hash(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("normalized_url".to_string(), self.normalized_url.clone());
        map.insert("html".to_string(), self.html.clone());
        map.insert("content_type".to_string(), self.content_type.clone());
        map.insert("status_code".to_string(), self.status_code.to_string());
        map.insert("last_crawled".to_string(), self.last_crawled.to_rfc2822());
        map
    }

    pub fn from_hash(data: &HashMap<String, String>) -> Result<Self, String> {
        let normalized_url = data.get("normalized_url").ok_or("Missing 'normalized_url'")?.clone();
        let html = data.get("html").ok_or("Missing 'html'")?.clone();
        let content_type = data.get("content_type").ok_or("Missing 'content_type'")?.clone();

        let status_code = data.get("status_code")
            .ok_or("Missing 'status_code'")?
            .parse::<i32>()
            .map_err(|e| format!("Invalid status_code: {}", e))?;

        let last_crawled = data.get("last_crawled")
            .ok_or("Missing 'last_crawled'")?
            .parse::<DateTime<Utc>>()
            .map_err(|e| format!("Invalid last_crawled: {}", e))?;

        Ok(Self {
            normalized_url,
            html,
            content_type,
            status_code,
            last_crawled,
        })
    }
}
