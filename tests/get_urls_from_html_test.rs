#[cfg(test)]
mod tests {
    use spider::crawler::get_urls_from_html::get_urls_from_html;
    use std::collections::HashSet;

    #[test]
    fn test_get_urls_from_html() {
        struct TestCase<'a> {
            name: &'a str,
            input_url: &'a str,
            input_body: &'a str,
            expected: Vec<&'a str>,
        }

        let tests = vec![
            TestCase {
                name: "absolute and relative URLs",
                input_url: "https://randomsite.com",
                input_body: r#"
                    <html>
                        <body>
                            <h1>Some text here</h1>
                            <a href="/path/one">
                                <span>randomsite</span>
                            </a>
                            <a href="https://othersite.com/path/one">
                                <span>othersite.com</span>
                            </a>
                        </body>
                    </html>
                "#,
                expected: vec![
                    "https://randomsite.com/path/one",
                    "https://othersite.com/path/one",
                ],
            },
            TestCase {
                name: "no URLs",
                input_url: "https://randomsite.com",
                input_body: r#"
                    <html>
                        <body>
                            <h1>Empty Website</h1>
                        </body>
                    </html>
                "#,
                expected: vec![],
            },
            TestCase {
                name: "malformed HTML but valid links",
                input_url: "https://example.com",
                input_body: r#"
                    <html>
                        <body>
                            <a href="/valid-link"><span>Valid</span></a>
                            <a href="<invalid></a>"><span>Broken</span></a>
                            <a href="https://valid.com/path"></a>
                        </body>
                    </html>
                "#,
                expected: vec![
                    "https://example.com/valid-link",
                    "https://valid.com/path",
                ],
            },
            TestCase {
                name: "remove duplicate links",
                input_url: "https://example.com",
                input_body: r#"
                    <html>
                        <body>
                            <a href="/valid-link"><span>Valid</span></a>
                            <a href="<invalid></a>"><span>Broken</span></a>
                            <a href="https://valid.com/path"></a>
                            <a href="/valid-link"><span>Valid</span></a>
                            <a href="<invalid></a>"><span>Broken</span></a>
                            <a href="https://valid.com/path"></a>
                        </body>
                    </html>
                "#,
                expected: vec![
                    "https://example.com/valid-link",
                    "https://valid.com/path",
                ],
            },
            TestCase {
                name: "ignore non-ASCII links",
                input_url: "https://example.com",
                input_body: r#"
                    <html>
                        <body>
                            <a href="/valid-link"><span>Valid</span></a>
                            <a href="https://valid.com/path"></a>
                            <a href="https://пример.рф">Cyrillic</a>
                            <a href="https://例子.com">Chinese</a>
                            <a href="https://テスト.jp">Japanese</a>
                            <a href="/another-valid"></a>
                        </body>
                    </html>
                "#,
                expected: vec![
                    "https://example.com/valid-link",
                    "https://valid.com/path",
                    "https://example.com/another-valid",
                ],
            },
        ];

        for (i, tc) in tests.iter().enumerate() {
            let (actual, _images) = get_urls_from_html(tc.input_body, tc.input_url)
                .expect(&format!("Test {} - '{}' failed to parse HTML", i, tc.name));

            let expected_set: HashSet<_> = tc.expected.iter().cloned().collect();
            let actual_set: HashSet<_> = actual.iter().map(|s| s.as_str()).collect();

            assert_eq!(
                expected_set, actual_set,
                "Test {} - '{}' FAIL: expected URLs: {:?}, actual: {:?}",
                i, tc.name, tc.expected, actual
            );
        }
    }
}
