use millis::{Options, Value, ms};

// String input tests

#[test]
fn string_should_not_error() {
    assert!(ms(Value::String("1m".to_string()), None).is_ok());
}

#[test]
fn string_should_preserve_ms() {
    match ms(Value::String("100".to_string()), None).unwrap() {
        Value::Number(n) => assert_eq!(n, 100),
        _ => panic!("Expected number"),
    }
}

#[test]
fn string_should_convert_from_m_to_ms() {
    match ms(Value::String("1m".to_string()), None).unwrap() {
        Value::Number(n) => assert_eq!(n, 60000),
        _ => panic!("Expected number"),
    }
}

#[test]
fn string_should_convert_from_h_to_ms() {
    match ms(Value::String("1h".to_string()), None).unwrap() {
        Value::Number(n) => assert_eq!(n, 3600000),
        _ => panic!("Expected number"),
    }
}

#[test]
fn string_should_convert_d_to_ms() {
    match ms(Value::String("2d".to_string()), None).unwrap() {
        Value::Number(n) => assert_eq!(n, 172800000),
        _ => panic!("Expected number"),
    }
}

#[test]
fn string_should_convert_w_to_ms() {
    match ms(Value::String("3w".to_string()), None).unwrap() {
        Value::Number(n) => assert_eq!(n, 1814400000),
        _ => panic!("Expected number"),
    }
}

#[test]
fn string_should_convert_s_to_ms() {
    match ms(Value::String("1s".to_string()), None).unwrap() {
        Value::Number(n) => assert_eq!(n, 1000),
        _ => panic!("Expected number"),
    }
}

#[test]
fn string_should_convert_ms_to_ms() {
    match ms(Value::String("100ms".to_string()), None).unwrap() {
        Value::Number(n) => assert_eq!(n, 100),
        _ => panic!("Expected number"),
    }
}

#[test]
fn string_should_convert_y_to_ms() {
    match ms(Value::String("1y".to_string()), None).unwrap() {
        Value::Number(n) => assert_eq!(n, 31557600000),
        _ => panic!("Expected number"),
    }
}

#[test]
fn string_should_work_with_decimals() {
    match ms(Value::String("1.5h".to_string()), None).unwrap() {
        Value::Number(n) => assert_eq!(n, 5400000),
        _ => panic!("Expected number"),
    }
}

#[test]
fn string_should_work_with_multiple_spaces() {
    match ms(Value::String("1   s".to_string()), None).unwrap() {
        Value::Number(n) => assert_eq!(n, 1000),
        _ => panic!("Expected number"),
    }
}

#[test]
fn string_should_return_error_if_invalid() {
    assert!(ms(Value::String("☃".to_string()), None).is_err());
    assert!(ms(Value::String("10-.5".to_string()), None).is_err());
    assert!(ms(Value::String("ms".to_string()), None).is_err());
}

#[test]
fn string_should_be_case_insensitive() {
    match ms(Value::String("1.5H".to_string()), None).unwrap() {
        Value::Number(n) => assert_eq!(n, 5400000),
        _ => panic!("Expected number"),
    }
}

#[test]
fn string_should_work_with_numbers_starting_with_dot() {
    match ms(Value::String(".5ms".to_string()), None).unwrap() {
        Value::Number(n) => assert_eq!(n, 1), // 0.5 rounds to 1
        _ => panic!("Expected number"),
    }
}

#[test]
fn string_should_work_with_negative_integers() {
    match ms(Value::String("-100ms".to_string()), None).unwrap() {
        Value::Number(n) => assert_eq!(n, -100),
        _ => panic!("Expected number"),
    }
}

#[test]
fn string_should_work_with_negative_decimals() {
    match ms(Value::String("-1.5h".to_string()), None).unwrap() {
        Value::Number(n) => assert_eq!(n, -5400000),
        _ => panic!("Expected number"),
    }
    match ms(Value::String("-10.5h".to_string()), None).unwrap() {
        Value::Number(n) => assert_eq!(n, -37800000),
        _ => panic!("Expected number"),
    }
}

#[test]
fn string_should_work_with_negative_decimals_starting_with_dot() {
    match ms(Value::String("-.5h".to_string()), None).unwrap() {
        Value::Number(n) => assert_eq!(n, -1800000),
        _ => panic!("Expected number"),
    }
}

// Long strings

#[test]
fn long_string_should_not_error() {
    assert!(ms(Value::String("53 milliseconds".to_string()), None).is_ok());
}

#[test]
fn long_string_should_convert_milliseconds() {
    match ms(Value::String("53 milliseconds".to_string()), None).unwrap() {
        Value::Number(n) => assert_eq!(n, 53),
        _ => panic!("Expected number"),
    }
}

#[test]
fn long_string_should_convert_msecs() {
    match ms(Value::String("17 msecs".to_string()), None).unwrap() {
        Value::Number(n) => assert_eq!(n, 17),
        _ => panic!("Expected number"),
    }
}

#[test]
fn long_string_should_convert_sec() {
    match ms(Value::String("1 sec".to_string()), None).unwrap() {
        Value::Number(n) => assert_eq!(n, 1000),
        _ => panic!("Expected number"),
    }
}

#[test]
fn long_string_should_convert_min() {
    match ms(Value::String("1 min".to_string()), None).unwrap() {
        Value::Number(n) => assert_eq!(n, 60000),
        _ => panic!("Expected number"),
    }
}

#[test]
fn long_string_should_convert_hr() {
    match ms(Value::String("1 hr".to_string()), None).unwrap() {
        Value::Number(n) => assert_eq!(n, 3600000),
        _ => panic!("Expected number"),
    }
}

#[test]
fn long_string_should_convert_days() {
    match ms(Value::String("2 days".to_string()), None).unwrap() {
        Value::Number(n) => assert_eq!(n, 172800000),
        _ => panic!("Expected number"),
    }
}

#[test]
fn long_string_should_convert_weeks() {
    match ms(Value::String("1 week".to_string()), None).unwrap() {
        Value::Number(n) => assert_eq!(n, 604800000),
        _ => panic!("Expected number"),
    }
}

#[test]
fn long_string_should_convert_years() {
    match ms(Value::String("1 year".to_string()), None).unwrap() {
        Value::Number(n) => assert_eq!(n, 31557600000),
        _ => panic!("Expected number"),
    }
}

#[test]
fn long_string_should_work_with_decimals() {
    match ms(Value::String("1.5 hours".to_string()), None).unwrap() {
        Value::Number(n) => assert_eq!(n, 5400000),
        _ => panic!("Expected number"),
    }
}

#[test]
fn long_string_should_work_with_negative_integers() {
    match ms(Value::String("-100 milliseconds".to_string()), None).unwrap() {
        Value::Number(n) => assert_eq!(n, -100),
        _ => panic!("Expected number"),
    }
}

#[test]
fn long_string_should_work_with_negative_decimals() {
    match ms(Value::String("-1.5 hours".to_string()), None).unwrap() {
        Value::Number(n) => assert_eq!(n, -5400000),
        _ => panic!("Expected number"),
    }
}

#[test]
fn long_string_should_work_with_negative_decimals_starting_with_dot() {
    match ms(Value::String("-.5 hr".to_string()), None).unwrap() {
        Value::Number(n) => assert_eq!(n, -1800000),
        _ => panic!("Expected number"),
    }
}

// Number with long option

#[test]
fn number_long_should_not_error() {
    assert!(ms(Value::Number(500), Some(Options { long: true })).is_ok());
}

#[test]
fn number_long_should_support_milliseconds() {
    match ms(Value::Number(500), Some(Options { long: true })).unwrap() {
        Value::String(s) => assert_eq!(s, "500 ms"),
        _ => panic!("Expected string"),
    }
    match ms(Value::Number(-500), Some(Options { long: true })).unwrap() {
        Value::String(s) => assert_eq!(s, "-500 ms"),
        _ => panic!("Expected string"),
    }
}

#[test]
fn number_long_should_support_seconds() {
    match ms(Value::Number(1000), Some(Options { long: true })).unwrap() {
        Value::String(s) => assert_eq!(s, "1 second"),
        _ => panic!("Expected string"),
    }
    match ms(Value::Number(1200), Some(Options { long: true })).unwrap() {
        Value::String(s) => assert_eq!(s, "1 second"),
        _ => panic!("Expected string"),
    }
    match ms(Value::Number(10000), Some(Options { long: true })).unwrap() {
        Value::String(s) => assert_eq!(s, "10 seconds"),
        _ => panic!("Expected string"),
    }
    match ms(Value::Number(-1000), Some(Options { long: true })).unwrap() {
        Value::String(s) => assert_eq!(s, "-1 second"),
        _ => panic!("Expected string"),
    }
    match ms(Value::Number(-1200), Some(Options { long: true })).unwrap() {
        Value::String(s) => assert_eq!(s, "-1 second"),
        _ => panic!("Expected string"),
    }
    match ms(Value::Number(-10000), Some(Options { long: true })).unwrap() {
        Value::String(s) => assert_eq!(s, "-10 seconds"),
        _ => panic!("Expected string"),
    }
}

#[test]
fn number_long_should_support_minutes() {
    match ms(Value::Number(60 * 1000), Some(Options { long: true })).unwrap() {
        Value::String(s) => assert_eq!(s, "1 minute"),
        _ => panic!("Expected string"),
    }
    match ms(Value::Number(60 * 1200), Some(Options { long: true })).unwrap() {
        Value::String(s) => assert_eq!(s, "1 minute"),
        _ => panic!("Expected string"),
    }
    match ms(Value::Number(60 * 10000), Some(Options { long: true })).unwrap() {
        Value::String(s) => assert_eq!(s, "10 minutes"),
        _ => panic!("Expected string"),
    }
}

#[test]
fn number_long_should_support_hours() {
    match ms(Value::Number(60 * 60 * 1000), Some(Options { long: true })).unwrap() {
        Value::String(s) => assert_eq!(s, "1 hour"),
        _ => panic!("Expected string"),
    }
    match ms(Value::Number(60 * 60 * 1200), Some(Options { long: true })).unwrap() {
        Value::String(s) => assert_eq!(s, "1 hour"),
        _ => panic!("Expected string"),
    }
    match ms(Value::Number(60 * 60 * 10000), Some(Options { long: true })).unwrap() {
        Value::String(s) => assert_eq!(s, "10 hours"),
        _ => panic!("Expected string"),
    }
}

#[test]
fn number_long_should_support_days() {
    match ms(
        Value::Number(24 * 60 * 60 * 1000),
        Some(Options { long: true }),
    )
    .unwrap()
    {
        Value::String(s) => assert_eq!(s, "1 day"),
        _ => panic!("Expected string"),
    }
    match ms(
        Value::Number(24 * 60 * 60 * 1200),
        Some(Options { long: true }),
    )
    .unwrap()
    {
        Value::String(s) => assert_eq!(s, "1 day"),
        _ => panic!("Expected string"),
    }
    match ms(
        Value::Number(6 * 24 * 60 * 60 * 1000),
        Some(Options { long: true }),
    )
    .unwrap()
    {
        Value::String(s) => assert_eq!(s, "6 days"),
        _ => panic!("Expected string"),
    }
}

#[test]
fn number_long_should_support_weeks() {
    match ms(
        Value::Number(7 * 24 * 60 * 60 * 1000),
        Some(Options { long: true }),
    )
    .unwrap()
    {
        Value::String(s) => assert_eq!(s, "1 week"),
        _ => panic!("Expected string"),
    }
    match ms(
        Value::Number(2 * 7 * 24 * 60 * 60 * 1000),
        Some(Options { long: true }),
    )
    .unwrap()
    {
        Value::String(s) => assert_eq!(s, "2 weeks"),
        _ => panic!("Expected string"),
    }
}

#[test]
fn number_long_should_support_months() {
    // 30.4375 days = 2629800 seconds = 2629800000 ms
    match ms(Value::Number(2629800000), Some(Options { long: true })).unwrap() {
        Value::String(s) => assert_eq!(s, "1 month"),
        _ => panic!("Expected string"),
    }
    match ms(Value::Number(3155760000), Some(Options { long: true })).unwrap() {
        Value::String(s) => assert_eq!(s, "1 month"),
        _ => panic!("Expected string"),
    }
    match ms(Value::Number(26298000000), Some(Options { long: true })).unwrap() {
        Value::String(s) => assert_eq!(s, "10 months"),
        _ => panic!("Expected string"),
    }
}

#[test]
fn number_long_should_support_years() {
    // 365.25 days = 31557600 seconds = 31557600000 ms
    match ms(Value::Number(31557600001), Some(Options { long: true })).unwrap() {
        Value::String(s) => assert_eq!(s, "1 year"),
        _ => panic!("Expected string"),
    }
    match ms(Value::Number(37869120001), Some(Options { long: true })).unwrap() {
        Value::String(s) => assert_eq!(s, "1 year"),
        _ => panic!("Expected string"),
    }
    match ms(Value::Number(315576000001), Some(Options { long: true })).unwrap() {
        Value::String(s) => assert_eq!(s, "10 years"),
        _ => panic!("Expected string"),
    }
}

#[test]
fn number_long_should_round() {
    match ms(Value::Number(234234234), Some(Options { long: true })).unwrap() {
        Value::String(s) => assert_eq!(s, "3 days"),
        _ => panic!("Expected string"),
    }
    match ms(Value::Number(-234234234), Some(Options { long: true })).unwrap() {
        Value::String(s) => assert_eq!(s, "-3 days"),
        _ => panic!("Expected string"),
    }
}

// Number without option (short format)

#[test]
fn number_should_not_error() {
    assert!(ms(Value::Number(500), None).is_ok());
}

#[test]
fn number_should_support_milliseconds() {
    match ms(Value::Number(500), None).unwrap() {
        Value::String(s) => assert_eq!(s, "500ms"),
        _ => panic!("Expected string"),
    }
    match ms(Value::Number(-500), None).unwrap() {
        Value::String(s) => assert_eq!(s, "-500ms"),
        _ => panic!("Expected string"),
    }
}

#[test]
fn number_should_support_seconds() {
    match ms(Value::Number(1000), None).unwrap() {
        Value::String(s) => assert_eq!(s, "1s"),
        _ => panic!("Expected string"),
    }
    match ms(Value::Number(10000), None).unwrap() {
        Value::String(s) => assert_eq!(s, "10s"),
        _ => panic!("Expected string"),
    }
    match ms(Value::Number(-1000), None).unwrap() {
        Value::String(s) => assert_eq!(s, "-1s"),
        _ => panic!("Expected string"),
    }
    match ms(Value::Number(-10000), None).unwrap() {
        Value::String(s) => assert_eq!(s, "-10s"),
        _ => panic!("Expected string"),
    }
}

#[test]
fn number_should_support_minutes() {
    match ms(Value::Number(60 * 1000), None).unwrap() {
        Value::String(s) => assert_eq!(s, "1m"),
        _ => panic!("Expected string"),
    }
    match ms(Value::Number(60 * 10000), None).unwrap() {
        Value::String(s) => assert_eq!(s, "10m"),
        _ => panic!("Expected string"),
    }
}

#[test]
fn number_should_support_hours() {
    match ms(Value::Number(60 * 60 * 1000), None).unwrap() {
        Value::String(s) => assert_eq!(s, "1h"),
        _ => panic!("Expected string"),
    }
    match ms(Value::Number(60 * 60 * 10000), None).unwrap() {
        Value::String(s) => assert_eq!(s, "10h"),
        _ => panic!("Expected string"),
    }
}

#[test]
fn number_should_support_days() {
    match ms(Value::Number(24 * 60 * 60 * 1000), None).unwrap() {
        Value::String(s) => assert_eq!(s, "1d"),
        _ => panic!("Expected string"),
    }
    match ms(Value::Number(24 * 60 * 60 * 6000), None).unwrap() {
        Value::String(s) => assert_eq!(s, "6d"),
        _ => panic!("Expected string"),
    }
}

#[test]
fn number_should_support_weeks() {
    match ms(Value::Number(7 * 24 * 60 * 60 * 1000), None).unwrap() {
        Value::String(s) => assert_eq!(s, "1w"),
        _ => panic!("Expected string"),
    }
    match ms(Value::Number(2 * 7 * 24 * 60 * 60 * 1000), None).unwrap() {
        Value::String(s) => assert_eq!(s, "2w"),
        _ => panic!("Expected string"),
    }
}

#[test]
fn number_should_support_months() {
    match ms(Value::Number(2629800000), None).unwrap() {
        Value::String(s) => assert_eq!(s, "1mo"),
        _ => panic!("Expected string"),
    }
    match ms(Value::Number(3155760000), None).unwrap() {
        Value::String(s) => assert_eq!(s, "1mo"),
        _ => panic!("Expected string"),
    }
    match ms(Value::Number(26298000000), None).unwrap() {
        Value::String(s) => assert_eq!(s, "10mo"),
        _ => panic!("Expected string"),
    }
}

#[test]
fn number_should_support_years() {
    match ms(Value::Number(31557600001), None).unwrap() {
        Value::String(s) => assert_eq!(s, "1y"),
        _ => panic!("Expected string"),
    }
    match ms(Value::Number(37869120001), None).unwrap() {
        Value::String(s) => assert_eq!(s, "1y"),
        _ => panic!("Expected string"),
    }
    match ms(Value::Number(315576000001), None).unwrap() {
        Value::String(s) => assert_eq!(s, "10y"),
        _ => panic!("Expected string"),
    }
}

#[test]
fn number_should_round() {
    match ms(Value::Number(234234234), None).unwrap() {
        Value::String(s) => assert_eq!(s, "3d"),
        _ => panic!("Expected string"),
    }
    match ms(Value::Number(-234234234), None).unwrap() {
        Value::String(s) => assert_eq!(s, "-3d"),
        _ => panic!("Expected string"),
    }
}

// Invalid inputs

#[test]
fn should_throw_error_on_empty_string() {
    assert!(ms(Value::String("".to_string()), None).is_err());
}

#[test]
fn should_throw_error_on_undefined() {
    // Rust doesn't have undefined, but we test with invalid Value types
    // This is implicitly tested by the type system
}

// #[test]
// fn should_throw_error_on_nan() {
//     assert!(ms(Value::Number(f64::NAN), None).is_err());
// }

// #[test]
// fn should_throw_error_on_infinity() {
//     assert!(ms(Value::Number(f64::INFINITY), None).is_err());
// }

// #[test]
// fn should_throw_error_on_neg_infinity() {
//     assert!(ms(Value::Number(f64::NEG_INFINITY), None).is_err());
// }
