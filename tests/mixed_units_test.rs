use std::io::{self, Write};
use std::process::{Command, Stdio};

#[derive(Clone)]
struct TestCase {
    input: &'static str,
    expected_output: &'static str,
}

fn run_test(test_case: TestCase) -> Result<(), String> {
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
        print!("Test case {}: '{}' ... ", i + 1, test_case.input);
        io::stdout().flush().unwrap();

        match run_test(test_case.clone()) {
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
fn test_simple_mixed_units() {
    // basic mixed units test
    let test_cases = vec![
        // using fractions
        TestCase {
            input: "5/2ft 3/4in = m",
            expected_output: "[LENGTH] 2.5 ft 0.75 in = 0.78 m",
        },
        // using 'to' syntax
        TestCase {
            input: "5ft 10in to m",
            expected_output: "[LENGTH] 5 ft 10 in = 1.78 m",
        },
        // more than two units
        TestCase {
            input: "1yd 2ft 4.7in = cm",
            expected_output: "[LENGTH] 1 yd 2 ft 4.7 in = 164.34 cm",
        },
        // more than two units, different precision
        TestCase {
            input: "1yd 2ft 4.7in = cm:3",
            expected_output: "[LENGTH] 1 yd 2 ft 4.7 in = 164.338 cm",
        },
        // more than two units, using target unit matching one of the source units
        TestCase {
            input: "1yd 2ft 3in = in",
            expected_output: "[LENGTH] 1 yd 2 ft 3 in = 63 in",
        },
        // different precision, multiple target unit types
        TestCase {
            input: "900 kg 100000 gm = t:3",
            expected_output: "\
            [MASS] 900 kg 100000 gm = 1 t (Metric)\n\
            [MASS] 900 kg 100000 gm = 0.984 t (Imperial)\n\
            [MASS] 900 kg 100000 gm = 1.102 t (US)",
        },
    ];

    run_tests(test_cases);
}

#[test]
fn test_edge_cases() {
    let test_cases = vec![
        // different ordering of units
        TestCase {
            input: "10in 5ft = m",
            expected_output: "[LENGTH] 5 ft 10 in = 1.78 m",
        },
        // different ordering of units, higher precision
        TestCase {
            input: "10in 5ft = m:3",
            expected_output: "[LENGTH] 5 ft 10 in = 1.778 m",
        },
        // using fractions
        TestCase {
            input: "5.5ft 2.25in = cm",
            expected_output: "[LENGTH] 5.5 ft 2.25 in = 173.36 cm",
        },
        // using fractions, higher precision
        TestCase {
            input: "5.5ft 2.25in = cm:3",
            expected_output: "[LENGTH] 5.5 ft 2.25 in = 173.355 cm",
        },
        // fractional format
        TestCase {
            input: "5/2ft 3/4in = m",
            expected_output: "[LENGTH] 2.5 ft 0.75 in = 0.78 m",
        },
        // fractional format, higher precision
        TestCase {
            input: "5/2ft 3/4in = m:5",
            expected_output: "[LENGTH] 2.5 ft 0.75 in = 0.78105 m",
        },
        // maximum precision
        TestCase {
            input: "5/2ft 3/4in = km:*",
            expected_output: "[LENGTH] 2.5 ft 0.75 in = 0.00078105 km",
        },
    ];

    run_tests(test_cases);
}

#[test]
fn test_failure_cases() {
    let test_cases = vec![
        // mixed unit types
        TestCase {
            input: "5ft 10kg = m",
            expected_output: "[ Unit type mismatch: mixed 'LENGTH' with 'MASS' ]",
        },
        // unknown target unit
        TestCase {
            input: "5ft 10in = unknown",
            expected_output: "[ Unknown unit: unknown ]",
        },
        // multiple unknown source units
        TestCase {
            input: "5ft 10abc 20xyz = m",
            expected_output: "\
            [ Unknown unit: abc ]\n\
            [ Unknown unit: xyz ]",
        },
    ];

    run_tests(test_cases);
}
