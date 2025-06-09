#[cfg(test)]
mod tests {
    use spider::utils::normalize_url::normalize_url;


    #[test]
    fn test_normalize_url() {
        struct TestCase<'a> {
            name: &'a str,
            input_url: &'a str,
            expected: &'a str,
            want_err: bool,
        }

        let tests = [
            TestCase {
                name: "remove https scheme",
                input_url: "https://en.wikipedia.org/wiki/Mega_Man_X",
                expected: "en.wikipedia.org/wiki/Mega_Man_X",
                want_err: false,
            },
            TestCase {
                name: "remove http scheme",
                input_url: "http://en.wikipedia.org/wiki/Mega_Man_X",
                expected: "en.wikipedia.org/wiki/Mega_Man_X",
                want_err: false,
            },
            TestCase {
                name: "remove trailing slash",
                input_url: "http://en.wikipedia.org/wiki/Mega_Man_X/",
                expected: "en.wikipedia.org/wiki/Mega_Man_X",
                want_err: false,
            },
            TestCase {
                name: "remove fragments",
                input_url: "https://en.wikipedia.org/wiki/Mega_Man_X#Plot",
                expected: "en.wikipedia.org/wiki/Mega_Man_X",
                want_err: false,
            },
            TestCase {
                name: "remove www.",
                input_url: "https://www.mults.com/",
                expected: "mults.com",
                want_err: false,
            },
            TestCase {
                name: "invalid scheme",
                input_url: "htps://www.mults.com/",
                expected: "",
                want_err: true,
            },
        ];

        for test in tests {
            let result = normalize_url(test.input_url);
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
