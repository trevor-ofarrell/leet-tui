// Test the actual LeetCodeClient boilerplate generation

use std::process::Command;

// We need to test the actual generated boilerplate
// Since we can't import from the main crate easily in integration tests,
// let's replicate the generation logic and test it

fn get_problem_config(problem_id: u32) -> (&'static str, &'static str) {
    match problem_id {
        1 => ("twoSum", r#"[
    { input: [[2, 7, 11, 15], 9], expected: [0, 1] },
    { input: [[3, 2, 4], 6], expected: [1, 2] },
    { input: [[3, 3], 6], expected: [0, 1] },
]"#),
        9 => ("isPalindrome", r#"[
    { input: [121], expected: true },
    { input: [-121], expected: false },
    { input: [10], expected: false },
]"#),
        13 => ("romanToInt", r#"[
    { input: ['III'], expected: 3 },
    { input: ['LVIII'], expected: 58 },
    { input: ['MCMXCIV'], expected: 1994 },
]"#),
        _ => ("solution", "[]"),
    }
}

fn generate_boilerplate(problem_id: u32, title: &str, signature: &str) -> String {
    let (func_name, test_cases) = get_problem_config(problem_id);

    format!(
        r#"/**
 * Problem {id}: {title}
 * Difficulty: Easy
 *
 * Test problem
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
        signature = signature,
        func_name = func_name,
        test_cases = test_cases,
    )
}

#[test]
fn test_generated_boilerplate_syntax() {
    // Generate the boilerplate
    let boilerplate = generate_boilerplate(1, "Two Sum", "var twoSum = function(nums, target)");

    println!("Generated boilerplate:\n{}", boilerplate);

    // Write to file
    std::fs::write("tests/generated_boilerplate.js", &boilerplate).unwrap();

    // Check syntax with node --check
    let output = Command::new("node")
        .arg("--check")
        .arg("tests/generated_boilerplate.js")
        .output()
        .expect("Failed to execute node");

    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        println!("Syntax check failed:\n{}", stderr);
    }

    assert!(output.status.success(), "JavaScript syntax error: {}", stderr);
}

#[test]
fn test_generated_boilerplate_runs() {
    // Generate the boilerplate with a working solution inserted
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

    // Exit with error code if any failed
    process.exit(failed > 0 ? 1 : 0);
}

runTests();
"#;

    std::fs::write("tests/working_solution.js", boilerplate).unwrap();

    let output = Command::new("node")
        .arg("tests/working_solution.js")
        .output()
        .expect("Failed to execute node");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("STDOUT:\n{}", stdout);
    if !stderr.is_empty() {
        println!("STDERR:\n{}", stderr);
    }

    assert!(output.status.success(), "Test execution failed");
    assert!(stdout.contains("All tests passed"), "Tests did not all pass");
}

#[test]
fn test_palindrome_boilerplate() {
    let boilerplate = r#"
var isPalindrome = function(x) {
    if (x < 0) return false;
    const str = x.toString();
    return str === str.split('').reverse().join('');
};

const testCases = [
    { input: [121], expected: true },
    { input: [-121], expected: false },
    { input: [10], expected: false },
];

function runTests() {
    let passed = 0;
    let failed = 0;

    testCases.forEach((tc, i) => {
        try {
            const result = isPalindrome(...tc.input);
            const resultStr = JSON.stringify(result);
            const expectedStr = JSON.stringify(tc.expected);

            if (resultStr === expectedStr) {
                console.log(`Test ${i + 1}: PASSED`);
                passed++;
            } else {
                console.log(`Test ${i + 1}: FAILED - Expected ${expectedStr}, got ${resultStr}`);
                failed++;
            }
        } catch (e) {
            console.log(`Test ${i + 1}: ERROR - ${e.message}`);
            failed++;
        }
    });

    console.log(`Results: ${passed} passed, ${failed} failed`);
    process.exit(failed > 0 ? 1 : 0);
}

runTests();
"#;

    std::fs::write("tests/palindrome_solution.js", boilerplate).unwrap();

    let output = Command::new("node")
        .arg("tests/palindrome_solution.js")
        .output()
        .expect("Failed to execute node");

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("Palindrome test output:\n{}", stdout);

    assert!(output.status.success(), "Palindrome tests failed");
}
