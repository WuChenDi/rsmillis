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
//! let milliseconds = ms("2h").unwrap();
//! assert_eq!(milliseconds, 7200000);
//!
//! let milliseconds = ms("1d").unwrap();
//! assert_eq!(milliseconds, 86400000);
//!
//! // Format milliseconds
//! let formatted = ms(60000).unwrap();
//! assert_eq!(formatted, "1m");
//!
//! // With long format - use format() function
//! let formatted = format(60000, Some(Options { long: true }));
//! assert_eq!(formatted, "1 minute");
//! ```

// Time unit constants in milliseconds
const S: f64 = 1000.0;
const M: f64 = S * 60.0;
const H: f64 = M * 60.0;
const D: f64 = H * 24.0;
const W: f64 = D * 7.0;
const Y: f64 = D * 365.25;
const MO: f64 = Y / 12.0;

/// Errors that can occur when parsing a time string.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// The input string is empty.
    Empty,
    /// The input string is longer than 100 characters.
    TooLong,
    /// The input string is not a valid time string.
    InvalidFormat,
    /// The numeric part of the input could not be parsed.
    InvalidNumber,
    /// The parsed value does not fit in an `i64` number of milliseconds.
    Overflow,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Empty => write!(f, "value provided to parse() must not be empty"),
            Error::TooLong => write!(
                f,
                "value provided to parse() must not exceed 100 characters"
            ),
            Error::InvalidFormat => write!(f, "invalid time string format"),
            Error::InvalidNumber => write!(f, "invalid numeric value"),
            Error::Overflow => write!(f, "value does not fit in an i64 number of milliseconds"),
        }
    }
}

impl std::error::Error for Error {}

/// Options for formatting milliseconds
#[derive(Debug, Clone, Copy, Default)]
pub struct Options {
    /// Set to `true` to use verbose formatting. Defaults to `false`.
    pub long: bool,
}

/// Trait for types that can be converted to/from milliseconds
pub trait ToMillis {
    type Output;
    fn to_millis(self) -> Self::Output;
}

/// Implementation for &str - converts string to milliseconds
impl ToMillis for &str {
    type Output = Result<i64, Error>;

    fn to_millis(self) -> Result<i64, Error> {
        parse(self)
    }
}

/// Implementation for String - converts string to milliseconds
impl ToMillis for String {
    type Output = Result<i64, Error>;

    fn to_millis(self) -> Result<i64, Error> {
        parse(&self)
    }
}

/// Implementation for i64 - converts milliseconds to formatted string
impl ToMillis for i64 {
    type Output = Result<String, Error>;

    fn to_millis(self) -> Result<String, Error> {
        Ok(format(self, None))
    }
}

/// Parse or format the given value.
///
/// This is a unified interface that can handle both strings (parsing to milliseconds)
/// and numbers (formatting to time strings). The return type is inferred from the input.
///
/// # Arguments
///
/// * `value` - The string or number to convert
///
/// # Returns
///
/// * `Result<i64, Error>` if input was a string (parsed to milliseconds)
/// * `Result<String, Error>` if input was a number (formatted to time string)
///
/// # Errors
///
/// Returns an error if the value cannot be parsed or formatted.
///
/// # Examples
///
/// ```
/// use millis::ms;
///
/// // Parse string to milliseconds
/// let milliseconds = ms("2h").unwrap();
/// assert_eq!(milliseconds, 7200000);
///
/// let milliseconds = ms("1 day").unwrap();
/// assert_eq!(milliseconds, 86400000);
///
/// // Format milliseconds to string
/// let formatted = ms(7200000).unwrap();
/// assert_eq!(formatted, "2h");
/// ```
pub fn ms<T: ToMillis>(value: T) -> T::Output {
    value.to_millis()
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
/// use millis::{parse, Error};
///
/// assert_eq!(parse("2h").unwrap(), 7200000);
/// assert_eq!(parse("1 day").unwrap(), 86400000);
/// assert_eq!(parse("30 minutes").unwrap(), 1800000);
/// assert_eq!(parse("-1h").unwrap(), -3600000);
/// assert!(parse("invalid").is_err());
/// assert_eq!(parse("10000000000y"), Err(Error::Overflow));
/// ```
pub fn parse(s: &str) -> Result<i64, Error> {
    if s.is_empty() {
        return Err(Error::Empty);
    }
    if s.len() > 100 {
        return Err(Error::TooLong);
    }

    // Scan the numeric part: optional leading '-', then digits with at most one '.'.
    let bytes = s.as_bytes();
    let mut i = usize::from(bytes[0] == b'-');
    let mut digits = 0usize;
    let mut seen_dot = false;
    while let Some(&b) = bytes.get(i) {
        match b {
            b'0'..=b'9' => digits += 1,
            b'.' if !seen_dot => seen_dot = true,
            _ => break,
        }
        i += 1;
    }
    // At least one digit, and the number must end with a digit
    // (".5" is valid, "5." is not).
    if digits == 0 || bytes[i - 1] == b'.' {
        return Err(Error::InvalidFormat);
    }

    let value: f64 = s[..i].parse().map_err(|_| Error::InvalidNumber)?;

    // Optional whitespace, then an optional alphabetic unit reaching the end.
    let unit = s[i..].trim_start();
    let multiplier = if unit.is_empty() {
        1.0
    } else {
        match unit.to_ascii_lowercase().as_str() {
            "years" | "year" | "yrs" | "yr" | "y" => Y,
            "months" | "month" | "mo" => MO,
            "weeks" | "week" | "w" => W,
            "days" | "day" | "d" => D,
            "hours" | "hour" | "hrs" | "hr" | "h" => H,
            "minutes" | "minute" | "mins" | "min" | "m" => M,
            "seconds" | "second" | "secs" | "sec" | "s" => S,
            "milliseconds" | "millisecond" | "msecs" | "msec" | "ms" => 1.0,
            _ => return Err(Error::InvalidFormat),
        }
    };

    let result = value * multiplier;
    let rounded = result.round();
    // `i64::MIN as f64` is exactly -(2^63). `i64::MAX as f64` rounds up to exactly
    // 2^63, the first value that does not fit, so the upper bound must be strict.
    if !rounded.is_finite() || rounded < i64::MIN as f64 || rounded >= i64::MAX as f64 {
        return Err(Error::Overflow);
    }
    Ok(rounded as i64)
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
pub fn parse_strict(s: &str) -> Result<i64, Error> {
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
///
/// // Extreme values do not panic
/// assert_eq!(format(i64::MIN, None), "-292271023y");
/// assert_eq!(format(i64::MAX, None), "292271023y");
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
    // unsigned_abs avoids the negation overflow of i64::MIN.abs()
    let ms_abs = ms.unsigned_abs();
    let ms_f64 = ms as f64;

    if ms_abs >= Y as u64 {
        format!("{}y", (ms_f64 / Y).round() as i64)
    } else if ms_abs >= MO as u64 {
        format!("{}mo", (ms_f64 / MO).round() as i64)
    } else if ms_abs >= W as u64 {
        format!("{}w", (ms_f64 / W).round() as i64)
    } else if ms_abs >= D as u64 {
        format!("{}d", (ms_f64 / D).round() as i64)
    } else if ms_abs >= H as u64 {
        format!("{}h", (ms_f64 / H).round() as i64)
    } else if ms_abs >= M as u64 {
        format!("{}m", (ms_f64 / M).round() as i64)
    } else if ms_abs >= S as u64 {
        format!("{}s", (ms_f64 / S).round() as i64)
    } else {
        format!("{}ms", ms)
    }
}

/// Long format for milliseconds
fn fmt_long(ms: i64) -> String {
    // unsigned_abs avoids the negation overflow of i64::MIN.abs()
    let ms_abs = ms.unsigned_abs();
    let ms_f64 = ms as f64;

    if ms_abs >= Y as u64 {
        plural(ms_f64, ms_abs as f64, Y, "year")
    } else if ms_abs >= MO as u64 {
        plural(ms_f64, ms_abs as f64, MO, "month")
    } else if ms_abs >= W as u64 {
        plural(ms_f64, ms_abs as f64, W, "week")
    } else if ms_abs >= D as u64 {
        plural(ms_f64, ms_abs as f64, D, "day")
    } else if ms_abs >= H as u64 {
        plural(ms_f64, ms_abs as f64, H, "hour")
    } else if ms_abs >= M as u64 {
        plural(ms_f64, ms_abs as f64, M, "minute")
    } else if ms_abs >= S as u64 {
        plural(ms_f64, ms_abs as f64, S, "second")
    } else {
        format!("{} ms", ms)
    }
}

/// Pluralization helper
fn plural(ms: f64, ms_abs: f64, n: f64, name: &str) -> String {
    let is_plural = ms_abs >= n * 1.5;
    let value = (ms / n).round() as i64;
    format!("{} {}{}", value, name, if is_plural { "s" } else { "" })
}
