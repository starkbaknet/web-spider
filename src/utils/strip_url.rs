use url::Url;
use std::error::Error;

pub fn strip_url(raw_url: &str) -> Result<String, Box<dyn Error>> {
    let u = Url::parse(raw_url)?;

    if u.scheme().is_empty() {
        return Err("URL has no field 'Scheme'".into());
    }

    if u.host_str().is_none() {
        return Err("URL has no field 'Host'".into());
    }

    let mut stripped_url = format!("{}://{}", u.scheme(), u.host_str().unwrap());

    if !u.path().is_empty() {
        let trimmed_path = u.path().trim_end_matches('/');
        stripped_url.push_str(trimmed_path);
    }

    Ok(stripped_url)
}
