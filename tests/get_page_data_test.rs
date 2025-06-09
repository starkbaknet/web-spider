#[cfg(test)]
mod tests {
    use spider::crawler::get_page_data::get_page_data;

    #[tokio::test]
    async fn test_get_page_data() {
        let test_cases = vec![
            ("absolute https url", "https://example.com/"),
            ("absolute http url", "http://example.com/"),
        ];

        for (i, (name, url)) in test_cases.iter().enumerate() {
            println!("Running test {i} - {name}");
            match get_page_data(url).await {
                Ok((body, status_code, content_type)) => {
                    assert!(
                        status_code < 400,
                        "Test {} - '{}' FAIL: expected status < 400, got {}",
                        i,
                        name,
                        status_code
                    );
                    assert!(
                        content_type.starts_with("text/html"),
                        "Test {} - '{}' FAIL: unexpected content-type: {}",
                        i,
                        name,
                        content_type
                    );
                    assert!(
                        !body.is_empty(),
                        "Test {} - '{}' FAIL: body is empty",
                        i,
                        name
                    );
                }
                Err(e) => panic!("Test {} - '{}' FAIL: unexpected error: {}", i, name, e),
            }
        }
    }
}
