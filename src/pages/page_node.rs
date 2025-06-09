use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Clone)]
pub struct PageNode {
    pub normalized_url: String,
    normalized_link_urls: HashSet<String>,
}

impl PageNode {
    pub fn new(normalized_url: String) -> Self {
        Self {
            normalized_url,
            normalized_link_urls: HashSet::new(),
        }
    }

    pub fn append_link(&mut self, new_normalized_link: String) {
        self.normalized_link_urls.insert(new_normalized_link);
    }

    pub fn get_links(&self) -> Vec<String> {
        self.normalized_link_urls.iter().cloned().collect()
    }
}

impl fmt::Display for PageNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut links: Vec<_> = self.normalized_link_urls.iter().cloned().collect();
        links.sort(); // optional: for consistent order

        write!(
            f,
            "\n-------------------------------------------------\n\
             {} has {} backlinks:\n\
             {}\n\
             -------------------------------------------------\n",
            self.normalized_url,
            links.len(),
            links.join("\n")
        )
    }
}
