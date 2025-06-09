use url::Url;
use std::fmt;

#[derive(Debug)]
pub enum NormalizeUrlError {
    ParseError(url::ParseError),
    InvalidScheme,
    MissingHost,
}

impl fmt::Display for NormalizeUrlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NormalizeUrlError::ParseError(e) => write!(f, "Could not parse raw URL: {}", e),
            NormalizeUrlError::InvalidScheme => write!(f, "URL has invalid field 'Scheme'"),
            NormalizeUrlError::MissingHost => write!(f, "URL has no field 'Host'"),
        }
    }
}

impl std::error::Error for NormalizeUrlError {}

pub fn normalize_url(raw_url: &str) -> Result<String, NormalizeUrlError> {
    let u = Url::parse(raw_url).map_err(NormalizeUrlError::ParseError)?;

    let scheme = u.scheme();
    if scheme != "https" && scheme != "http" {
        return Err(NormalizeUrlError::InvalidScheme);
    }

    let host = u.host_str().ok_or(NormalizeUrlError::MissingHost)?;

    let host = if host.starts_with("www.") {
        &host[4..]
    } else {
        host
    };

    let mut normalized_url = host.to_string();

    let path = u.path();
    if !path.is_empty() && path != "/" {
        let trimmed_path = path.trim_end_matches('/');
        normalized_url.push_str(trimmed_path);
    }

    Ok(normalized_url)
}
