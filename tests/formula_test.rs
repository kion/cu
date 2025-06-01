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

    let output = child.wait_with_output().expect("Failed to wait on child");
    let stdout = String::from_utf8(output.stdout).expect("Failed to convert stdout to string");
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
fn test_formula_field_usage() {
    // this test specifically tests temperature conversions
    // which require using formulas instead of simple ratio conversions
    let test_cases = vec![
        TestCase {
            input: "32°F = °C",
            expected_output: "[TEMPERATURE] 32 °F = 0 °C",
            description: "Freezing point: Fahrenheit to Celsius",
        },
        TestCase {
            input: "32F = °C",
            expected_output: "[TEMPERATURE] 32 °F = 0 °C",
            description: "Freezing point: Fahrenheit to Celsius",
        },
        TestCase {
            input: "32 F = °C",
            expected_output: "[TEMPERATURE] 32 °F = 0 °C",
            description: "Freezing point: Fahrenheit to Celsius",
        },
        TestCase {
            input: "100°C = °F",
            expected_output: "[TEMPERATURE] 100 °C = 212 °F",
            description: "Boiling point: Celsius to Fahrenheit",
        },
        TestCase {
            input: "0K = °C",
            expected_output: "[TEMPERATURE] 0 K = -273.15 °C",
            description: "Absolute zero: Kelvin to Celsius",
        },
        TestCase {
            input: "273.15K = °C",
            expected_output: "[TEMPERATURE] 273.15 K = 0 °C",
            description: "Kelvin to Celsius",
        },
        TestCase {
            input: "36.6C = F",
            expected_output: "[TEMPERATURE] 36.6 °C = 97.88 °F",
            description: "Body temperature: Celsius to Fahrenheit",
        },
        TestCase {
            input: "36.6°C = °F",
            expected_output: "[TEMPERATURE] 36.6 °C = 97.88 °F",
            description: "Body temperature: Celsius to Fahrenheit",
        },
        TestCase {
            input: "97.88°F = °C",
            expected_output: "[TEMPERATURE] 97.88 °F = 36.6 °C",
            description: "Body temperature: Fahrenheit to Celsius",
        },
        TestCase {
            input: "36.6°C = K",
            expected_output: "[TEMPERATURE] 36.6 °C = 309.75 K",
            description: "Body temperature: Celsius to Kelvin",
        },
    ];

    run_tests(test_cases);
}
