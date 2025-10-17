use millis::{Options, format};

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================================
    // Test format(number, long=True)
    // ============================================================================

    mod test_format_long {
        use super::*;

        #[test]
        fn should_not_throw_an_error() {
            // should not throw an error
            let _ = format(500, Some(Options { long: true }));
        }

        #[test]
        fn should_support_milliseconds() {
            // should support milliseconds
            assert_eq!(format(500, Some(Options { long: true })), "500 ms");
            assert_eq!(format(-500, Some(Options { long: true })), "-500 ms");
        }

        #[test]
        fn should_support_seconds() {
            // should support seconds
            assert_eq!(format(1000, Some(Options { long: true })), "1 second");
            assert_eq!(format(1200, Some(Options { long: true })), "1 second");
            assert_eq!(format(10000, Some(Options { long: true })), "10 seconds");

            assert_eq!(format(-1000, Some(Options { long: true })), "-1 second");
            assert_eq!(format(-1200, Some(Options { long: true })), "-1 second");
            assert_eq!(format(-10000, Some(Options { long: true })), "-10 seconds");
        }

        #[test]
        fn should_support_minutes() {
            // should support minutes
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
        fn should_support_hours() {
            // should support hours
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
        fn should_support_days() {
            // should support days
            assert_eq!(
                format(1 * 24 * 60 * 60 * 1000, Some(Options { long: true })),
                "1 day"
            );
            assert_eq!(
                format(1 * 24 * 60 * 60 * 1200, Some(Options { long: true })),
                "1 day"
            );
            assert_eq!(
                format(6 * 24 * 60 * 60 * 1000, Some(Options { long: true })),
                "6 days"
            );

            assert_eq!(
                format(-1 * 1 * 24 * 60 * 60 * 1000, Some(Options { long: true })),
                "-1 day"
            );
            assert_eq!(
                format(-1 * 1 * 24 * 60 * 60 * 1200, Some(Options { long: true })),
                "-1 day"
            );
            assert_eq!(
                format(-1 * 6 * 24 * 60 * 60 * 1000, Some(Options { long: true })),
                "-6 days"
            );
        }

        #[test]
        fn should_support_weeks() {
            // should support weeks
            assert_eq!(
                format(1 * 7 * 24 * 60 * 60 * 1000, Some(Options { long: true })),
                "1 week"
            );
            assert_eq!(
                format(2 * 7 * 24 * 60 * 60 * 1000, Some(Options { long: true })),
                "2 weeks"
            );

            assert_eq!(
                format(
                    -1 * 1 * 7 * 24 * 60 * 60 * 1000,
                    Some(Options { long: true })
                ),
                "-1 week"
            );
            assert_eq!(
                format(
                    -1 * 2 * 7 * 24 * 60 * 60 * 1000,
                    Some(Options { long: true })
                ),
                "-2 weeks"
            );
        }

        #[test]
        fn should_support_months() {
            // should support months
            let one_month = (30.4375 * 24.0 * 60.0 * 60.0 * 1000.0) as i64;
            assert_eq!(format(one_month, Some(Options { long: true })), "1 month");
            assert_eq!(
                format(
                    (30.4375 * 24.0 * 60.0 * 60.0 * 1200.0) as i64,
                    Some(Options { long: true })
                ),
                "1 month"
            );
            assert_eq!(
                format(
                    (30.4375 * 24.0 * 60.0 * 60.0 * 10000.0) as i64,
                    Some(Options { long: true })
                ),
                "10 months"
            );

            assert_eq!(format(-one_month, Some(Options { long: true })), "-1 month");
            assert_eq!(
                format(
                    -(30.4375 * 24.0 * 60.0 * 60.0 * 1200.0) as i64,
                    Some(Options { long: true })
                ),
                "-1 month"
            );
            assert_eq!(
                format(
                    -(30.4375 * 24.0 * 60.0 * 60.0 * 10000.0) as i64,
                    Some(Options { long: true })
                ),
                "-10 months"
            );
        }

        #[test]
        fn should_support_years() {
            // should support years
            let one_year = (365.25 * 24.0 * 60.0 * 60.0 * 1000.0) as i64 + 1;
            assert_eq!(format(one_year, Some(Options { long: true })), "1 year");
            assert_eq!(
                format(
                    (365.25 * 24.0 * 60.0 * 60.0 * 1200.0) as i64 + 1,
                    Some(Options { long: true })
                ),
                "1 year"
            );
            assert_eq!(
                format(
                    (365.25 * 24.0 * 60.0 * 60.0 * 10000.0) as i64 + 1,
                    Some(Options { long: true })
                ),
                "10 years"
            );

            assert_eq!(format(-one_year, Some(Options { long: true })), "-1 year");
            assert_eq!(
                format(
                    -((365.25 * 24.0 * 60.0 * 60.0 * 1200.0) as i64 + 1),
                    Some(Options { long: true })
                ),
                "-1 year"
            );
            assert_eq!(
                format(
                    -((365.25 * 24.0 * 60.0 * 60.0 * 10000.0) as i64 + 1),
                    Some(Options { long: true })
                ),
                "-10 years"
            );
        }

        #[test]
        fn should_round() {
            // should round
            assert_eq!(format(234234234, Some(Options { long: true })), "3 days");
            assert_eq!(format(-234234234, Some(Options { long: true })), "-3 days");
        }
    }

    // ============================================================================
    // Test format(number) - short format
    // ============================================================================

    mod test_format_short {
        use super::*;

        #[test]
        fn should_not_throw_an_error() {
            // should not throw an error
            let _ = format(500, None);
        }

        #[test]
        fn should_support_milliseconds() {
            // should support milliseconds
            assert_eq!(format(500, None), "500ms");
            assert_eq!(format(-500, None), "-500ms");
        }

        #[test]
        fn should_support_seconds() {
            // should support seconds
            assert_eq!(format(1000, None), "1s");
            assert_eq!(format(10000, None), "10s");

            assert_eq!(format(-1000, None), "-1s");
            assert_eq!(format(-10000, None), "-10s");
        }

        #[test]
        fn should_support_minutes() {
            // should support minutes
            assert_eq!(format(60 * 1000, None), "1m");
            assert_eq!(format(60 * 10000, None), "10m");

            assert_eq!(format(-1 * 60 * 1000, None), "-1m");
            assert_eq!(format(-1 * 60 * 10000, None), "-10m");
        }

        #[test]
        fn should_support_hours() {
            // should support hours
            assert_eq!(format(60 * 60 * 1000, None), "1h");
            assert_eq!(format(60 * 60 * 10000, None), "10h");

            assert_eq!(format(-1 * 60 * 60 * 1000, None), "-1h");
            assert_eq!(format(-1 * 60 * 60 * 10000, None), "-10h");
        }

        #[test]
        fn should_support_days() {
            // should support days
            assert_eq!(format(24 * 60 * 60 * 1000, None), "1d");
            assert_eq!(format(24 * 60 * 60 * 6000, None), "6d");

            assert_eq!(format(-1 * 24 * 60 * 60 * 1000, None), "-1d");
            assert_eq!(format(-1 * 24 * 60 * 60 * 6000, None), "-6d");
        }

        #[test]
        fn should_support_weeks() {
            // should support weeks
            assert_eq!(format(1 * 7 * 24 * 60 * 60 * 1000, None), "1w");
            assert_eq!(format(2 * 7 * 24 * 60 * 60 * 1000, None), "2w");

            assert_eq!(format(-1 * 1 * 7 * 24 * 60 * 60 * 1000, None), "-1w");
            assert_eq!(format(-1 * 2 * 7 * 24 * 60 * 60 * 1000, None), "-2w");
        }

        #[test]
        fn should_support_months() {
            // should support months
            let one_month = (30.4375 * 24.0 * 60.0 * 60.0 * 1000.0) as i64;
            assert_eq!(format(one_month, None), "1mo");
            assert_eq!(
                format((30.4375 * 24.0 * 60.0 * 60.0 * 1200.0) as i64, None),
                "1mo"
            );
            assert_eq!(
                format((30.4375 * 24.0 * 60.0 * 60.0 * 10000.0) as i64, None),
                "10mo"
            );

            assert_eq!(format(-one_month, None), "-1mo");
            assert_eq!(
                format(-(30.4375 * 24.0 * 60.0 * 60.0 * 1200.0) as i64, None),
                "-1mo"
            );
            assert_eq!(
                format(-(30.4375 * 24.0 * 60.0 * 60.0 * 10000.0) as i64, None),
                "-10mo"
            );
        }

        #[test]
        fn should_support_years() {
            // should support years
            let one_year = (365.25 * 24.0 * 60.0 * 60.0 * 1000.0) as i64 + 1;
            assert_eq!(format(one_year, None), "1y");
            assert_eq!(
                format((365.25 * 24.0 * 60.0 * 60.0 * 1200.0) as i64 + 1, None),
                "1y"
            );
            assert_eq!(
                format((365.25 * 24.0 * 60.0 * 60.0 * 10000.0) as i64 + 1, None),
                "10y"
            );

            assert_eq!(format(-one_year, None), "-1y");
            assert_eq!(
                format(-((365.25 * 24.0 * 60.0 * 60.0 * 1200.0) as i64 + 1), None),
                "-1y"
            );
            assert_eq!(
                format(-((365.25 * 24.0 * 60.0 * 60.0 * 10000.0) as i64 + 1), None),
                "-10y"
            );
        }

        #[test]
        fn should_round() {
            // should round
            assert_eq!(format(234234234, None), "3d");
            assert_eq!(format(-234234234, None), "-3d");
        }
    }
}
