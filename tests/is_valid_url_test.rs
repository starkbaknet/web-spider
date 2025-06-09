#[cfg(test)]
mod tests {
    use spider::utils::is_valid_url::is_valid_url;

    #[test]
    fn test_is_valid_url() {
        struct TestCase<'a> {
            name: &'a str,
            input_url: &'a str,
            expected: bool,
        }

        let tests = [
            TestCase {
                name: "valid url",
                input_url: "https://en.wikipedia.org/wiki/Mega_Man_X",
                expected: true,
            },
            TestCase {
                name: "valid normalized url",
                input_url: "en.wikipedia.org/wiki/Mega_Man_X",
                expected: true,
            },
            TestCase {
                name: "invalid url (japanese)",
                input_url: "https://ja.wikipedia.org/wiki/仮面ライダーシリーズ",
                expected: false,
            },
            TestCase {
                name: "invalid url (japanese 2)",
                input_url: "wuu.wikipedia.org/wiki/假面骑士系列",
                expected: false,
            },
            TestCase {
                name: "invalid url (cyrillic)",
                input_url: "https://uk.wikipedia.org/wiki/Камен_Райдер_(франшиза)",
                expected: false,
            },
            TestCase {
                name: "invalid url (weird)",
                input_url: "https://zh-classical.wikipedia.org/wiki/%E7%B6%AD%E5%9F%BA%E5%A4%A7%E5%85%B8:%E5%B8%82%E9%9B%86",
                expected: false,
            },
        ];

        for test in tests {
            let result = is_valid_url(test.input_url);
            assert_eq!(
                result, test.expected,
                "Test '{}' FAILED: expected {}, got {}",
                test.name, test.expected, result
            );
        }
    }
}
