#[macro_use]
extern crate lazy_static;

mod units;

use crate::units::{Unit, UNITS};
use regex::Regex;

const DEFAULT_PRECISION: i32 = 2;

fn version() {
    println!("Version: 1.0.0");
}

fn usage() {
    println!("Usage: <sv> <su> =|to <tu> [:<dp>]");
    println!("<sv> (required) - the value to convert");
    println!("<su> (required) - the unit of the value to convert");
    println!("<tu> (required) - the unit to convert the value into");
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

fn format_number(num: &f64, precision: Option<i32>) -> String {
    return if precision.is_none() {
        format!("{}", num)
    } else {
        let p = (10 as f64).powf(precision.unwrap() as f64);
        format!("{}", (num * p).round() / p)
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input: String = args[1..].join(" ");
    if input == "--version" || input == "-version" || input == "-v" || input == "version" {
        version();
    } else if input == "--help" || input == "-help" || input == "-h" || input == "help" {
        usage();
    } else if input == "--help units" || input == "-help units" || input == "-hu" || input == "help units" {
        supported_units();
    } else {
        let mut s_val: Option<f64> = None;
        let mut s_unit: Option<String> = None;
        let mut t_unit: Option<String> = None;
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
            let left_right: Vec<String> = input.split(sep.unwrap()).map(|s| s.trim().to_string()).collect();
            if left_right.len() == 2 {
                let left_regex = Regex::new(r"(-?[\d/.]+)(.*)").unwrap();
                let left_captures_opt = left_regex.captures(&left_right[0]);
                if left_captures_opt.is_some() {
                    let left_captures = left_captures_opt.unwrap();
                    if left_captures.len() == 3 {
                        let lv = left_captures.get(1).unwrap().as_str().trim();
                        if lv.contains("/") {
                            let dd: Vec<String> = lv.split("/").map(|s| s.trim().to_string()).collect();
                            if dd.len() == 2 {
                                let dividend_str = dd[0].to_string();
                                let dividend_result = dividend_str.parse::<f64>();
                                match dividend_result {
                                    Ok(dividend) => {
                                        let divisor_str = dd[1].to_string();
                                        let divisor_result = divisor_str.parse::<f64>();
                                        match divisor_result {
                                            Ok(divisor) => {
                                                s_val = Some(dividend / divisor);
                                            },
                                            Err(_) => println!("Not a valid divisor number: {}", divisor_str)
                                        }
                                    },
                                    Err(_) => println!("Not a valid dividend number: {}", dividend_str)
                                }
                            }
                        } else {
                            let s_val_parse_result = lv.parse::<f64>();
                            match s_val_parse_result {
                                Ok(num) => {
                                    s_val = Some(num);
                                },
                                Err(_) => println!("Not a valid number: {}", lv)
                            }
                        }
                        if s_val.is_some() {
                            s_unit = Some(left_captures.get(2).unwrap().as_str().trim().to_string());
                            let right = &left_right[1];
                            if right.contains(":") {
                                let target_unit_precision: Vec<String> = right.split(":").map(|s| s.trim().to_string()).collect();
                                let target_unit = &target_unit_precision[0];
                                if &target_unit_precision[1] == "*" {
                                    precision = None;
                                } else {
                                    let target_unit_precision_parse_result = &target_unit_precision[1].parse::<i32>();
                                    match target_unit_precision_parse_result {
                                        Ok(num) => {
                                            precision = Some(*num);
                                        },
                                        Err(_) => println!("Not a valid precision: {} (using the default precision of {} instead)", target_unit_precision[1], DEFAULT_PRECISION)
                                    }
                                }
                                t_unit = Some(target_unit.trim().to_string());
                            } else {
                                t_unit = Some(right.trim().to_string());
                            }
                        }
                    }
                }
            }
            if s_val.is_none() || s_unit.is_none() || t_unit.is_none() {
                usage();
            } else {
                let sv = s_val.unwrap();
                let su = s_unit.unwrap();
                let tu = t_unit.unwrap();
                let result = |ut: &str, s_unit: &Unit, t_unit: &Unit| {
                    if *&t_unit.formula.is_some() {
                        let formula_result = &t_unit.formula.unwrap()(s_unit, sv);
                        match formula_result {
                            Ok(tv) => println!(
                                "[{}] {} {} = {} {}",
                                ut, sv, s_unit.abbr,
                                format_number(tv, precision), t_unit.abbr
                            ),
                            Err(u) => unknown_unit(u)
                        }
                    } else if *&s_unit.ratios.is_some() {
                        for sur in s_unit.ratios.as_ref().unwrap() {
                            if *&t_unit.ratios.is_some() {
                                for tur in t_unit.ratios.as_ref().unwrap() {
                                    let tv = sur.1 / tur.1 * sv;
                                    println!(
                                        "[{}] {} {}{} = {} {}{}",
                                        ut, sv,
                                        s_unit.abbr, if sur.0 != "" { format!(" ({})", sur.0) } else { "".to_string() },
                                        format_number(&tv, precision),
                                        t_unit.abbr, if tur.0 != "" { format!(" ({})", tur.0) } else { "".to_string() }
                                    );
                                }
                            }
                        }
                    }
                };
                let su_lc = su.to_lowercase();
                let tu_lc = tu.to_lowercase();
                let mut s_match_found = false;
                let mut t_match_found = false;
                let mut results: Vec<(&'static str, (&Unit, &Unit))> = vec![];
                'u_loop: for unit in UNITS.iter() {
                    let mut s_unit: Option<&Unit> = None;
                    let mut t_unit: Option<&Unit> = None;
                    for u in unit.1.iter() {
                        if s_unit.is_none() && u.abbr == su {
                            s_unit = Some(u);
                        }
                        if t_unit.is_none() && u.abbr == tu {
                            t_unit = Some(u);
                        }
                        if s_unit.is_some() && t_unit.is_some() {
                            results.push((unit.0, (s_unit.unwrap(), t_unit.unwrap())));
                            continue 'u_loop;
                        }
                    }
                    for u in unit.1.iter() {
                        let u_abbr_lc = u.abbr.to_lowercase();
                        if s_unit.is_none() && u_abbr_lc == su_lc {
                            s_unit = Some(u);
                        }
                        if t_unit.is_none() && u_abbr_lc == tu_lc {
                            t_unit = Some(u);
                        }
                        if s_unit.is_some() && t_unit.is_some() {
                            results.push((unit.0, (s_unit.unwrap(), t_unit.unwrap())));
                            continue 'u_loop;
                        }
                    }
                    for u in unit.1.iter() {
                        for a in u.aliases {
                            let a_lc = a.to_lowercase();
                            if s_unit.is_none() && a_lc == su_lc {
                                s_unit = Some(u);
                            }
                            if t_unit.is_none() && a_lc == tu_lc {
                                t_unit = Some(u);
                            }
                            if s_unit.is_some() && t_unit.is_some() {
                                results.push((unit.0, (s_unit.unwrap(), t_unit.unwrap())));
                                continue 'u_loop;
                            }
                        }
                    }
                    s_match_found = s_match_found || s_unit.is_some();
                    t_match_found = t_match_found || t_unit.is_some();
                }
                if !results.is_empty() {
                    for r in results {
                        result(r.0, r.1.0, r.1.1);
                    }
                } else {
                    if s_match_found && t_match_found {
                        println!("[ Unit type mismatch ]");
                    } else {
                        if !s_match_found {
                            unknown_unit(&su);
                        }
                        if !t_match_found {
                            unknown_unit(&tu);
                        }
                    }
                }
            }
        }
    }
}
