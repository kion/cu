use crate::units::{Unit, DEFAULT_PRECISION, MAX_PRECISION, UNITS};
use regex::Regex;

// parse a value-unit pair and return the value and unit
pub fn parse_value_unit(value_unit_str: &str) -> Option<(f64, String)> {
    let value_regex = Regex::new(r"(-?[\d/.]+)(.*)").unwrap();
    let value_captures_opt = value_regex.captures(value_unit_str);

    if let Some(value_captures) = value_captures_opt {
        if value_captures.len() == 3 {
            let value_str = value_captures.get(1).unwrap().as_str().trim();
            let mut value: Option<f64> = None;

            if value_str.contains("/") {
                let dd: Vec<String> = value_str.split("/").map(|s| s.trim().to_string()).collect();
                if dd.len() == 2 {
                    let dividend_result = dd[0].parse::<f64>();
                    let divisor_result = dd[1].parse::<f64>();

                    if let (Ok(dividend), Ok(divisor)) = (dividend_result, divisor_result) {
                        value = Some(dividend / divisor);
                    } else {
                        return None;
                    }
                }
            } else {
                let value_parse_result = value_str.parse::<f64>();
                match value_parse_result {
                    Ok(num) => value = Some(num),
                    Err(_) => return None,
                }
            }

            if let Some(v) = value {
                let unit = value_captures.get(2).unwrap().as_str().trim().to_string();
                return Some((v, unit));
            }
        }
    }
    None
}

// find a unit in the UNITS collection and return its type and ratio
pub fn find_unit(unit_str: &str) -> Option<(&'static str, &Unit)> {
    for unit_type in UNITS.iter() {
        // ============================================================
        // separate full UNITS enum iteration is needed here
        // to prioritize case-sensitive matching
        // (e.g. to recognize "B" as bytes and "b" as bits)
        // ============================================================
        for unit in unit_type.1.iter() {
            // check exact match
            if unit.abbr == unit_str {
                return Some((unit_type.0, unit));
            }
        }
        // ============================================================
        let unit_lc = unit_str.to_lowercase();
        for unit in unit_type.1.iter() {
            // check case-insensitive match
            if unit.abbr.to_lowercase() == unit_lc {
                return Some((unit_type.0, unit));
            }
            // check aliases
            for alias in unit.aliases {
                if alias.to_lowercase() == unit_lc {
                    return Some((unit_type.0, unit));
                }
            }
        }
    }
    None
}

// format a number with the specified precision
pub fn format_number(num: &f64, precision: Option<i32>) -> String {
    let mut prc = DEFAULT_PRECISION;
    if precision.is_some() {
        prc = precision.unwrap();
        if prc > MAX_PRECISION {
            prc = MAX_PRECISION;
        }
    };
    let p = (10 as f64).powf(prc as f64);
    format!("{}", (num * p).round() / p)
}
