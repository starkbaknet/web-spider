#[cfg(test)]
mod tests {
    use spider::utils::strip_url::strip_url;

    struct TestCase<'a> {
        name: &'a str,
        input_url: &'a str,
        expected: &'a str,
        want_err: bool,
    }

    #[test]
    fn test_strip_url() {
        let tests = [
            TestCase {
                name: "remove trailing slash",
                input_url: "http://en.wikipedia.org/wiki/Mega_Man_X/",
                expected: "http://en.wikipedia.org/wiki/Mega_Man_X",
                want_err: false,
            },
            TestCase {
                name: "remove fragments",
                input_url: "https://en.wikipedia.org/wiki/Mega_Man_X#Plot",
                expected: "https://en.wikipedia.org/wiki/Mega_Man_X",
                want_err: false,
            },
            TestCase {
                name: "remove query parameters",
                input_url: "https://en.wikipedia.org/wiki/Mega_Man_X?version=1.0&language=en",
                expected: "https://en.wikipedia.org/wiki/Mega_Man_X",
                want_err: false,
            },
            TestCase {
                name: "don't remove www.",
                input_url: "https://www.mults.com/",
                expected: "https://www.mults.com",
                want_err: false,
            },
        ];

        for test in tests {
            let result = strip_url(test.input_url);
            match result {
                Ok(actual) => {
                    if test.want_err {
                        panic!("Test '{}' FAILED: expected error but got Ok({})", test.name, actual);
                    }
                    assert_eq!(actual, test.expected, "Test '{}' FAILED: expected '{}', got '{}'", test.name, test.expected, actual);
                }
                Err(_) => {
                    if !test.want_err {
                        panic!("Test '{}' FAILED: unexpected error", test.name);
                    }
                }
            }
        }
    }
}
