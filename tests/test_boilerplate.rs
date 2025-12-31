// Test file to check boilerplate generation
use std::process::Command;

#[test]
fn test_boilerplate_generates_valid_javascript() {
    // Create a sample boilerplate for problem 1 (Two Sum)
    let boilerplate = r#"/**
 * Problem 1: Two Sum
 * Difficulty: Easy
 *
 * Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
 */

var twoSum = function(nums, target) {
    // Simple working solution for testing
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

    // Write to a temp file
    std::fs::write("tests/test_solution.js", boilerplate).unwrap();

    // Run with node
    let output = Command::new("node")
        .arg("tests/test_solution.js")
        .output()
        .expect("Failed to execute node");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("STDOUT:\n{}", stdout);
    println!("STDERR:\n{}", stderr);

    // Check that it ran successfully
    assert!(output.status.success(), "Node execution failed: {}", stderr);
    assert!(stdout.contains("PASSED"), "No PASSED tests found in output");
    assert!(stdout.contains("All tests passed"), "Not all tests passed");
}

#[test]
fn test_empty_solution_shows_errors() {
    // Test that an empty solution produces failures
    let boilerplate = r#"
var twoSum = function(nums, target) {
    // Empty - should fail
};

const testCases = [
    { input: [[2, 7, 11, 15], 9], expected: [0, 1] },
];

function runTests() {
    let passed = 0;
    let failed = 0;

    testCases.forEach((tc, i) => {
        try {
            const result = twoSum(...tc.input);
            const resultStr = JSON.stringify(result);
            const expectedStr = JSON.stringify(tc.expected);

            if (resultStr === expectedStr) {
                console.log(`PASSED`);
                passed++;
            } else {
                console.log(`FAILED`);
                failed++;
            }
        } catch (e) {
            console.log(`ERROR: ${e.message}`);
            failed++;
        }
    });

    console.log(`Results: ${passed} passed, ${failed} failed`);
}

runTests();
"#;

    std::fs::write("tests/test_empty.js", boilerplate).unwrap();

    let output = Command::new("node")
        .arg("tests/test_empty.js")
        .output()
        .expect("Failed to execute node");

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("Empty solution output:\n{}", stdout);

    // Should have failures since solution is empty
    assert!(stdout.contains("FAILED") || stdout.contains("failed"),
            "Empty solution should fail tests");
}
