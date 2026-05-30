use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::process::Stdio;

// To run a specific group of tests: LCR_GROUPS=3-Types cargo test
// To run multiple groups: LCR_GROUPS=3-Types,10-Loops cargo test

// With passes: LCR_GROUPS=2-Comments LCR_VERBOSE=1 cargo test -- --nocapture
// LCR_GROUPS=1-Structure,2-Comments,3-Types,4-Output,5-Input,6-Variables,7-Operators LCR_VERBOSE=1 cargo test -- --nocapture

struct TestResult {
    path: String,
    case: String,
    passed: bool,
    message: Option<String>,
}

#[derive(Debug)]
struct TestConfig {
    filters: Vec<String>,
    verbose: bool,
}

fn parse_config() -> TestConfig {
    let filters = std::env::var("LCR_GROUPS")
        .ok()
        .map(|s| s.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_default();

    let verbose = std::env::var("LCR_VERBOSE").is_ok();

    TestConfig { filters, verbose }
}

fn find_test_dirs(dir: &Path, out: &mut Vec<PathBuf>) {
    if dir.join("test.lol").exists() {
        out.push(dir.to_path_buf());
        return;
    }

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                find_test_dirs(&path, out);
            }
        }
    }
}

#[test]
fn run_lci_tests() {
    let config = parse_config();

    let mut test_dirs = Vec::new();
    find_test_dirs(Path::new("tests"), &mut test_dirs);

    let mut results = Vec::new();
    let mut executed = 0;

    for dir in test_dirs {
        let path_str = dir.to_string_lossy().to_string();
        let case = dir.file_name().unwrap().to_string_lossy().to_string();

        if !config.filters.is_empty() && !config.filters.iter().any(|f| path_str.contains(f)) {
            continue;
        }

        executed += 1;

        let program = dir.join("test.lol");
        let expected_out = dir.join("test.out");
        let expected_err = dir.join("test.err");
        let input_file = dir.join("test.in");

        let output = (|| -> std::io::Result<std::process::Output> {
            let mut child = Command::new(env!("CARGO_BIN_EXE_lolcoders"))
                .arg(&program)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()?;

            if input_file.exists() {
                let input = fs::read_to_string(&input_file)?;

                if let Some(stdin) = child.stdin.as_mut() {
                    stdin.write_all(input.as_bytes())?;
                }
            }

            child.wait_with_output()
        })();

        let mut result = TestResult {
            path: path_str.clone(),
            case,
            passed: false,
            message: None,
        };

        match output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);

                if expected_out.exists() {
                    let expected = fs::read_to_string(expected_out).unwrap();

                    if !output.status.success() {
                        result.message =
                            Some(format!("Expected success but failed.\nstderr:\n{}", stderr));
                    } else if stdout.trim() != expected.trim() {
                        result.message = Some(format!(
                            "Output mismatch.\nExpected:\n{}\nGot:\n{}",
                            expected, stdout
                        ));
                    } else {
                        result.passed = true;
                    }
                } else if expected_err.exists() {
                    let expected = fs::read_to_string(expected_err).unwrap();

                    if output.status.success() {
                        result.message = Some("Expected failure but succeeded.".into());
                    } else if !stderr.contains(expected.trim()) {
                        result.message = Some(format!(
                            "Error mismatch.\nExpected:\n{}\nGot:\n{}",
                            expected, stderr
                        ));
                    } else {
                        result.passed = true;
                    }
                } else {
                    result.message = Some("Missing test.out or test.err".into());
                }
            }
            Err(e) => {
                result.message = Some(format!("Failed to run binary: {}", e));
            }
        }

        if result.passed && config.verbose {
            println!("PASS: {}", result.path);
        }

        results.push(result);
    }

    if executed == 0 {
        eprintln!("No tests matched given filters");
        return;
    }

    let failures: Vec<_> = results.iter().filter(|r| !r.passed).collect();

    if !failures.is_empty() {
        eprintln!("\n=== FAILURES ===\n");

        for f in &failures {
            eprintln!("{}", f.path);
            eprintln!("{}", f.case);
            if let Some(msg) = &f.message {
                eprintln!("{}", msg);
            }
            eprintln!("------------------------");
        }

        eprintln!(
            "\n{} failed; {} passed",
            failures.len(),
            results.len() - failures.len()
        );

        panic!("Some tests failed");
    } else {
        println!("All {} tests passed!", results.len());
    }
}
