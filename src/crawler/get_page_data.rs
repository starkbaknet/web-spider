use std::error::Error;
use std::time::Duration;

pub async fn get_page_data(url: &str) -> Result<(String, u16, String), Box<dyn Error>> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(10))
        .build()?;
    
    let response = client.get(url).send().await?;

    let status = response.status();
    let status_code = status.as_u16();

    if status.is_client_error() || status.is_server_error() {
        return Err(format!("HTTP error: {} {}", status_code, status.canonical_reason().unwrap_or("Unknown")).into());
    }

    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|val| val.to_str().ok())
        .unwrap_or("")
        .to_string();

    if !content_type.starts_with("text/html") {
        return Err(format!("Invalid content type: {}", content_type).into());
    }

    let body = response.text().await?;

    Ok((body, status_code, "text/html".to_string()))
}
