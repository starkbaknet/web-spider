use std::num::ParseIntError;
use chrono::{DateTime, Utc, TimeZone, ParseError};
use serde_json;
use anyhow::{Context, Result};

/// Parses a string into an integer.
pub fn parse_int(value: &str) -> Result<i32> {
    value.parse::<i32>()
        .context("Error parsing integer")
}

/// Parses a string into a DateTime using RFC1123 format.
pub fn parse_time(value: &str) -> Result<DateTime<Utc>> {
    DateTime::parse_from_rfc2822(value)
        .map(|dt| dt.with_timezone(&Utc))
        .context("Error parsing timestamp")
}

/// Parses a JSON array string into a vector of strings.
pub fn parse_strings_slice(value: &str) -> Result<Vec<String>> {
    serde_json::from_str::<Vec<String>>(value)
        .context("Error parsing JSON string slice")
}
