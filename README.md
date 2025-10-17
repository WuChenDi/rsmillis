[![Crates.io](https://img.shields.io/crates/v/millis.svg)](https://crates.io/crates/millis)

# millis

A tiny Rust library that converts various time formats to milliseconds.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
millis = "1.0.2"
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

// Format milliseconds to strings
ms(60000)?            // "1m"
ms(2 * 60000)?        // "2m"
ms(-3 * 60000)?       // "-3m"
ms(172800000)?        // "2d"
```

### Long Format

For verbose output, use the `format()` function with options:

```rust
use millis::{format, Options};

format(60000, Some(Options { long: true }))           // "1 minute"
format(2 * 60000, Some(Options { long: true }))       // "2 minutes"
format(172800000, Some(Options { long: true }))       // "2 days"

// Combine parse and format
let ms_value = ms("10 hours")?;
format(ms_value, Some(Options { long: true }))        // "10 hours"
```

## API

### `ms(value)`

Parse or format the given value. The return type is automatically inferred based on the input type.

**Parameters:**
- `value`: Can be `&str`, `String` (to parse), or `i64` (to format)

**Returns:**
- `Result<i64, String>` when input is a string (parsing to milliseconds)
- `Result<String, String>` when input is a number (formatting to time string)

**Errors:**
- Returns `Err(String)` if the value cannot be parsed

**Examples:**

```rust
use millis::ms;

// Parse string to milliseconds
let milliseconds: i64 = ms("2h")?;
assert_eq!(milliseconds, 7200000);

let milliseconds = ms("1 day")?;
assert_eq!(milliseconds, 86400000);

// Format milliseconds to string
let formatted: String = ms(7200000)?;
assert_eq!(formatted, "2h");

let formatted = ms(60000)?;
assert_eq!(formatted, "1m");
```

### `parse(value)`

Parse the given string and return milliseconds.

**Parameters:**
- `value` (`&str`): A string to parse to milliseconds

**Returns:**
- `Result<i64, String>`: The parsed value in milliseconds

**Errors:**
- Returns `Err` if the string is empty, longer than 100 characters, or cannot be parsed

**Examples:**

```rust
use millis::parse;

let ms = parse("2h")?;           // 7200000
let ms = parse("1d")?;           // 86400000
let ms = parse("10 seconds")?;   // 10000
let ms = parse("-1h")?;          // -3600000
```

### `format(ms_value, options)`

Format the given milliseconds as a string.

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
```

### Import Options

```rust
// Import main function
use millis::ms;

// Import specific functions
use millis::{parse, format, parse_strict};

// Import types
use millis::Options;

// Import everything
use millis::{ms, parse, format, parse_strict, Options};
```

## Supported Time Units

### Short Format

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
- 🦀 Minimal dependencies (only `regex`)
- 🔄 Bidirectional conversion (string ↔ milliseconds)
- ⏱️ Supports negative time values
- 📝 Long and short format options
- 🎯 Type-safe with Rust's type system and trait-based design
- 🔥 Cached regex compilation for better performance
- ✅ Comprehensive error handling

## Common Use Cases

### Setting Timeouts

```rust
use std::thread;
use std::time::Duration;
use millis::ms;

// Convert to Duration for thread::sleep()
let timeout = ms("5s")?;
thread::sleep(Duration::from_millis(timeout as u64));
```

### Async Operations

```rust
use tokio::time::{sleep, Duration};
use millis::ms;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let delay = ms("2s")?;
    sleep(Duration::from_millis(delay as u64)).await;
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
use millis::ms;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Config {
    timeout_str: String,
}

fn get_timeout_ms(config: &Config) -> Result<i64, String> {
    ms(&config.timeout_str)  // Automatically parse "30s", "5m", etc.
}
```

## Error Handling

The library returns `Result` types for proper error handling:

```rust
use millis::ms;

// Invalid format
match ms("invalid") {
    Ok(_) => println!("Unexpected success"),
    Err(e) => println!("Error: {}", e),
}

// Empty string
match ms("") {
    Ok(_) => println!("Unexpected success"),
    Err(e) => println!("Error: {}", e),
}

// String too long (>100 characters)
match ms(&"a".repeat(101)) {
    Ok(_) => println!("Unexpected success"),
    Err(e) => println!("Error: {}", e),
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

## Advanced Usage

### Custom Type Conversion

The library uses the `ToMillis` trait for type conversion. You can extend it for custom types:

```rust
use millis::ToMillis;

struct Duration {
    value: i64,
}

// Note: The trait is used internally by the ms() function
// For custom types, use parse() and format() directly
impl Duration {
    fn from_str(s: &str) -> Result<Self, String> {
        Ok(Duration { value: s.to_millis()? })
    }
    
    fn to_string(&self) -> Result<String, String> {
        self.value.to_millis()
    }
}
```

## 📜 License

[MIT](./LICENSE) License © 2025-PRESENT [wudi](https://github.com/WuChenDi)
