#[macro_use]
extern crate lazy_static;

mod units;
mod utils;

use indexmap::IndexMap;

use regex::Regex;
use units::{DEFAULT_PRECISION, MAX_PRECISION, UNITS};
use utils::{find_unit, format_number, parse_value_unit};

fn version() {
    println!("cu 1.1.1");
}

fn usage() {
    println!("Usage: <sv> <su> [<sv2> <su2> ...] =|to <tu> [:<dp>]");
    println!("<sv> (required) - value to convert");
    println!("<su> (required) - unit of the value to convert");
    println!("<tu> (required) - unit to convert the value into");
    println!("<dp> (optional) - the decimal places precision for the conversion result: either an integer (implicit/default value is 2) or an \"*\" to use max precision");
    println!("Use one of the following parameters to print the list of supported units:");
    println!("--help units, -help units, -hu, help units");
}

fn supported_units() {
    println!("Supported units:");
    for unit in UNITS.iter() {
        println!("-------------------------------------------");
        println!("| {0: <39} |", unit.0);
        println!("-------------------------------------------");
        for u in unit.1.iter() {
            println!("| {0: <30} | {1:6} |", u.name, u.abbr);
        }
    }
    println!("-------------------------------------------");
}

fn unknown_unit(u: &str) {
    println!("[ Unknown unit: {} ]", u);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input: String = args[1..].join(" ");
    if input == "--version" || input == "-version" || input == "-v" || input == "version" {
        version();
    } else if input == "--help" || input == "-help" || input == "-h" || input == "help" {
        usage();
    } else if input == "--help units"
        || input == "-help units"
        || input == "-hu"
        || input == "help units"
    {
        supported_units();
    } else {
        let mut sep: Option<&str> = None;
        if input.contains("=") {
            sep = Some("=");
        } else if input.contains(" to ") {
            sep = Some(" to ");
        }
        if sep.is_none() {
            usage();
        } else {
            let mut precision = Some(DEFAULT_PRECISION);
            let left_right: Vec<String> = input
                .split(sep.unwrap())
                .map(|s| s.trim().to_string())
                .collect();
            if left_right.len() == 2 {
                // parse the right side (target unit and precision)
                let right = &left_right[1];
                let t_unit_str;

                if right.contains(":") {
                    let target_unit_precision: Vec<String> =
                        right.split(":").map(|s| s.trim().to_string()).collect();
                    let target_unit = &target_unit_precision[0];
                    if &target_unit_precision[1] == "*" {
                        precision = Some(MAX_PRECISION);
                    } else {
                        let target_unit_precision_parse_result =
                            &target_unit_precision[1].parse::<i32>();
                        match target_unit_precision_parse_result {
                            Ok(num) => {
                                if *num >= 0 {
                                    if *num <= MAX_PRECISION {
                                        precision = Some(*num);
                                    } else {
                                        println!("Precision too high: {} (using the max allowed precision of {} instead)", target_unit_precision[1], MAX_PRECISION);
                                        precision = Some(MAX_PRECISION);
                                    }
                                } else {
                                    println!("Can't use negative precision: {} (using the default precision of {} instead)", target_unit_precision[1], DEFAULT_PRECISION);
                                }
                            }
                            Err(_) => {
                                println!("Not a valid precision: {} (using the default precision of {} instead)", target_unit_precision[1], DEFAULT_PRECISION);
                            }
                        }
                    }
                    t_unit_str = target_unit.trim().to_string();
                } else {
                    t_unit_str = right.trim().to_string();
                }

                let t_unit_type;
                let t_unit;
                if let Some((found_unit_type, unit)) = find_unit(&t_unit_str, None) {
                    t_unit_type = found_unit_type;
                    t_unit = unit;
                } else {
                    unknown_unit(&t_unit_str);
                    return;
                }

                // parse the left side (source values and units)
                let left = &left_right[0];

                // split the left side by space, but preserve number-unit pairs
                let re = Regex::new(r"(-?[\d/.]+(\s*[^\d\s-]\d{0,1})+)").unwrap();
                let mut value_unit_pairs: Vec<String> = Vec::new();

                for cap in re.captures_iter(left) {
                    value_unit_pairs.push(cap[1].trim().to_string());
                }

                if value_unit_pairs.is_empty() {
                    usage();
                    return;
                }

                if t_unit.formula.is_some() && value_unit_pairs.len() > 1 {
                    // formulas only work with a single source unit
                    println!(
                        "[ Formula based conversions cannot be used with compound/mixed units ]"
                    );
                    return;
                }

                // process each value-unit pair
                let mut total_value_in_target_unit = IndexMap::<&'static str, f64>::new();
                let mut all_units_match = true;
                let mut mismatched_units: Vec<String> = Vec::new();

                for pair in value_unit_pairs.iter() {
                    if let Some((value, unit_str)) = parse_value_unit(pair) {
                        if let Some((_, unit)) = find_unit(&unit_str, Some(t_unit_type)) {
                            if t_unit.formula.is_none() {
                                if let Some(ratios) = &unit.ratios {
                                    for (source_unit_type, ratio) in ratios {
                                        // convert to base unit using ratio and add to total
                                        let v = total_value_in_target_unit
                                            .get(source_unit_type)
                                            .unwrap_or(&0.0)
                                            + (value * ratio);
                                        total_value_in_target_unit.insert(source_unit_type, v);
                                    }
                                }
                            }
                        } else {
                            all_units_match = false;
                            mismatched_units.push(unit_str.to_string());
                        }
                    } else {
                        usage();
                        return;
                    }
                }

                if !all_units_match {
                    for unit in mismatched_units {
                        println!(
                            "[ Unit '{}' not found in type '{}' ]",
                            unit, t_unit_type
                        );
                    }
                    return;
                }

                // ============================================================
                // build a representation of units for display
                // ============================================================
                // initialize a structure to preserve order
                let mut unit_values = Vec::<(String, f64, f64)>::new();
                // collect values by unit type and keep track of the first ratio value
                for pair in value_unit_pairs.iter() {
                    if let Some((value, unit_str)) = parse_value_unit(&pair) {
                        if let Some((_, unit)) = find_unit(&unit_str, Some(t_unit_type)) {
                            // get the first ratio value for sorting
                            let first_ratio_value = unit
                                .ratios
                                .as_ref()
                                .and_then(|ratios| ratios.first())
                                .map(|(_, ratio)| *ratio)
                                .unwrap_or(0.0);
                            // find existing entry or add new one
                            if let Some(index) = unit_values
                                .iter()
                                .position(|(abbr, _, _)| *abbr == unit.abbr)
                            {
                                unit_values[index].1 += value;
                            } else {
                                unit_values.push((unit.abbr.to_string(), value, first_ratio_value));
                            }
                        }
                    }
                }
                // sort by the first ratio value (descending, i.e. so that the larger units go first)
                unit_values
                    .sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
                // build the display string with combined values
                let mut mixed_units_str = String::new();
                for (i, (unit_abbr, value, _)) in unit_values.iter().enumerate() {
                    if i > 0 {
                        mixed_units_str.push_str(" ");
                    }
                    mixed_units_str.push_str(&value.to_string());
                    mixed_units_str.push_str(" ");
                    mixed_units_str.push_str(unit_abbr);
                }
                // ============================================================

                if let Some(formula) = &t_unit.formula {
                    if let Some((_, source_unit)) = find_unit(
                        &value_unit_pairs[0]
                            .replace(
                                |c: char| {
                                    c.is_ascii_digit()
                                        || c == '.'
                                        || c == '/'
                                        || c == '-'
                                        || c.is_whitespace()
                                },
                                "",
                            )
                            .trim(),
                        Some(t_unit_type)
                    ) {
                        let source_value = parse_value_unit(&value_unit_pairs[0]).unwrap().0;
                        match formula(source_unit, source_value) {
                            Ok(result) => {
                                println!(
                                    "[{}] {} = {} {}",
                                    t_unit_type,
                                    mixed_units_str,
                                    format_number(&result, precision),
                                    t_unit.abbr
                                );
                            }
                            Err(e) => println!("[ Formula error: {} ]", e),
                        }
                    }
                } else if let Some(ratios) = &t_unit.ratios {
                    let multiple_source_units = total_value_in_target_unit.len() > 1;
                    if ratios.len() == 1 {
                        for sur in total_value_in_target_unit {
                            let target_ratio = ratios[0].1;
                            let result = sur.1 / target_ratio;

                            println!(
                                "[{}] {}{} = {} {}",
                                t_unit_type,
                                mixed_units_str,
                                if multiple_source_units {
                                    format!(" ({})", sur.0)
                                } else {
                                    "".to_string()
                                },
                                format_number(&result, precision),
                                t_unit.abbr
                            );
                        }
                    } else {
                        for sur in total_value_in_target_unit {
                            for ratio in ratios {
                                let target_ratio_label = ratio.0;
                                let target_ratio = ratio.1;
                                let result = sur.1 / target_ratio;

                                println!(
                                    "[{}] {}{} = {} {} ({})",
                                    t_unit_type,
                                    mixed_units_str,
                                    if multiple_source_units {
                                        format!(" ({})", sur.0)
                                    } else {
                                        "".to_string()
                                    },
                                    format_number(&result, precision),
                                    t_unit.abbr,
                                    target_ratio_label
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}
