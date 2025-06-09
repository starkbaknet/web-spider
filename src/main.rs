mod utils;               // declare the `utls` module folder
use utils::parse::*;     // import all parse functions

fn main() {
    // Test parse_int
    match parse_int("123") {
        Ok(n) => println!("Parsed integer: {}", n),
        Err(e) => eprintln!("Failed to parse int: {}", e),
    }

    // Test parse_time (RFC2822 format)
    let time_str = "Mon, 09 Jun 2025 10:18:14 GMT";
    match parse_time(time_str) {
        Ok(t) => println!("Parsed time: {}", t),
        Err(e) => eprintln!("Failed to parse time: {}", e),
    }

    // Test parse_strings_slice
    let json_str = r#"["https://example.com", "https://rust-lang.org"]"#;
    match parse_strings_slice(json_str) {
        Ok(links) => {
            println!("Parsed links:");
            for link in links {
                println!(" - {}", link);
            }
        }
        Err(e) => eprintln!("Failed to parse JSON string slice: {}", e),
    }
}
