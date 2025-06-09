use regex::Regex;
use scraper::{Html, Selector};
use std::collections::{HashMap, HashSet};
use url::Url;

/// Returns (links, images), where:
/// - links: unique valid URLs (from <a href="...">)
/// - images: map of image src -> { alt: ..., src: ... }
pub fn get_urls_from_html(
    html_body: &str,
    raw_url: &str,
) -> Result<(Vec<String>, HashMap<String, HashMap<String, String>>), Box<dyn std::error::Error>> {
    let base_url = Url::parse(raw_url)?;

    let document = Html::parse_document(html_body);
    let a_selector = Selector::parse("a[href]").unwrap();
    let img_selector = Selector::parse("img[src]").unwrap();

    let non_ascii = Regex::new(r"[^\x20-\x7E]")?;

    let mut link_set = HashSet::new();
    let mut image_map = HashMap::new();

    // <a> tag parsing
    for element in document.select(&a_selector) {
        if let Some(href) = element.value().attr("href") {
            if href.contains(&['<', '>', '"', ' '][..]) || non_ascii.is_match(href) {
                continue;
            }

            if let Ok(parsed) = base_url.join(href) {
                link_set.insert(parsed.into_string());
            }
        }
    }

    // <img> tag parsing
    for element in document.select(&img_selector) {
        let mut image_data = HashMap::new();

        if let Some(src) = element.value().attr("src") {
            if src.contains(&['<', '>', '"', ' '][..]) || non_ascii.is_match(src) {
                continue;
            }

            if let Ok(joined) = base_url.join(src) {
                let normalized = normalize_url(&joined.into_string());
                if let Ok(norm_url) = normalized {
                    image_data.insert("src".to_string(), norm_url.clone());

                    if let Some(alt) = element.value().attr("alt") {
                        image_data.insert("alt".to_string(), alt.to_string());
                    }

                    image_map.insert(norm_url, image_data);
                }
            }
        }
    }

    Ok((link_set.into_iter().collect(), image_map))
}
