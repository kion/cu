use std::io::{self, Write};
use std::process::{Command, Stdio};

#[derive(Clone)]
struct TestCase {
    input: &'static str,
    expected_output: &'static str,
    description: &'static str,
}

fn run_test(test_case: &TestCase) -> Result<(), String> {
    let child = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg(test_case.input)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    // get the stdout from the child process
    let output = child.wait_with_output().expect("Failed to wait on child");
    let stdout = String::from_utf8(output.stdout).expect("Failed to convert stdout to string");

    // remove any trailing newlines and trim for clean comparison
    let stdout_trimmed = stdout.trim();
    let expected_trimmed = test_case.expected_output.trim();

    if stdout_trimmed == expected_trimmed {
        Ok(())
    } else {
        Err(format!(
            "Test failed for input: '{}'\nExpected: '{}'\nGot: '{}'",
            test_case.input, expected_trimmed, stdout_trimmed
        ))
    }
}

fn run_tests(test_cases: Vec<TestCase>) {
    println!("Running {} test cases...", test_cases.len());

    let mut passed = 0;
    let mut failed = 0;

    for (i, test_case) in test_cases.iter().enumerate() {
        print!("Test case {}: {} ... ", i + 1, test_case.description);
        io::stdout().flush().unwrap();

        match run_test(test_case) {
            Ok(_) => {
                passed += 1;
                println!("PASSED");
            }
            Err(err) => {
                failed += 1;
                println!("FAILED");
                println!("{}", err);
            }
        }
    }

    println!("\nTest Results: {} passed, {} failed", passed, failed);

    if failed > 0 {
        panic!("Some tests failed");
    }
}

#[test]
fn test_complex_length_units() {
    let test_cases = vec![
        // multiple unit types in different orders
        TestCase {
            input: "1mile 2yd 1ft 300in = km",
            expected_output: "[LENGTH] 1 mi 2 yd 1 ft 300 in = 1.62 km",
            description: "Multiple length units in descending order",
        },
        TestCase {
            input: "300in 1ft 2yd 1mile = km",
            expected_output: "[LENGTH] 1 mi 2 yd 1 ft 300 in = 1.62 km",
            description: "Multiple length units in ascending order",
        },
        // mixed imperial and metric units
        TestCase {
            input: "1km 2m 30cm 5mm = ft",
            expected_output: "[LENGTH] 1 km 2 m 30 cm 5 mm = 3288.4 ft",
            description: "Mixed metric units to imperial",
        },
        // mixed imperial and metric units, higher precision
        TestCase {
            input: "1km 2m 30cm 7mm = ft:4",
            expected_output: "[LENGTH] 1 km 2 m 30 cm 7 mm = 3288.4088 ft",
            description: "Mixed metric units to imperial with higher precision",
        },
        TestCase {
            input: "5ft 10in = cm:4",
            expected_output: "[LENGTH] 5 ft 10 in = 177.8 cm",
            description: "Imperial to metric with high precision",
        },
        // fractional values
        TestCase {
            input: "1/2mile 1/4yd 1/8ft = m",
            expected_output: "[LENGTH] 0.5 mi 0.25 yd 0.125 ft = 804.94 m",
            description: "Fractional imperial units to metric",
        },
        // negative values
        TestCase {
            input: "-5ft -10in = m",
            expected_output: "[LENGTH] -5 ft -10 in = -1.78 m",
            description: "Negative values",
        },
        // many small units adding up
        TestCase {
            input: "1in 1in 1in 1in 1in 1in 1in 1in 1in 1in 1in 1in = ft",
            expected_output: "[LENGTH] 12 in = 1 ft",
            description: "Many small units adding up to a larger unit",
        },
        TestCase {
            input: "1 yd 1/2 yd 1in 1in 1in = ft",
            expected_output: "[LENGTH] 1.5 yd 3 in = 4.75 ft",
            description: "Many small units with different types adding up to a larger unit",
        },
    ];

    run_tests(test_cases);
}

#[test]
fn test_mixed_units_with_other_unit_types() {
    let test_cases = vec![
        // weight units
        TestCase {
            input: "1kg 500gm = lb",
            expected_output: "[MASS] 1 kg 500 gm = 3.31 lb",
            description: "Compound weight units",
        },
        TestCase {
            input: "1kg 0.5lb = lb:3",
            expected_output: "[MASS] 1 kg 0.5 lb = 2.705 lb",
            description: "Mixed weight units, higher precision",
        },
        TestCase {
            input: "1lb 0.5lb = kg",
            expected_output: "[MASS] 1.5 lb = 0.68 kg",
            description: "Imperial weight units to metric",
        },
        TestCase {
            input: "1lb 0.5lb = kg:5",
            expected_output: "[MASS] 1.5 lb = 0.68039 kg",
            description: "Imperial weight units to metric, higher precision",
        },
        // single unit type word vs multiple unit type words
        TestCase {
            input: "1000 m2 to acres",
            expected_output: "[AREA] 1000 m² = 0.25 a",
            description: "single unit type word",
        },
        TestCase {
            input: "1000 square meters to acres",
            expected_output: "[AREA] 1000 m² = 0.25 a",
            description: "multiple unit type words",
        },
        // volume units with multiple source unit types
        TestCase {
            input: "1gal 2qt 1pt = l",
            expected_output: "\
            [VOLUME] 1 gal 2 qt 1 pt (Imperial) = 7.39 l\n\
            [VOLUME] 1 gal 2 qt 1 pt (US liquid) = 6.15 l",
            description: "Mixed volume units with multiple source unit types",
        },
        // volume units with multiple target unit types
        TestCase {
            input: "1l = qt",
            expected_output: "\
            [VOLUME] 1 l = 0.88 qt (Imperial)\n\
            [VOLUME] 1 l = 1.06 qt (US liquid)",
            description: "Volume units with multiple target unit types",
        },
        // volume units with multiple source and target unit types
        TestCase {
            input: "1gal = qt",
            expected_output: "\
            [VOLUME] 1 gal (Imperial) = 4 qt (Imperial)\n\
            [VOLUME] 1 gal (Imperial) = 4.8 qt (US liquid)\n\
            [VOLUME] 1 gal (US liquid) = 3.33 qt (Imperial)\n\
            [VOLUME] 1 gal (US liquid) = 4 qt (US liquid)",
            description: "Volume units with multiple source and target unit types",
        },
        // volume units with multiple compound source values/types and target unit types
        TestCase {
            input: "1gal 2qt = pt",
            expected_output: "\
            [VOLUME] 1 gal 2 qt (Imperial) = 12 pt (Imperial)\n\
            [VOLUME] 1 gal 2 qt (Imperial) = 14.41 pt (US liquid)\n\
            [VOLUME] 1 gal 2 qt (US liquid) = 9.99 pt (Imperial)\n\
            [VOLUME] 1 gal 2 qt (US liquid) = 12 pt (US liquid)",
            description:
                "Volume units with multiple compound source values/types and target unit types",
        },
        // time units
        TestCase {
            input: "1h 30min 45s = min",
            expected_output: "[TIME] 1 hr 30 min 45 s = 90.75 min",
            description: "Mixed time units",
        },
    ];

    run_tests(test_cases);
}

#[test]
fn test_edge_cases_and_errors() {
    let test_cases = vec![
        // maximum precision
        TestCase {
            input: "1ft 1/16in = mm:14",
            expected_output: "[LENGTH] 1 ft 0.0625 in = 306.38750000000005 mm",
            description: "Testing with maximum precision",
        },
        // very large numbers
        TestCase {
            input: "9999mi 9999yd 9999ft 9999in = km",
            expected_output: "[LENGTH] 9999 mi 9999 yd 9999 ft 9999 in = 16104.24 km",
            description: "Very large mixed numbers",
        },
        // different unit types
        TestCase {
            input: "5ft 10kg 20W = m",
            expected_output: "[ Unit type mismatch: mixed 'LENGTH' with 'MASS' ]",
            description: "Mixing different unit types (length and weight)",
        },
        // unknown units
        TestCase {
            input: "5ft 10parsecs = m",
            expected_output: "[ Unknown unit: parsecs ]",
            description: "Using an unknown unit",
        },
    ];

    run_tests(test_cases);
}
