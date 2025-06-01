extern crate cu;
extern crate regex;

#[cfg(test)]
mod utils_test {
    use cu::{
        units::MAX_PRECISION,
        utils::{find_unit, format_number, parse_value_unit},
    };

    #[test]
    fn test_parse_value_unit() {
        // test regular number with unit
        let result = parse_value_unit("5ft");
        assert!(result.is_some());
        let (value, unit) = result.unwrap();
        assert_eq!(value, 5.0);
        assert_eq!(unit, "ft");

        // test decimal number with unit
        let result = parse_value_unit("3.14m");
        assert!(result.is_some());
        let (value, unit) = result.unwrap();
        assert_eq!(value, 3.14);
        assert_eq!(unit, "m");

        // test fraction with unit
        let result = parse_value_unit("1/2in");
        assert!(result.is_some());
        let (value, unit) = result.unwrap();
        assert_eq!(value, 0.5);
        assert_eq!(unit, "in");

        // test negative number with unit
        let result = parse_value_unit("-10km");
        assert!(result.is_some());
        let (value, unit) = result.unwrap();
        assert_eq!(value, -10.0);
        assert_eq!(unit, "km");

        // test unit with space
        let result = parse_value_unit("42 kg");
        assert!(result.is_some());
        let (value, unit) = result.unwrap();
        assert_eq!(value, 42.0);
        assert_eq!(unit, "kg");

        // test invalid input
        let result = parse_value_unit("abc");
        assert!(result.is_none());

        // test empty input
        let result = parse_value_unit("");
        assert!(result.is_none());
    }

    #[test]
    fn test_format_number() {
        // test with default precision (2)
        assert_eq!(format_number(&3.14159, Some(2)), "3.14");

        // test with higher precision
        assert_eq!(format_number(&3.14159, Some(4)), "3.1416");

        // test with higher precision
        assert_eq!(format_number(&3.14159, Some(4)), "3.1416");

        // test with max precision
        assert_eq!(
            format_number(&306.38750000000005, Some(MAX_PRECISION)),
            "306.38750000000005"
        );

        // test with None (default) precision
        assert_eq!(format_number(&3.14159, None), "3.14");

        // test rounding
        assert_eq!(format_number(&3.145, Some(2)), "3.15");
        assert_eq!(format_number(&3.144, Some(2)), "3.14");

        // test negative numbers
        assert_eq!(format_number(&-3.14159, Some(2)), "-3.14");

        // test very large numbers
        assert_eq!(format_number(&1234567.89, Some(2)), "1234567.89");

        // test very small numbers
        assert_eq!(format_number(&0.000123, Some(5)), "0.00012");
    }

    #[test]
    fn test_find_unit() {
        // test valid unit with exact match
        let result = find_unit("m");
        assert!(result.is_some());
        let (unit_type, unit) = result.unwrap();
        assert_eq!(unit_type, "LENGTH");
        assert_eq!(unit.name, "Meter");

        // test potentially conflicting unit abbreviations via case-sensitive match - #1
        let result = find_unit("B");
        assert!(result.is_some());
        let (unit_type, unit) = result.unwrap();
        assert_eq!(unit_type, "DIGITAL STORAGE");
        assert_eq!(unit.name, "Byte");

        // test potentially conflicting unit abbreviations via case-sensitive match - #2
        let result = find_unit("b");
        assert!(result.is_some());
        let (unit_type, unit) = result.unwrap();
        assert_eq!(unit_type, "DIGITAL STORAGE");
        assert_eq!(unit.name, "Bit");

        // test valid unit with case-insensitive match
        let result = find_unit("KM");
        assert!(result.is_some());
        let (unit_type, unit) = result.unwrap();
        assert_eq!(unit_type, "LENGTH");
        assert_eq!(unit.name, "Kilometer");

        // test valid unit with short alias
        let result = find_unit("m2");
        assert!(result.is_some());
        let (unit_type, unit) = result.unwrap();
        assert_eq!(unit_type, "AREA");
        assert_eq!(unit.abbr, "m²");

        // test valid unit with long alias
        let result = find_unit("square meter");
        assert!(result.is_some());
        let (unit_type, unit) = result.unwrap();
        assert_eq!(unit_type, "AREA");
        assert_eq!(unit.abbr, "m²");

        // test unit with special characters
        let result = find_unit("°C");
        assert!(result.is_some());
        let (unit_type, unit) = result.unwrap();
        assert_eq!(unit_type, "TEMPERATURE");
        assert_eq!(unit.name, "Celsius");

        // test invalid unit
        let result = find_unit("invalid_unit");
        assert!(result.is_none());

        // test empty string
        let result = find_unit("");
        assert!(result.is_none());
    }
}
