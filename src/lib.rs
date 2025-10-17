//! A tiny Rust library that converts various time formats to milliseconds.
//!
//! This library provides functionality to parse time strings into milliseconds
//! and format milliseconds into human-readable time strings.
//!
//! # Examples
//!
//! ```
//! use millis::{ms, parse, format, Options};
//!
//! // Parse time strings
//! assert_eq!(parse("2h").unwrap(), 7200000);
//! assert_eq!(parse("1d").unwrap(), 86400000);
//! assert_eq!(parse("10 seconds").unwrap(), 10000);
//!
//! // Format milliseconds
//! assert_eq!(format(60000, None), "1m");
//! assert_eq!(format(60000, Some(Options { long: true })), "1 minute");
//!
//! // Use the unified ms function
//! use millis::Value;
//! let result = ms(Value::String("2h".to_string()), None).unwrap();
//! match result {
//!     Value::Number(n) => assert_eq!(n, 7200000),
//!     _ => panic!("Expected number"),
//! }
//! ```

use regex::Regex;
use std::sync::OnceLock;

// Time unit constants in milliseconds
const S: f64 = 1000.0;
const M: f64 = S * 60.0;
const H: f64 = M * 60.0;
const D: f64 = H * 24.0;
const W: f64 = D * 7.0;
const Y: f64 = D * 365.25;
const MO: f64 = Y / 12.0;

/// Options for formatting milliseconds
#[derive(Debug, Clone, Copy)]
pub struct Options {
    /// Set to `true` to use verbose formatting. Defaults to `false`.
    pub long: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options { long: false }
    }
}

/// Represents either a string or number value
#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Number(i64),
}

/// Parse or format the given value.
///
/// # Arguments
///
/// * `value` - The string or number to convert
/// * `options` - Options for the conversion
///
/// # Returns
///
/// * `Value::Number` if input was a string (parsed to milliseconds)
/// * `Value::String` if input was a number (formatted to time string)
///
/// # Errors
///
/// Returns an error if the value cannot be parsed or formatted.
///
/// # Examples
///
/// ```
/// use millis::{ms, Value, Options};
///
/// // Parse string to number
/// let result = ms(Value::String("2h".to_string()), None).unwrap();
/// match result {
///     Value::Number(n) => assert_eq!(n, 7200000),
///     _ => panic!("Expected number"),
/// }
///
/// // Format number to string
/// let result = ms(Value::Number(7200000), None).unwrap();
/// match result {
///     Value::String(s) => assert_eq!(s, "2h"),
///     _ => panic!("Expected string"),
/// }
/// ```
pub fn ms(value: Value, options: Option<Options>) -> Result<Value, String> {
    match value {
        Value::String(s) => {
            if s.is_empty() {
                return Err("Value provided to ms() must be a non-empty string".to_string());
            }
            let result = parse(&s)?;
            Ok(Value::Number(result))
        }
        Value::Number(n) => {
            Ok(Value::String(format(n, options)))
        }
    }
}

/// Parse the given string and return milliseconds.
///
/// # Arguments
///
/// * `s` - A string to parse to milliseconds (e.g., "2h", "1d", "10 seconds")
///
/// # Returns
///
/// The parsed value in milliseconds as `i64`, or an error for invalid inputs.
///
/// # Note
///
/// Values are rounded to the nearest integer millisecond.
///
/// # Examples
///
/// ```
/// use millis::parse;
///
/// assert_eq!(parse("2h").unwrap(), 7200000);
/// assert_eq!(parse("1 day").unwrap(), 86400000);
/// assert_eq!(parse("30 minutes").unwrap(), 1800000);
/// assert_eq!(parse("-1h").unwrap(), -3600000);
/// assert!(parse("invalid").is_err());
/// ```
pub fn parse(s: &str) -> Result<i64, String> {
    if s.is_empty() || s.len() > 100 {
        return Err(format!(
            "Value provided to parse() must be a string with length between 1 and 100. value={:?}",
            s
        ));
    }

    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| {
        Regex::new(r"(?i)^(?P<value>-?\d*\.?\d+)\s*(?P<unit>milliseconds?|msecs?|ms|seconds?|secs?|s|minutes?|mins?|m|hours?|hrs?|h|days?|d|weeks?|w|months?|mo|years?|yrs?|y)?$")
            .unwrap()
    });

    let caps = match re.captures(s) {
        Some(c) => c,
        None => return Err(format!("Invalid time string format. value={:?}", s)),
    };

    let value: f64 = match caps["value"].parse() {
        Ok(v) => v,
        Err(_) => return Err(format!("Invalid numeric value. value={:?}", s)),
    };

    let unit = caps
        .name("unit")
        .map(|m| m.as_str().to_lowercase())
        .unwrap_or_else(|| "ms".to_string());

    let multiplier = match unit.as_str() {
        "years" | "year" | "yrs" | "yr" | "y" => Y,
        "months" | "month" | "mo" => MO,
        "weeks" | "week" | "w" => W,
        "days" | "day" | "d" => D,
        "hours" | "hour" | "hrs" | "hr" | "h" => H,
        "minutes" | "minute" | "mins" | "min" | "m" => M,
        "seconds" | "second" | "secs" | "sec" | "s" => S,
        "milliseconds" | "millisecond" | "msecs" | "msec" | "ms" => 1.0,
        _ => return Err(format!("Unknown unit {:?}. value={:?}", unit, s)),
    };

    let result = value * multiplier;
    Ok(result.round() as i64)
}

/// Parse the given string and return milliseconds (strict version).
///
/// This is functionally equivalent to `parse()` but exists for API compatibility.
///
/// # Examples
///
/// ```
/// use millis::parse_strict;
///
/// assert_eq!(parse_strict("2h").unwrap(), 7200000);
/// ```
pub fn parse_strict(s: &str) -> Result<i64, String> {
    parse(s)
}

/// Format the given milliseconds as a string.
///
/// # Arguments
///
/// * `ms` - milliseconds to format
/// * `options` - Options for the conversion (use `long` format if specified)
///
/// # Returns
///
/// The formatted string
///
/// # Examples
///
/// ```
/// use millis::{format, Options};
///
/// assert_eq!(format(60000, None), "1m");
/// assert_eq!(format(60000, Some(Options { long: true })), "1 minute");
/// assert_eq!(format(3600000, None), "1h");
/// assert_eq!(format(-3600000, None), "-1h");
/// ```
pub fn format(ms: i64, options: Option<Options>) -> String {
    let opts = options.unwrap_or_default();
    if opts.long {
        fmt_long(ms)
    } else {
        fmt_short(ms)
    }
}

/// Short format for milliseconds
fn fmt_short(ms: i64) -> String {
    let ms_abs = ms.abs();
    let ms_f64 = ms as f64;
    
    if ms_abs >= Y as i64 {
        format!("{}y", (ms_f64 / Y).round() as i64)
    } else if ms_abs >= MO as i64 {
        format!("{}mo", (ms_f64 / MO).round() as i64)
    } else if ms_abs >= W as i64 {
        format!("{}w", (ms_f64 / W).round() as i64)
    } else if ms_abs >= D as i64 {
        format!("{}d", (ms_f64 / D).round() as i64)
    } else if ms_abs >= H as i64 {
        format!("{}h", (ms_f64 / H).round() as i64)
    } else if ms_abs >= M as i64 {
        format!("{}m", (ms_f64 / M).round() as i64)
    } else if ms_abs >= S as i64 {
        format!("{}s", (ms_f64 / S).round() as i64)
    } else {
        format!("{}ms", ms)
    }
}

/// Long format for milliseconds
fn fmt_long(ms: i64) -> String {
    let ms_abs = ms.abs();
    let ms_f64 = ms as f64;
    
    if ms_abs >= Y as i64 {
        plural(ms_f64, ms_abs as f64, Y, "year")
    } else if ms_abs >= MO as i64 {
        plural(ms_f64, ms_abs as f64, MO, "month")
    } else if ms_abs >= W as i64 {
        plural(ms_f64, ms_abs as f64, W, "week")
    } else if ms_abs >= D as i64 {
        plural(ms_f64, ms_abs as f64, D, "day")
    } else if ms_abs >= H as i64 {
        plural(ms_f64, ms_abs as f64, H, "hour")
    } else if ms_abs >= M as i64 {
        plural(ms_f64, ms_abs as f64, M, "minute")
    } else if ms_abs >= S as i64 {
        plural(ms_f64, ms_abs as f64, S, "second")
    } else {
        format!("{} ms", ms)
    }
}

/// Pluralization helper
fn plural(ms: f64, ms_abs: f64, n: f64, name: &str) -> String {
    let is_plural = ms_abs >= n * 1.5;
    let value = (ms / n).round() as i64;
    format!(
        "{} {}{}",
        value,
        name,
        if is_plural { "s" } else { "" }
    )
}
