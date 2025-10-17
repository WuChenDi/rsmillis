use millis::{format, Options};

// Short format tests

#[test]
fn should_not_panic() {
    let _ = format(500, None);
}

#[test]
fn should_support_milliseconds() {
    assert_eq!(format(500, None), "500ms");
    assert_eq!(format(-500, None), "-500ms");
}

#[test]
fn should_support_seconds() {
    assert_eq!(format(1000, None), "1s");
    assert_eq!(format(10000, None), "10s");
    assert_eq!(format(-1000, None), "-1s");
    assert_eq!(format(-10000, None), "-10s");
}

#[test]
fn should_support_minutes() {
    assert_eq!(format(60 * 1000, None), "1m");
    assert_eq!(format(60 * 10000, None), "10m");
    assert_eq!(format(-1 * 60 * 1000, None), "-1m");
    assert_eq!(format(-1 * 60 * 10000, None), "-10m");
}

#[test]
fn should_support_hours() {
    assert_eq!(format(60 * 60 * 1000, None), "1h");
    assert_eq!(format(60 * 60 * 10000, None), "10h");
    assert_eq!(format(-1 * 60 * 60 * 1000, None), "-1h");
    assert_eq!(format(-1 * 60 * 60 * 10000, None), "-10h");
}

#[test]
fn should_support_days() {
    assert_eq!(format(24 * 60 * 60 * 1000, None), "1d");
    assert_eq!(format(24 * 60 * 60 * 6000, None), "6d");
    assert_eq!(format(-1 * 24 * 60 * 60 * 1000, None), "-1d");
    assert_eq!(format(-1 * 24 * 60 * 60 * 6000, None), "-6d");
}

#[test]
fn should_support_weeks() {
    assert_eq!(format(7 * 24 * 60 * 60 * 1000, None), "1w");
    assert_eq!(format(2 * 7 * 24 * 60 * 60 * 1000, None), "2w");
    assert_eq!(format(-1 * 7 * 24 * 60 * 60 * 1000, None), "-1w");
    assert_eq!(format(-1 * 2 * 7 * 24 * 60 * 60 * 1000, None), "-2w");
}

#[test]
fn should_support_months() {
    // 30.4375 days = 2629800 seconds = 2629800000 ms
    assert_eq!(format(2629800000, None), "1mo");
    assert_eq!(format(3155760000, None), "1mo");
    assert_eq!(format(26298000000, None), "10mo");
    assert_eq!(format(-2629800000, None), "-1mo");
    assert_eq!(format(-3155760000, None), "-1mo");
    assert_eq!(format(-26298000000, None), "-10mo");
}

#[test]
fn should_support_years() {
    // 365.25 days = 31557600 seconds = 31557600000 ms
    assert_eq!(format(31557600001, None), "1y");
    assert_eq!(format(37869120001, None), "1y");
    assert_eq!(format(315576000001, None), "10y");
    assert_eq!(format(-31557600001, None), "-1y");
    assert_eq!(format(-37869120001, None), "-1y");
    assert_eq!(format(-315576000001, None), "-10y");
}

#[test]
fn should_round() {
    assert_eq!(format(234234234, None), "3d");
    assert_eq!(format(-234234234, None), "-3d");
}

// Long format tests

#[test]
fn long_should_not_panic() {
    let _ = format(500, Some(Options { long: true }));
}

#[test]
fn long_should_support_milliseconds() {
    assert_eq!(format(500, Some(Options { long: true })), "500 ms");
    assert_eq!(format(-500, Some(Options { long: true })), "-500 ms");
}

#[test]
fn long_should_support_seconds() {
    assert_eq!(format(1000, Some(Options { long: true })), "1 second");
    assert_eq!(format(1200, Some(Options { long: true })), "1 second");
    assert_eq!(format(10000, Some(Options { long: true })), "10 seconds");
    assert_eq!(format(-1000, Some(Options { long: true })), "-1 second");
    assert_eq!(format(-1200, Some(Options { long: true })), "-1 second");
    assert_eq!(format(-10000, Some(Options { long: true })), "-10 seconds");
}

#[test]
fn long_should_support_minutes() {
    assert_eq!(format(60 * 1000, Some(Options { long: true })), "1 minute");
    assert_eq!(format(60 * 1200, Some(Options { long: true })), "1 minute");
    assert_eq!(
        format(60 * 10000, Some(Options { long: true })),
        "10 minutes"
    );
    assert_eq!(
        format(-1 * 60 * 1000, Some(Options { long: true })),
        "-1 minute"
    );
    assert_eq!(
        format(-1 * 60 * 1200, Some(Options { long: true })),
        "-1 minute"
    );
    assert_eq!(
        format(-1 * 60 * 10000, Some(Options { long: true })),
        "-10 minutes"
    );
}

#[test]
fn long_should_support_hours() {
    assert_eq!(
        format(60 * 60 * 1000, Some(Options { long: true })),
        "1 hour"
    );
    assert_eq!(
        format(60 * 60 * 1200, Some(Options { long: true })),
        "1 hour"
    );
    assert_eq!(
        format(60 * 60 * 10000, Some(Options { long: true })),
        "10 hours"
    );
    assert_eq!(
        format(-1 * 60 * 60 * 1000, Some(Options { long: true })),
        "-1 hour"
    );
    assert_eq!(
        format(-1 * 60 * 60 * 1200, Some(Options { long: true })),
        "-1 hour"
    );
    assert_eq!(
        format(-1 * 60 * 60 * 10000, Some(Options { long: true })),
        "-10 hours"
    );
}

#[test]
fn long_should_support_days() {
    assert_eq!(
        format(24 * 60 * 60 * 1000, Some(Options { long: true })),
        "1 day"
    );
    assert_eq!(
        format(24 * 60 * 60 * 1200, Some(Options { long: true })),
        "1 day"
    );
    assert_eq!(
        format(6 * 24 * 60 * 60 * 1000, Some(Options { long: true })),
        "6 days"
    );
    assert_eq!(
        format(-1 * 24 * 60 * 60 * 1000, Some(Options { long: true })),
        "-1 day"
    );
    assert_eq!(
        format(-1 * 24 * 60 * 60 * 1200, Some(Options { long: true })),
        "-1 day"
    );
    assert_eq!(
        format(-1 * 6 * 24 * 60 * 60 * 1000, Some(Options { long: true })),
        "-6 days"
    );
}

#[test]
fn long_should_support_weeks() {
    assert_eq!(
        format(7 * 24 * 60 * 60 * 1000, Some(Options { long: true })),
        "1 week"
    );
    assert_eq!(
        format(2 * 7 * 24 * 60 * 60 * 1000, Some(Options { long: true })),
        "2 weeks"
    );
    assert_eq!(
        format(-1 * 7 * 24 * 60 * 60 * 1000, Some(Options { long: true })),
        "-1 week"
    );
    assert_eq!(
        format(-1 * 2 * 7 * 24 * 60 * 60 * 1000, Some(Options { long: true })),
        "-2 weeks"
    );
}

#[test]
fn long_should_support_months() {
    assert_eq!(
        format(2629800000, Some(Options { long: true })),
        "1 month"
    );
    assert_eq!(
        format(3155760000, Some(Options { long: true })),
        "1 month"
    );
    assert_eq!(
        format(26298000000, Some(Options { long: true })),
        "10 months"
    );
    assert_eq!(
        format(-2629800000, Some(Options { long: true })),
        "-1 month"
    );
    assert_eq!(
        format(-3155760000, Some(Options { long: true })),
        "-1 month"
    );
    assert_eq!(
        format(-26298000000, Some(Options { long: true })),
        "-10 months"
    );
}

#[test]
fn long_should_support_years() {
    assert_eq!(
        format(31557600001, Some(Options { long: true })),
        "1 year"
    );
    assert_eq!(
        format(37869120001, Some(Options { long: true })),
        "1 year"
    );
    assert_eq!(
        format(315576000001, Some(Options { long: true })),
        "10 years"
    );
    assert_eq!(
        format(-31557600001, Some(Options { long: true })),
        "-1 year"
    );
    assert_eq!(
        format(-37869120001, Some(Options { long: true })),
        "-1 year"
    );
    assert_eq!(
        format(-315576000001, Some(Options { long: true })),
        "-10 years"
    );
}

#[test]
fn long_should_round() {
    assert_eq!(format(234234234, Some(Options { long: true })), "3 days");
    assert_eq!(format(-234234234, Some(Options { long: true })), "-3 days");
}

// Invalid inputs (should panic)

// #[test]
// #[should_panic]
// fn should_throw_error_on_nan() {
//     format(f64::NAN, None);
// }

// #[test]
// #[should_panic]
// fn should_throw_error_on_infinity() {
//     format(f64::INFINITY, None);
// }

// #[test]
// #[should_panic]
// fn should_throw_error_on_neg_infinity() {
//     format(f64::NEG_INFINITY, None);
// }
