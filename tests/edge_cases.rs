//! Edge-case regression tests for the 2.0.0 release.

use millis::{Error, Options, format, parse, parse_duration};
use std::time::Duration;

#[test]
fn test_format_extreme_values_do_not_panic() {
    // Short form
    assert_eq!(format(i64::MIN, None), "-292271023y");
    assert_eq!(format(i64::MAX, None), "292271023y");

    // Long form
    assert_eq!(
        format(i64::MIN, Some(Options { long: true })),
        "-292271023 years"
    );
    assert_eq!(
        format(i64::MAX, Some(Options { long: true })),
        "292271023 years"
    );
}

#[test]
fn test_parse_overflow() {
    assert!(matches!(parse("10000000000y"), Err(Error::Overflow)));
}

#[test]
fn test_parse_empty() {
    assert!(matches!(parse(""), Err(Error::Empty)));
}

#[test]
fn test_parse_too_long() {
    let input = "1".repeat(101);
    assert!(matches!(parse(&input), Err(Error::TooLong)));
}

#[test]
fn test_parse_invalid_input() {
    // No leading digits, so the parser rejects the format before
    // attempting numeric conversion.
    assert!(matches!(parse("invalid"), Err(Error::InvalidFormat)));
}

#[test]
fn test_parse_duration_negative_is_err() {
    assert!(parse_duration("-1s").is_err());
}

#[test]
fn test_parse_duration_fractional_seconds() {
    assert_eq!(parse_duration("1.5s").unwrap(), Duration::from_millis(1500));
}
