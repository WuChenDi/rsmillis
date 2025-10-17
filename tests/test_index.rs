use millis::ms;

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================================
    // Test ms(string)
    // ============================================================================

    mod test_ms_string {
        use super::*;

        #[test]
        fn should_not_throw_an_error() {
            // should not throw an error
            let _ = ms("1m");
        }

        #[test]
        fn should_preserve_ms() {
            // should preserve ms
            assert_eq!(ms("100").unwrap(), 100);
        }

        #[test]
        fn should_convert_from_m_to_ms() {
            // should convert from m to ms
            assert_eq!(ms("1m").unwrap(), 60000);
        }

        #[test]
        fn should_convert_from_h_to_ms() {
            // should convert from h to ms
            assert_eq!(ms("1h").unwrap(), 3600000);
        }

        #[test]
        fn should_convert_d_to_ms() {
            // should convert d to ms
            assert_eq!(ms("2d").unwrap(), 172800000);
        }

        #[test]
        fn should_convert_w_to_ms() {
            // should convert w to ms
            assert_eq!(ms("3w").unwrap(), 1814400000);
        }

        #[test]
        fn should_convert_s_to_ms() {
            // should convert s to ms
            assert_eq!(ms("1s").unwrap(), 1000);
        }

        #[test]
        fn should_convert_ms_to_ms() {
            // should convert ms to ms
            assert_eq!(ms("100ms").unwrap(), 100);
        }

        #[test]
        fn should_convert_y_to_ms() {
            // should convert y to ms
            assert_eq!(ms("1y").unwrap(), 31557600000);
        }

        #[test]
        fn should_work_with_decimals() {
            // should work with decimals
            assert_eq!(ms("1.5h").unwrap(), 5400000);
        }

        #[test]
        fn should_work_with_multiple_spaces() {
            // should work with multiple spaces
            assert_eq!(ms("1   s").unwrap(), 1000);
        }

        #[test]
        fn should_return_error_if_invalid() {
            // should return error if invalid
            assert!(ms("☃").is_err());
            assert!(ms("10-.5").is_err());
            assert!(ms("ms").is_err());
        }

        #[test]
        fn should_be_case_insensitive() {
            // should be case-insensitive
            assert_eq!(ms("1.5H").unwrap(), 5400000);
        }

        // #[test]
        // fn should_work_with_numbers_starting_with_dot() {
        //     // should work with numbers starting with .
        //     assert_eq!(ms(".5ms").unwrap(), 0); // rounds to 0
        // }

        #[test]
        fn should_work_with_negative_integers() {
            // should work with negative integers
            assert_eq!(ms("-100ms").unwrap(), -100);
        }

        #[test]
        fn should_work_with_negative_decimals() {
            // should work with negative decimals
            assert_eq!(ms("-1.5h").unwrap(), -5400000);
            assert_eq!(ms("-10.5h").unwrap(), -37800000);
        }

        #[test]
        fn should_work_with_negative_decimals_starting_with_dot() {
            // should work with negative decimals starting with "."
            assert_eq!(ms("-.5h").unwrap(), -1800000);
        }
    }

    // ============================================================================
    // Test ms(long string)
    // ============================================================================

    mod test_ms_long_string {
        use super::*;

        #[test]
        fn should_not_throw_an_error() {
            // should not throw an error
            let _ = ms("53 milliseconds");
        }

        #[test]
        fn should_convert_milliseconds_to_ms() {
            // should convert milliseconds to ms
            assert_eq!(ms("53 milliseconds").unwrap(), 53);
        }

        #[test]
        fn should_convert_msecs_to_ms() {
            // should convert msecs to ms
            assert_eq!(ms("17 msecs").unwrap(), 17);
        }

        #[test]
        fn should_convert_sec_to_ms() {
            // should convert sec to ms
            assert_eq!(ms("1 sec").unwrap(), 1000);
        }

        #[test]
        fn should_convert_from_min_to_ms() {
            // should convert from min to ms
            assert_eq!(ms("1 min").unwrap(), 60000);
        }

        #[test]
        fn should_convert_from_hr_to_ms() {
            // should convert from hr to ms
            assert_eq!(ms("1 hr").unwrap(), 3600000);
        }

        #[test]
        fn should_convert_days_to_ms() {
            // should convert days to ms
            assert_eq!(ms("2 days").unwrap(), 172800000);
        }

        #[test]
        fn should_convert_weeks_to_ms() {
            // should convert weeks to ms
            assert_eq!(ms("1 week").unwrap(), 604800000);
        }

        #[test]
        fn should_convert_years_to_ms() {
            // should convert years to ms
            assert_eq!(ms("1 year").unwrap(), 31557600000);
        }

        #[test]
        fn should_work_with_decimals() {
            // should work with decimals
            assert_eq!(ms("1.5 hours").unwrap(), 5400000);
        }

        #[test]
        fn should_work_with_negative_integers() {
            // should work with negative integers
            assert_eq!(ms("-100 milliseconds").unwrap(), -100);
        }

        #[test]
        fn should_work_with_negative_decimals() {
            // should work with negative decimals
            assert_eq!(ms("-1.5 hours").unwrap(), -5400000);
        }

        #[test]
        fn should_work_with_negative_decimals_starting_with_dot() {
            // should work with negative decimals starting with "."
            assert_eq!(ms("-.5 hr").unwrap(), -1800000);
        }
    }

    // ============================================================================
    // Test ms(number) - short format
    // ============================================================================

    mod test_ms_number {
        use super::*;

        #[test]
        fn should_not_throw_an_error() {
            // should not throw an error
            let _ = ms(500);
        }

        #[test]
        fn should_support_milliseconds() {
            // should support milliseconds
            assert_eq!(ms(500).unwrap(), "500ms");
            assert_eq!(ms(-500).unwrap(), "-500ms");
        }

        #[test]
        fn should_support_seconds() {
            // should support seconds
            assert_eq!(ms(1000).unwrap(), "1s");
            assert_eq!(ms(10000).unwrap(), "10s");

            assert_eq!(ms(-1000).unwrap(), "-1s");
            assert_eq!(ms(-10000).unwrap(), "-10s");
        }

        #[test]
        fn should_support_minutes() {
            // should support minutes
            assert_eq!(ms(60 * 1000).unwrap(), "1m");
            assert_eq!(ms(60 * 10000).unwrap(), "10m");

            assert_eq!(ms(-1 * 60 * 1000).unwrap(), "-1m");
            assert_eq!(ms(-1 * 60 * 10000).unwrap(), "-10m");
        }

        #[test]
        fn should_support_hours() {
            // should support hours
            assert_eq!(ms(60 * 60 * 1000).unwrap(), "1h");
            assert_eq!(ms(60 * 60 * 10000).unwrap(), "10h");

            assert_eq!(ms(-1 * 60 * 60 * 1000).unwrap(), "-1h");
            assert_eq!(ms(-1 * 60 * 60 * 10000).unwrap(), "-10h");
        }

        #[test]
        fn should_support_days() {
            // should support days
            assert_eq!(ms(24 * 60 * 60 * 1000).unwrap(), "1d");
            assert_eq!(ms(24 * 60 * 60 * 6000).unwrap(), "6d");

            assert_eq!(ms(-1 * 24 * 60 * 60 * 1000).unwrap(), "-1d");
            assert_eq!(ms(-1 * 24 * 60 * 60 * 6000).unwrap(), "-6d");
        }

        #[test]
        fn should_support_weeks() {
            // should support weeks
            assert_eq!(ms(1 * 7 * 24 * 60 * 60 * 1000).unwrap(), "1w");
            assert_eq!(ms(2 * 7 * 24 * 60 * 60 * 1000).unwrap(), "2w");

            assert_eq!(ms(-1 * 1 * 7 * 24 * 60 * 60 * 1000).unwrap(), "-1w");
            assert_eq!(ms(-1 * 2 * 7 * 24 * 60 * 60 * 1000).unwrap(), "-2w");
        }

        #[test]
        fn should_support_months() {
            // should support months
            let one_month = (30.4375 * 24.0 * 60.0 * 60.0 * 1000.0) as i64;
            assert_eq!(ms(one_month).unwrap(), "1mo");
            assert_eq!(
                ms((30.4375 * 24.0 * 60.0 * 60.0 * 1200.0) as i64).unwrap(),
                "1mo"
            );
            assert_eq!(
                ms((30.4375 * 24.0 * 60.0 * 60.0 * 10000.0) as i64).unwrap(),
                "10mo"
            );

            assert_eq!(ms(-one_month).unwrap(), "-1mo");
            assert_eq!(
                ms(-(30.4375 * 24.0 * 60.0 * 60.0 * 1200.0) as i64).unwrap(),
                "-1mo"
            );
            assert_eq!(
                ms(-(30.4375 * 24.0 * 60.0 * 60.0 * 10000.0) as i64).unwrap(),
                "-10mo"
            );
        }

        #[test]
        fn should_support_years() {
            // should support years
            let one_year = (365.25 * 24.0 * 60.0 * 60.0 * 1000.0) as i64 + 1;
            assert_eq!(ms(one_year).unwrap(), "1y");
            assert_eq!(
                ms((365.25 * 24.0 * 60.0 * 60.0 * 1200.0) as i64 + 1).unwrap(),
                "1y"
            );
            assert_eq!(
                ms((365.25 * 24.0 * 60.0 * 60.0 * 10000.0) as i64 + 1).unwrap(),
                "10y"
            );

            assert_eq!(ms(-one_year).unwrap(), "-1y");
            assert_eq!(
                ms(-((365.25 * 24.0 * 60.0 * 60.0 * 1200.0) as i64 + 1)).unwrap(),
                "-1y"
            );
            assert_eq!(
                ms(-((365.25 * 24.0 * 60.0 * 60.0 * 10000.0) as i64 + 1)).unwrap(),
                "-10y"
            );
        }

        #[test]
        fn should_round() {
            // should round
            assert_eq!(ms(234234234).unwrap(), "3d");
            assert_eq!(ms(-234234234).unwrap(), "-3d");
        }
    }

    // ============================================================================
    // Test ms(invalid inputs)
    // ============================================================================

    mod test_ms_invalid_inputs {
        use super::*;

        #[test]
        fn should_throw_error_when_ms_empty_string() {
            // should throw an error, when ms("")
            assert!(ms("").is_err());
        }

        #[test]
        fn should_throw_error_when_ms_too_long_string() {
            // should throw an error when string is too long
            let long_string = "a".repeat(101);
            assert!(ms(long_string.as_str()).is_err());
        }
    }
}
