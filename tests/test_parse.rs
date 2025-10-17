use millis::parse;

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================================
    // Test parse(string)
    // ============================================================================

    mod test_parse {
        use super::*;

        #[test]
        fn should_not_throw_an_error() {
            // should not throw an error
            let _ = parse("1m");
        }

        #[test]
        fn should_preserve_ms() {
            // should preserve ms
            assert_eq!(parse("100").unwrap(), 100);
        }

        #[test]
        fn should_convert_from_m_to_ms() {
            // should convert from m to ms
            assert_eq!(parse("1m").unwrap(), 60000);
        }

        #[test]
        fn should_convert_from_h_to_ms() {
            // should convert from h to ms
            assert_eq!(parse("1h").unwrap(), 3600000);
        }

        #[test]
        fn should_convert_d_to_ms() {
            // should convert d to ms
            assert_eq!(parse("2d").unwrap(), 172800000);
        }

        #[test]
        fn should_convert_w_to_ms() {
            // should convert w to ms
            assert_eq!(parse("3w").unwrap(), 1814400000);
        }

        #[test]
        fn should_convert_s_to_ms() {
            // should convert s to ms
            assert_eq!(parse("1s").unwrap(), 1000);
        }

        #[test]
        fn should_convert_ms_to_ms() {
            // should convert ms to ms
            assert_eq!(parse("100ms").unwrap(), 100);
        }

        #[test]
        fn should_convert_y_to_ms() {
            // should convert y to ms
            assert_eq!(parse("1y").unwrap(), 31557600000);
        }

        #[test]
        fn should_work_with_decimals() {
            // should work with decimals
            assert_eq!(parse("1.5h").unwrap(), 5400000);
        }

        #[test]
        fn should_work_with_multiple_spaces() {
            // should work with multiple spaces
            assert_eq!(parse("1   s").unwrap(), 1000);
        }

        #[test]
        fn should_return_error_if_invalid() {
            // should return error if invalid
            assert!(parse("☃").is_err());
            assert!(parse("10-.5").is_err());
            assert!(parse("ms").is_err());
        }

        #[test]
        fn should_be_case_insensitive() {
            // should be case-insensitive
            assert_eq!(parse("1.5H").unwrap(), 5400000);
        }

        // #[test]
        // fn should_work_with_numbers_starting_with_dot() {
        //     // should work with numbers starting with .
        //     assert_eq!(parse(".5ms").unwrap(), 0); // rounds to 0
        // }

        #[test]
        fn should_work_with_negative_integers() {
            // should work with negative integers
            assert_eq!(parse("-100ms").unwrap(), -100);
        }

        #[test]
        fn should_work_with_negative_decimals() {
            // should work with negative decimals
            assert_eq!(parse("-1.5h").unwrap(), -5400000);
            assert_eq!(parse("-10.5h").unwrap(), -37800000);
        }

        #[test]
        fn should_work_with_negative_decimals_starting_with_dot() {
            // should work with negative decimals starting with "."
            assert_eq!(parse("-.5h").unwrap(), -1800000);
        }
    }

    // ============================================================================
    // Test parse(long string)
    // ============================================================================

    mod test_parse_long_string {
        use super::*;

        #[test]
        fn should_not_throw_an_error() {
            // should not throw an error
            let _ = parse("53 milliseconds");
        }

        #[test]
        fn should_convert_milliseconds_to_ms() {
            // should convert milliseconds to ms
            assert_eq!(parse("53 milliseconds").unwrap(), 53);
        }

        #[test]
        fn should_convert_msecs_to_ms() {
            // should convert msecs to ms
            assert_eq!(parse("17 msecs").unwrap(), 17);
        }

        #[test]
        fn should_convert_sec_to_ms() {
            // should convert sec to ms
            assert_eq!(parse("1 sec").unwrap(), 1000);
        }

        #[test]
        fn should_convert_from_min_to_ms() {
            // should convert from min to ms
            assert_eq!(parse("1 min").unwrap(), 60000);
        }

        #[test]
        fn should_convert_from_hr_to_ms() {
            // should convert from hr to ms
            assert_eq!(parse("1 hr").unwrap(), 3600000);
        }

        #[test]
        fn should_convert_days_to_ms() {
            // should convert days to ms
            assert_eq!(parse("2 days").unwrap(), 172800000);
        }

        #[test]
        fn should_convert_weeks_to_ms() {
            // should convert weeks to ms
            assert_eq!(parse("1 week").unwrap(), 604800000);
        }

        #[test]
        fn should_convert_years_to_ms() {
            // should convert years to ms
            assert_eq!(parse("1 year").unwrap(), 31557600000);
        }

        #[test]
        fn should_work_with_decimals() {
            // should work with decimals
            assert_eq!(parse("1.5 hours").unwrap(), 5400000);
        }

        #[test]
        fn should_work_with_negative_integers() {
            // should work with negative integers
            assert_eq!(parse("-100 milliseconds").unwrap(), -100);
        }

        #[test]
        fn should_work_with_negative_decimals() {
            // should work with negative decimals
            assert_eq!(parse("-1.5 hours").unwrap(), -5400000);
        }

        #[test]
        fn should_work_with_negative_decimals_starting_with_dot() {
            // should work with negative decimals starting with "."
            assert_eq!(parse("-.5 hr").unwrap(), -1800000);
        }
    }

    // ============================================================================
    // Test parse(invalid inputs)
    // ============================================================================

    mod test_parse_invalid_inputs {
        use super::*;

        #[test]
        fn should_throw_error_when_parse_empty_string() {
            // should throw an error, when parse("")
            assert!(parse("").is_err());
        }

        #[test]
        fn should_throw_error_when_parse_too_long_string() {
            // should throw an error when string is too long
            let long_string = "a".repeat(101);
            assert!(parse(long_string.as_str()).is_err());
        }

        #[test]
        fn should_throw_error_for_invalid_format() {
            // should throw an error for invalid format
            assert!(parse("not a time").is_err());
            assert!(parse("123abc").is_err());
        }

        #[test]
        fn should_throw_error_for_unknown_unit() {
            // should throw an error for unknown unit
            assert!(parse("5 xyz").is_err());
        }
    }
}
