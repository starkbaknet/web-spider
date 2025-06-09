#[cfg(test)]
mod tests {
    use spider::utils::parse::{parse_int, parse_time, parse_strings_slice};
    use chrono::{TimeZone, Utc};

    struct ParseIntTestCase<'a> {
        name: &'a str,
        input: &'a str,
        expected: Option<i32>,
    }

    struct ParseTimeTestCase<'a> {
        name: &'a str,
        input: &'a str,
        expected: Option<&'a str>, // RFC2822 string for comparison
    }

    struct ParseStringsSliceTestCase<'a> {
        name: &'a str,
        input: &'a str,
        expected: Option<Vec<&'a str>>,
    }

    #[test]
    fn test_parse_int() {
        let tests = [
            ParseIntTestCase { name: "valid int", input: "42", expected: Some(42) },
            ParseIntTestCase { name: "negative int", input: "-15", expected: Some(-15) },
            ParseIntTestCase { name: "invalid int", input: "abc", expected: None },
        ];

        for test in tests {
            let result = parse_int(test.input);
            match result {
                Ok(val) => {
                    if let Some(expected_val) = test.expected {
                        assert_eq!(val, expected_val, "Test '{}' FAILED: expected {}, got {}", test.name, expected_val, val);
                    } else {
                        panic!("Test '{}' FAILED: expected error but got Ok({})", test.name, val);
                    }
                }
                Err(_) => {
                    if test.expected.is_some() {
                        panic!("Test '{}' FAILED: unexpected error", test.name);
                    }
                }
            }
        }
    }

    #[test]
    fn test_parse_time() {
        let tests = [
            ParseTimeTestCase {
                name: "valid time",
                input: "Wed, 02 Oct 2002 15:00:00 +0200",
                expected: Some("2002-10-02T13:00:00Z"), // converted to UTC
            },
            ParseTimeTestCase {
                name: "invalid time",
                input: "invalid time string",
                expected: None,
            },
        ];

        for test in tests {
            let result = parse_time(test.input);
            match result {
                Ok(dt) => {
                    if let Some(expected_str) = test.expected {
                        let expected_dt = expected_str.parse::<chrono::DateTime<Utc>>().unwrap();
                        assert_eq!(dt, expected_dt, "Test '{}' FAILED: expected {}, got {}", test.name, expected_dt, dt);
                    } else {
                        panic!("Test '{}' FAILED: expected error but got Ok({})", test.name, dt);
                    }
                }
                Err(_) => {
                    if test.expected.is_some() {
                        panic!("Test '{}' FAILED: unexpected error", test.name);
                    }
                }
            }
        }
    }

    #[test]
    fn test_parse_strings_slice() {
        let tests = [
            ParseStringsSliceTestCase {
                name: "valid JSON array",
                input: r#"["one", "two", "three"]"#,
                expected: Some(vec!["one", "two", "three"]),
            },
            ParseStringsSliceTestCase {
                name: "empty JSON array",
                input: r#"[]"#,
                expected: Some(vec![]),
            },
            ParseStringsSliceTestCase {
                name: "invalid JSON",
                input: r#"["unclosed string]"#,
                expected: None,
            },
        ];

        for test in tests {
            let result = parse_strings_slice(test.input);
            match result {
                Ok(vec) => {
                    if let Some(expected_vec) = test.expected {
                        let expected_vec: Vec<String> = expected_vec.iter().map(|s| s.to_string()).collect();
                        assert_eq!(vec, expected_vec, "Test '{}' FAILED: expected {:?}, got {:?}", test.name, expected_vec, vec);
                    } else {
                        panic!("Test '{}' FAILED: expected error but got Ok({:?})", test.name, vec);
                    }
                }
                Err(_) => {
                    if test.expected.is_some() {
                        panic!("Test '{}' FAILED: unexpected error", test.name);
                    }
                }
            }
        }
    }
}
