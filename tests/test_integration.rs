// Integration test - simulates what the app does

use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Simulates the actual boilerplate generation from leetcode.rs
fn generate_actual_boilerplate() -> String {
    let problem_id = 1u32;
    let title = "Two Sum";
    let difficulty = "Easy";
    let description = "Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.";
    let signature = "var twoSum = function(nums, target)";
    let func_name = "twoSum";
    let test_cases = r#"[
    { input: [[2, 7, 11, 15], 9], expected: [0, 1] },
    { input: [[3, 2, 4], 6], expected: [1, 2] },
    { input: [[3, 3], 6], expected: [0, 1] },
]"#;

    format!(
        r#"/**
 * Problem {id}: {title}
 * Difficulty: {difficulty}
 *
 * {description}
 */

{signature} {{
    // Your solution here
}};

// ============== TEST RUNNER (do not modify below) ==============
const testCases = {test_cases};

function runTests() {{
    console.log('\n' + '='.repeat(50));
    console.log('Running tests for: {title}');
    console.log('='.repeat(50) + '\n');

    let passed = 0;
    let failed = 0;

    testCases.forEach((tc, i) => {{
        try {{
            const result = {func_name}(...tc.input);
            const resultStr = JSON.stringify(result);
            const expectedStr = JSON.stringify(tc.expected);

            // Handle array comparison (order might not matter for some problems)
            const isEqual = resultStr === expectedStr;

            if (isEqual) {{
                console.log(`✓ Test ${{i + 1}}: PASSED`);
                passed++;
            }} else {{
                console.log(`✗ Test ${{i + 1}}: FAILED`);
                console.log(`  Input:    ${{JSON.stringify(tc.input)}}`);
                console.log(`  Expected: ${{expectedStr}}`);
                console.log(`  Got:      ${{resultStr}}`);
                failed++;
            }}
        }} catch (e) {{
            console.log(`✗ Test ${{i + 1}}: ERROR`);
            console.log(`  ${{e.message}}`);
            failed++;
        }}
    }});

    console.log('\n' + '-'.repeat(50));
    console.log(`Results: ${{passed}} passed, ${{failed}} failed out of ${{testCases.length}} tests`);
    console.log('-'.repeat(50) + '\n');

    if (failed === 0) {{
        console.log('🎉 All tests passed!');
    }}
}}

runTests();
"#,
        id = problem_id,
        title = title,
        difficulty = difficulty,
        description = description,
        signature = signature,
        func_name = func_name,
        test_cases = test_cases,
    )
}

#[test]
fn test_full_workflow_empty_solution() {
    // Create solutions directory like the app does
    fs::create_dir_all("solutions").unwrap();

    // Generate boilerplate
    let boilerplate = generate_actual_boilerplate();

    // Write to file like the app does
    let solution_file = PathBuf::from("solutions/test_problem_1.js");
    fs::write(&solution_file, &boilerplate).unwrap();

    // Verify file was written correctly
    let read_content = fs::read_to_string(&solution_file).unwrap();
    assert_eq!(read_content, boilerplate, "File content doesn't match");

    // Run tests like the app does
    let output = Command::new("node")
        .arg(&solution_file)
        .output()
        .expect("Failed to execute node");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("=== Empty Solution Test ===");
    println!("STDOUT:\n{}", stdout);
    if !stderr.is_empty() {
        println!("STDERR:\n{}", stderr);
    }

    // With empty solution, tests should FAIL (undefined returned)
    assert!(stdout.contains("FAILED") || stdout.contains("ERROR"),
            "Empty solution should fail tests, got: {}", stdout);

    // Clean up
    fs::remove_file(&solution_file).ok();
}

#[test]
fn test_full_workflow_with_solution() {
    fs::create_dir_all("solutions").unwrap();

    // Generate boilerplate with a working solution
    let boilerplate = r#"/**
 * Problem 1: Two Sum
 * Difficulty: Easy
 */

var twoSum = function(nums, target) {
    const map = new Map();
    for (let i = 0; i < nums.length; i++) {
        const complement = target - nums[i];
        if (map.has(complement)) {
            return [map.get(complement), i];
        }
        map.set(nums[i], i);
    }
    return [];
};

// ============== TEST RUNNER (do not modify below) ==============
const testCases = [
    { input: [[2, 7, 11, 15], 9], expected: [0, 1] },
    { input: [[3, 2, 4], 6], expected: [1, 2] },
    { input: [[3, 3], 6], expected: [0, 1] },
];

function runTests() {
    console.log('\n' + '='.repeat(50));
    console.log('Running tests for: Two Sum');
    console.log('='.repeat(50) + '\n');

    let passed = 0;
    let failed = 0;

    testCases.forEach((tc, i) => {
        try {
            const result = twoSum(...tc.input);
            const resultStr = JSON.stringify(result);
            const expectedStr = JSON.stringify(tc.expected);

            const isEqual = resultStr === expectedStr;

            if (isEqual) {
                console.log(`✓ Test ${i + 1}: PASSED`);
                passed++;
            } else {
                console.log(`✗ Test ${i + 1}: FAILED`);
                console.log(`  Input:    ${JSON.stringify(tc.input)}`);
                console.log(`  Expected: ${expectedStr}`);
                console.log(`  Got:      ${resultStr}`);
                failed++;
            }
        } catch (e) {
            console.log(`✗ Test ${i + 1}: ERROR`);
            console.log(`  ${e.message}`);
            failed++;
        }
    });

    console.log('\n' + '-'.repeat(50));
    console.log(`Results: ${passed} passed, ${failed} failed out of ${testCases.length} tests`);
    console.log('-'.repeat(50) + '\n');

    if (failed === 0) {
        console.log('🎉 All tests passed!');
    }
}

runTests();
"#;

    let solution_file = PathBuf::from("solutions/test_problem_1_solved.js");
    fs::write(&solution_file, boilerplate).unwrap();

    let output = Command::new("node")
        .arg(&solution_file)
        .output()
        .expect("Failed to execute node");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("=== Working Solution Test ===");
    println!("STDOUT:\n{}", stdout);
    if !stderr.is_empty() {
        println!("STDERR:\n{}", stderr);
    }

    // With working solution, all tests should pass
    assert!(stdout.contains("All tests passed"),
            "Working solution should pass all tests, got: {}", stdout);

    // Clean up
    fs::remove_file(&solution_file).ok();
}

#[test]
fn test_run_tests_function_like_app() {
    // This simulates the App::run_tests function
    fn run_tests(solution_file: &PathBuf) -> Result<String, std::io::Error> {
        let output = Command::new("node")
            .arg(solution_file)
            .output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !stderr.is_empty() {
            Ok(format!("{}\n\nErrors:\n{}", stdout, stderr))
        } else {
            Ok(stdout.to_string())
        }
    }

    fs::create_dir_all("solutions").unwrap();

    let boilerplate = r#"
var twoSum = function(nums, target) {
    const map = new Map();
    for (let i = 0; i < nums.length; i++) {
        if (map.has(target - nums[i])) {
            return [map.get(target - nums[i]), i];
        }
        map.set(nums[i], i);
    }
    return [];
};

const testCases = [
    { input: [[2, 7, 11, 15], 9], expected: [0, 1] },
];

testCases.forEach((tc, i) => {
    const result = twoSum(...tc.input);
    const pass = JSON.stringify(result) === JSON.stringify(tc.expected);
    console.log(`Test ${i + 1}: ${pass ? 'PASSED' : 'FAILED'}`);
});
"#;

    let solution_file = PathBuf::from("solutions/test_app_run.js");
    fs::write(&solution_file, boilerplate).unwrap();

    let result = run_tests(&solution_file).unwrap();
    println!("App run_tests output:\n{}", result);

    assert!(result.contains("PASSED"), "Test should pass");

    fs::remove_file(&solution_file).ok();
}

#[test]
fn test_node_is_available() {
    let output = Command::new("node")
        .arg("--version")
        .output()
        .expect("Node.js is not installed or not in PATH");

    let version = String::from_utf8_lossy(&output.stdout);
    println!("Node.js version: {}", version);

    assert!(output.status.success(), "Node.js should be available");
}
