[![Crates.io](https://img.shields.io/crates/v/millis.svg)](https://crates.io/crates/millis)

# millis

A tiny Rust library that converts various time formats to milliseconds. Zero runtime dependencies.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
millis = "2.0.0"
```

## Usage

### Basic Usage

```rust
use millis::ms;

// Parse time strings to milliseconds
ms("2 days")?         // 172800000
ms("1d")?             // 86400000
ms("10h")?            // 36000000
ms("2.5 hrs")?        // 9000000
ms("2h")?             // 7200000
ms("1m")?             // 60000
ms("5s")?             // 5000
ms("1y")?             // 31557600000
ms("100")?            // 100
ms("-3 days")?        // -259200000
ms("-1h")?            // -3600000
ms("-200")?           // -200

// Format milliseconds to strings (infallible, returns String directly)
ms(60000)             // "1m"
ms(2 * 60000)         // "2m"
ms(-3 * 60000)        // "-3m"
ms(172800000)         // "2d"
```

### Long Format

For verbose output, use the `format()` function with options:

```rust
use millis::{ms, format, Options};

format(60000, Some(Options { long: true }));           // "1 minute"
format(2 * 60000, Some(Options { long: true }));       // "2 minutes"
format(172800000, Some(Options { long: true }));       // "2 days"

// Combine parse and format
let ms_value = ms("10 hours")?;
format(ms_value, Some(Options { long: true }));        // "10 hours"
```

### Durations

Use `parse_duration()` to get a `std::time::Duration` directly:

```rust
use std::time::Duration;
use millis::parse_duration;

assert_eq!(parse_duration("1.5s")?, Duration::from_millis(1500));
assert_eq!(parse_duration("2h")?, Duration::from_millis(7200000));

// Duration cannot be negative, so negative inputs return an error
assert!(parse_duration("-1h").is_err());
```

## API

### `ms(value)`

Parse or format the given value. The return type is inferred from the input type via the `ToMillis` trait.

**Parameters:**
- `value`: Can be `&str` or `String` (to parse), or `i64` (to format)

**Returns:**
- `Result<i64, Error>` when the input is a string (parsing to milliseconds)
- `String` when the input is an `i64` (formatting is infallible and never panics)

**Examples:**

```rust
use millis::ms;

// Parse string to milliseconds
let milliseconds: i64 = ms("2h")?;
assert_eq!(milliseconds, 7200000);

let milliseconds = ms("1 day")?;
assert_eq!(milliseconds, 86400000);

// Format milliseconds to string — returns String directly, no Result
let formatted: String = ms(7200000);
assert_eq!(formatted, "2h");

let formatted = ms(60000);
assert_eq!(formatted, "1m");
```

### `parse(value)`

Parse the given string and return milliseconds.

**Parameters:**
- `value` (`&str`): A string to parse to milliseconds

**Returns:**
- `Result<i64, Error>`: The parsed value in milliseconds, rounded to the nearest integer

**Errors:**
- `Error::Empty` if the string is empty
- `Error::TooLong` if the string is longer than 100 characters
- `Error::InvalidFormat` if the string is not a valid time string
- `Error::InvalidNumber` if the numeric part cannot be parsed
- `Error::Overflow` if the value does not fit in an `i64` number of milliseconds

**Examples:**

```rust
use millis::parse;

let ms = parse("2h")?;           // 7200000
let ms = parse("1d")?;           // 86400000
let ms = parse("10 seconds")?;   // 10000
let ms = parse("-1h")?;          // -3600000
```

### `parse_duration(value)`

Parse the given string and return a `std::time::Duration`. Accepts the same input as `parse()`.

**Parameters:**
- `value` (`&str`): A string to parse

**Returns:**
- `Result<std::time::Duration, Error>`

**Errors:**
- The same errors as `parse()`. Additionally, because `Duration` cannot represent negative spans, inputs that parse to a negative number of milliseconds return `Err(Error::InvalidFormat)`.

**Examples:**

```rust
use std::time::Duration;
use millis::parse_duration;

assert_eq!(parse_duration("1.5s")?, Duration::from_millis(1500));
assert!(parse_duration("-1h").is_err());
```

### `format(ms_value, options)`

Format the given milliseconds as a string. Infallible — extreme values such as `i64::MIN` and `i64::MAX` are handled without panicking.

**Parameters:**
- `ms_value` (`i64`): Milliseconds to format
- `options` (`Option<Options>`): Use `Some(Options { long: true })` for verbose formatting

**Returns:**
- `String`: The formatted string

**Examples:**

```rust
use millis::{format, Options};

let s = format(60000, None);  // "1m"
let s = format(60000, Some(Options { long: true }));  // "1 minute"
let s = format(3600000, None);  // "1h"
let s = format(-3600000, None);  // "-1h"
let s = format(i64::MIN, None);  // "-292271023y" — no panic
```

### `parse_strict(value)` (deprecated)

Deprecated since 2.0.0 — use `parse()` instead. It is functionally equivalent to `parse()` and kept only for API compatibility.

### `Error`

All fallible functions return the `Error` enum:

```rust
#[non_exhaustive]
pub enum Error {
    Empty,         // the input string is empty
    TooLong,       // the input string is longer than 100 characters
    InvalidFormat, // the input is not a valid time string
    InvalidNumber, // the numeric part could not be parsed
    Overflow,      // the value does not fit in an i64 number of milliseconds
}
```

`Error` implements `std::error::Error`, `Display`, `Debug`, `Clone`, `Copy`, `PartialEq`, and `Eq`, so it composes with `?` and `Box<dyn std::error::Error>`. It is marked `#[non_exhaustive]`: new variants may be added in future versions, so `match` statements need a wildcard arm.

```rust
use millis::{parse, Error};

assert_eq!(parse(""), Err(Error::Empty));
assert_eq!(parse("10000000000y"), Err(Error::Overflow));
match parse("invalid") {
    Ok(v) => println!("parsed: {}", v),
    Err(Error::InvalidFormat) => println!("not a valid time string"),
    Err(e) => println!("error: {}", e),
}
```

### Import Options

```rust
// Import main function
use millis::ms;

// Import specific functions
use millis::{parse, format, parse_duration};

// Import types
use millis::{Error, Options};

// Import everything
use millis::{ms, parse, format, parse_duration, Error, Options};
```

## Supported Time Units

- `ms`, `msec`, `msecs`, `millisecond`, `milliseconds` - Milliseconds
- `s`, `sec`, `secs`, `second`, `seconds` - Seconds
- `m`, `min`, `mins`, `minute`, `minutes` - Minutes
- `h`, `hr`, `hrs`, `hour`, `hours` - Hours
- `d`, `day`, `days` - Days
- `w`, `week`, `weeks` - Weeks
- `mo`, `month`, `months` - Months (calculated as 1/12 of a year)
- `y`, `yr`, `yrs`, `year`, `years` - Years (calculated as 365.25 days)

### Case Insensitive

All units are case-insensitive, so `1D`, `1d`, `1 Day`, `1 DAY` are all equivalent.

## Features

- 🚀 Simple and intuitive API
- 📦 Zero runtime dependencies
- 🔄 Bidirectional conversion (string ↔ milliseconds)
- ⏱️ Supports negative time values
- ⏳ `parse_duration()` for `std::time::Duration` interop
- 📝 Long and short format options
- 🎯 Type-safe with Rust's type system and trait-based design
- ✅ Structured error handling with a dedicated `Error` enum
- 🛡️ Overflow-checked parsing; formatting never panics, even on `i64::MIN`/`i64::MAX`

## Common Use Cases

### Setting Timeouts

```rust
use std::thread;
use millis::parse_duration;

// Parse directly to Duration for thread::sleep()
let timeout = parse_duration("5s")?;
thread::sleep(timeout);
```

### Async Operations

```rust
use tokio::time::sleep;
use millis::parse_duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let delay = parse_duration("2s")?;
    sleep(delay).await;
    Ok(())
}
```

### Caching

```rust
use std::time::{SystemTime, UNIX_EPOCH};
use millis::ms;

// Set cache expiration
let cache_duration = ms("1h")?;
let now = SystemTime::now()
    .duration_since(UNIX_EPOCH)?
    .as_millis() as i64;
let expires_at = now + cache_duration;
```

### Rate Limiting

```rust
use millis::ms;

// Define rate limit window
let rate_limit_window = ms("1m")?;
let max_requests = 100;

println!("Allow {} requests per {} ms", max_requests, rate_limit_window);
```

### Calculating Durations

```rust
use millis::ms;

// Calculate time differences
let meeting_duration = ms("2h")? - ms("30m")?;  // 5400000 ms (1.5 hours)
println!("Meeting duration: {} ms", meeting_duration);
```

### Working with Configuration Files

```rust
use millis::{ms, Error};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Config {
    timeout_str: String,
}

fn get_timeout_ms(config: &Config) -> Result<i64, Error> {
    ms(config.timeout_str.as_str())  // Automatically parse "30s", "5m", etc.
}
```

## Error Handling

All parsing functions return `Result<_, millis::Error>`:

```rust
use millis::{ms, parse, Error};

// Invalid format
assert_eq!(ms("invalid"), Err(Error::InvalidFormat));

// Empty string
assert_eq!(ms(""), Err(Error::Empty));

// String too long (>100 characters)
assert_eq!(parse(&"1".repeat(101)), Err(Error::TooLong));

// Value too large for i64 milliseconds
assert_eq!(parse("10000000000y"), Err(Error::Overflow));

// Errors implement Display and std::error::Error
if let Err(e) = ms("oops") {
    eprintln!("Error: {}", e);
}
```

## Notes

### Precision

- **Month calculation**: 1 month = 1/12 year ≈ 30.44 days (average value)
- **Year calculation**: 1 year = 365.25 days (accounting for leap years)
- **Return type**: All parsed values are returned as `i64` (no decimal points)

### Rounding

When parsing, decimal values are rounded to the nearest integer millisecond:

```rust
use millis::parse;

assert_eq!(parse("1.5s")?, 1500);  // 1.5 * 1000 = 1500
assert_eq!(parse(".5ms")?, 1);     // 0.5 rounds to 1
```

When formatting, values are rounded to the nearest integer for the selected unit:

```rust
use millis::format;

assert_eq!(format(1500, None), "2s");   // rounded from 1.5s
assert_eq!(format(90000, None), "2m");  // rounded from 1.5m
```

### Overflow

Parsing checks that the result fits in an `i64` number of milliseconds and returns `Error::Overflow` otherwise. Formatting accepts the full `i64` range, including `i64::MIN` and `i64::MAX`, without panicking.

## Migrating from 1.x

- `parse()`, `ms()`, and `parse_strict()` now return `Result<i64, millis::Error>` instead of `Result<i64, String>`.
- `ms(i64)` now returns `String` directly instead of a `Result` — formatting is infallible.
- `parse_strict()` is deprecated; it behaves identically to `parse()`. Use `parse()` instead.
- `parse_duration()` is new: it parses straight to `std::time::Duration`.
- The `regex` dependency was removed; the crate now has zero runtime dependencies.

## 📜 License

[MIT](./LICENSE) License © 2025-PRESENT [wudi](https://github.com/WuChenDi)
