use millis::parse_strict;

#[test]
fn should_not_throw_error() {
    assert!(parse_strict("1m").is_ok());
}

#[test]
fn should_preserve_ms() {
    assert_eq!(parse_strict("100").unwrap(), 100);
}

#[test]
fn should_convert_from_m_to_ms() {
    assert_eq!(parse_strict("1m").unwrap(), 60000);
}

#[test]
fn should_convert_from_h_to_ms() {
    assert_eq!(parse_strict("1h").unwrap(), 3600000);
}

#[test]
fn should_convert_d_to_ms() {
    assert_eq!(parse_strict("2d").unwrap(), 172800000);
}

#[test]
fn should_convert_w_to_ms() {
    assert_eq!(parse_strict("3w").unwrap(), 1814400000);
}

#[test]
fn should_convert_s_to_ms() {
    assert_eq!(parse_strict("1s").unwrap(), 1000);
}

#[test]
fn should_convert_ms_to_ms() {
    assert_eq!(parse_strict("100ms").unwrap(), 100);
}

#[test]
fn should_convert_mo_to_ms() {
    assert_eq!(parse_strict("1mo").unwrap(), 2629800000);
}

#[test]
fn should_convert_y_to_ms() {
    assert_eq!(parse_strict("1y").unwrap(), 31557600000);
}

#[test]
fn should_work_with_decimals() {
    assert_eq!(parse_strict("1.5h").unwrap(), 5400000);
}

#[test]
fn should_work_with_multiple_spaces() {
    assert_eq!(parse_strict("1   s").unwrap(), 1000);
}

#[test]
fn should_return_error_if_invalid() {
    assert!(parse_strict("☃").is_err());
    assert!(parse_strict("10-.5").is_err());
    assert!(parse_strict("foo").is_err());
}

#[test]
fn should_be_case_insensitive() {
    assert_eq!(parse_strict("53 YeArS").unwrap(), 1672552800000);
    assert_eq!(parse_strict("53 WeEkS").unwrap(), 32054400000);
    assert_eq!(parse_strict("53 DaYS").unwrap(), 4579200000);
    assert_eq!(parse_strict("53 HoUrs").unwrap(), 190800000);
    assert_eq!(parse_strict("53 MiLliSeCondS").unwrap(), 53);
}

#[test]
fn should_work_with_numbers_starting_with_dot() {
    assert_eq!(parse_strict(".5ms").unwrap(), 1); // 0.5 rounds to 1
}

#[test]
fn should_work_with_negative_integers() {
    assert_eq!(parse_strict("-100ms").unwrap(), -100);
}

#[test]
fn should_work_with_negative_decimals() {
    assert_eq!(parse_strict("-1.5h").unwrap(), -5400000);
    assert_eq!(parse_strict("-10.5h").unwrap(), -37800000);
}

#[test]
fn should_work_with_negative_decimals_starting_with_dot() {
    assert_eq!(parse_strict("-.5h").unwrap(), -1800000);
}

// Long strings

#[test]
fn long_should_not_throw_error() {
    assert!(parse_strict("53 milliseconds").is_ok());
}

#[test]
fn long_should_convert_milliseconds() {
    assert_eq!(parse_strict("53 milliseconds").unwrap(), 53);
}

#[test]
fn long_should_convert_msecs() {
    assert_eq!(parse_strict("17 msecs").unwrap(), 17);
}

#[test]
fn long_should_convert_sec() {
    assert_eq!(parse_strict("1 sec").unwrap(), 1000);
}

#[test]
fn long_should_convert_min() {
    assert_eq!(parse_strict("1 min").unwrap(), 60000);
}

#[test]
fn long_should_convert_hr() {
    assert_eq!(parse_strict("1 hr").unwrap(), 3600000);
}

#[test]
fn long_should_convert_days() {
    assert_eq!(parse_strict("2 days").unwrap(), 172800000);
}

#[test]
fn long_should_convert_weeks() {
    assert_eq!(parse_strict("1 week").unwrap(), 604800000);
}

#[test]
fn long_should_convert_months() {
    assert_eq!(parse_strict("1 month").unwrap(), 2629800000);
}

#[test]
fn long_should_convert_years() {
    assert_eq!(parse_strict("1 year").unwrap(), 31557600000);
}

#[test]
fn long_should_work_with_decimals() {
    assert_eq!(parse_strict("1.5 hours").unwrap(), 5400000);
}

#[test]
fn long_should_work_with_negative_integers() {
    assert_eq!(parse_strict("-100 milliseconds").unwrap(), -100);
}

#[test]
fn long_should_work_with_negative_decimals() {
    assert_eq!(parse_strict("-1.5 hours").unwrap(), -5400000);
}

#[test]
fn long_should_work_with_negative_decimals_starting_with_dot() {
    assert_eq!(parse_strict("-.5 hr").unwrap(), -1800000);
}

// Invalid inputs

#[test]
fn should_throw_error_on_empty_string() {
    assert!(parse_strict("").is_err());
}

#[test]
fn should_throw_error_on_long_string() {
    let long_string = "▲".repeat(101);
    assert!(parse_strict(&long_string).is_err());
}
